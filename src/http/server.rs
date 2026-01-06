use embedded_svc::{
    http::{Headers, Method},
    io::{Read, Write},
};

use esp_idf_svc::{
    bt::BdAddr,
    http::{
        self,
        server::{EspHttpConnection, EspHttpServer, Request},
    },
};

use anyhow::Result;
use hex::FromHex;
use log::*;

use serde::Deserialize;

use crate::devices::{DEVICES, Device, DeviceType, Key, Mac};

static INDEX_HTML: &str = include_str!("config.html");

// Max payload length
const MAX_LEN: usize = 1024;

// Need lots of stack to parse JSON
const STACK_SIZE: usize = 10240;

#[derive(Deserialize)]
struct ConfigData<'a> {
    mppt_mac: &'a str,
    mppt_key: &'a str,
    bmv_mac: &'a str,
    bmv_key: &'a str,
    inv_mac: &'a str,
    inv_key: &'a str,
    inv_pin: u32,
}

pub struct HttpServer<'a> {
    _server: EspHttpServer<'a>,
}

impl<'a> HttpServer<'a> {
    pub fn new() -> Result<Self> {
        let server_configuration = http::server::Configuration {
            stack_size: STACK_SIZE,
            ..Default::default()
        };

        let mut server = EspHttpServer::<'static>::new(&server_configuration)?;

        server.fn_handler::<anyhow::Error, _>("/", Method::Get, HttpServer::index)?;
        server.fn_handler::<anyhow::Error, _>("/post", Method::Post, HttpServer::config)?;

        Ok(Self { _server: server })
    }

    fn index(req: Request<&mut EspHttpConnection<'_>>) -> Result<()> {
        let devices = DEVICES.read().unwrap();

        let values = format!(
            "invmac: '{}', invkey: '{}', invpin: '{}', mpptmac: '{}', mpptkey: '{}', bmvmac: '{}', bmvkey: '{}'",
            hex::encode_upper(devices.device_addr(DeviceType::Inverter).addr()),
            hex::encode_upper(devices.device_key(DeviceType::Inverter)),
            format!(
                "{:06}",
                devices.device_pin(DeviceType::Inverter).unwrap_or(0)
            ),
            hex::encode_upper(devices.device_addr(DeviceType::Mppt).addr()),
            hex::encode_upper(devices.device_key(DeviceType::Mppt)),
            hex::encode_upper(devices.device_addr(DeviceType::Bmv).addr()),
            hex::encode_upper(devices.device_key(DeviceType::Bmv)),
        );

        let config_page = INDEX_HTML.replace("${values}", &values);

        req.into_ok_response()?
            .write_all(config_page.as_bytes())
            .map(|_| ())?;

        Ok(())
    }

    fn config(mut req: Request<&mut EspHttpConnection<'_>>) -> Result<()> {
        let len = req.content_len().unwrap_or(0) as usize;

        if len > MAX_LEN {
            req.into_status_response(413)?
                .write_all("Request too big".as_bytes())?;
            return Ok(());
        }

        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        match serde_json::from_slice::<ConfigData>(&buf) {
            Ok(form) => {
                let mac = match <Mac>::from_hex(form.inv_mac) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with inverter mac: {e}\n")?;
                        None
                    }
                };

                let key = match <Key>::from_hex(form.inv_key) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with inverter key: {e}\n")?;
                        None
                    }
                };

                let msg =
                    HttpServer::save_device(DeviceType::Inverter, mac, key, Some(form.inv_pin));
                write!(resp, "Inverter: {msg}\n")?;

                let mac = match <Mac>::from_hex(form.mppt_mac) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with mppt mac: {e}\n")?;
                        None
                    }
                };

                let key = match <Key>::from_hex(form.mppt_key) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with mppt key: {e}\n")?;
                        None
                    }
                };

                let msg = HttpServer::save_device(DeviceType::Mppt, mac, key, None);
                write!(resp, "MPPT: {msg}\n")?;

                let mac = match <Mac>::from_hex(form.bmv_mac) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with bmv mac: {e}\n")?;
                        None
                    }
                };

                let key = match <Key>::from_hex(form.bmv_key) {
                    Ok(bytes) => Some(bytes),
                    Err(e) => {
                        write!(resp, "Error with bmv key: {e}\n")?;
                        None
                    }
                };

                let msg = HttpServer::save_device(DeviceType::Bmv, mac, key, None);
                write!(resp, "BMV: {msg}\n")?;
            }
            Err(e) => {
                info!("Parse error {e:?}");

                resp.write_all("JSON error".as_bytes())?
            }
        };

        Ok(())
    }

    fn save_device(
        device: DeviceType,
        mac: Option<Mac>,
        key: Option<Key>,
        pin: Option<u32>,
    ) -> String {
        if let Some(mac) = mac
            && let Some(key) = key
        {
            let new_device = Device::new(device, BdAddr::from_bytes(mac), key, pin);

            DEVICES.write().unwrap().add_device(new_device)
        } else {
            "no mac or key, not saved".to_owned()
        }
    }
}
