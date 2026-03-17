use crate::arch::mm::PAGE_SIZE;
use crate::prelude::*;

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

    #[inline(always)]
    pub(super) fn end(&mut self) -> usize {
        self.next_free
    }
}
