use crate::{print, println};

pub mod qemu;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::test_main();

    unreachable!();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) -> () {
        print!("[{}] ", core::any::type_name::<T>());
        self();
        println!("✅");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::QemuExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::util::panic::test_panic_handler(info)
}
