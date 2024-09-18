#![no_std]
#![no_main]

#[macro_use]
pub mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

fn main() {
    println!("hello world");
    println!(Red, "hello world");
    println!(Blue, "hello world");
}
