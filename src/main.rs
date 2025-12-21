#![feature(atomic_try_update)]

use std::sync::Arc;
use std::thread::{self};
use std::time::Duration;

use esp_idf_svc::bt::ble::gatt::client::EspGattc;
use esp_idf_svc::sys::lcd_bindings::{
    lvgl_port_lock, lvgl_port_unlock, ui_init, ui_tick, waveshare_esp32_s3_rgb_lcd_init,
};

use anyhow::Result;

use esp_idf_svc::bt::ble::gap::EspBleGap;
use esp_idf_svc::bt::BtDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;

mod client;
mod devices;
mod ui;

use client::Client;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    // esp_idf_svc::log::EspLogger::initialize_default();
    let _logger = esp_idf_svc::log::init_from_esp_idf();
    ::log::set_max_level(log::LevelFilter::Debug);

    let peripherals = Peripherals::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let bt = Arc::new(BtDriver::new(peripherals.modem, Some(nvs.clone()))?);

    let client = Client::new(
        Arc::new(EspBleGap::new(bt.clone())?),
        Arc::new(EspGattc::new(bt.clone())?),
    );

    info!("BLE Gap and Gattc initialized");

    client.subscribe_gap()?;
    client.subscribe_gattc()?;

    info!("BLE Gap and Gattc subscriptions initialized");

    unsafe {
        waveshare_esp32_s3_rgb_lcd_init(); // calls port init (which creates timer task) and tick init
    }
    info!("lcd init");

    let backlight_on_rx = unsafe {
        if lvgl_port_lock(-1) {
            ui_init();
            info!("ui init");

            let backlight_on_rx = ui::setup_backlight();

            lvgl_port_unlock();

            Some(backlight_on_rx)
        } else {
            None
        }
    };

    if let Some(backlight_on_rx) = backlight_on_rx {
        client.backlight_handler(backlight_on_rx);
        info!("Client backlight handler started");
    }

    client.start()?;
    info!("Vicmon app started");

    loop {
        unsafe {
            if lvgl_port_lock(-1) {
                ui_tick();

                lvgl_port_unlock();
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
