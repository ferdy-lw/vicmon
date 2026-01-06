use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    sync::{LazyLock, RwLock},
};

use esp_idf_svc::{
    bt::BdAddr,
    nvs::{EspKeyValueStorage, EspNvs, EspNvsPartition, NvsDefault},
};
use log::{error, info};
use postcard::{from_bytes, to_vec};
use serde::{Deserialize, Serialize, de::Visitor};

use crate::ui::ui;

pub type Mac = [u8; 6];
pub type Key = [u8; 16];

//------------
// Device Type
//------------
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum DeviceType {
    Inverter,
    Mppt,
    Bmv,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inverter => write!(f, "inv"),
            Self::Mppt => write!(f, "mppt"),
            Self::Bmv => write!(f, "bmv"),
        }
    }
}

//---------------
// Device BdAddr
// Need to wrap a BdAddr so it can be used as a hashmap key, for lookups from a scanned bda to a device key and pin
//---------------
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct BdAddrKey(BdAddr);

impl Display for BdAddrKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for BdAddrKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.raw().hash(state);
    }
}

impl Serialize for BdAddrKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.0.raw())
    }
}

struct BdAddrKeyVisitor;

impl<'de> Visitor<'de> for BdAddrKeyVisitor {
    type Value = BdAddrKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a byte array of length 6")
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v_len = v.len();
        let bytes: [u8; 6] = v.try_into().map_err(|_| {
            serde::de::Error::invalid_length(v_len, &format!("should be 6").as_str())
        })?;

        Ok(BdAddrKey(BdAddr::from_bytes(bytes)))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let bytes: [u8; 6] = v.try_into().map_err(|e| {
            serde::de::Error::invalid_length(v.len(), &format!("should be 6, {e}").as_str())
        })?;

        Ok(BdAddrKey(BdAddr::from_bytes(bytes)))
    }
}

impl<'de> Deserialize<'de> for BdAddrKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_byte_buf(BdAddrKeyVisitor)
    }
}

//-------
// Device
//-------
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Device {
    device_type: DeviceType,
    addr: BdAddrKey,
    key: Key,
    pin: Option<u32>,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device {{ device: {} addr: {}, key: {}, pin: {:?} }}",
            self.device_type,
            self.addr,
            hex::encode_upper(&self.key),
            self.pin
        )
    }
}

impl Device {
    pub fn new(device: DeviceType, addr: BdAddr, key: Key, pin: Option<u32>) -> Self {
        Self {
            device_type: device,
            addr: BdAddrKey(addr),
            key,
            pin,
        }
    }

    pub fn _is_valid(&self) -> bool {
        self.addr.0 != EMPTY_ADDR
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn addr(&self) -> &BdAddr {
        &self.addr.0
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn pin(&self) -> Option<u32> {
        self.pin
    }
}

//---------
// Devices
//---------
// The set of devices to get advertisement data from
pub struct Devices {
    devices: [Device; 3],
    addr_to_device: HashMap<BdAddrKey, DeviceType>,
    nvs: Option<EspKeyValueStorage<NvsDefault>>,
}

impl Devices {
    pub fn device(&self, device: DeviceType) -> &Device {
        &self.devices[device as usize]
    }

    pub fn device_addr(&self, device: DeviceType) -> &BdAddr {
        self.device(device).addr()
    }

    pub fn device_key(&self, device: DeviceType) -> &Key {
        self.device(device).key()
    }

    pub fn device_pin(&self, device: DeviceType) -> Option<u32> {
        self.device(device).pin()
    }

    pub fn get_key(&self, addr: BdAddr) -> Option<&Key> {
        self.addr_to_device
            .get(&BdAddrKey(addr))
            .map(|device_type| self.device_key(*device_type))
    }

    pub fn get_pin(&self, addr: BdAddr) -> Option<u32> {
        self.addr_to_device
            .get(&BdAddrKey(addr))
            .and_then(|device_type| self.device_pin(*device_type))
    }

    pub fn add_device(&mut self, device: Device) -> String {
        let resp;

        // Only add a device if the addr is not 'empty'
        if device.addr.0 == EMPTY_ADDR {
            resp = "has 0 mac, ignoring";
            info!("Not adding device with invalid addr, {device}");
        } else {
            let existing = &self.devices[device.device_type as usize];
            // If device is the same, ignore
            if existing == &device {
                resp = "already exists, not updated";
                info!("{device} already exists")
            } else {
                // If the addr is different update the map
                let diff_addr = &existing.addr != &device.addr;
                if diff_addr {
                    self.addr_to_device.remove(&existing.addr);
                }
                self.addr_to_device.insert(device.addr, device.device_type);

                resp = if self.store_device(&device) {
                    if !diff_addr { "updated" } else { "added" }
                } else {
                    "not saved to storage"
                };

                ui::config_device(&device);

                self.devices[device.device_type as usize] = device;
                info!("Added {device}");
            }
        }

        resp.to_owned()
    }

    pub fn num_devices(&self) -> usize {
        self.addr_to_device.len()
    }

    // If this fails, don't panic on error because we can probably recover by saving a new
    // configuration
    pub fn load_from_nvs(&mut self, nvs: EspNvsPartition<NvsDefault>) {
        let vicmon_namespace = "vicmon_ns";

        match EspNvs::new(nvs, vicmon_namespace, true) {
            Ok(nvs) => {
                info!("Got namespace {vicmon_namespace} from default partition");

                let storage = EspKeyValueStorage::new(nvs);

                let buf: &mut [u8; 100] = &mut [0; 100];

                for key in [DeviceType::Inverter, DeviceType::Mppt, DeviceType::Bmv] {
                    if let Some(device) = self.load_device(&storage, key, buf) {
                        self.addr_to_device.insert(device.addr, device.device_type);
                        ui::config_device(&device);
                        self.devices[key as usize] = device;
                        info!("Loaded device {} {}", device.device_type, device.addr)
                    }

                    buf.fill(0);
                }

                self.nvs = Some(storage);
            }
            Err(e) => error!("Could't get namespace {vicmon_namespace} {e:?}"),
        };
    }

    fn load_device(
        &self,
        storage: &EspKeyValueStorage<NvsDefault>,
        key: DeviceType,
        buf: &mut [u8; 100],
    ) -> Option<Device> {
        let key = &key.to_string();

        match storage.get_raw(key, buf) {
            Ok(bytes) => {
                if let Some(bytes) = bytes {
                    match from_bytes::<Device>(bytes) {
                        Ok(device) => Some(device),
                        Err(e) => {
                            error!("Failed to deserialize {key} {e:?}");
                            None
                        }
                    }
                } else {
                    info!("Device {key} not found in nvs");
                    None
                }
            }
            Err(e) => {
                error!("Failed to get key {key} bytes from nvs {e:?}");
                None
            }
        }
    }

    fn store_device(&self, device: &Device) -> bool {
        if let Some(storage) = self.nvs.as_ref() {
            let key = device.device_type.to_string();

            match to_vec::<Device, 100>(&device) {
                Ok(buf) => match storage.set_raw(&key, &buf) {
                    Ok(_) => {
                        info!("Added '{key}' to nvs {device}");
                        true
                    }
                    Err(e) => {
                        error!("Failed to add '{key}' to nvs {device} {e:?}");
                        false
                    }
                },
                Err(e) => {
                    error!("Failed to serialize '{key}' {device} {e:?}");
                    false
                }
            }
        } else {
            error!("nvs storge not set");
            false
        }
    }
}

//--------
// DEVICES
//--------
const EMPTY_ADDR: BdAddr = BdAddr::from_bytes([0; 6]);

const EMPTY_DEVICE_INVERTER: Device = Device {
    device_type: DeviceType::Inverter,
    addr: BdAddrKey(EMPTY_ADDR),
    key: [0; 16],
    pin: None,
};

const EMPTY_DEVICE_MPPT: Device = Device {
    device_type: DeviceType::Mppt,
    addr: BdAddrKey(EMPTY_ADDR),
    key: [0; 16],
    pin: None,
};

const EMPTY_DEVICE_BMV: Device = Device {
    device_type: DeviceType::Bmv,
    addr: BdAddrKey(EMPTY_ADDR),
    key: [0; 16],
    pin: None,
};

/// The set of system devices, some may be not configured, i.e. EMPTY
pub static DEVICES: LazyLock<RwLock<Devices>> = LazyLock::new(|| {
    RwLock::new(Devices {
        devices: [EMPTY_DEVICE_INVERTER, EMPTY_DEVICE_MPPT, EMPTY_DEVICE_BMV],
        addr_to_device: HashMap::new(),
        nvs: None,
    })
});
