use crate::println;
use crate::serial_println;
#[cfg(test)]
use crate::testable::Testable;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::exit_qemu;

    serial_println!("Running {} tests", tests.len());

    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(crate::QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
