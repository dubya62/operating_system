#![no_std]
#![no_main]
// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod vga;

#[cfg(test)]
pub mod test;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();
    loop {}
}

fn main() {
    println!("hello world");
    println!(Red, "hello world");
    println!(Blue, "hello world");
}
