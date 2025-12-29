use std::{
    ffi::CString,
    sync::{atomic::AtomicBool, RwLock},
};

use self::ui::OnDuration;

const DEFAULT_DELAY: u64 = 30_u64;

pub static ON_DURATION: OnDuration = OnDuration::new();

pub static INVERTER_ON: AtomicBool = AtomicBool::new(false);
pub static INVERTER_PREV: AtomicBool = AtomicBool::new(false);
pub static INV_MODE: RwLock<Option<CString>> = RwLock::new(None);
pub static INV_ERROR: RwLock<Option<CString>> = RwLock::new(None);
pub static AC_WATTS: RwLock<i32> = RwLock::new(0i32);
pub static BATT_SOC: RwLock<f32> = RwLock::new(0f32);
pub static BATT_VOLT: RwLock<f32> = RwLock::new(0f32);
pub static BATT_AMP: RwLock<f32> = RwLock::new(0f32);
pub static BATT_TEMP: RwLock<i32> = RwLock::new(0i32);
pub static BATT_ALARM: RwLock<Option<CString>> = RwLock::new(None);
pub static SOLAR_WATTS: RwLock<i32> = RwLock::new(0i32);
pub static SOLAR_YIELD: RwLock<i32> = RwLock::new(0i32);
pub static SOLAR_MODE: RwLock<Option<CString>> = RwLock::new(None);
pub static SOLAR_ERROR: RwLock<Option<CString>> = RwLock::new(None);
pub static IP_ADDR: RwLock<Option<CString>> = RwLock::new(None);

pub mod ui;
pub mod vars;
