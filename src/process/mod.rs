use core::sync::atomic::{AtomicU64, Ordering};

use alloc::{boxed::Box, collections::vec_deque::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use object::{Object, ObjectSegment};
use spin::RwLock;
use x86_64::{structures::paging::PageTableFlags, VirtAddr};

use crate::interrupts::INTERRUPT_CONTEXT_SIZE;
use crate::memory;
use crate::{
    gdt,
    interrupts::{self, Context},
};

const UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);

const USER_CODE_START: u64 = 0x5000000;
const USER_CODE_END: u64 = 0x80000000;

const KERNEL_STACK_SIZE: usize = 4096 * 2;
const USER_STACK_SIZE: usize = 4096 * 5;

const USER_HEAP_START: u64 = 0x280_0060_0000;
const USER_HEAP_SIZE: u64 = 4 * 1024 * 1024; //0x28002e00000 - 0x28000600000;

lazy_static! {
    static ref RUNNING_QUEUE: RwLock<VecDeque<Box<Thread>>> = RwLock::new(VecDeque::new());
    static ref CURRENT_THREAD: RwLock<Option<Box<Thread>>> = RwLock::new(None);
}

#[derive(Debug)]
pub struct Thread {
    kernel_stack: Vec<u8>,
    kernel_stack_end: u64, // This address goes in the TSS
    user_stack_end: usize,
    context: *mut Context, // Address of Context on kernel stack
}

// Trust in the developer ;)
unsafe impl Sync for Thread {}
unsafe impl Send for Thread {}

impl Thread {
    /// SAFETY: When a [`Thread`] is created, one must ensure that the `context` field is valid
    unsafe fn context_mut(&mut self) -> &mut Context {
        &mut *self.context
    }
}

pub fn new_kernel_thread(function: fn() -> ()) {
    let new_thread = {
        let kernel_stack = Vec::with_capacity(KERNEL_STACK_SIZE);
        let kernel_stack_end =
            (VirtAddr::from_ptr(kernel_stack.as_ptr()) + KERNEL_STACK_SIZE as u64).as_u64();
        let user_stack = Vec::with_capacity(USER_STACK_SIZE);
        let user_stack_end =
            (VirtAddr::from_ptr(user_stack.as_ptr()) + USER_STACK_SIZE as u64).as_u64() as usize;
        let context = kernel_stack_end - interrupts::INTERRUPT_CONTEXT_SIZE as u64;

        Box::new(Thread {
            kernel_stack,
            user_stack,
            kernel_stack_end,
            user_stack_end,
            context,
        })
    };

    // Set context registers
    let context = unsafe { &mut *(new_thread.context as *mut Context) };
    context.rip = function as usize; // Instruction pointer
    context.rsp = new_thread.user_stack_end; // Stack pointer
    context.rflags = 0x200; // Interrupts enabled

    let (code_selector, data_selector) = gdt::get_kernel_segments();
    context.cs = code_selector.0 as usize;
    context.ss = data_selector.0 as usize;

    // Add Thread to RUNNING_QUEUE
    x86_64::instructions::interrupts::without_interrupts(|| {
        RUNNING_QUEUE.write().push_back(new_thread);
    });
}

pub fn schedule_next(context: &Context) -> usize {
    let mut running_queue = RUNNING_QUEUE.write();
    let mut current_thread = CURRENT_THREAD.write();

    if let Some(mut thread) = current_thread.take() {
        // Save the location of the Context struct
        thread.context = context as *const Context as u64;
        // Put to the back of the queue
        running_queue.push_back(thread);
    }
    // Get the next thread in the queue
    *current_thread = running_queue.pop_front();
    match current_thread.as_ref() {
        Some(thread) => {
            // Set the kernel stack for the next interrupt
            gdt::set_interrupt_stack_table(
                gdt::TIMER_INTERRUPT_INDEX as usize,
                VirtAddr::new(thread.kernel_stack_end),
            );
            // Point the stack to the new context
            thread.context as usize
        }
        None => 0, // Timer handler won't modify stack
    }
}

pub fn new_user_thread(bin: &[u8]) -> Result<usize, &'static str> {
    // Check the header
    const ELF_MAGIC: [u8; 4] = *b"\x7fELF";

    if bin[0..4] != ELF_MAGIC {
        return Err("Invalid ELF header");
    }

    // Use the object crate to parse the ELF file
    // https://crates.io/crates/object
    let Ok(obj) = object::File::parse(bin) else {
        return Err("Could not parse ELF");
    };

    let entry_point = obj.entry();
    println!("Entry point: {:#016X}", entry_point);

    // Create a user pagetable with only kernel pages
    let (user_page_table_ptr, user_page_table_physaddr) = memory::create_new_user_pagetable();

    for segment in obj.segments() {
        let segment_address = segment.address() as u64;

        println!("Section {:?} : {:#016X}", segment.name(), segment_address);

        let start_address = VirtAddr::new(segment_address);
        let end_address = start_address + segment.size() as u64;
        if !(start_address..=end_address).contains(&VirtAddr::new(USER_CODE_START)) {
            return Err("ELF segment outside allowed range");
        }

        // Allocate memory in the pagetable
        if memory::allocate_pages(
            user_page_table_ptr,
            VirtAddr::new(segment_address), // Start address
            segment.size() as u64,          // Size (bytes)
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE,
        )
        .is_err()
        {
            return Err("Could not allocate memory");
        }

        if let Ok(data) = segment.data() {
            // Copy data
            let dest_ptr = segment_address as *mut u8;
            for (i, value) in data.iter().enumerate() {
                unsafe {
                    let ptr = dest_ptr.add(i);
                    core::ptr::write(ptr, *value);
                }
            }
        }
    }

    // Create the new Thread struct
    let mut new_thread = {
        // Note: Kernel stack needs to be mapped in all pages
        //       because the page table will be changed during
        //       context switch
        let kernel_stack = Vec::with_capacity(KERNEL_STACK_SIZE);
        let kernel_stack_start = VirtAddr::from_ptr(kernel_stack.as_ptr());
        let kernel_stack_end = (kernel_stack_start + KERNEL_STACK_SIZE as u64).as_u64();

        // Allocate user stack
        let (_user_stack_start, user_stack_end) = memory::allocate_user_stack(user_page_table_ptr)?;

        Box::new(Thread {
            kernel_stack,
            kernel_stack_end,
            user_stack_end: user_stack_end as usize,
            // Note that stacks move backwards, so SP points to the end
            context: (kernel_stack_end - INTERRUPT_CONTEXT_SIZE as u64) as *mut Context,
            // tid: unique_id(),
            // Push a Context struct on the kernel stack
            // process: Arc::new(RwLock::new(Process {
            //     page_table_physaddr: user_page_table_physaddr,
            //     handles: handles.drain(..).map(|h| Some(h)).collect(),
            //     mounts: params.mounts,
            // })),
            // page_table_physaddr: user_page_table_physaddr,
        })
    };

    // Cast context address to Context struct
    let context = unsafe { new_thread.context_mut() };

    // context.rip = entry_point as usize;

    // // // Set flags
    // // context.rflags = if params.io_privileges {
    // //     0x200 + 0x3000 // Interrupt enable + IOPL 3
    // // } else {
    // //     0x200 // Interrupt enable
    // // };

    let (code_selector, data_selector) = gdt::get_user_segments();
    context.cs = code_selector.0 as usize; // Code segment flags
    context.ss = data_selector.0 as usize; // Without this we get a GPF

    // // Note: Need to point to the end of the allocated region
    // //       because the stack moves down in memory
    // context.rsp = new_thread.user_stack_end as usize;

    // Modify the context to pass information to the new thread
    context.rax = USER_HEAP_START as usize;
    context.rcx = USER_HEAP_SIZE as usize;

    Ok(0)
}

pub fn unique_id() -> u64 {
    UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed)
}
