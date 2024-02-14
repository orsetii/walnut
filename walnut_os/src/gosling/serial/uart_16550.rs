pub struct SerialPort {
    base_addr: usize,
    dlab: DivisorLatchAccessBitState,
}

impl SerialPort {
    pub fn new(base_addr: usize) -> Self {
        let ret = Self {
            base_addr,
            dlab: DivisorLatchAccessBitState::Disabled,
        };

        ret
    }

    pub fn line_control_register(&self) -> bool {
        let s = SerialPort::new(0x3F8);
        let port_val = s.read_port(LineStatusRegister {}, AccessMode::Read);
        crate::println!("Got port_val: {:#x?}", port_val);
        loop {}
        true
    }

    fn set_baud_rate_115200() {}

    fn read_port(&self, port: impl UartRegister, access_mode: AccessMode) -> u8 {
        let res = self.port_ptr(port.access(access_mode));
        unsafe { res.read_volatile() }
    }

    fn read_bit(&self, pos: u8) -> bool {
        true
    }

    fn write_bit(&self, pos: u8, val: bool) {
        //
    }

    fn port_ptr(&self, offset: u8) -> *mut u8 {
        let ptr = (self.base_addr + offset as usize) as *mut u8;
        crate::println!("Got port pointer to: {:#x?}", ptr);
        ptr
    }
}

trait UartRegister {
    fn access(self, m: AccessMode) -> u8;
}

struct LineStatusRegister {}
impl UartRegister for LineStatusRegister {
    fn access(self, m: AccessMode) -> u8 {
        match m {
            AccessMode::Read => 5,
            AccessMode::ReadDlab => 5,
            _ => unreachable!(),
        }
    }
}

enum DivisorLatchAccessBitState {
    Disabled = 0,
    Enabled = 1,
}

enum AccessMode {
    Read,
    Write,
    ReadDlab,
    WriteDlab,
}

#[test_case]
fn test_port_address_read() {
    let s = SerialPort::new(0x3F8);
    let port_val = s.read_port(LineStatusRegister {}, AccessMode::Read);
    crate::println!("Got port_val: {:#x?}", port_val);
    loop {}
}
