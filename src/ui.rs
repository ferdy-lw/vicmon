use std::{
    ffi::CString,
    ptr,
    sync::{
        atomic::AtomicBool,
        mpsc::{sync_channel, Receiver, SyncSender},
        LazyLock, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use esp_idf_svc::sys::lcd_bindings::{
    lv_event_code_t, lv_event_code_t_LV_EVENT_PRESSED, lv_event_get_code, lv_event_get_indev,
    lv_event_t, lv_indev_get_point, lv_obj_add_event_cb, lv_point_t, objects,
    wavesahre_rgb_lcd_bl_off, wavesahre_rgb_lcd_bl_on,
};
use log::info;

pub const ON_DURATION: Duration = Duration::from_secs(30);
pub static LAST_TOUCH: RwLock<Option<Instant>> = RwLock::new(None);
pub static DEBOUNCE_INV_SWITCH: RwLock<Option<Instant>> = RwLock::new(None);

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

pub unsafe fn setup_backlight() -> Receiver<bool> {
    LAST_TOUCH.write().unwrap().replace(Instant::now());

    let (tx, rx) = sync_channel(1);
    backlight_control(tx);

    let solar_cont = objects.solar_container;
    let inv_cont = objects.inverter_container;

    lv_obj_add_event_cb(
        solar_cont,
        Some(touch_event_handler),
        lv_event_code_t_LV_EVENT_PRESSED,
        ptr::null_mut(),
    );

    lv_obj_add_event_cb(
        inv_cont,
        Some(touch_event_handler),
        lv_event_code_t_LV_EVENT_PRESSED,
        ptr::null_mut(),
    );

    rx
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn touch_event_handler(e: *mut lv_event_t) {
    let code: lv_event_code_t = lv_event_get_code(e);
    if code == lv_event_code_t_LV_EVENT_PRESSED {
        let indev = lv_event_get_indev(e);
        let p: lv_point_t = Default::default();
        lv_indev_get_point(indev, &p as *const _ as *mut _);

        // info!("Touch X: {}, Y: {}", p.x, p.y);

        LAST_TOUCH.write().unwrap().replace(Instant::now());
    }
}

fn backlight_control(backlight_on_tx: SyncSender<bool>) {
    let mut backlight_off = false;

    thread::spawn(move || loop {
        let now = Instant::now();
        if let Some(last_touch) = *LAST_TOUCH.read().unwrap() {
            let not_touched_recently = now.duration_since(last_touch) > ON_DURATION;

            if not_touched_recently && !backlight_off {
                unsafe { wavesahre_rgb_lcd_bl_off() };
                backlight_off = true;
            } else if !not_touched_recently {
                if backlight_off {
                    unsafe { wavesahre_rgb_lcd_bl_on() };
                    backlight_off = false;
                    let _ = backlight_on_tx.send(true);
                }
            }
        }
        thread::sleep(Duration::from_millis(20));
    });

    info!("Started backlight controller");
}
