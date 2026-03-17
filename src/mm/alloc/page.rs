use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::prelude::*;

use crate::mm::addr::PhysPageNum;

use super::AllocFlags;
use super::NumaPolicy;
use super::Order;

/// Physical page frame allocator.
pub unsafe trait PageAllocator: Sized {
    fn alloc_pages(
        order: Order,
        flags: AllocFlags,
        policy: NumaPolicy,
    ) -> KResult<PhysPageNum>;

    /// Core deallocation primitive: Deallocates 2^order consecutive
    /// physical pages.
    ///
    /// # Safety
    /// 1. `ppn` must be allocated by this allocator and must be
    ///    active.
    /// 2. `order` must match the value specified at allocation
    ///    exactly.
    /// 3. The caller must ensure that these physical pages will no
    ///    longer be accessed.
    unsafe fn free_pages(ppn: PhysPageNum, order: Order);
}

/// Owns 2^order contiguous physical page frames.
pub struct PageBox<A: PageAllocator> {
    ppn: PhysPageNum,
    order: Order,
    _alloc: PhantomData<A>,
}

impl<A: PageAllocator> PageBox<A> {
    pub fn new_numa(
        order: Order,
        policy: NumaPolicy,
        flags: AllocFlags,
    ) -> KResult<Self> {
        let ppn = A::alloc_pages(order, flags, policy)?;
        Ok(Self {
            ppn,
            order,
            _alloc: PhantomData,
        })
    }

    #[inline(always)]
    pub fn new_numa_size(
        size: usize,
        policy: NumaPolicy,
        flags: AllocFlags,
    ) -> KResult<Self> {
        let order = Order::from_size(size);
        Self::new_numa(order, policy, flags)
    }

    #[inline(always)]
    pub fn new(order: Order, flags: AllocFlags) -> KResult<Self> {
        Self::new_numa(order, NumaPolicy::Preferred, flags)
    }

    #[inline(always)]
    pub fn new_numa_page(
        policy: NumaPolicy,
        flags: AllocFlags,
    ) -> KResult<Self> {
        Self::new_numa(Order::new(0), policy, flags)
    }

    #[inline(always)]
    pub fn new_page(flags: AllocFlags) -> KResult<Self> {
        Self::new_numa(Order::new(0), NumaPolicy::Preferred, flags)
    }

    #[inline(always)]
    pub fn new_size(size: usize, flags: AllocFlags) -> KResult<Self> {
        Self::new_numa_size(size, NumaPolicy::Preferred, flags)
    }

    #[inline(always)]
    pub const fn ppn(&self) -> PhysPageNum {
        self.ppn
    }

    #[inline(always)]
    pub const fn size(&self) -> usize {
        self.order.byte_size()
    }

    #[inline(always)]
    pub const fn page_count(&self) -> usize {
        self.order.page_count()
    }

    /// Convert to a specific type reference (caller-safe)
    pub const unsafe fn cast<T>(&self) -> &T {
        unsafe {
            NonNull::new(
                self.ppn.to_addr().to_virt().as_mut_ptr::<T>(),
            )
            .unwrap()
            .as_ref()
        }
    }
}

impl<A: PageAllocator> core::fmt::Debug for PageBox<A> {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        f.debug_struct("PageBox")
            .field("ppn", &self.ppn)
            .field("order", &self.order)
            .field("page_count", &self.page_count())
            .finish()
    }
}

impl<A: PageAllocator> Drop for PageBox<A> {
    fn drop(&mut self) {
        unsafe {
            A::free_pages(self.ppn, self.order);
        }
    }
}
