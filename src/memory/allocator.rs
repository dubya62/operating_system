use core::{
    alloc::{GlobalAlloc, Layout},
    mem,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use spin::Mutex;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

/// Literally just a wrapper around a Mutex
// We can't just use a Mutex because rust does not allow implementing traits for external crates
// ... no `impl GlobalAlloc for spin::Mutex<T>` :(
struct Locked<T>(Mutex<T>);

impl<T> Locked<T> {
    const fn new(t: T) -> Self {
        Self(Mutex::new(t))
    }
}

impl<T> Deref for Locked<T> {
    type Target = Mutex<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Locked<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

struct ListNode {
    next: Option<&'static mut ListNode>,
}

struct FixedSizeBlockAllocator {
    heads: [Option<&'static mut ListNode>; Self::BLOCK_SIZES.len()],
    fallback_alloc: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    /// The block sizes to use.
    // The sizes must each be power of 2 because they are also used as the block alignment.
    const BLOCK_SIZES: [usize; 9] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048];

    const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            heads: [EMPTY; Self::BLOCK_SIZES.len()],
            fallback_alloc: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initialize the allocator with the given heap bounds.
    unsafe fn init(&mut self, heap_start: *mut u8, heap_size: usize) {
        self.fallback_alloc.init(heap_start, heap_size);
    }

    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        self.fallback_alloc
            .allocate_first_fit(layout)
            .map(|p| p.as_ptr())
            .unwrap_or(ptr::null_mut())
    }

    /// Choose an appropriate block size for the given layout.
    fn list_index(layout: &Layout) -> Option<usize> {
        let size = layout.size().max(layout.align());
        Self::BLOCK_SIZES.iter().position(|&s| s >= size)
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let Some(index) = Self::list_index(&layout) else {
            return self.fallback_alloc(layout);
        };

        if let Some(node) = self.heads[index].take() {
            self.heads[index] = node.next.take();
            node as *mut ListNode as *mut u8
        } else {
            let size = Self::BLOCK_SIZES[index];
            self.fallback_alloc(Layout::from_size_align(size, size).unwrap())
        }
    }

    fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let Some(index) = Self::list_index(&layout) else {
            let ptr = NonNull::new(ptr).unwrap();
            unsafe { self.fallback_alloc.deallocate(ptr, layout) };
            return;
        };

        // verify that block has size and alignment required for storing node
        assert!(mem::size_of::<ListNode>() <= Self::BLOCK_SIZES[index]);
        assert!(mem::align_of::<ListNode>() <= Self::BLOCK_SIZES[index]);

        let new_node_ptr = ptr as *mut ListNode;
        unsafe {
            new_node_ptr.write(ListNode {
                next: self.heads[index].take(),
            })
        };
        self.heads[index] = Some(unsafe { &mut *new_node_ptr });
    }
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        Page::range_inclusive(
            Page::containing_address(heap_start),
            Page::containing_address(heap_end),
        )
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }
    Ok(())
}
