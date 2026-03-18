use std::{
    ffi::CStr,
    sync::{LazyLock, atomic::Ordering},
};

use crate::{client::mppt::HISTORY, ui::history::history_details};

use super::*;

const EMPTY_STR: &CStr = c"";
type Cstring = *const ::core::ffi::c_char;

//----------
// INVERTER
//----------
#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_switch() -> bool {
    INVERTER_ON.load(std::sync::atomic::Ordering::Relaxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_switch(value: bool) {
    INVERTER_ON.store(value, std::sync::atomic::Ordering::Relaxed);
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_mode() -> Cstring {
    INV_MODE
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_mode(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_error() -> Cstring {
    INV_ERROR
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_error(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_ac_watts() -> i32 {
    *AC_WATTS.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_ac_watts(_value: i32) {
    // NOOP
}

//---------
// BATTERY
//---------
#[unsafe(no_mangle)]
pub extern "C" fn get_var_batt_soc() -> f32 {
    *BATT_SOC.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_batt_soc(_value: f32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_batt_volt() -> f32 {
    *BATT_VOLT.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_batt_volt(_value: f32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_batt_amp() -> f32 {
    *BATT_AMP.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_batt_amp(_value: f32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_batt_temp() -> i32 {
    *BATT_TEMP.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_batt_temp(_value: i32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_batt_alarm() -> Cstring {
    BATT_ALARM
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_batt_alarm(_value: Cstring) {
    // NOOP
}

//-------
// SOLAR
//-------
#[unsafe(no_mangle)]
pub extern "C" fn get_var_solar_watts() -> i32 {
    *SOLAR_WATTS.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_solar_watts(_value: i32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_solar_yield() -> i32 {
    *SOLAR_YIELD.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_solar_yield(_value: i32) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_solar_mode() -> Cstring {
    SOLAR_MODE
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_solar_mode(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_solar_error() -> Cstring {
    SOLAR_ERROR
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_solar_error(_value: Cstring) {
    // NOOP
}

//--------
// HISTORY
//--------
#[unsafe(no_mangle)]
pub extern "C" fn get_var_hist_det_day() -> i32 {
    HIST_DET_DAY.load(Ordering::Relaxed)
}

/// Set the day of the history details to show, where 0 is today.
/// Set to -1 to hide the details dialog pane
/// Set to -2 to show the loading widget and reload the history data
#[unsafe(no_mangle)]
pub extern "C" fn set_var_hist_det_day(day: i32) {
    HIST_DET_DAY.store(day, Ordering::Relaxed);

    if day == -2 {
        HISTORY.write().as_mut().unwrap().reset();
    } else if day > -1 {
        if let Some(Some(history)) = HISTORY.read().unwrap().history.get(day as usize) {
            unsafe {
                history_details(history);
            }
        }
    }
}

//--------
// CONFIG
//--------
#[unsafe(no_mangle)]
pub extern "C" fn get_var_ip_addr() -> Cstring {
    IP_ADDR
        .read()
        .unwrap()
        .as_ref()
        .map(|s| s.as_c_str())
        .unwrap_or(EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_ip_addr(_value: Cstring) {
    // NOOP
}

static BACKLIGHT_DELAY: RwLock<i32> = RwLock::new(DEFAULT_DELAY as _);
#[unsafe(no_mangle)]
pub extern "C" fn get_var_backlight_delay() -> i32 {
    *BACKLIGHT_DELAY.read().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_backlight_delay(value: i32) {
    *BACKLIGHT_DELAY.write().unwrap() = value
}

pub(super) static CONFIG_INV: LazyLock<RwLock<(CString, CString, CString)>> = LazyLock::new(|| {
    RwLock::new((
        EMPTY_STR.to_owned(),
        EMPTY_STR.to_owned(),
        EMPTY_STR.to_owned(),
    ))
});

pub(super) static CONFIG_MPPT: LazyLock<RwLock<(CString, CString, CString)>> =
    LazyLock::new(|| {
        RwLock::new((
            EMPTY_STR.to_owned(),
            EMPTY_STR.to_owned(),
            EMPTY_STR.to_owned(),
        ))
    });

pub(super) static CONFIG_BMV: LazyLock<RwLock<(CString, CString)>> =
    LazyLock::new(|| RwLock::new((EMPTY_STR.to_owned(), EMPTY_STR.to_owned())));

#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_mac() -> Cstring {
    CONFIG_INV.read().unwrap().0.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_mac(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_key() -> Cstring {
    CONFIG_INV.read().unwrap().1.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_key(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_inv_pin() -> Cstring {
    CONFIG_INV.read().unwrap().2.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_inv_pin(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_mppt_mac() -> Cstring {
    CONFIG_MPPT.read().unwrap().0.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_mppt_mac(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_mppt_key() -> Cstring {
    CONFIG_MPPT.read().unwrap().1.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_mppt_key(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_mppt_pin() -> Cstring {
    CONFIG_MPPT.read().unwrap().2.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_mppt_pin(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_bmv_mac() -> Cstring {
    CONFIG_BMV.read().unwrap().0.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_bmv_mac(_value: Cstring) {
    // NOOP
}

#[unsafe(no_mangle)]
pub extern "C" fn get_var_bmv_key() -> Cstring {
    CONFIG_BMV.read().unwrap().1.as_c_str().as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_bmv_key(_value: Cstring) {
    // NOOP
}
