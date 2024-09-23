use core::arch::asm;

use x86_64::structures::idt::InterruptStackFrame;

use crate::process;

use super::{eoi, Context, InterruptIndex};

macro_rules! wrap_naked {
    ($func: ident, $naked: ident) => {
        #[naked]
        pub extern "x86-interrupt" fn timer_handler_naked(_stack_frame: InterruptStackFrame) {
            unsafe {
                asm!(
                    // Disable interrupts
                    "cli",
                    // Push registers
                    "push rax",
                    "push rbx",
                    "push rcx",
                    "push rdx",

                    "push rdi",
                    "push rsi",
                    "push rbp",
                    "push r8",

                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",

                    "push r13",
                    "push r14",
                    "push r15",

                    // First argument in rdi with C calling convention
                    "mov rdi, rsp",
                    // Call the hander function
                    "call {handler}",

                    // New: stack pointer is in RAX
                    "cmp rax, 0",
                    "je 2f",        // if rax != 0 {
                    "mov rsp, rax", //   rsp = rax;
                    "2:",           // }

                    // Pop scratch registers
                    "pop r15",
                    "pop r14",
                    "pop r13",

                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",

                    "pop r8",
                    "pop rbp",
                    "pop rsi",
                    "pop rdi",

                    "pop rdx",
                    "pop rcx",
                    "pop rbx",
                    "pop rax",
                    // Enable interrupts
                    "sti",
                    // Interrupt return
                    "iretq",
                    // Note: Getting the handler pointer here using `sym` operand, because
                    // an `in` operand would clobber a register that we need to save, and we
                    // can't have two asm blocks
                    handler = sym $func,
                    options(noreturn)
                );
            }
        }
    }
}

wrap_naked!(timer_handler, timer_handler_naked);

extern "C" fn timer_handler(context: &mut Context) -> usize {
    print!("+");

    let next_stack = process::schedule_next(context);

    eoi(InterruptIndex::Timer);
    next_stack
}
