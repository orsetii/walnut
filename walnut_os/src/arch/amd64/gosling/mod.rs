//! Defines drivers for the system.
//! Still need to figure out *how* these will get
//! inserted, compile flags i guess?
pub mod serial;

pub use serial::SerialPort;
