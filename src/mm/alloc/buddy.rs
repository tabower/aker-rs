use crate::prelude::*;

use crate::mm::addr::PhysPageNum;

use super::Order;
use super::PageAllocator;

pub struct Buddy {}

unsafe impl PageAllocator for Buddy {
    fn alloc_pages(
        order: Order,
        flags: super::AllocFlags,
        policy: super::NumaPolicy,
    ) -> KResult<PhysPageNum> {
        todo!()
    }

    unsafe fn free_pages(ppn: PhysPageNum, order: Order) {
        todo!()
    }
}
