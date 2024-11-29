#[macro_export]
macro_rules! modbus_map {
    ($num_coils:expr, $num_discrete_inputs:expr, $num_holding_registers:expr, $num_input_registers:expr) => {
        use $crate::embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
        use $crate::embassy_sync::channel::Channel;

        pub const NUM_COILS: usize = $num_coils;
        pub const NUM_DISCRETE_INPUTS: usize = $num_discrete_inputs;
        pub const NUM_HOLDING_REGISTERS: usize = $num_holding_registers;
        pub const NUM_INPUT_REGISTERS: usize = $num_input_registers;

        pub struct ModbusData {
            coils: [bool; NUM_COILS],
            discrete_inputs: [bool; NUM_DISCRETE_INPUTS],
            holding_registers: [u16; NUM_HOLDING_REGISTERS],
            input_registers: [u16; NUM_INPUT_REGISTERS],
        }

        pub struct ModbusMap {
            modbus_data: ModbusData,
            channel: Channel<CriticalSectionRawMutex, ModbusData, 2>,
        }

        impl ModbusMap {
            // Create a new instance of ModbusData and initialize the channel
            fn new() -> Self {
                let modbus_data = ModbusData {
                    coils: [false; NUM_COILS],
                    discrete_inputs: [false; NUM_DISCRETE_INPUTS],
                    holding_registers: [0; NUM_HOLDING_REGISTERS],
                    input_registers: [0; NUM_INPUT_REGISTERS],
                };
                let modbus = ModbusMap {
                    modbus_data,
                    channel: Channel::new(),
                };
                modbus
            }

            // Coils
            async fn update_coil(&self, index: usize, value: bool) {
                info!("0 - Coil updated");
                let mut modbus_data = self.channel.receive().await;
                if index < NUM_COILS {
                    info!("1 - Coil updated");
                    modbus_data.coils[index] = value;
                }
                info!("2 - Coil updated");
                self.channel.send(modbus_data).await;
            }

            async fn read_coil(&self, index: usize) -> Option<bool> {
                let modbus_data = self.channel.receive().await;
                if index < NUM_COILS {
                    Some(modbus_data.coils[index])
                } else {
                    None
                }
            }

            async fn get_all_coils(&self) -> [bool; NUM_COILS] {
                let modbus_data = self.channel.receive().await;
                modbus_data.coils
            }

            async fn update_all_coils(&self, new_coils: [bool; NUM_COILS]) {
                let mut modbus_data = self.channel.receive().await;
                modbus_data.coils = new_coils;
                self.channel.send(modbus_data).await;
            }

            async fn init_modbus_channel(&self) {
                let initial_modbus_data = ModbusData {
                    coils: [false; NUM_COILS],
                    discrete_inputs: [false; NUM_DISCRETE_INPUTS],
                    holding_registers: [0; NUM_HOLDING_REGISTERS],
                    input_registers: [0; NUM_INPUT_REGISTERS],
                };
                self.channel.send(initial_modbus_data).await;
            }
        }
    };
}
