#![feature(atomic_try_update)]

use std::sync::Arc;
use std::thread::{self};
use std::time::Duration;

use esp_idf_svc::bt::ble::gatt::client::EspGattc;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::sys::lcd_bindings::{
    lvgl_port_lock, lvgl_port_unlock, ui_init, ui_tick, waveshare_esp32_s3_rgb_lcd_init,
};

use anyhow::Result;

use esp_idf_svc::bt::BtDriver;
use esp_idf_svc::bt::ble::gap::EspBleGap;
use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;

mod client;
mod devices;
mod http;
mod ui;
mod wifi;

use client::Client;
use http::server::HttpServer;
use wifi::Wifi;

use crate::devices::DEVICES;
use crate::ui::ui::{setup_backlight, subscribe_ui_events};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    // esp_idf_svc::log::EspLogger::initialize_default();
    let _logger = esp_idf_svc::log::init_from_esp_idf();
    ::log::set_max_level(log::LevelFilter::Debug);

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    //-------
    // Config
    //-------
    DEVICES.write().unwrap().load_from_nvs(nvs.clone());
    info!(
        "Config loaded. Found {}",
        DEVICES.read().unwrap().num_devices()
    );

    let (wifi_modem, bt_modem) = peripherals.modem.split();

    let wifi = Wifi::new(BlockingWifi::wrap(
        EspWifi::new(wifi_modem, sys_loop.clone(), Some(nvs.clone()))?,
        sys_loop.clone(),
    )?)?;

    let _server = HttpServer::new()?;

    let bt = Arc::new(BtDriver::new(bt_modem, Some(nvs.clone()))?);

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
    info!("LCD init");

    setup_backlight(sys_loop.clone());
    info!("LCD backlight controller started");

    unsafe {
        if lvgl_port_lock(-1) {
            ui_init();
            info!("UI init");

            subscribe_ui_events(&sys_loop, client.clone(), wifi)?;
            info!("UI event subscriptions initialized");

            // let backlight_on_rx = setup_backlight(sys_loop.clone());
            // setup_backlight(sys_loop.clone());
            // client.backlight_handler(backlight_on_rx);
            // info!("Client backlight handler started");

            lvgl_port_unlock();
        }
    };

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
