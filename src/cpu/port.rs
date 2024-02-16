#[repr(transparent)]
pub struct Port {
    num: u32,
}

impl Port {
    pub fn new(num: u32) -> Self {
        Self { num }
    }
    /// # Safety
    ///
    /// Accessing a CPU port is unsafe, as we have not verified the address is correct.
    pub unsafe fn readb(&self) -> u8 {
        (self.num as *const u8).read_volatile()
    }

    /// # Safety
    ///
    /// Writing to a CPU port is unsafe.
    pub unsafe fn writeb(&self, value: u8) {
        (self.num as *mut u8).write_volatile(value)
    }
}
