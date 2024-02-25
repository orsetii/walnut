pub mod uart_16550;

use uart_16550::SerialPort;

use crate::sync::spinlock::{OnceCell, SpinLock};

pub static mut SERIAL: OnceCell<SpinLock<SerialPort>> = OnceCell::new();

#[macro_export]
macro_rules! print {
     	($($args:tt)+) => ({
                use core::fmt::Write;
                unsafe {
                        if $crate::cpu::util::my_hart() != 0 {
                                while !$crate::drivers::SERIAL.is_initialized() {
                                core::hint::spin_loop();
                                }
                        }
                        let _ = write!($crate::drivers::SERIAL.get_or_init(||
                        $crate::sync::spinlock::SpinLock::new(
                                $crate::drivers::uart_16550::SerialPort::new(0x1000_0000)
                        )).lock(), $($args)+);
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
