use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
//use embassy_sync::channel::signal::Signal;
//use embassy_futures::util::Forever;

const NUM_COILS: usize = 64;
const NUM_DISCRETE_INPUTS: usize = 64;
const NUM_HOLDING_REGISTERS: usize = 32;
const NUM_INPUT_REGISTERS: usize = 32;

#[derive(Default)]
struct ModbusMap {
    coils: [bool; NUM_COILS],
    discrete_inputs: [bool; NUM_DISCRETE_INPUTS],
    holding_registers: [u16; NUM_HOLDING_REGISTERS],
    input_registers: [u16; NUM_INPUT_REGISTERS],
}

static MODBUS_MAP: Forever<Mutex<ThreadModeRawMutex, ModbusMap>> = Forever::new();
//static MAP_UPDATE_SIGNAL: Signal<()> = Signal::new();

fn init_modbus_map() -> &'static Mutex<ThreadModeRawMutex, ModbusMap> {
    MODBUS_MAP.put(Mutex::new(ModbusMap::default()))
}