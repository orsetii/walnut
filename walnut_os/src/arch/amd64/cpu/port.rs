use core::arch::asm;

#[repr(transparent)]
pub struct Port {
    num: u16,
}

impl Port {
    pub fn new(num: u16) -> Self {
        Self { num }
    }
    /// # Safety
    ///
    /// Accessing a CPU port is unsafe, as we have not verified the address is correct.
    pub unsafe fn readb(&self) -> u8 {
        let r: u8;
        asm!("in al, dx", out("al") r,  in("dx") self.num);
        r
    }

    /// # Safety
    ///
    /// Writing to a CPU port is unsafe.
    pub unsafe fn writeb(&self, value: u8) {
        asm!("out dx, al", in("dx") self.num, in("al") value)
    }
}
