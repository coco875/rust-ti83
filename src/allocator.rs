use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn wrapper_malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn wrapper_realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

pub struct LibcAllocator;

unsafe impl GlobalAlloc for LibcAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        wrapper_malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        wrapper_realloc(ptr, new_size)
    }
}

#[global_allocator]
static ALLOCATOR: LibcAllocator = LibcAllocator;
