use std::collections::HashMap;
use std::ffi::CString;
use std::hash::Hash;
use std::slice;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self};
use std::time::{Duration, Instant};

use esp_idf_svc::bt::ble::gatt::client::{
    CharacteristicElement, ConnectionId, EspGattc, GattAuthReq, GattCreateConnParams,
    GattWriteType, GattcEvent, ServiceSource,
};
use esp_idf_svc::bt::ble::gatt::{GattInterface, GattStatus, Handle};
use esp_idf_svc::sys::*;

use anyhow::Result;

use esp_idf_svc::bt::ble::gap::{
    self, BleGapEvent, EspBleGap, GapSearchEvent, GapSearchResult, ScanParams, ScanType,
    SecurityConfiguration,
};
use esp_idf_svc::bt::{BdAddr, Ble, BtDriver, BtStatus, BtUuid};
use esp_idf_svc::sys::EspError;
use log::{debug, error, info, warn};
use victron_ble::{DeviceState, ErrorState, Mode};

use crate::devices::*;
use crate::ui::{self, ON_DURATION};

type VicBtDriver = BtDriver<'static, Ble>;
type VicEspBleGap = Arc<EspBleGap<'static, Ble, Arc<VicBtDriver>>>;
type VicEspGattc = Arc<EspGattc<'static, Ble, Arc<VicBtDriver>>>;

const APP_ID: u16 = 0;

const VICTRON: [u8; 2] = [0xE1, 0x02];

// inverter service UUID
const SERVICE_UUID: BtUuid = BtUuid::uuid128(0x306b0001b081403783dce59fcc3cdfd0);
// Write control characteristic UUID
const INV_CTRL_CHARACTERISITIC_UUID: BtUuid = BtUuid::uuid128(0x306b0003b081403783dce59fcc3cdfd0);

const TURN_ON_INVERTER: [u8; 8] = [0x06, 0x03, 0x82, 0x19, 0x02, 0x00, 0x41, 0x03];
// const TURN_ON_INVERTER: [u8; 8] = [0x03, 0x41, 0x00, 0x02, 0x19, 0x82, 0x03, 0x06];
const TURN_OFF_INVERTER: [u8; 8] = [0x06, 0x03, 0x82, 0x19, 0x02, 0x00, 0x41, 0x04];

static DEBOUNCE_INV_SWITCH: RwLock<Option<Instant>> = RwLock::new(None);

#[derive(Eq, PartialEq)]
struct BdAddrKey(BdAddr);

impl Hash for BdAddrKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.raw().hash(state);
    }
}

struct ScanData {
    bda: BdAddr,
    ble_adv: Option<Vec<u8>>,
    data_len: usize,
}

#[derive(Default)]
struct State {
    scanning: bool,
    gattc_if: Option<GattInterface>,
    conn_id: Option<ConnectionId>,
    remote_addr: Option<BdAddr>,
    connect: bool,
    service_start_end_handle: Option<(Handle, Handle)>,
}

#[derive(Clone)]
pub struct Client {
    pub gap: VicEspBleGap,
    pub gattc: VicEspGattc,
    state: Arc<Mutex<State>>,
    tx: SyncSender<ScanData>,
}

impl Client {
    pub fn new(gap: VicEspBleGap, gattc: VicEspGattc) -> Self {
        let bda_to_key = HashMap::from([
            (BdAddrKey(INVERTER_ADDR), INVERTER_KEY),
            (BdAddrKey(MPPT_ADDR), MPPT_KEY),
            // DC not supported in decryption
            // (BdAddrKey(DC_DC_ADDR), DC_DC_KEY),
            (BdAddrKey(BMV_ADDR), BMV_KEY),
        ]);

        let (tx, rx) = sync_channel(2);

        let _ = thread::Builder::new()
            .stack_size(5096)
            .spawn(move || Client::decode_advertisement(bda_to_key, rx));
        info!("Start Adv Decoder");

        let security_conf = SecurityConfiguration {
            auth_req_mode: gap::AuthenticationRequest::MitmBonding,
            io_capabilities: gap::IOCapabilities::KeyboardOnly,
            static_passkey: Some(123456),
            ..Default::default()
        };

        gap.set_security_conf(&security_conf)
            .expect("Failed to set gap security");

        info!("BLE Gap security configuration");

        Self {
            gap,
            gattc,
            state: Arc::new(Mutex::new(Default::default())),
            tx,
        }
    }

    pub fn subscribe_gap(&self) -> Result<(), EspError> {
        let gap_client = self.clone();

        self.gap.subscribe(move |event| {
            Self::check_esp_status(gap_client.on_gap_event(event));
        })
    }

    pub fn subscribe_gattc(&self) -> Result<(), EspError> {
        let gattc_client = self.clone();

        self.gattc.subscribe(move |(gatt_if, event)| {
            Self::check_esp_status(gattc_client.on_gattc_event(gatt_if, event))
        })
    }

    pub fn start(&self) -> Result<(), EspError> {
        self.gattc.register_app(APP_ID)
    }

    /// The main event handler for the GAP events
    fn on_gap_event(&self, event: BleGapEvent) -> Result<(), EspError> {
        // info!("Got gap event: {:?}", mem::discriminant(&event));

        match event {
            BleGapEvent::PasskeyRequest => {
                let state = self.state.lock().unwrap();
                if let Some(addr) = state.remote_addr {
                    let passkey = 123456;
                    esp!(unsafe {
                        esp_ble_passkey_reply(&addr.raw() as *const _ as *mut _, true, passkey)
                    })?;
                }
            }
            BleGapEvent::ScanParameterConfigured(status) => {
                self.check_bt_status(status)?;

                info!("Scan params configured");
                self.gap.start_scanning(ON_DURATION.as_secs() as _)?;
            }
            BleGapEvent::ScanStarted(status) => {
                self.check_bt_status(status)?;

                info!("Started ble scanning...");
                self.state.lock().unwrap().scanning = true;
            }
            BleGapEvent::ScanStopped(status) => {
                self.check_bt_status(status)?;

                info!("Stopped ble scanning...");
                self.state.lock().unwrap().scanning = false;
            }
            BleGapEvent::ScanResult(search_evt) => match search_evt {
                GapSearchEvent::InquiryResult(GapSearchResult {
                    bda,
                    ble_adv,
                    adv_data_len,
                    scan_rsp_len,
                    ble_addr_type,
                    ..
                }) => {
                    let ble_adv = ble_adv.map(|d| d.to_owned());
                    let scan_data = ScanData {
                        bda: bda.into(),
                        ble_adv: ble_adv,
                        data_len: adv_data_len as usize + scan_rsp_len as usize,
                    };

                    if ui::INVERTER_PREV.load(std::sync::atomic::Ordering::Relaxed)
                        != ui::INVERTER_ON.load(std::sync::atomic::Ordering::Relaxed)
                        && bda == INVERTER_ADDR
                    {
                        info!("Inverter found, changing state");

                        let mut state = self.state.lock().unwrap();

                        if !state.connect {
                            state.connect = true;
                            info!("Connect to remote {bda}");
                            self.gap.stop_scanning()?;

                            let conn_params = GattCreateConnParams::new(bda, ble_addr_type);

                            self.gattc.enh_open(state.gattc_if.unwrap(), &conn_params)?;
                        }
                    } else {
                        let _ = self.tx.send(scan_data);
                    }
                }
                GapSearchEvent::InquiryComplete(num) => {
                    info!("Scan timeout, completed. Found {num}");
                    self.state.lock().unwrap().scanning = false;

                    if ON_DURATION.recent_touch() {
                        // if let Some(last_touch) = *LAST_TOUCH.read().unwrap() {
                        //     if Instant::now().duration_since(last_touch) < ON_DURATION {
                        if self.start_scanning()? {
                            info!("Recently touched, start scan after timeout on");
                        }
                    }
                    // }
                }
                _ => {
                    info!("Got unsupported scan search {search_evt:?}")
                }
            },

            BleGapEvent::Other {
                raw_event,
                raw_data,
            } => {
                info!("Raw event: {:?}, data: {:?}", raw_event, raw_data);
                if raw_event == esp_gap_ble_cb_event_t_ESP_GAP_BLE_PERIODIC_ADV_REPORT_EVT {
                    unsafe {
                        info!("Periodic adv: {:?}", raw_data.0.period_adv_report.params);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn decode_advertisement(bda_to_key: HashMap<BdAddrKey, [u8; 16]>, rx: Receiver<ScanData>) {
        fn get_manufacturer_data(ble_adv: &[u8], adv_data_len: usize) -> Option<&[u8]> {
            if adv_data_len > 0 {
                let mut length: u8 = 0;
                unsafe {
                    let man_data = esp_ble_resolve_adv_data_by_type(
                        ble_adv as *const _ as *mut _,
                        adv_data_len as _,
                        esp_ble_adv_data_type_ESP_BLE_AD_MANUFACTURER_SPECIFIC_TYPE,
                        &mut length as *mut _,
                    );

                    if length > 0 {
                        Some(slice::from_raw_parts(man_data, length as usize))
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        }

        loop {
            if let Ok(result) = rx.recv() {
                let ble_adv = result.ble_adv.unwrap();
                let manufacturer_data = get_manufacturer_data(ble_adv.as_slice(), result.data_len);

                if let Some(man_data) = manufacturer_data {
                    if man_data[0..2] == VICTRON && man_data[2] == 0x10 {
                        if let Some(key) = bda_to_key.get(&BdAddrKey(result.bda)) {
                            match victron_ble::parse_manufacturer_data(&man_data[2..], key) {
                                Ok(DeviceState::SolarCharger(device_state)) => {
                                    debug!("Read mppt: {device_state:?} ");

                                    let lock = ui::SOLAR_WATTS.write();
                                    *(lock.unwrap()) =
                                        device_state.pv_power_w.unwrap_or(0_f32) as i32;

                                    let lock = ui::SOLAR_YIELD.write();
                                    *(lock.unwrap()) =
                                        (device_state.yield_today_kwh.unwrap_or(0_f32) * 1_000.0)
                                            as i32;

                                    if device_state.mode != Mode::NotApplicable {
                                        ui::SOLAR_MODE.write().unwrap().replace(
                                            CString::new(format!("{}", device_state.mode)).unwrap(),
                                        );
                                    }

                                    let cur_error = ui::SOLAR_ERROR.read().unwrap().is_some();
                                    if device_state.error_state != ErrorState::NoError
                                        && device_state.error_state != ErrorState::NotApplicable
                                    {
                                        ui::SOLAR_ERROR.write().unwrap().replace(
                                            CString::new(format!("{}", device_state.error_state))
                                                .unwrap(),
                                        );
                                    } else if cur_error {
                                        ui::SOLAR_ERROR.write().unwrap().take();
                                    }
                                }
                                Ok(DeviceState::VeBus(device_state)) => {
                                    debug!("Read VeBus: {device_state:?} ");
                                    // Give it a few seconds after switching the inverter before reading state again
                                    let last_switch_lock = DEBOUNCE_INV_SWITCH.read().unwrap();
                                    let last_switch = last_switch_lock.as_ref();

                                    if last_switch.is_none_or(|when| {
                                        when.elapsed().gt(&Duration::from_secs(3))
                                    }) {
                                        match device_state.mode {
                                            Mode::Off => {
                                                ui::INVERTER_ON.store(
                                                    false,
                                                    std::sync::atomic::Ordering::Relaxed,
                                                );
                                                ui::INVERTER_PREV.store(
                                                    false,
                                                    std::sync::atomic::Ordering::Relaxed,
                                                );
                                            }
                                            Mode::Inverting => {
                                                ui::INVERTER_ON.store(
                                                    true,
                                                    std::sync::atomic::Ordering::Relaxed,
                                                );
                                                ui::INVERTER_PREV.store(
                                                    true,
                                                    std::sync::atomic::Ordering::Relaxed,
                                                );
                                            }
                                            mode => {
                                                ui::INV_MODE.write().unwrap().replace(
                                                    CString::new(format!("{mode}")).unwrap(),
                                                );
                                            }
                                        }

                                        if last_switch.is_some() {
                                            drop(last_switch_lock);
                                            DEBOUNCE_INV_SWITCH.write().unwrap().take();
                                        }
                                    }

                                    let lock = ui::BATT_TEMP.write();
                                    *(lock.unwrap()) =
                                        device_state.battery_temperature_c.unwrap_or(0_f32) as i32;

                                    let lock = ui::AC_WATTS.write();
                                    *(lock.unwrap()) =
                                        device_state.ac_out_power_w.unwrap_or(0_f32) as i32;

                                    let cur_error = ui::INV_ERROR.read().unwrap().is_some();
                                    if device_state.error != ErrorState::NoError
                                        && device_state.error != ErrorState::NotApplicable
                                    {
                                        ui::INV_ERROR.write().unwrap().replace(
                                            CString::new(format!("{}", device_state.error))
                                                .unwrap(),
                                        );
                                    } else if cur_error {
                                        ui::INV_ERROR.write().unwrap().take();
                                    }
                                }
                                Ok(DeviceState::BatteryMonitor(device_state)) => {
                                    debug!("Read Batt: {device_state:?} ");

                                    let lock = ui::BATT_SOC.write();
                                    *(lock.unwrap()) = digits(
                                        device_state.state_of_charge_pct.unwrap_or(0_f32),
                                        false,
                                    );

                                    let lock = ui::BATT_VOLT.write();
                                    *(lock.unwrap()) = digits(
                                        device_state.battery_voltage_v.unwrap_or(0_f32),
                                        true,
                                    );

                                    let lock = ui::BATT_AMP.write();
                                    *(lock.unwrap()) = digits(
                                        device_state.battery_current_a.unwrap_or(0_f32),
                                        true,
                                    );

                                    let cur_alarm = ui::BATT_ALARM.read().unwrap().is_some();
                                    if !device_state.alarm_reason.is_empty() {
                                        ui::BATT_ALARM.write().unwrap().replace(
                                            CString::new(format!(
                                                "{:?}",
                                                device_state.alarm_reason
                                            ))
                                            .unwrap(),
                                        );
                                    } else if cur_alarm {
                                        ui::BATT_ALARM.write().unwrap().take();
                                    }
                                }
                                Ok(_device) => {
                                    // info!("{} Unknown device state {device:?}", result.bda);
                                }
                                Err(e) => {
                                    error!("Failed to read inverter data, {e}");
                                    error!("inverter man data, {man_data:?}")
                                } // _ => {} // Err(e) => error!("Failed to read inverter data, {e}"),
                            }
                        }
                    } else {
                        // info!("got manufacturing data: {man_data:?}")
                    }
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// The main event handler for the GATTC events
    fn on_gattc_event(&self, gattc_if: GattInterface, event: GattcEvent) -> Result<(), EspError> {
        // info!("Got gattc event: {:?}", mem::discriminant(&event));
        info!("Got gattc event: {:?}", event);

        match event {
            GattcEvent::ClientRegistered { status, app_id } => {
                self.check_gatt_status(status)?;
                if APP_ID == app_id {
                    self.state.lock().unwrap().gattc_if = Some(gattc_if);

                    let scan_params = ScanParams {
                        scan_type: ScanType::Passive,
                        // scan_interval: 0x50,
                        // scan_window: 0x30,
                        scan_interval: 0x96,
                        scan_window: 0x96,
                        ..Default::default()
                    };

                    // This will start scanning in the scan param set gap event
                    self.gap.set_scan_params(&scan_params)?;
                }
            }
            GattcEvent::Connected { conn_id, addr, .. } => {
                info!("Connected");

                let mut state = self.state.lock().unwrap();

                state.conn_id = Some(conn_id);
                state.remote_addr = Some(addr);

                // self.gattc.mtu_req(gattc_if, conn_id)?;
            }
            GattcEvent::Open {
                status, addr, mtu, ..
            } => {
                self.check_gatt_status(status)?;

                info!("Open successfully with {addr}, MTU {mtu}");
            }
            GattcEvent::DiscoveryCompleted { status, conn_id } => {
                self.check_gatt_status(status)?;

                info!("Service discover complete, conn_id {conn_id}");

                self.gattc
                    .search_service(gattc_if, conn_id, Some(&SERVICE_UUID))?;
                info!("search for {:?}", Some(SERVICE_UUID));

                // let mut results = [ServiceElement::new(); 5];
                // let size = self.gattc.get_service(
                //     gattc_if,
                //     conn_id,
                //     // None,
                //     Some(&SERVICE_UUID),
                //     0,
                //     &mut results,
                // )?;

                // info!("Get service Found {size}");
                // for service in results[..size].iter() {
                //     info!("{:?}", service);
                // }
            }
            GattcEvent::Mtu { status, mtu, .. } => {
                info!("MTU exchange, status {status:?}, MTU {mtu}");
            }
            GattcEvent::SearchResult {
                conn_id,
                start_handle,
                end_handle,
                srvc_id,
                is_primary,
            } => {
                info!("Service search result, conn_id {conn_id}, is primary service {is_primary}, start handle {start_handle}, end handle {end_handle}, current handle value {}", srvc_id.inst_id);

                if srvc_id.uuid == SERVICE_UUID {
                    info!("Service found, uuid {:?}", srvc_id.uuid);

                    self.state.lock().unwrap().service_start_end_handle =
                        Some((start_handle, end_handle));
                }
            }
            GattcEvent::SearchComplete {
                status,
                conn_id,
                searched_service_source,
            } => {
                self.check_gatt_status(status)?;

                match searched_service_source {
                    ServiceSource::RemoteDevice => {
                        info!("Get service information from remote device")
                    }
                    ServiceSource::Nvs => {
                        info!("Get service information from flash")
                    }
                    _ => {
                        info!("Unknown service source")
                    }
                };
                info!("Service search complete");

                let state = self.state.lock().unwrap();

                if let Some((start_handle, end_handle)) = state.service_start_end_handle {
                    let mut chars = [CharacteristicElement::new(); 1];
                    match self.gattc.get_characteristic_by_uuid(
                        gattc_if,
                        conn_id,
                        start_handle,
                        end_handle,
                        &INV_CTRL_CHARACTERISITIC_UUID,
                        &mut chars,
                    ) {
                        Ok(chars_count) => {
                            info!("Found inv ctrl len {chars_count}");
                            if chars_count > 0 {
                                if let Some(inv_ctrl_char_elem) = chars.first() {
                                    info!("Inverter ctrl handle {}", inv_ctrl_char_elem.handle());

                                    let command = if ui::INVERTER_ON
                                        .load(std::sync::atomic::Ordering::Relaxed)
                                    {
                                        info!("Going to turn inverter ON");
                                        &TURN_ON_INVERTER
                                    } else {
                                        info!("Going to turn inverter OFF");
                                        &TURN_OFF_INVERTER
                                    };

                                    self.gattc.write_characteristic(
                                        gattc_if,
                                        conn_id,
                                        inv_ctrl_char_elem.handle(),
                                        command,
                                        GattWriteType::NoResponse,
                                        GattAuthReq::Mitm,
                                    )?;
                                }
                            } else {
                                error!("No inv ctrl characteristic found");
                            }
                        }
                        Err(status) => {
                            error!("get inv ctrl characteristic error {status:?}");
                        }
                    };
                };
            }

            GattcEvent::WriteCharacteristic { status, handle, .. } => {
                self.check_gatt_status(status)?;

                info!("Characteristic write successful handle {handle}");
                let current = ui::INVERTER_ON.load(std::sync::atomic::Ordering::Relaxed);
                ui::INVERTER_PREV.store(current, std::sync::atomic::Ordering::Relaxed);
                info!("Setting prev to current {current}");

                info!("Disconnecting");
                self.disconnect()?;
            }
            GattcEvent::Disconnected { addr, reason, .. } => {
                let mut state = self.state.lock().unwrap();
                state.connect = false;
                state.remote_addr = None;
                state.conn_id = None;
                state.service_start_end_handle = None;

                DEBOUNCE_INV_SWITCH.write().unwrap().replace(Instant::now());

                info!("Disconnected, remote {addr}, reason {reason:?}");
                self.gap.start_scanning(ON_DURATION.as_secs() as _)?;
            }
            _ => (),
        }
        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), EspError> {
        let state = self.state.lock().unwrap();

        if let Some(addr) = state.remote_addr {
            self.gap.disconnect(addr)
        } else {
            Ok(())
        }
    }

    pub fn start_scanning(&self) -> Result<bool, EspError> {
        let state = self.state.lock().unwrap();
        if !state.connect && !state.scanning {
            self.gap.start_scanning(ON_DURATION.as_secs() as _)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn stop_scanning(&self) -> Result<bool> {
        let state = self.state.lock().unwrap();
        if state.scanning {
            self.gap.stop_scanning()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_esp_status(status: Result<(), EspError>) {
        if let Err(e) = status {
            error!("Got status: {e:?}");
        }
    }

    fn check_bt_status(&self, status: BtStatus) -> Result<(), EspError> {
        if !matches!(status, BtStatus::Success) {
            warn!("Got status: {status:?}");
            Err(EspError::from_infallible::<ESP_FAIL>())
        } else {
            Ok(())
        }
    }

    fn check_gatt_status(&self, status: GattStatus) -> Result<(), EspError> {
        if !matches!(status, GattStatus::Ok) {
            warn!("Got status: {status:?}");
            Err(EspError::from_infallible::<ESP_FAIL>())
        } else {
            Ok(())
        }
    }
}

const ONE_POINT: f32 = 10_f32;
const TWO_POINT: f32 = 100_f32;
fn digits(from: f32, two_point: bool) -> f32 {
    let prec: f32 = if two_point { TWO_POINT } else { ONE_POINT };
    (from * prec).trunc() / prec
}
