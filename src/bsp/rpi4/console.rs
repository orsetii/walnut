//! BSP console facilities.

use crate::sync::interface::Mutex;
use crate::console;
use core::fmt;
use crate::sync::NullLock;


static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

struct QEMUOutputInner {
    chars_written: usize,
}

pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}

impl QEMUOutputInner {

    pub const fn new() -> Self {
        Self {
            chars_written: 0
        }
    }

    #[inline(always)]
    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }
        self.chars_written += 1;
    }

}


impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {

        for c in s.chars() {

            if c == '\n' {
                self.write_char('\r');
            }

            self.write_char(c);
        }

        Ok(())
    }

}

impl QEMUOutput {
    pub const fn new() -> Self {
        Self {
            inner: NullLock::new(QEMUOutputInner::new())
        }
    }
}




/// Return a reference to the console.
pub fn console() -> &'static impl console::interface::All {
    &QEMU_OUTPUT
}

/// Passthrough of `args` to the `core::fmt::Write` implementation, but guarded by a Mutex to
/// serialize access.
impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        // Fully qualified syntax for the call to `core::fmt::Write::write:fmt()` to increase
        // readability.
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}
