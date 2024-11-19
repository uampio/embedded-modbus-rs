use defmt::*;
use modbus_core::rtu::{server, RequestAdu};
use {defmt_rtt as _, panic_probe as _};

pub fn decode_serial_buffer() { //Just for testing prupose
    // Example buffer received from the master
    let received_buffer: &[u8] = &[0x01, 0x03, 0x00, 0x00, 0x00, 0x02, 0xC4, 0x0B];

    // Decode the received buffer
    match server::decode_request(received_buffer) {
        Ok(Some(request)) => {
            info!("ID slave target: {:?}", request.hdr.slave);
            // Process the request here
        }
        Ok(None) => {
            info!("No request decoded");
        }
        Err(e) => {
            ;//info!("Error decoding request: {:?}", e);
        }
    }


}