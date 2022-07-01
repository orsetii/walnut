use uart_16550::SerialPort;
use crate::println;
use spin::Mutex;
use core::fmt::{Arguments, Write};

pub static COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });


pub fn init() {
    COM1.lock().init();
    println!("Init Serial");
}



pub fn putfmt(fmt: Arguments) {
        // Out put Serial
    unsafe {
        COM1.force_unlock();
    }
    COM1.lock().write_fmt(fmt).unwrap();


}






