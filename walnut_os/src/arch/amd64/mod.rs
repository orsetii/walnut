pub mod cpu;
pub mod gosling;
pub mod graphics;

pub fn initialize() {
    cpu::interrupts::init_idt()
}
