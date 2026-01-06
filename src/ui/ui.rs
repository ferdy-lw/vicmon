use std::{
    ffi::{self, CStr},
    mem,
    sync::mpsc::sync_channel,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use esp_idf_svc::{
    eventloop::{
        EspEvent, EspEventDeserializer, EspEventLoop, EspEventPostData, EspEventSerializer,
        EspEventSource, System,
    },
    hal::delay,
    sys::{
        ESP_ERR_TIMEOUT, ESP_OK, esp_event_post,
        lcd_bindings::{
            lv_event_cb_t, lv_event_code_t, lv_event_code_t_LV_EVENT_PRESSED, lv_event_get_code,
            lv_event_get_user_data, lv_event_t, lv_obj_add_event_cb, objects,
            wavesahre_rgb_lcd_bl_off, wavesahre_rgb_lcd_bl_on,
        },
    },
};
use log::{error, info};
use num_enum::TryFromPrimitive;

use crate::{
    client::Client,
    devices::{Device, DeviceType},
    ui::vars::{CONFIG_BMV, CONFIG_INV, CONFIG_MPPT, get_var_backlight_delay},
    wifi::Wifi,
};

use super::*;

static LAST_TOUCH: RwLock<Option<Instant>> = RwLock::new(None);

pub struct OnDuration {
    duration: RwLock<Duration>,
}

impl OnDuration {
    pub(super) const fn new() -> Self {
        Self {
            duration: RwLock::new(Duration::from_secs(DEFAULT_DELAY)),
        }
    }

    pub fn new_time(&self, secs: u64) {
        if secs != self.as_secs() {
            *self.duration.write().unwrap() = Duration::from_secs(secs);
        }
    }

    pub fn recent_touch(&self) -> bool {
        let duration = self.duration.read().unwrap();

        if let Some(last_touch) = *LAST_TOUCH.read().unwrap() {
            Instant::now().duration_since(last_touch) < *duration
        } else {
            false
        }
    }

    pub fn as_secs(&self) -> u64 {
        self.duration.read().unwrap().as_secs()
    }
}

pub fn setup_backlight(sys_loop: EspEventLoop<System>) {
    // starts with backlight on
    LAST_TOUCH.write().unwrap().replace(Instant::now());

    let mut backlight_on = true;

    thread::spawn(move || {
        loop {
            // Only turn the backlight off if we're on the main screen
            if LAST_TOUCH.read().unwrap().is_some() {
                let touched_recently = ON_DURATION.recent_touch();

                if !touched_recently && backlight_on {
                    backlight_on = turn_backlight_on(false);
                } else if touched_recently {
                    if !backlight_on {
                        backlight_on = turn_backlight_on(true);
                        match sys_loop.post::<UiEvent>(&UiEvent::BacklightOn, delay::BLOCK) {
                            Ok(false) => {
                                error!("Timeout posting event {:?}", UiEvent::BacklightOn);
                            }
                            Err(e) => {
                                error!("Failed to post event {e:?}");
                            }
                            _ => {}
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });
}

fn turn_backlight_on(on: bool) -> bool {
    if on {
        unsafe { wavesahre_rgb_lcd_bl_on() };
    } else {
        unsafe { wavesahre_rgb_lcd_bl_off() };
    }

    on
}

#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum UiObject {
    WifiBtn,
    MainScreen,
    GoMain,
    GoConfig,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum UiEvent {
    BacklightOn,
    Pressed(UiObject),
}

unsafe impl EspEventSource for UiEvent {
    fn source() -> Option<&'static CStr> {
        Some(c"UI-EVENT")
    }
}

impl EspEventSerializer for UiEvent {
    type Data<'a> = UiEvent;

    fn serialize<F, R>(event: &Self::Data<'_>, f: F) -> R
    where
        F: FnOnce(&EspEventPostData) -> R,
    {
        f(&unsafe { EspEventPostData::new(Self::source().unwrap(), Self::event_id(), event) })
    }
}

impl EspEventDeserializer for UiEvent {
    type Data<'a> = UiEvent;

    fn deserialize<'a>(data: &EspEvent<'a>) -> Self::Data<'a> {
        *unsafe { data.as_payload::<UiEvent>() }
    }
}

pub unsafe fn subscribe_ui_events(
    sys_loop: &EspEventLoop<System>,
    client: Client,
    wifi: Wifi<'static>,
) -> Result<()> {
    // Setup the UI event handler
    on_ui_event(sys_loop, client, wifi)?;

    let lv_event_cb: lv_event_cb_t = Some(lv_event_to_sys_loop_cb);

    unsafe {
        lv_obj_add_event_cb(
            objects.main,
            lv_event_cb,
            lv_event_code_t_LV_EVENT_PRESSED,
            &(UiObject::MainScreen as u8) as *const _ as *mut _,
        );

        lv_obj_add_event_cb(
            objects.go_config,
            lv_event_cb,
            lv_event_code_t_LV_EVENT_PRESSED,
            &(UiObject::GoConfig as u8) as *const _ as *mut _,
        );

        lv_obj_add_event_cb(
            objects.go_main,
            lv_event_cb,
            lv_event_code_t_LV_EVENT_PRESSED,
            &(UiObject::GoMain as u8) as *const _ as *mut _,
        );

        lv_obj_add_event_cb(
            objects.wifi_btn,
            lv_event_cb,
            lv_event_code_t_LV_EVENT_PRESSED,
            &(UiObject::WifiBtn as u8) as *const _ as *mut _,
        );
    }

    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lv_event_to_sys_loop_cb(e: *mut lv_event_t) {
    let code: lv_event_code_t = unsafe { lv_event_get_code(e) };
    if code == lv_event_code_t_LV_EVENT_PRESSED {
        let ui_object: UiObject = unsafe {
            (*(lv_event_get_user_data(e) as *mut u8))
                .try_into()
                .unwrap()
        };

        let event = UiEvent::Pressed(ui_object);

        let result = unsafe {
            esp_event_post(
                UiEvent::source().unwrap().as_ptr(),
                UiEvent::event_id().unwrap_or(0),
                (&event as *const _ as *const ffi::c_void).as_ref().unwrap(),
                mem::size_of::<UiEvent>(),
                delay::BLOCK,
            )
        };

        if result == ESP_ERR_TIMEOUT {
            error!("Timeout posting lv event {event:?}");
        } else if result != ESP_OK {
            error!("Error posting lv event {event:?}");
        }
    }
}

fn on_ui_event(
    sys_loop: &EspEventLoop<System>,
    client: Client,
    mut wifi: Wifi<'static>,
) -> Result<()> {
    let (ui_event_tx, ui_event_rx) = sync_channel(1);

    let _ = thread::Builder::new()
        .name("ui_event_loop".to_owned())
        .stack_size(5096)
        .spawn(move || {
            loop {
                let event = ui_event_rx.recv().unwrap();

                match event {
                    // BACKLIGHT
                    // ---------
                    UiEvent::BacklightOn => {
                        info!("Backlight on!!!");

                        if let Ok(true) = client.start_scanning() {
                            info!("Start scan after BL on");
                        }
                    }
                    // PRESSED
                    // --------
                    UiEvent::Pressed(ui_object) => match ui_object {
                        UiObject::MainScreen => {
                            // Record the last touch to keep the backlight on
                            LAST_TOUCH.write().unwrap().replace(Instant::now());
                        }
                        UiObject::WifiBtn => {
                            if let Err(e) = wifi.start_wifi() {
                                error!("Failed to start wifi, {e:?}");
                                ui::IP_ADDR.write().unwrap().replace(
                                    CString::new(format!("Failed to start wifi, {e:?}")).unwrap(),
                                );
                            }
                        }
                        UiObject::GoMain => {
                            // Update the duration time
                            ON_DURATION.new_time(get_var_backlight_delay() as _);

                            // Start backlight off handling
                            LAST_TOUCH.write().unwrap().replace(Instant::now());

                            // Stop wifi
                            if let Err(e) = wifi.stop_wifi() {
                                error!("Failed to stop wifi, {e:?}");
                            }

                            if let Ok(true) = client.start_scanning() {
                                info!("Start scan on return to main screen");
                            }
                        }
                        UiObject::GoConfig => {
                            // Stop turning backlight off
                            LAST_TOUCH.write().unwrap().take();

                            turn_backlight_on(true);

                            if let Ok(true) = client.stop_scanning() {
                                info!("Stop scan in config screen");
                            }
                        }
                    },
                };

                thread::sleep(Duration::from_millis(20));
            }
        });

    let subscription = sys_loop.subscribe::<UiEvent, _>(move |event| {
        info!("[UI sys loop callback] Got event: {event:?}");

        ui_event_tx.send(event).unwrap();
    })?;

    // Keep the subscription around
    mem::forget(subscription);

    Ok(())
}

pub fn config_device(device: &Device) {
    match device.device_type() {
        DeviceType::Inverter => {
            let mut config = CONFIG_INV.write().unwrap();
            config.0 = CString::new(device.addr().to_string()).unwrap();
            config.1 = CString::new(hex::encode_upper(device.key())).unwrap();
            config.2 = CString::new(format!("{:06}", device.pin().unwrap_or(0))).unwrap();
        }
        DeviceType::Mppt => {
            let mut config = CONFIG_MPPT.write().unwrap();
            config.0 = CString::new(device.addr().to_string()).unwrap();
            config.1 = CString::new(hex::encode_upper(device.key())).unwrap();
        }
        DeviceType::Bmv => {
            let mut config = CONFIG_BMV.write().unwrap();
            config.0 = CString::new(device.addr().to_string()).unwrap();
            config.1 = CString::new(hex::encode_upper(device.key())).unwrap();
        }
    }
}
