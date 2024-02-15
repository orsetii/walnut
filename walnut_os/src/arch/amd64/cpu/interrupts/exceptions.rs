use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{
    arch::amd64::cpu::{interrupts::pic::InterruptIndex, port::Port},
    print, println,
    util::sync::SpinLock,
};

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT: {:?}\n{:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    crate::util::hlt_loop();
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        super::pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    };
}

pub extern "x86-interrupt" fn kb_interrupt_handler(_stack_frame: InterruptStackFrame) {
    lazy_static::lazy_static! {
        static ref KEYBOARD: SpinLock<Keyboard<layouts::Us104Key, ScancodeSet1>> = SpinLock::new(
            Keyboard::new(ScancodeSet1::new(),layouts::Us104Key,  HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.readb() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        super::pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    };
}
