#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn print(s: &str) {
    for (i, s) in s.chars().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = s as u8;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("hello");
    loop {}
}
