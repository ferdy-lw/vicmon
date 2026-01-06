use std::{
    ffi::CString,
    ptr,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use esp_idf_svc::{
    mdns::EspMdns,
    sys::{EspError, uxTaskGetStackHighWaterMark},
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use log::info;

use crate::ui;

const SSID: &str = "VICMON";
const HOSTNAME: &str = "vicmon";

#[derive(Clone)]
pub struct Wifi<'d> {
    wifi: Arc<Mutex<BlockingWifi<EspWifi<'d>>>>,
    mdns: Arc<Mutex<Option<EspMdns>>>,
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
            mdns: Arc::new(Mutex::new(None)),
        })
    }

    pub fn start_wifi(&mut self) -> Result<()> {
        let mut wifi = self.wifi.lock().unwrap();

        wifi.start()?;
        info!("Wifi started");

        wifi.wait_netif_up()?;
        info!("Wifi netif up");

        let mut mdns = EspMdns::take()?;
        mdns.set_hostname(HOSTNAME)?;

        self.mdns.lock().unwrap().replace(mdns);

        let ip_addr = wifi.wifi().ap_netif().get_ip_info()?.ip;
        info!("IP {HOSTNAME}.local ({ip_addr})");

        ui::IP_ADDR
            .write()
            .unwrap()
            .replace(CString::new(format!(" {HOSTNAME}.local ({ip_addr})")).unwrap());

        let stackhigh = unsafe { uxTaskGetStackHighWaterMark(ptr::null_mut()) };
        info!("Least stack free: {stackhigh}");

        Ok(())
    }

    pub fn stop_wifi(&mut self) -> Result<()> {
        if let Some(mdns) = self.mdns.lock().unwrap().take() {
            drop(mdns);
        }

        let mut wifi = self.wifi.lock().unwrap();
        wifi.stop()?;

        info!("Wifi stopped");

        ui::IP_ADDR.write().unwrap().take();

        Ok(())
    }
}
