use core::marker::PhantomData;

use crate::prelude::*;

use crate::mm::alloc::PageAllocator;

pub struct KMemCache<A: PageAllocator> {
    obj_size: usize,
    obj_align: usize,
    capacity: usize,
    partial: QueueList,
    _alloc: PhantomData<A>,
}

unsafe impl<A: PageAllocator> Send for KMemCache<A> {}
unsafe impl<A: PageAllocator> Sync for KMemCache<A> {}

impl<A: PageAllocator> KMemCache<A> {
    pub const fn new(_obj_size: usize, _obj_align: usize) -> Self {
        todo!()
    }

    pub unsafe fn destroy(&mut self) {
        todo!()
    }
}
