use bt_hci::cmd::info;
use defmt::*;
use modbus_core::rtu::{server, RequestAdu};
use {defmt_rtt as _, panic_probe as _};

pub fn decode_serial_buffer(buf: &[u8]) {
    match server::decode_request(buf) {
        Ok(Some(request)) => {
            info!("ID slave target: {:?}", request.hdr.slave);
            //info!("Function code: {:?}", request.pdu);
            // Process the request here
        }
        Ok(None) => {
            info!("No request decoded");
        }
        Err(e) => {
            info!("Error decoding");//info!("Error decoding request: {:?}", e);
        }
    }


}