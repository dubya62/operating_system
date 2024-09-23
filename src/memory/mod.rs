use bootloader::{
    bootinfo::{MemoryMap, MemoryRegionType},
    BootInfo,
};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub mod allocator;
pub mod kernel_info;

static mut MEMORY_INFO: Option<MemoryInfo> = None;

const THREAD_STACK_PAGE_INDEX: [u8; 3] = [5, 0, 0];

struct MemoryInfo {
    boot_info: &'static BootInfo,
    physical_memory_offset: VirtAddr,
    frame_allocator: BootInfoFrameAllocator,
    kernel_l4_table: &'static mut PageTable,
}

/// ## SAFETY
///
/// - The complete physical memory must be mapped to virtual memory at the passed
/// `physical_memory_offset`.
/// - This function must be only called once to avoid aliasing `&mut` references.
pub fn init(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();

    let mut mapper = unsafe { OffsetPageTable::new(&mut *virt.as_mut_ptr(), phys_mem_offset) };
    let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_map);

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let memory_info = MemoryInfo {
        boot_info,
        physical_memory_offset: phys_mem_offset,
        frame_allocator,
        kernel_l4_table: unsafe { active_level_4_table(phys_mem_offset) },
    };
    unsafe { MEMORY_INFO = Some(memory_info) };
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
    pub fn new(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        // TODO: This is so painful, we should just make [`BootInfoFrameAllocatior`] be an iterator
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// This should only be called from the init function because each call will result in a mutable
/// reference
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub fn allocate_pages_mapper(
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    start_addr: VirtAddr,
    size: u64,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let end_addr = start_addr + size - 1u64;
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::range_inclusive(start_page, end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    Ok(())
}

pub fn allocate_pages(
    level_4_table: *mut PageTable,
    start_addr: VirtAddr,
    size: u64,
    flags: PageTableFlags,
) -> Result<(), MapToError<Size4KiB>> {
    let memory_info = unsafe { MEMORY_INFO.as_mut() }.unwrap();

    let mut mapper =
        unsafe { OffsetPageTable::new(&mut *level_4_table, memory_info.physical_memory_offset) };

    dbg!();
    allocate_pages_mapper(
        &mut memory_info.frame_allocator,
        &mut mapper,
        start_addr,
        size,
        flags,
    )
}

/// Create a new page table
///
/// ## Returns
///
/// - A pointer to the PageTable (virtual address in kernel mapped pages)
/// - The physical address which can be written to cr3
fn create_empty_pagetable() -> (*mut PageTable, u64) {
    // Need to borrow as mutable so that we can allocate new frames
    // and so modify the frame allocator
    let memory_info = unsafe { MEMORY_INFO.as_mut().unwrap() };

    // Get a frame to store the level 4 table
    let level_4_table_frame = memory_info.frame_allocator.allocate_frame().unwrap();
    let phys = level_4_table_frame.start_address(); // Physical address
    let virt = memory_info.physical_memory_offset + phys.as_u64(); // Kernel virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // Clear all entries in the page table
    unsafe {
        (*page_table_ptr).zero();
    }

    (page_table_ptr, phys.as_u64())
}

/// Copy a set of pagetables
fn copy_pagetables(level_4_table: &PageTable) -> (*mut PageTable, u64) {
    // Create a new level 4 pagetable
    let (table_ptr, table_physaddr) = create_empty_pagetable();
    let table = unsafe { &mut *table_ptr };

    fn copy_pages_rec(
        physical_memory_offset: VirtAddr,
        from_table: &PageTable,
        to_table: &mut PageTable,
        level: u16,
    ) {
        for (i, entry) in from_table.iter().enumerate() {
            if !entry.is_unused() {
                if (level == 1) || entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                    // Maps a frame, not a page table
                    to_table[i].set_addr(entry.addr(), entry.flags());
                } else {
                    // Create a new table at level - 1
                    let (new_table_ptr, new_table_physaddr) = create_empty_pagetable();
                    let to_table_m1 = unsafe { &mut *new_table_ptr };

                    // Point the entry to the new table
                    to_table[i].set_addr(PhysAddr::new(new_table_physaddr), entry.flags());

                    // Get reference to the input level-1 table
                    let from_table_m1 = {
                        let virt = physical_memory_offset + entry.addr().as_u64();
                        unsafe { &*virt.as_ptr() }
                    };

                    // Copy level-1 entries
                    copy_pages_rec(
                        physical_memory_offset,
                        from_table_m1,
                        to_table_m1,
                        level - 1,
                    );
                }
            }
        }
    }

    let memory_info = unsafe { MEMORY_INFO.as_mut().unwrap() };
    copy_pages_rec(memory_info.physical_memory_offset, level_4_table, table, 4);

    return (table_ptr, table_physaddr);
}

/// Creates a PageTable containing only kernel pages
///
/// - Copies the kernel pagetables into a new set of tables
/// - Adds the KernelInfo read-only page
///
/// ## Returns
///
/// - A pointer to the PageTable (virtual address in kernel mapped pages)
/// - The physical address which can be written to cr3
pub fn create_new_user_pagetable() -> (*mut PageTable, u64) {
    dbg!();
    let memory_info = unsafe { MEMORY_INFO.as_mut().unwrap() };

    dbg!();
    // Copy kernel pages
    let (user_page_table_ptr, user_page_table_physaddr) =
        copy_pagetables(memory_info.kernel_l4_table);
    dbg!();

    // Add KernelInfo page
    let memory_info = unsafe { MEMORY_INFO.as_mut().unwrap() };
    let mut mapper = unsafe {
        OffsetPageTable::new(
            &mut *user_page_table_ptr,
            memory_info.physical_memory_offset,
        )
    };

    _ = kernel_info::add_to_user_table(&mut mapper, &mut memory_info.frame_allocator);

    (user_page_table_ptr, user_page_table_physaddr)
}

/// Allocate memory for a thread's user stack
///
/// Uses 8 pages per thread: 7 for user stack, one guard page
///
/// ## Returns
///
/// (user_stack_start, user_stack_end)
pub fn allocate_user_stack(level_4_table: *mut PageTable) -> Result<(u64, u64), &'static str> {
    let memory_info = unsafe { MEMORY_INFO.as_mut().unwrap() };

    let mut table = unsafe { &mut *level_4_table };
    for index in THREAD_STACK_PAGE_INDEX {
        let entry = &mut table[index as usize];
        if entry.is_unused() {
            // Page not allocated -> Create page table
            let (_new_table_ptr, new_table_physaddr) = create_empty_pagetable();
            entry.set_addr(
                PhysAddr::new(new_table_physaddr),
                PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE,
            );
        }
        table = unsafe {
            &mut *(memory_info.physical_memory_offset + entry.addr().as_u64()).as_mut_ptr()
        };
    }

    // Table should now be the level 1 page table
    //
    // Find an unused set of 8 pages. The lowest page is always unused
    // (guard), but the first should be used so look in pages
    // (1 + 8*n) where n=0..64
    //
    // Choose a random n to start looking, and check entries
    // sequentially from there. For now just use process::unique_id
    use crate::process;
    let n_start = process::unique_id(); // Modulo 64 soon
    for i in 0..64 {
        let n = ((n_start + i) % 64) as usize;

        if table[n * 8 + 1].is_unused() {
            // Found an empty slot:
            //  [n * 8] -> Empty (guard)
            //  [n * 8 + 1] -> User stack (read-only)
            //      ...
            //  [n * 8 + 7] -> User stack (writable)

            // Note: Only one frame is going to be allocated, and the rest
            //       are going to be read-only references to the same frame.
            //       When a thread tries to write to them a page fault will
            //       be triggered and the frame allocated.
            let frame = memory_info
                .frame_allocator
                .allocate_frame()
                .ok_or("Failed to allocate frame")?;

            for j in 1..7 {
                // These pages are read-only
                let entry = &mut table[n * 8 + j];
                entry.set_addr(
                    frame.start_address(),
                    PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE,
                );
            }
            let entry = &mut table[n * 8 + 7];
            entry.set_addr(
                frame.start_address(),
                PageTableFlags::PRESENT |
                           PageTableFlags::WRITABLE | // Note!
                           PageTableFlags::USER_ACCESSIBLE,
            );

            // Return the virtual addresses of the top of the kernel and user stacks
            let slot_address: u64 = ((THREAD_STACK_PAGE_INDEX[0] as u64) << 39)
                + ((THREAD_STACK_PAGE_INDEX[1] as u64) << 30)
                + ((THREAD_STACK_PAGE_INDEX[2] as u64) << 21)
                + (((n * 8) as u64) << 12);

            return Ok((slot_address + 4096, slot_address + 8 * 4096)); // User stack
        }
    }

    Err("All thread stack slots full")
}
