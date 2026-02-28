#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use bsos::{
    hlt_loop,
    memory::{self},
    println, serial_println,
};
use x86_64::VirtAddr;

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("hello hello {}", 1.0 / 3.0);
    serial_println!("hello hello {}", 1.0 / 3.0);

    bsos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::BootInfoFrameAllocator::new(&boot_info.memory_map);

    bsos::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Failed to initialize heap");

    let x = Box::new(41);
    println!("x: {x:?}");

    #[cfg(test)]
    test_main();

    hlt_loop();
}

entry_point!(kernel_main);

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hlt_loop()
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bsos::{QemuExitCode, exit_qemu, serial_println};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
