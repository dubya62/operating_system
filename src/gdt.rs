use core::ptr::addr_of;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{
    instructions::tables,
    registers::segmentation::{Segment, CS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_FAULT_IST_INDEX: u16 = 0;
pub const GENERAL_PROTECTION_FAULT_IST_INDEX: u16 = 0;
pub const TIMER_INTERRUPT_INDEX: u16 = 1;

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
    user_code_selector: SegmentSelector,
    user_data_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: Mutex<TaskStateSegment> = {
        let mut tss = TaskStateSegment::new();
        let t = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = t;
        tss.interrupt_stack_table[TIMER_INTERRUPT_INDEX as usize] = t;

        Mutex::new(tss)
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(unsafe { tss_reference() }));
        let user_code_selector = gdt.append(Descriptor::user_code_segment());
        let user_data_selector = gdt.append(Descriptor::user_data_segment());
        (
            gdt,
            Selectors {
                code_selector,
                data_selector,
                tss_selector,
                user_code_selector,
                user_data_selector,
            },
        )
    };
}

pub fn get_kernel_segments() -> (SegmentSelector, SegmentSelector) {
    (GDT.1.code_selector, GDT.1.data_selector)
}

pub fn get_user_segments() -> (SegmentSelector, SegmentSelector) {
    (GDT.1.user_code_selector, GDT.1.user_data_selector)
}

pub fn init() {
    GDT.0.load();
    // SAFETY: These are valid selectors, because we just created them.
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        tables::load_tss(GDT.1.tss_selector);
    }
}

unsafe fn tss_reference() -> &'static TaskStateSegment {
    let tss_ptr = &*TSS.lock() as *const TaskStateSegment;
    &*tss_ptr
}

pub fn set_interrupt_stack_table(index: usize, stack_end: VirtAddr) {
    TSS.lock().interrupt_stack_table[index] = stack_end;
}
