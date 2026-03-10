#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;

use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point};
use kernel::{
    hlt_loop,
    memory::{self},
    println, serial_println,
    task::{Task, executor::Executor, keyboard},
};
use x86_64::VirtAddr;

async fn async_number() -> u32 {
    42
}

async fn async_print_number() {
    let number = async_number().await;
    println!("async number: {}", number);
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("hello hello {}", 1.0 / 3.0);
    serial_println!("hello hello {}", 1.0 / 3.0);

    kernel::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.take().unwrap());

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::BootInfoFrameAllocator::new(&boot_info.memory_regions);

    kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Failed to initialize heap");

    let mut executor = Executor::new();
    executor.spawn(Task::new(async_print_number()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

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
    use kernel::{QemuExitCode, exit_qemu, serial_println};

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
