use crate::{print, println};

mod qemu;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn core::ops::Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::QemuExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
