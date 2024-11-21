use defmt::*;
use modbus_core::rtu::server;
use modbus_core::Request;
use {defmt_rtt as _, panic_probe as _};

pub fn decode_serial_buffer(buf: &[u8]) {
    match server::decode_request(buf) {
        Ok(Some(request)) => {
            info!("ID slave target: {:?}", request.hdr.slave);
            if request.hdr.slave == 1 {

                    match request.pdu.0 {
                        Request::ReadCoils (address, quantity ) => {

                                info!("Address: {:?}", address);
                                info!("Quantity: {:?}", quantity);
                 
                            // Handle Read Coils request
                        }
                        Request::ReadDiscreteInputs { .. } => {
                            info!("Function: Read Discrete Inputs");
                            // Handle Read Discrete Inputs request
                        }
                        Request::ReadHoldingRegisters { .. } => {
                            info!("Function: Read Holding Registers");
                            // Handle Read Holding Registers request
                        }
                        Request::ReadInputRegisters { .. } => {
                            info!("Function: Read Input Registers");
                            // Handle Read Input Registers request
                        }
                        Request::WriteSingleCoil { .. } => {
                            info!("Function: Write Single Coil");
                            // Handle Write Single Coil request
                        }
                        Request::WriteSingleRegister { .. } => {
                            info!("Function: Write Single Register");
                            // Handle Write Single Register request
                        }
                        Request::WriteMultipleCoils { .. } => {
                            info!("Function: Write Multiple Coils");
                            // Handle Write Multiple Coils request
                        }
                        Request::WriteMultipleRegisters { .. } => {
                            info!("Function: Write Multiple Registers");
                            // Handle Write Multiple Registers request
                        }
                        _ => {
                            info!("Unknown function");
                            // Handle unknown function
                        }
                    }
            
            }

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