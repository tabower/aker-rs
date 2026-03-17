use core::alloc::GlobalAlloc;
use core::alloc::Layout;

struct RustGlobalAlloc;

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        panic!(
            "unexpected global alloc: use custom allocator instead"
        )
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!(
            "unexpected global dealloc: use custom allocator instead"
        )
    }
}

#[global_allocator]
static GLOBAL: RustGlobalAlloc = RustGlobalAlloc;
