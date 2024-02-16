//! UART driver for the 16550 chip.
//! Note this currently does not support reading into a buffer.
#![allow(dead_code)]

use crate::{arch::amd64::cpu::port::Port, util::sync::SpinLock};

pub struct SerialPort {
    regs: SpinLock<Registers>,
}

struct Registers {
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

impl Registers {
    pub fn new(base: u16) -> Self {
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
    pub fn read_char(&self) -> u8 {
        while !self.read_rdy() {
            core::hint::spin_loop()
        }
        unsafe { self.data.readb() }
    }

    #[inline]
    fn write_rdy(&self) -> bool {
        unsafe { self.line_status.readb() & 0x20 != 0 }
    }
    #[inline]
    fn read_rdy(&self) -> bool {
        unsafe { self.line_status.readb() & 0x1 == 1 }
    }
}

impl SerialPort {
    pub fn new(base_addr: u16) -> Self {
        let mut regs = Registers::new(base_addr);

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

    #[inline]
    fn print(&self, s: &str) {
        let r = self.regs.lock();
        for b in s.bytes() {
            r.write_char(b)
        }
    }
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.print(s))
    }
}

#[test_case]
fn test_register_default_io_state() {
    let regs = Registers::new(0x3F8);
    // validate reads and writes via scratch register

    assert!(regs.write_rdy());
    // There will be nothing to read..
    assert!(!regs.read_rdy());
}

// TODO add more tests
