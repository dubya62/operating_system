#![no_std]
#![no_main]
// Features
#![feature(abi_x86_interrupt)] // For interrupts
#![feature(const_mut_refs)] // For allocator
// Testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::Page;

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod gdt;
pub mod interrupts;
pub mod memory;

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

fn init() {
    interrupts::init();
    gdt::init();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    init();

    {
        // TODO: Move this into memory::init
        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { memory::init(phys_mem_offset) };
        let mut frame_allocator = memory::BootInfoFrameAllocator::new(&boot_info.memory_map);

        // map an unused page
        let page = Page::containing_address(VirtAddr::new(0));
        memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

        // write the string `New!` to the screen through the new mapping
        let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
        unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

        memory::allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");
    }

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
