#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bsos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello hello {}", 1.0 / 3.0);

    bsos::init();

    #[cfg(test)]
    test_main();

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

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
    use bsos::{QemuExitCode, exit_qemu, serial_println};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
