#![no_std]
#![no_main]
// Features
#![feature(abi_x86_interrupt)] // For interrupts
#![feature(const_mut_refs)] // For allocator
// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::string::String;
use bootloader::{entry_point, BootInfo};

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod crypt;
pub mod error;
pub mod file;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod time;

#[cfg(test)]
pub mod test;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

fn init(boot_info: &'static BootInfo) {
    interrupts::init();
    gdt::init();
    memory::init(boot_info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    main();

    hlt_loop();
}

use crate::file::fat32;

fn main() {
    println!("hello world");
    println!(Red, "hello world");
    println!(Blue, "hello world");

    // fs::Stat::default();

    // let test_pipe: pipe::Pipe = pipe::Pipe::new(64);

    //file::pci::enumerate_pci();

    //fat32::test();
}
