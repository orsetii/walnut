pub mod uart;

pub const BS: char = 8 as char;

lazy_static::lazy_static! {
    pub static ref SERIAL: uart::SerialPort = uart::SerialPort::new(0x1000_0000);
}

#[macro_export]
macro_rules! print {
    	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::io::serial::SERIAL.lock(), $($args)+);
	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
