
#[macro_export]
macro_rules! modbus_map {
    ($num_coils:expr, $num_discrete_inputs:expr, $num_holding_registers:expr, $num_input_registers:expr) => {

        use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use embassy_sync::channel::Channel;

        const NUM_COILS: usize = $num_coils;
        const NUM_DISCRETE_INPUTS: usize = $num_discrete_inputs;
        const NUM_HOLDING_REGISTERS: usize = $num_holding_registers;
        const NUM_INPUT_REGISTERS: usize = $num_input_registers;

        struct ModbusMap {
            coils: [bool; NUM_COILS],
            discrete_inputs: [bool; NUM_DISCRETE_INPUTS],
            holding_registers: [u16; NUM_HOLDING_REGISTERS],
            input_registers: [u16; NUM_INPUT_REGISTERS],
        }

        impl ModbusMap {

            // Coils

            async fn update_coil(index: usize, value: bool) {
                let mut modbus_map = MODBUS_CHANNEL.receive().await;
                if index < NUM_COILS {
                    modbus_map.coils[index] = value;
                }
                MODBUS_CHANNEL.send(modbus_map).await;
            }

            async fn read_coil(index: usize) -> Option<bool> {
                let modbus_map = MODBUS_CHANNEL.receive().await;
                if index < NUM_COILS {
                    Some(modbus_map.coils[index])
                } else {
                    None
                }
            }

            async fn get_all_coils() -> [bool; NUM_COILS] {
                let modbus_map = MODBUS_CHANNEL.receive().await;
                modbus_map.coils
            }

            async fn update_all_coils(new_coils: [bool; NUM_COILS]) {
                let mut modbus_map = MODBUS_CHANNEL.receive().await;
                modbus_map.coils = new_coils;
                MODBUS_CHANNEL.send(modbus_map).await;
            }
        }

        static MODBUS_CHANNEL: Channel<CriticalSectionRawMutex, ModbusMap, 2> = Channel::new();
    };
}

