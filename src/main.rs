#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bsos::QemuExitCode;
use bsos::exit_qemu;
use bsos::println;
use bsos::serial_println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello hello {}", 1.0 / 3.0);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
