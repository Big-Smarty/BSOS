#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{QemuExitCode, exit_qemu, hlt_loop, init, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    breakpoint();
    exit_qemu(QemuExitCode::Success);
    hlt_loop()
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
