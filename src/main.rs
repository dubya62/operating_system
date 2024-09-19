#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
pub mod vga;
pub mod gdt;
pub mod interrupts;

#[cfg(test)]
pub mod test;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

fn init() {
    interrupts::init();
    gdt::init();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();
    hlt_loop();
}

fn main() {
    println!("hello world");
    println!(Red, "hello world");
    println!(Blue, "hello world");
}
