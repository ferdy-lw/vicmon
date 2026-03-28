/// MPPT protocol, most responses are VE.direct registers
/// Register details: https://www.victronenergy.com/upload/documents/BlueSolar-HEX-protocol.pdf
///
/// NOTE: Bytes are little endian, i.e. in data: 79 06 is 0x0679 = 1657 (at 0.01v == 16.57v)
use core::option::Option;
use std::{
    fmt::Display,
    sync::{
        LazyLock,
        atomic::{AtomicU8, Ordering},
        mpsc::{self, Sender},
    },
};

use esp_idf_svc::{
    bt::ble::gatt::{
        GattInterface,
        client::{DescriptorElement, GattcEvent},
    },
    sys::EspError,
};

use super::*;
use std::string::ToString;
use strum::{self, FromRepr};

use crate::ui::history::update_history_charts;

#[derive(Debug, Clone, Copy, Default, PartialEq, FromRepr, strum::Display)]
#[repr(u8)]
pub enum MpptError {
    #[default]
    #[strum(to_string = "")]
    None = 0,
    #[strum(to_string = "Battery voltage too high")]
    BatVoltHigh = 2,
    #[strum(to_string = "Battery temperature sensor issue 3")]
    BatTempSensor3 = 3,
    #[strum(to_string = "Battery temperature sensor issue 4 ")]
    BatTempSensor4 = 4,
    #[strum(to_string = "Battery temperature sensor issue 5")]
    BatTempSensor5 = 5,
    #[strum(to_string = "Battery voltage sensor issue 6")]
    BattVoltSensor6 = 6,
    #[strum(to_string = "Battery voltage sensor issue 7")]
    BattVoltSensor7 = 7,
    #[strum(to_string = "Battery voltage sensor issue 8")]
    BattVoltSensor8 = 8,
    #[strum(to_string = "Battery temperature too low (charging not allowed)")]
    BattTempLow = 14,
    #[strum(to_string = "Charger internal temperature too high")]
    ChargerTempHigh = 17,
    #[strum(to_string = "Charger excessive output current")]
    ChargerExcessiveOutCurrent = 18,
    #[strum(to_string = "Charger current polarity reversed")]
    ChargerCurrentPolarityReversed = 19,
    #[strum(to_string = "Charger bulk time expired (when 10 hour bulk time protection active)")]
    ChargerBulkTimeExpired = 20,
    #[strum(
        to_string = "Charger current sensor issue (bias not within expected limits during off state)"
    )]
    ChargerCurrentSensor = 21,
    #[strum(to_string = "Charger internal temperature sensor issue 22")]
    ChargerTempSensor22 = 22,
    #[strum(to_string = "Charger internal temperature sensor issue 23")]
    ChargerTempSensor23 = 23,
    #[strum(to_string = "Charger terminals overheated")]
    ChargerTerminalsOverheated = 26,
    #[strum(to_string = "Charger short-circuit")]
    ChargerShortCircuit = 27,
    #[strum(
        to_string = "Converter issue (dual converter models, one of the converters is not working)"
    )]
    ConverterIssue = 28,
    #[strum(to_string = "Battery over-charge protection")]
    BatteryOverChargeProtection = 29,
    #[strum(to_string = "Input voltage too high")]
    InputVoltageTooHight = 33,
    #[strum(to_string = "Input excessive current")]
    InputExcessiveCurrent = 34,
    #[strum(to_string = "Input shutdown (due to excessive battery voltage)")]
    InputShutdownExcessiveBattVoltage = 38,
    #[strum(to_string = "Input shutdown (current flowing while the converter is switched off)")]
    InputShutdownCurrentWhenConverterOff = 39,
    #[strum(to_string = "Incompatible device in the network (for synchronized charging)")]
    IncompatibleDevice = 66,
    #[strum(to_string = "BMS connection lost")]
    BmsConnectionLost = 67,
    #[strum(to_string = "Network misconfigured (e.g. combining ESS with ve.smart networking)")]
    NetworkMisconfigured = 68,
    #[strum(to_string = "Calibration data lost")]
    CalibrationDataLost = 116,
    #[strum(to_string = "Incompatible firmware (i.e. not for this model)")]
    IncompatibleFirmware = 117,
    #[strum(
        to_string = "Settings data invalid / corrupted (use restore to defaults and reset to recover)"
    )]
    SettingsDataInvalid = 119,
    #[strum(to_string = "Uknown error: {0}")]
    Uknown(u8) = 255,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HistoryDay {
    pub day: u8,
    pub yield_: u32,
    pub p_max: u32,
    pub v_max: f32,
    pub bat_max: f32,
    pub bat_min: f32,
    pub float: Duration,
    pub abs: Duration,
    pub bulk: Duration,
    pub errors: MpptError,
}

impl HistoryDay {
    /// | Bytes | Description | scale | type | unit |
    /// | :--- | :--- | :--- | :--- | :--- |
    /// |0 | Reserved (=0) | | | un8 | |
    /// |1 | Yield | 0.01 | un32 | kWh |
    /// |5 | Consumed (na if no load=0XFF..) | 0.01 | un32 | kWh |
    /// |9 | Battery voltage maximum | 0.01 | un16 | V |
    /// |11 | Battery voltage minimum | 0.01 | un16 | V |
    /// |13 | Err Database | | | un8 | |
    /// |14 | Err 0 most recent | | | un8 | |
    /// |15 | Err 1 | | | un8 | |
    /// |16 | Err 2 | | | un8 | |
    /// |17 | Err 3 oldest | | | un8 | |
    /// |18 | Time bulk | 1 | un16 | sec |
    /// |20 | Time absorption | 1 | un16 | sec |
    /// |22 | Time float | 1 | un16 | sec |
    /// |24 | Power maximum | 1 | un32 | W |
    /// |28 | Battery current maximum | 0.1 | un16 | A |
    /// |30 | Panel voltage maximum | 0.01 | un16 | V |
    /// |32 | Day sequence number (no data yet =0x04) | | un16 | |
    ///
    /// Errors
    ///
    /// | Error |  Meaning |
    /// | :--- | :--- |
    /// |0 | No error |
    /// |2 | Battery voltage too high|
    /// |3..5 | Battery temperature sensor issue|
    /// |6..8 | Battery voltage sensor issue|
    /// |14 | Battery temperature too low (charging not allowed)|
    /// |17 | Charger internal temperature too high|
    /// |18 | Charger excessive output current|
    /// |19 | Charger current polarity reversed|
    /// |20 | Charger bulk time expired (when 10 hour bulk time protection active)|
    /// |21 | Charger current sensor issue (bias not within expected limits during off state)|
    /// |22,23 | Charger internal temperature sensor issue|
    /// |26 | Charger terminals overheated|
    /// |27 | Charger short-circuit|
    /// |28 | Converter issue (dual converter models, one of the converters is not working)|
    /// |29 | Battery over-charge protection|
    /// |33 | Input voltage too high|
    /// |34 | Input excessive current|
    /// |38 | Input shutdown (due to excessive battery voltage)|
    /// |39 | Input shutdown (current flowing while the converter is switched off)|
    /// |66 | Incompatible device in the network (for synchronized charging)|
    /// |67 | BMS connection lost|
    /// |68 | Network misconfigured (e.g. combining ESS with ve.smart networking)|
    /// |116 | Calibration data lost|
    /// |117 | Incompatible firmware (i.e. not for this model)|
    /// |119 | Settings data invalid / corrupted (use restore to defaults and reset to recover)|
    pub fn from_raw(day: u8, bytes: &[u8]) -> Self {
        // Example
        // 0000   |data 08| |source 03 |2byte cmd 19 |id 10 51| |type arr? 58  |num bytes 22
        // |rsv 00 |yield 4c 00 00  00 |cons ff ff ff ff
        // |bmx a3 05| |bmn f7 04|edb 00| |e0 00 |e1 00 |e2 00 |e3 00 |blk 4c 01| |abs 05 00| |flt 61 01|
        // |pmx 04 01 00 00 |bamx 00 |panvmx 67 12 |day fc 00

        let yield_ = u32::from_le_bytes(bytes[1..=4].try_into().unwrap()) * 10; // wh (yield is in .01kwh units)
        // consumed: 5..==8
        let bat_max = u16::from_le_bytes(bytes[9..=10].try_into().unwrap()) as f32 * 0.01; // v (bmx is in .01v units)
        let bat_min = u16::from_le_bytes(bytes[11..=12].try_into().unwrap()) as f32 * 0.01; // v (bmn is in .01v units)
        // error DB: 13
        let error = bytes[14];
        let errors = MpptError::from_repr(error).unwrap_or(MpptError::Uknown(error));
        // error 1: 15
        // error 2: 16
        // error 3: 17
        let bulk =
            Duration::from_mins(u16::from_le_bytes(bytes[18..=19].try_into().unwrap()) as u64);
        let abs =
            Duration::from_mins(u16::from_le_bytes(bytes[20..=21].try_into().unwrap()) as u64);
        let float =
            Duration::from_mins(u16::from_le_bytes(bytes[22..=23].try_into().unwrap()) as u64);
        let p_max = u32::from_le_bytes(bytes[24..=27].try_into().unwrap()); // w
        // batt current max: 28..=29
        let v_max = u16::from_le_bytes(bytes[30..=31].try_into().unwrap()) as f32 * 0.01; // v (vmx is in .01v units)
        // day: 32..=33

        Self {
            day,
            yield_,
            p_max,
            v_max,
            bat_max,
            bat_min,
            float,
            abs,
            bulk,
            errors,
        }
    }

    pub fn abs_pct(&self) -> f32 {
        self.abs.as_secs_f32()
            / self
                .bulk
                .saturating_add(self.abs)
                .saturating_add(self.float)
                .as_secs_f32()
    }

    pub fn bulk_pct(&self) -> f32 {
        self.bulk.as_secs_f32()
            / self
                .bulk
                .saturating_add(self.abs)
                .saturating_add(self.float)
                .as_secs_f32()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HistoryLifetime {
    pub lifetime_yield: u32,
    pub _yield_since_reset: u32,
}

impl HistoryLifetime {
    /// | Bytes | Description | scale | type | unit |
    /// | :--- | :--- | :--- | :--- | :--- |
    /// |0 | Reserved (=1 firmware 1.17+) | | | un8 | |
    /// |1 | Err databaes (=0) | | | un8 | |
    /// |2 | Err 0 most recent | | | un8 | |
    /// |3 | Err 1 | | | un8 | |
    /// |4 | Err 2 | | | un8 | |
    /// |5 | Err 3 oldest | | | un8 | |
    /// |6 | Total yield (user resettable) | 0.01 | un32 | kWh |
    /// |10 | Total yield (system) | 0.01 | un32 | kWh |
    /// |14 | Panel voltage maximum | 0.01 | un16 | V |
    /// |16 | Battery voltage maximum | 0.01 | un16 | V |
    /// |18 | Num days available | | un8 | |
    /// |19 | Battery voltage minimum | 0.01 | un16 | V |
    /// |21 | 13 reserved bytes (=0xFF) | | un8 | |
    pub fn from_raw(bytes: &[u8]) -> Self {
        // Example:
        // |rsv 01 | 00 00 00 00 00 |lftm usr b5 03 01 00
        // |lftm sys b5 03 01 00| pnl vmax ae 13 | batt vmax 79 06 | num days 1e | batt min 00 00|
        // |rsv ff ff ff ff ff ff ff ff ff ff ff ff ff

        let yield_since_reset = u32::from_le_bytes(bytes[6..=9].try_into().unwrap()) * 10; // wh (yield is in .01kwh units)
        let lifetime_yield = u32::from_le_bytes(bytes[10..=13].try_into().unwrap()) * 10; // wh (yield is in .01kwh units)

        Self {
            lifetime_yield,
            _yield_since_reset: yield_since_reset,
        }
    }
}

#[derive(Debug)]
pub struct History {
    pub last_loaded: Option<Instant>,
    pub lifetime: Option<HistoryLifetime>,
    pub history: Box<[Option<HistoryDay>]>,
}

impl History {
    pub fn should_load(&self) -> bool {
        self.last_loaded.is_none()
            || self
                .last_loaded
                .is_some_and(|last| Instant::now().duration_since(last).as_secs() > 1800)
    }

    pub fn reset(&mut self) {
        self.last_loaded.take();
        self.history.fill(None);
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("Last Loaded: {:?}\n", self.last_loaded));
        for hist in self.history.iter() {
            let _ = f.write_fmt(format_args!("{hist:?}\n"));
        }
        f.write_fmt(format_args!("Lifetime: {:?}", self.lifetime))
    }
}

pub static HISTORY: RwLock<LazyLock<History>> = RwLock::new(LazyLock::new(|| History {
    last_loaded: None,
    lifetime: None,
    history: Box::new([None; DAYS]),
}));

static RECEIVED_REQUEST: &[u8] = &[0xf9, 0x01]; // ACK when the mppt has accepted the command request
static START_CTL_FLOW_1: &[u8] = &[0xfa, 0x80, 0xff];
static START_CTL_FLOW_2: &[u8] = &[0xf9, 0x80];
static COMMAND_REQUEST_BEGIN: &[u8] = &[0x01];
static COMMAND_REQUEST_TYPE_3: &[u8] = &[0x03, 0x03];

// History data registers all start with 0x10..
// 0x104F - Total History
// 0x1050..=0x106E Daily History (0x1050=today, 0x1051-yesterday..)(If there is no data then flag position is set to 0x04)
// 0x10A0..=0x10BE Daily MPPT history (0x10A0=today, 0x10A1-yesterday..)(If there is no data then flag position is set to 0x04)
const HISTORY_LIFETIME_COMMAND: u8 = 0x4F; // This is the 'day' part of a history command that indicates lifetime data
const HISTORY_DAY_0_COMMAND: u8 = 0x50; // Day 0 is 0x50, -1 is 0x51 ...
pub const DAYS: usize = 14;
const HISTORY_REQUEST_PREFIX: [u8; 5] = [0x05, 0x03, 0x81, 0x19, 0x10]; // command starts with this and ends with a 0x50+day_num up to 4f
const HISTORY_REQUEST_PREFIX_LEN: usize = HISTORY_REQUEST_PREFIX.len();
const fn history_command() -> [u8; (HISTORY_REQUEST_PREFIX_LEN + 1) * (DAYS + 1)] {
    let mut command = [0; (HISTORY_REQUEST_PREFIX_LEN + 1) * (DAYS + 1)];

    let mut day = 0;
    // add a day for the lifetime command
    while day < DAYS + 1 {
        let mut idx = 0;

        while idx < HISTORY_REQUEST_PREFIX_LEN {
            command[HISTORY_REQUEST_PREFIX_LEN * day + idx + day] = HISTORY_REQUEST_PREFIX[idx];
            idx += 1;
        }

        command[HISTORY_REQUEST_PREFIX_LEN * day + idx + day] = if day == DAYS {
            HISTORY_LIFETIME_COMMAND
        } else {
            HISTORY_DAY_0_COMMAND + day as u8
        };

        day += 1;
    }

    command
}
static HISTORY_REQUEST_COMMANDS: &[u8] = &history_command();
static HISTORY_RESPONSE_PREFIX: &[u8] = &[0x08, 0x03, 0x19, 0x10]; // command response starts with this and ends with a 0x50+day_num up to 4f

#[derive(Clone)]
pub(super) struct Mppt {
    command: Arc<AtomicU8>,
    notify_tx: Arc<Mutex<Option<Sender<(u16, Vec<u8>)>>>>,
    long_req_desc_handle: Arc<Mutex<Option<Handle>>>,
}

impl Mppt {
    pub(super) fn new() -> Self {
        Self {
            command: Arc::new(AtomicU8::new(0)),
            notify_tx: Arc::new(Mutex::new(None)),
            // history: Arc::new(Mutex::new(Vec::new())),
            long_req_desc_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub(super) fn on_gattc_event(
        &self,
        client: &Client,
        gattc_if: GattInterface,
        event: GattcEvent,
    ) -> Result<(), EspError> {
        match event {
            GattcEvent::SearchComplete { conn_id, .. } => {
                let mut state = client.state.lock().unwrap();

                if let Some((start_handle, end_handle)) = state.service_start_end_handle {
                    let mut chars = [CharacteristicElement::new(); 5];
                    let chars_count = client
                        .gattc
                        .get_all_characteristics(
                            gattc_if,
                            conn_id,
                            start_handle,
                            end_handle,
                            0,
                            &mut chars,
                        )
                        .map_err(|s| {
                            info!("Not all char found {s:?}");
                            EspError::from_infallible::<ESP_FAIL>()
                        })?;
                    info!("Found {chars_count} chars in service");

                    if chars_count > 0 {
                        // Start the history building thread
                        let (notify_tx, notify_rx) = mpsc::channel();

                        *self.notify_tx.lock().unwrap() = Some(notify_tx);
                        // Mppt::on_notify(notify_rx, Arc::clone(&self.history));
                        Mppt::on_notify(notify_rx);

                        // For all the characteristics, register for notify
                        for char in chars[..chars_count].iter() {
                            if char.uuid() == FLOW_CTL_CHARACTERISITIC_UUID {
                                state.flow_char_handle = Some(char.handle());
                            } else if char.uuid() == REQUEST_CHARACTERISITIC_UUID {
                                state.request_char_handle = Some(char.handle());
                            } else if char.uuid() == LONG_REQUEST_CHARACTERISITIC_UUID {
                                state.long_request_char_handle = Some(char.handle());
                            } else {
                                info!("Unknown characteristic {:?}", char.uuid())
                            }
                            client.gattc.register_for_notify(
                                gattc_if,
                                state.remote_addr.as_ref().unwrap(),
                                char.handle(),
                            )?;
                        }
                    }
                };
            }
            GattcEvent::RegisterNotify { status, handle } => {
                info!("Register Nofify {status:?} char handle {handle}");

                client.check_gatt_status(status)?;
                let state = client.state.lock().unwrap();

                if let Some(conn_id) = state.conn_id {
                    let mut descrs = [DescriptorElement::new(); 1];

                    match client.gattc.get_descriptor_by_char_handle(
                        gattc_if,
                        conn_id,
                        handle,
                        &CLIENT_CONFIGURATION_DESCRIPTOR_UUID,
                        &mut descrs,
                    ) {
                        Ok(descrs_count) => {
                            if descrs_count > 0 {
                                if let Some(descr) = descrs.first() {
                                    client.gattc.write_descriptor(
                                        gattc_if,
                                        conn_id,
                                        descr.handle(),
                                        &1_u16.to_le_bytes(),
                                        GattWriteType::RequireResponse,
                                        GattAuthReq::Mitm,
                                    )?;

                                    if let Some(long_req_char_handle) =
                                        state.long_request_char_handle
                                        && long_req_char_handle == handle
                                    {
                                        self.long_req_desc_handle
                                            .lock()
                                            .unwrap()
                                            .replace(descr.handle());
                                    }
                                    info!(
                                        "Write if {gattc_if} cid {conn_id} descriptor {} char {handle} ",
                                        descr.handle()
                                    );
                                }
                            } else {
                                error!("No ind descriptor found for char handle {handle}");
                            }
                        }
                        Err(status) => {
                            error!(
                                "Get notify char descriptor for char handle {handle} error {status:?}"
                            );
                        }
                    }
                }
            }
            GattcEvent::Notify {
                addr,
                handle,
                value,
                is_notify,
                conn_id,
            } => {
                info!("Got is_notify {is_notify}, addr {addr}, handle {handle}, value {value:?}");

                let state = client.state.lock().unwrap();
                if let Some(flow_handle) = state.flow_char_handle {
                    if handle == flow_handle && value == RECEIVED_REQUEST {
                        let command_idx = self.command.load(Ordering::Relaxed);

                        let command: Option<&[u8]> = if command_idx == 0 {
                            Some(COMMAND_REQUEST_BEGIN)
                        } else if command_idx == 1 {
                            Some(COMMAND_REQUEST_TYPE_3)
                        } else if command_idx == 2 {
                            Some(HISTORY_REQUEST_COMMANDS)
                        } else {
                            None
                        };

                        if let Some(command) = command
                            && let Some(gattc_if) = state.gattc_if_mppt
                            && let Some(long_request_handle) = state.long_request_char_handle
                            && let Some(request_handle) = state.request_char_handle
                        {
                            let byte_count = command.len();
                            let mut start = 0;

                            while start < byte_count {
                                let (handle, write_command) = if start + 74 < byte_count {
                                    (long_request_handle, &command[start..(start + 74)])
                                } else {
                                    (request_handle, &command[start..])
                                };
                                start += 74;

                                info!("Writing {handle} command: {write_command:?}");

                                client.gattc.write_characteristic(
                                    gattc_if,
                                    conn_id,
                                    handle,
                                    write_command,
                                    GattWriteType::NoResponse,
                                    GattAuthReq::Mitm,
                                )?;
                            }

                            self.command.store(command_idx + 1, Ordering::Relaxed);
                        }
                    } else {
                        if value.len() >= 40
                            && value[4] >= HISTORY_LIFETIME_COMMAND
                            && value.starts_with(HISTORY_RESPONSE_PREFIX)
                        {
                            if value[4] == HISTORY_LIFETIME_COMMAND {
                                // The other history all comes before this
                                let lifetime = HistoryLifetime::from_raw(&value[7..]);

                                if let Ok(history) = HISTORY.write().as_mut() {
                                    history.last_loaded.replace(Instant::now());
                                    history.lifetime.replace(lifetime);
                                }

                                info!("Got last history command response, closing");
                                let _ = client.gattc.close(gattc_if, conn_id);
                            } else if let Some(tx) = self.notify_tx.lock().unwrap().as_ref() {
                                let _ = tx.send((handle, value.to_vec()));
                            }
                        }
                    }
                }
            }
            GattcEvent::WriteDescriptor {
                conn_id, handle, ..
            } => {
                // On writing of the last descriptor handle send the startup commands
                if self
                    .long_req_desc_handle
                    .lock()
                    .unwrap()
                    .is_some_and(|h| handle == h)
                {
                    let state = client.state.lock().unwrap();

                    if let Some(handle) = state.flow_char_handle {
                        client.gattc.write_characteristic(
                            gattc_if,
                            conn_id,
                            handle,
                            START_CTL_FLOW_1,
                            GattWriteType::NoResponse,
                            GattAuthReq::Mitm,
                        )?;
                        client.gattc.write_characteristic(
                            gattc_if,
                            conn_id,
                            handle,
                            START_CTL_FLOW_2,
                            GattWriteType::NoResponse,
                            GattAuthReq::Mitm,
                        )?;

                        info!("Wrote start seq");
                    }
                }
            }
            GattcEvent::Disconnected { .. } => {
                if let Some(tx) = self.notify_tx.lock().unwrap().take() {
                    drop(tx);
                }
                self.command.store(0, Ordering::Relaxed);

                let history = HISTORY.read().unwrap();

                if let Some(history) = LazyLock::get(&history) {
                    info!("History - {history}");
                }

                update_history_charts(&history.history, history.lifetime.as_ref());
            }
            _ => (),
        };

        Ok(())
    }

    // fn on_notify(notify_rx: Receiver<(u16, Vec<u8>)>, history: Arc<Mutex<Vec<History>>>) {
    fn on_notify(notify_rx: Receiver<(u16, Vec<u8>)>) {
        let _ = thread::Builder::new()
            .name("hist_bldr".to_string())
            .stack_size(3000)
            .spawn(move || {
                info!("Start history builder thread");
                loop {
                    match notify_rx.recv() {
                        Ok((_handle, value)) => {
                            let day = value[4] - 0x50;

                            let history_value = HistoryDay::from_raw(day, &value[7..]);

                            if let Some(history) = HISTORY
                                .write()
                                .as_mut()
                                .unwrap()
                                .history
                                .get_mut(day as usize)
                            {
                                history.replace(history_value);
                            }
                        }
                        Err(e) => {
                            info!("Stop history builder thread {e:?}");
                            break;
                        }
                    }
                }
            });
    }
}
