#[macro_export]
macro_rules! modbus_rtu_server {
    ($num_coils:expr, $num_discrete_inputs:expr, $num_holding_registers:expr, $num_input_registers:expr) => {
        use $crate::modbus_core::rtu::server;
        use $crate::modbus_core::Request;
        use $crate::defmt::*;
        use {defmt_rtt as _, panic_probe as _};

        use $crate::modbus_map;

        modbus_map!($num_coils, $num_discrete_inputs, $num_holding_registers, $num_input_registers);
        
        pub struct ModbusRTUServer {
            modbus_map: ModbusMap,
        }

        impl ModbusRTUServer {
            // Create a new instance of ModbusData and initialize the channel
            fn new() -> Self {
                let modbus_rtu_server = ModbusRTUServer {
                    modbus_map: ModbusMap::new(),
                };
                modbus_rtu_server
            }

            async fn init_modbus_rtu_server(&self) {
                self.modbus_map.init_modbus_channel().await;
            }

            async fn decode_serial_buffer(&self, buf: &[u8]) {
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
        }
    }
}
