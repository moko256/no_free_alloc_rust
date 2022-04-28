use std::alloc::{GlobalAlloc, System};

struct NoFreeAllocator;

unsafe impl GlobalAlloc for NoFreeAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        System.alloc(layout)
    }

    #[inline(always)]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {}
}

#[global_allocator]
static A: NoFreeAllocator = NoFreeAllocator;
