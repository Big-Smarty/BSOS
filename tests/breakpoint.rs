#![no_std]
#![no_main]

use bsos::{QemuExitCode, exit_qemu, init, serial_println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    breakpoint();
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn breakpoint() {
    x86_64::instructions::interrupts::int3();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
