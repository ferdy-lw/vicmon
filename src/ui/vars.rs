use std::sync::LazyLock;

use super::*;

static EMPTY_STR: LazyLock<CString> = LazyLock::new(|| CString::new("").unwrap());
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
        .unwrap_or(&EMPTY_STR)
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
        .unwrap_or(&EMPTY_STR)
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
        .unwrap_or(&EMPTY_STR)
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
        .unwrap_or(&EMPTY_STR)
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
        .unwrap_or(&EMPTY_STR)
        .as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_var_solar_error(_value: Cstring) {
    // NOOP
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
        .unwrap_or(&EMPTY_STR)
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
