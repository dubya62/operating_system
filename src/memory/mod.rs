use bootloader::{
    bootinfo::{MemoryMap, MemoryRegionType},
    BootInfo,
};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub mod allocator;

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
}

/// Creates an example mapping for the given page to frame `0xb8000`.
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
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
