use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::gdt;

mod interrupts;
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

pub const INTERRUPT_CONTEXT_SIZE: usize = core::mem::size_of::<Context>();

#[derive(Debug)]
#[repr(packed)]
pub struct Context {
    // These are pushed in the handler function
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,

    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,

    pub r8: usize,
    pub rbp: usize,
    pub rsi: usize,
    pub rdi: usize,

    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    // Below is the exception stack frame pushed by the CPU on interrupt
    // Note: For some interrupts (e.g. Page fault), an error code is pushed here
    pub rip: usize,    // Instruction pointer
    pub cs: usize,     // Code segment
    pub rflags: usize, // Processor flags
    pub rsp: usize,    // Stack pointer
    pub ss: usize,     // Stack segment
                       // Here the CPU may push values to align the stack on a 16-byte boundary (for SSE)
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint
            .set_handler_fn(exceptions::breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            idt.page_fault
                .set_handler_fn(exceptions::page_fault_handler)
                .set_stack_index(gdt::PAGE_FAULT_IST_INDEX);
            idt.general_protection_fault
                .set_handler_fn(exceptions::general_protection_fault_handler)
                .set_stack_index(gdt::GENERAL_PROTECTION_FAULT_IST_INDEX);
        }

        unsafe {
            idt[InterruptIndex::Timer as u8]
                .set_handler_fn(interrupts::timer_handler_naked)
                .set_stack_index(gdt::TIMER_INTERRUPT_INDEX);
            idt[InterruptIndex::Keyboard as u8]
                .set_handler_fn(keyboard::keyboard_interrupt_handler);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
    // SAFETY: We have configured the PICS properly
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// End of interrupt -- must be called after every interrupt is handled.
#[inline]
fn eoi(interrupt: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt as u8);
    }
}

mod exceptions {
    use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

    use crate::hlt_loop;

    pub extern "x86-interrupt" fn page_fault_handler(
        stack_frame: InterruptStackFrame,
        error_code: PageFaultErrorCode,
    ) {
        use x86_64::registers::control::Cr2;

        println!("EXCEPTION: PAGE FAULT");
        println!("Accessed Address: {:?}", Cr2::read());
        println!("Error Code: {:?}", error_code);
        println!("{:#?}", stack_frame);
        hlt_loop();
    }

    pub extern "x86-interrupt" fn general_protection_fault_handler(
        stack_frame: InterruptStackFrame,
        _error_code: u64,
    ) {
        panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
    }

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
