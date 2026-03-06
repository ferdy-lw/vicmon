use esp_idf_svc::{
    bt::ble::gatt::{GattInterface, client::GattcEvent},
    sys::EspError,
};

use super::*;

const TURN_ON_INVERTER: [u8; 8] = [0x06, 0x03, 0x82, 0x19, 0x02, 0x00, 0x41, 0x03];
const TURN_OFF_INVERTER: [u8; 8] = [0x06, 0x03, 0x82, 0x19, 0x02, 0x00, 0x41, 0x04];

#[derive(Clone)]
pub(super) struct Inverter {}

impl Inverter {
    pub(super) fn on_gattc_event(
        &self,
        client: &Client,
        gattc_if: GattInterface,
        event: GattcEvent,
    ) -> Result<(), EspError> {
        match event {
            GattcEvent::SearchComplete { conn_id, .. } => {
                let state = client.state.lock().unwrap();

                if let Some((start_handle, end_handle)) = state.service_start_end_handle {
                    let mut chars = [CharacteristicElement::new(); 1];
                    match client.gattc.get_characteristic_by_uuid(
                        gattc_if,
                        conn_id,
                        start_handle,
                        end_handle,
                        &REQUEST_CHARACTERISITIC_UUID,
                        &mut chars,
                    ) {
                        Ok(chars_count) => {
                            info!("Found inv ctrl len {chars_count}");
                            if chars_count > 0 {
                                if let Some(inv_ctrl_char_elem) = chars.first() {
                                    info!("Inverter ctrl handle {}", inv_ctrl_char_elem.handle());

                                    let command = if ui::INVERTER_ON
                                        .load(std::sync::atomic::Ordering::Relaxed)
                                    {
                                        info!("Going to turn inverter ON");
                                        &TURN_ON_INVERTER
                                    } else {
                                        info!("Going to turn inverter OFF");
                                        &TURN_OFF_INVERTER
                                    };

                                    client.gattc.write_characteristic(
                                        gattc_if,
                                        conn_id,
                                        inv_ctrl_char_elem.handle(),
                                        command,
                                        GattWriteType::NoResponse,
                                        GattAuthReq::Mitm,
                                    )?;
                                }
                            } else {
                                error!("No inv ctrl characteristic found");
                            }
                        }
                        Err(status) => {
                            error!("get inv ctrl characteristic error {status:?}");
                        }
                    };
                };
            }
            GattcEvent::WriteCharacteristic { .. } => {
                let current = ui::INVERTER_ON.load(std::sync::atomic::Ordering::Relaxed);
                ui::INVERTER_PREV.store(current, std::sync::atomic::Ordering::Relaxed);
                info!("Setting prev to current {current}");

                info!("Disconnecting");
                client.disconnect()?;
            }
            GattcEvent::Disconnected { .. } => {
                DEBOUNCE_INV_SWITCH.write().unwrap().replace(Instant::now());
            }
            _ => (),
        };

        Ok(())
    }
}
