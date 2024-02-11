mod qemu;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn core::ops::Fn()]) {
    for test in tests {
        test();
    }
    qemu::exit(qemu::QemuExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
