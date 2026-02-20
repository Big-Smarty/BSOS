#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bsos::QemuExitCode;
use bsos::Testable;
use bsos::exit_qemu;
use bsos::println;
use bsos::serial_println;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(crate::QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bsos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
