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

    fn line_control_register(&self) -> bool {
        true
    }
}

enum DivisorLatchAccessBitState {
    Disabled,
    Enabled,
}

enum AccessMode {
    Read,
    Write,
}

enum Port {
    // IO PORT -  R/W  - DLAB
    //
    // base +0 - read  - 0
    ReceiverBuffer,
    // base +0 - write - 0
    TransmitterHolding,
    // base +0 - read  - 1
    DivisorLatchLsbRead,
    // base +0 - write - 1
    DivisorLatchLsbWrite,

    // IO PORT -  R/W  - DLAB
    // base +1 - read  - 0
    InterruptEnableRead,
    // base +1 - write - 0
    InterruptEnableWrite,
    // base +1 - read  - 1
    DivisorLatchMsbRead,
    // base +1 - write - 1
    DivisorLatchMsbWrite,

    // IO PORT -  R/W  - DLAB
    // base +2 - read  - 0
    InterruptIdentificationReadDisabled,
    // base +2 - write - 0
    FifoControlDisabled,
    // base +2 - read  - 1
    InterruptIdentificationReadEnabled,
    // base +2 - write - 1
    FifoControlEnabled,

    // IO PORT -  R/W  - DLAB
    // base +3 - read  - 0
    LineControlReadDisabled,
    // base +3 - write - 0
    LineControlWriteDisabled,
    // base +3 - read  - 1
    LineControlReadEnabled,
    // base +3 - write - 1
    LineControlWriteEnabled,

    // IO PORT -  R/W  - DLAB
    // base +4 - read  - 0
    ModemControlReadDisabled,
    // base +4 - write - 0
    ModemControlWriteDisabled,
    // base +4 - read  - 1
    ModemControlReadEnabled,
    // base +4 - write - 1
    ModemControlWriteEnabled,

    // IO PORT -  R/W  - DLAB
    // base +5 - read  - 0
    LineStatusDisabled,
    // base +5 - write - 0
    FactoryTest,
    // base +5 - read  - 1
    LineStatusEnabled,
    // base +5 - write - 1
    FactoryTest,

    // IO PORT -  R/W  - DLAB
    // base +6 - read  - 0
    ModemStatusDisabled,
    // base +6 - write - 0
    NotUsed,
    // base +6 - read  - 1
    ModemStatusEnabled,
    // base +6 - write - 1
    FactoryTest,

    // IO PORT -  R/W  - DLAB
    // base +7 - read  - 0
    Scratch,
    // base +7 - write - 0
    Scratch,
    // base +7 - read  - 1
    Scratch,
    // base +7 - write - 1
    Scratch,
}

impl Port {
    pub fn from_info(
        port_offset: u8,
        access_mode: AccessMode,
        dlab_state: DivisorLatchAccessBitState,
    ) -> Port {
        match port_offset {
            0 => port_for_io_port_0(access_mode, dlab_state),
            _ => todo!(),
        }
    }

    fn port_for_io_port_0(access_mode: AccessMode, dlab_state: DivisorLatchAccessBitState) -> Port {
        match access_mode {
            AccessMode::Read => match dlab_state {
                DivisorLatchAccessBitState::Disabled => Port::ReceiverBuffer,
                DivisorLatchAccessBitState::Enabled => Port::DivisorLatchMsbRead,
            },
            AccessMode::Write => match dlab_state {
                DivisorLatchAccessBitState::Disabled => Port::TransmitterHolding,
                DivisorLatchAccessBitState::Enabled => Port::DivisorLatchLsbWrite,
            },
        }
    }
}
