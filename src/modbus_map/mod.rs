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

        struct Modbus {
            modbus_map: ModbusMap,
            channel: Channel<CriticalSectionRawMutex, ModbusMap, 2>,
        }

        impl Modbus {
            // Create a new instance of ModbusMap and initialize the channel
            fn new() -> Self {
                let modbus_map = ModbusMap {
                    coils: [false; NUM_COILS],
                    discrete_inputs: [false; NUM_DISCRETE_INPUTS],
                    holding_registers: [0; NUM_HOLDING_REGISTERS],
                    input_registers: [0; NUM_INPUT_REGISTERS],
                };
                let modbus = Modbus {
                    modbus_map,
                    channel: Channel::new(),
                };
                modbus
            }

            // Coils
            async fn update_coil(&self, index: usize, value: bool) {
                info!("0 - Coil updated");
                let mut modbus_map = self.channel.receive().await;
                if index < NUM_COILS {
                    info!("1 - Coil updated");
                    modbus_map.coils[index] = value;
                }
                info!("2 - Coil updated");
                self.channel.send(modbus_map).await;
            }

            async fn read_coil(&self, index: usize) -> Option<bool> {
                let modbus_map = self.channel.receive().await;
                if index < NUM_COILS {
                    Some(modbus_map.coils[index])
                } else {
                    None
                }
            }

            async fn get_all_coils(&self) -> [bool; NUM_COILS] {
                let modbus_map = self.channel.receive().await;
                modbus_map.coils
            }

            async fn update_all_coils(&self, new_coils: [bool; NUM_COILS]) {
                let mut modbus_map = self.channel.receive().await;
                modbus_map.coils = new_coils;
                self.channel.send(modbus_map).await;
            }

            async fn init_modbus_channel(&self) {
                let initial_modbus_map = ModbusMap {
                    coils: [false; NUM_COILS],
                    discrete_inputs: [false; NUM_DISCRETE_INPUTS],
                    holding_registers: [0; NUM_HOLDING_REGISTERS],
                    input_registers: [0; NUM_INPUT_REGISTERS],
                };
                self.channel.send(initial_modbus_map).await;
            }
        }
    };
}