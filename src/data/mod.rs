#[macro_export]
macro_rules! modbus_map {
    ($num_coils:expr, $num_discrete_inputs:expr, $num_holding_registers:expr, $num_input_registers:expr) => {
        // Use NoopRawMutex for single-core systems
        use $crate::core::result::Result;
        use $crate::defmt::*;

        pub const NUM_COILS: usize = $num_coils;
        pub const NUM_DISCRETE_INPUTS: usize = $num_discrete_inputs;
        pub const NUM_HOLDING_REGISTERS: usize = $num_holding_registers;
        pub const NUM_INPUT_REGISTERS: usize = $num_input_registers;

        struct ModbusData {
            coils: [bool; NUM_COILS],
            discrete_inputs: [bool; NUM_DISCRETE_INPUTS],
            holding_registers: [u16; NUM_HOLDING_REGISTERS],
            input_registers: [u16; NUM_INPUT_REGISTERS],
        }

        // Implement the Copy trait for ModbusData
        impl Copy for ModbusData {}

        // Implement the Clone trait for ModbusData
        impl Clone for ModbusData {
            fn clone(&self) -> ModbusData {
                *self
            }
        }

        impl ModbusData {
            // Create a new instance of ModbusData and initialize the channel
            fn new(modbus_data: ModbusData) -> Self {
                modbus_data
            }      

            fn update_coil(&mut self, index: usize, value: bool) -> Result<(), u8> {
                if index < NUM_COILS {
                    self.coils[index] = value;
                    //info!("Coil index: {:?} updated to {:?}", index, value);
                    Ok(())
                } else {
                    //info!("Coil index: {:?} out of range. Not updated", index);
                    Err(1)// Invalid coil index returns 1
                }
            }

            fn read_coil(&self, index: usize, value: bool) -> Result<bool, u8> {
                if index < NUM_COILS {
                    Ok(self.coils[index])
                } else {
                    Err(1) // Invalid coil index returns 1
                }
            }

            fn update_holding_register(&mut self, index: usize, value: u16) -> Result<(), u8> {
                if index < NUM_HOLDING_REGISTERS {
                    self.holding_registers[index] = value;
                    //info!("Holding register index: {:?} updated to {:?}", index, value);
                    Ok(())
                } else {
                    //info!("Holding register index: {:?} out of range. Not updated", index);
                    Err(1) // Invalid holding register index returns 1
                }
            }

            fn read_holding_register(&self, index: usize) -> Result<u16, u8> {
                if index < NUM_HOLDING_REGISTERS {
                    Ok(self.holding_registers[index])
                } else {
                    Err(1) // Invalid holding register index returns 1
                }
            }
        }
        
    };
}
