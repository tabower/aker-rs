use crate::arch::mm::PAGE_SIZE;

pub(super) struct BootAllocator {
    next_free: usize,
}

impl BootAllocator {
    #[inline(always)]
    pub(super) fn new(base: usize) -> Self {
        let aligned =
            (base.wrapping_add(PAGE_SIZE - 1)) & !(PAGE_SIZE - 1);
        Self { next_free: aligned }
    }

    #[inline(always)]
    pub(super) fn alloc(&mut self) -> usize {
        let paddr = self.next_free;
        self.next_free = self.next_free.wrapping_add(PAGE_SIZE);

        unsafe {
            core::ptr::write_bytes(paddr as *mut u8, 0, PAGE_SIZE);
        }
        paddr
    }

    /// Obtain the end position of the current allocator (the next
    /// available physical address)
    #[inline(always)]
    pub(super) fn end(&self) -> usize {
        self.next_free
    }
}
