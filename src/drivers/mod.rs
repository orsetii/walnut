pub mod uart_16550;

use uart_16550::SerialPort;

use crate::sync::spinlock::OnceCell;

pub static mut SERIAL: OnceCell<SerialPort> = OnceCell::new();

#[macro_export]
macro_rules! print {
     	($($args:tt)+) => ({
 			use core::fmt::Write;
            unsafe {
 			let _ = write!($crate::drivers::SERIAL.get_or_init(|| $crate::drivers::uart_16550::SerialPort::new(0x1000_0000)).lock(), $($args)+);
            }
 	});
 }

#[macro_export]
macro_rules! println
 {
 	() => ({
 		$crate::print!("\r\n")
 	});
 	($fmt:expr) => ({
 		$crate::print!(concat!($fmt, "\r\n"))
 	});
 	($fmt:expr, $($args:tt)+) => ({
 		$crate::print!(concat!($fmt, "\r\n"), $($args)+)
 	});
 }
