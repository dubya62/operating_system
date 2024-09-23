#![no_std]
#![no_main]
// Features
#![feature(abi_x86_interrupt)] // For interrupts
#![feature(const_mut_refs)] // For allocator
#![feature(naked_functions)] // For interrupts
// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod crypt;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod process;

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

fn kernel_thread_main() {
    println!("Kernel thread start");
    process::new_kernel_thread(|| {
        println!("Hello from kernel function 2!");
        loop {
            println!("   <2>");
            x86_64::instructions::hlt();
        }
    });
    process::new_kernel_thread(|| {
        println!("Hello from kernel function 3!");
        loop {
            println!("      <3>");
            x86_64::instructions::hlt();
        }
    });
    loop {
        println!("<1>");
        x86_64::instructions::hlt();
    }
}

fn main() {
    process::new_user_thread(include_bytes!("../user/hello")).unwrap();
    // // Set some registers
    // unsafe {
    //     core::arch::asm!("mov r11, 0x4242", "mov rdi, 0x22", "mov rcx, 0x93");
    // }

    // // Wait for an interrupt
    // unsafe {
    //     core::arch::asm!("hlt");
    // }

    // // Get the register values
    // let (r11, rdi, rcx): (i64, i64, i64);
    // unsafe {
    //     core::arch::asm!("nop",
    //          lateout("r11") r11,
    //          lateout("rdi") rdi,
    //          lateout("rcx") rcx);
    // }
    // println!("R11: 0x{:x} RDI: 0x{:x} RCX: 0x{:x}", r11, rdi, rcx);
    // process::new_kernel_thread(kernel_thread_main);
    println!("hello world");
    println!(Red, "hello world");
    println!(Blue, "hello world");
}
