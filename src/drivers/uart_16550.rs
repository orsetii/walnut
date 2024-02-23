//! UART driver for the 16550 chip.
//! Note this currently does not support reading into a buffer.
//!
//! # Usage Example:
//! ```
//! ```

#![allow(dead_code)]

use crate::{cpu::port::Port, sync::spinlock::SpinLock};

pub struct SerialPort {
    regs: SpinLock<SerialInner>,
}

pub struct SerialInner {
    data: Port,
    irq_enable: Port,
    irq_id: Port,
    line_ctrl: Port,
    modem_ctrl: Port,
    line_status: Port,
    modem_status: Port,
    scratch: Port,

    baud_rate_divisor: u16,
}

impl SerialInner {
    pub fn new(base: u32) -> Self {
        let regs = Self {
            data: Port::new(base),
            irq_enable: Port::new(base + 1),
            irq_id: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_status: Port::new(base + 5),
            modem_status: Port::new(base + 6),
            scratch: Port::new(base + 7),
            baud_rate_divisor: 3,
        };

        #[cfg(debug_assertions)]
        // validate reads and writes via scratch register
        // we set this to 0 before hand just to avert any weird race
        // conditions with this check
        unsafe {
            let before = regs.scratch.readb();
            regs.scratch.writeb(127);
            let after = regs.scratch.readb();
            assert_ne!(before, after);
            regs.scratch.writeb(0);
        }

        regs
    }

    const DLAB_BIT: u8 = 0b1000_0000;

    fn without_irqs<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        unsafe {
            self.irq_enable.writeb(0x00);
        }
        let res = f(self);
        unsafe {
            self.irq_enable.writeb(0x01);
        }
        res
    }

    fn set_baud_rate_divisor(&mut self, divisor: u16) -> u16 {
        let prev = self.baud_rate_divisor;
        if divisor == 0 {
            panic!("DLAB BIT NOT SET")
        }

        let lcr_state = unsafe { self.line_ctrl.readb() };
        if lcr_state & Self::DLAB_BIT != 0 {
            // TODO Error here
            panic!("DLAB BIT NOT SET")
        }

        unsafe {
            // set the Divisor Latch Access Bit. now, the data port and irq enable
            // port can be used to set the least and most significant bytes of the
            // divisor, respectively.
            self.line_ctrl.writeb(lcr_state | Self::DLAB_BIT);

            // least significant byte
            self.data.writeb((divisor & 0x00FF) as u8);
            // most significant byte
            self.irq_enable.writeb((divisor >> 8) as u8);

            self.line_ctrl.writeb(lcr_state);
        }

        self.baud_rate_divisor = divisor;

        prev
    }

    #[inline]
    pub fn write_char(&self, b: u8) {
        while !self.write_rdy() {
            core::hint::spin_loop()
        }
        unsafe {
            self.data.writeb(b);
        }
    }

    #[inline]
    fn write_rdy(&self) -> bool {
        unsafe { self.line_status.readb() & 0x20 != 0 }
    }

    #[inline]
    fn read_rdy(&self) -> bool {
        unsafe { self.line_status.readb() & 0x1 == 1 }
    }

    #[inline]
    fn print(&self, s: &str) {
        for b in s.bytes() {
            self.write_char(b)
        }
    }
}
impl core::fmt::Write for SerialInner {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl SerialPort {
    pub fn new(base_addr: u32) -> Self {
        let mut regs = SerialInner::new(base_addr);

        // Disable all interrupts
        regs.without_irqs(|registers| unsafe {
            // Set divisor to 38400 baud
            registers.set_baud_rate_divisor(3);

            // 8 bits, no parity, one stop bit
            registers.line_ctrl.writeb(0x03);

            // Enable FIFO with 14-byte threshold
            registers.irq_id.writeb(0xC7);

            // RTS/DSR set
            registers.modem_ctrl.writeb(0x0B);
        });

        Self {
            regs: SpinLock::new(regs),
        }
    }

    pub fn lock(&self) -> crate::sync::spinlock::Guard<SerialInner> {
        self.regs.lock()
    }

    pub fn read_char(&self) -> char {
        while !self.regs.lock().read_rdy() {
            core::hint::spin_loop()
        }
        unsafe { self.regs.lock().data.readb() as char }
    }

    pub fn read_char_non_blocking(&self) -> Option<char> {
        let r = self.regs.lock();
        if r.read_rdy() {
            unsafe { Some(r.data.readb() as char) }
        } else {
            None
        }
    }
}
