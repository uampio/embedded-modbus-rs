#![no_std]
#![no_main]

pub use modbus_core;
pub use embassy_sync;
pub use defmt;
pub use core;

pub mod data;
pub mod embbeded_server;
