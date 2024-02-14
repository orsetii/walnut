#[cfg(feature = "uart_16550")]
pub mod uart_16550;

use lazy_static::lazy_static;
#[cfg(feature = "uart_16550")]
pub use uart_16550::SerialPort;

use crate::util::sync::SpinLock;

lazy_static! {
    pub static ref SERIAL: SpinLock<SerialPort> = SpinLock::new(SerialPort::new(0x3F8));
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::arch::amd64::gosling::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}
