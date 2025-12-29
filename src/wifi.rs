use std::{
    ffi::CString,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use esp_idf_svc::{
    sys::EspError,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use log::info;

use crate::ui;

const SSID: &str = "VICMON";

#[derive(Clone)]
pub struct Wifi<'d> {
    wifi: Arc<Mutex<BlockingWifi<EspWifi<'d>>>>,
}

impl<'d> Wifi<'d> {
    pub fn new(mut wifi: BlockingWifi<EspWifi<'d>>) -> Result<Self, EspError> {
        let wifi_configuration = Configuration::AccessPoint(AccessPointConfiguration {
            ssid: SSID.try_into().unwrap(),
            ssid_hidden: false,
            // auth_method: AuthMethod::WPA2Personal,
            auth_method: AuthMethod::None,
            channel: 1,
            max_connections: 2,
            ..Default::default()
        });

        wifi.set_configuration(&wifi_configuration)?;

        Ok(Self {
            wifi: Arc::new(Mutex::new(wifi)),
        })
    }

    pub fn start_wifi(&mut self) -> Result<()> {
        let mut wifi = self.wifi.lock().unwrap();

        wifi.start()?;
        info!("Wifi started");

        wifi.wait_netif_up()?;
        info!("Wifi netif up");

        let ip_addr = wifi.wifi().ap_netif().get_ip_info()?.ip;
        info!("IP {ip_addr}");

        ui::IP_ADDR
            .write()
            .unwrap()
            .replace(CString::new(format!("{ip_addr}")).unwrap());

        Ok(())
    }

    pub fn stop_wifi(&mut self) -> Result<()> {
        let mut wifi = self.wifi.lock().unwrap();

        wifi.stop()?;

        info!("Wifi stopped");

        ui::IP_ADDR.write().unwrap().take();

        Ok(())
    }
}
