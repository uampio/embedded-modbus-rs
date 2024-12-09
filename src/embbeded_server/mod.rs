#[macro_export]
macro_rules! modbus_rtu_server {
    ($num_coils:expr, $num_discrete_inputs:expr, $num_holding_registers:expr, $num_input_registers:expr) => {
        use $crate::modbus_core::rtu::{server, ResponseAdu, Header};
        use $crate::modbus_core::{Request, Response, ResponsePdu, Data, Exception};
        use $crate::defmt::*;
        use {defmt_rtt as _, panic_probe as _};

        use $crate::modbus_map;

        modbus_map!($num_coils, $num_discrete_inputs, $num_holding_registers, $num_input_registers);

        struct ModbusRTUServer {
            id: u8,
            modbus_data: ModbusData,
        } 

        impl ModbusRTUServer {
            // Create a new instance of ModbusData and initialize the channel
            fn new(id: u8) -> Self {
                Self {
                    id,
                    modbus_data: ModbusData::new(ModbusData{
                        coils: [false; NUM_COILS],
                        discrete_inputs: [false; NUM_DISCRETE_INPUTS],
                        holding_registers: [0; NUM_HOLDING_REGISTERS],
                        input_registers: [0; NUM_INPUT_REGISTERS],
                    }),
                }
            }

            fn decode_serial_buffer(&self, buf_input: &[u8], buffer_ouput: &mut [u8]) -> Result<(bool, usize), u8> {
                match server::decode_request(buf_input) {
                    Ok(Some(request)) => {
                        info!("ID slave target: {:?}", request.hdr.slave);
                        if request.hdr.slave == self.id {

                                match request.pdu.0 {
                                    Request::ReadCoils (address, quantity) => {

                                            info!("Address: {:?}", address);
                                            info!("Quantity: {:?}", quantity);

                                            return Ok((false, 0));
                            
                                        // Handle Read Coils request
                                    }
                                    Request::ReadDiscreteInputs { .. } => {
                                        info!("Function: Read Discrete Inputs");
                                        // Handle Read Discrete Inputs request
                                        return Ok((false, 0));
                                    }
                                    Request::ReadHoldingRegisters (address, quantity) => {
                                        // Handle Read Holding Registers request
                                        let mut buffer: [u8; 255] = [0; 255];
                                        let mut adu : ResponseAdu;
                                        let mut data_response : Data;
                                        if address + quantity > NUM_HOLDING_REGISTERS as u16 {
                                            adu = ResponseAdu {
                                                hdr: Header { slave: self.id },
                                                pdu: ResponsePdu(Ok(Response::ReadExceptionStatus(Exception::IllegalDataAddress as u8))),
                                            };

                                        } else {
                                            data_response = Data::from_words(
                                                &self.modbus_data.holding_registers[address as usize..(address + quantity) as usize],
                                                &mut buffer
                                            ).map_err(|_| 550)?;
                                            adu = ResponseAdu {
                                                hdr: Header { slave: self.id },
                                                pdu: ResponsePdu(Ok(Response::ReadHoldingRegisters(data_response))),
                                            };
                                        };

                                        match server::encode_response(adu, buffer_ouput) {
                                            Ok(buffer_zie) => {
                                                info!("Response encoded");
                                                return Ok((true, buffer_zie));
                                            }
                                            Err(e) => {
                                                //info!("Error encoding response: {:?}", e);
                                                return Err(501);
                                            }
                                        }
                                    }
                                    Request::ReadInputRegisters { .. } => {
                                        info!("Function: Read Input Registers");
                                        // Handle Read Input Registers request
                                        return Ok((false, 0));
                                    }
                                    Request::WriteSingleCoil { .. } => {
                                        info!("Function: Write Single Coil");
                                        // Handle Write Single Coil request
                                        return Ok((false, 0));
                                    }
                                    Request::WriteSingleRegister { .. } => {
                                        info!("Function: Write Single Register");
                                        // Handle Write Single Register request
                                        return Ok((false, 0));
                                    }
                                    Request::WriteMultipleCoils { .. } => {
                                        info!("Function: Write Multiple Coils");
                                        // Handle Write Multiple Coils request
                                        return Ok((false, 0));
                                    }
                                    Request::WriteMultipleRegisters { .. } => {
                                        info!("Function: Write Multiple Registers");
                                        // Handle Write Multiple Registers request
                                        return Ok((false, 0));
                                    }
                                    _ => {
                                        //info!("Unknown function");
                                        // Handle unknown function
                                        //return Ok((false, 0));
                                        return Err(550);
                                    }
                                }
                        }
                        return Ok((false, 0));
                    }
                    Ok(None) => {
                        info!("No request decoded");
                        return Ok((false, 0));
                    }
                    Err(e) => {
                        info!("Error decoding");//info!("Error decoding request: {:?}", e);
                        return Err(550);
                    }
                }
            }
        }
    }
}
