//! HTTP Server with JSON POST handler
//!
//! Go to 192.168.71.1 to test

use embedded_svc::{
    http::{Headers, Method},
    io::{Read, Write},
};

use esp_idf_svc::http::{
    self,
    server::{EspHttpConnection, EspHttpServer, Request},
};

use anyhow::Result;
use log::*;

use serde::Deserialize;

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

    fn index(req: Request<&mut EspHttpConnection<'_>>) -> Result<(), anyhow::Error> {
        req.into_ok_response()?
            .write_all(INDEX_HTML.as_bytes())
            .map(|_| ())?;

        Ok(())
    }

    fn config(mut req: Request<&mut EspHttpConnection<'_>>) -> Result<(), anyhow::Error> {
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
            Ok(form) => write!(
                resp,
                "pin {}, mppt key {}, mppt mac {}, bmv key {}, bmv mac{}, inv key {} inv mac {}",
                form.inv_pin,
                form.mppt_key,
                form.mppt_mac,
                form.bmv_key,
                form.bmv_mac,
                form.inv_key,
                form.inv_mac
            )?,
            Err(e) => {
                info!("Parse error {e:?}");

                resp.write_all("JSON error".as_bytes())?
            }
        };

        Ok(())
    }
    /*
    // Keep wifi and the server running beyond when main() returns (forever)
    // Do not call this if you ever want to stop or access them later.
    // Otherwise you can either add an infinite loop so the main task
    // never returns, or you can move them to another thread.
    // https://doc.rust-lang.org/stable/core/mem/fn.forget.html
    core::mem::forget(wifi);
    core::mem::forget(server);

    // Main task no longer needed, free up some memory
    Ok(())
    */
}
