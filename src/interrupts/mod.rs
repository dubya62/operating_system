use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::gdt;

mod keyboard;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        // SAFETY: `gdt::DOUBLE_FAULT_IST_INDEX` is valid because we set it up
        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer as u8]
            .set_handler_fn(interrupts::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as u8]
            .set_handler_fn(keyboard::keyboard_interrupt_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
    // SAFETY: We have configured the PICS properly
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// End of interrupt
fn eoi(interrupt: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt as u8);
    }
}

mod interrupts {
    use x86_64::structures::idt::InterruptStackFrame;

    use super::{eoi, InterruptIndex};

    pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
        // print!(".");
        eoi(InterruptIndex::Timer);
    }
}

mod exceptions {
    use x86_64::structures::idt::InterruptStackFrame;

    pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
        println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    }

    pub extern "x86-interrupt" fn double_fault_handler(
        stack_frame: InterruptStackFrame,
        _error_code: u64,
    ) -> ! {
        panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    }
}
