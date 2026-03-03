//! Kernel Memory Allocator Traits
//!
//! All methods take `&self` (interior mutability via SpinLock etc.)
//! so that RAII wrappers can call free in Drop.

use core::alloc::Layout;
use core::fmt;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;

use crate::prelude::*;

use super::super::addr::PhysAddr;
use super::super::numa;

use super::flags::AllocFlags;
use super::numa_policy::NumaPolicy;

/// Physical page frame allocator
///
/// Implementors must use interior mutability (e.g. `SpinLock<Inner>`)
/// because all methods take `&self`, not `&mut self`.
/// This is required so `PageBox` can hold `&dyn PageAllocator` and
/// call `free_pages` in its `Drop`.
pub trait PageAllocator {
    /// Allocate 2^order contiguous physical page frames.
    ///
    /// - `order`: allocation order (2^order pages)
    /// - `nid`: target NUMA node
    /// - `policy`: fallback behavior
    fn alloc_pages(
        &self,
        order: u8,
        flags: AllocFlags,
        nid: numa::NId,
        policy: NumaPolicy,
    ) -> KResult<PageBox<'_>>;

    /// Free 2^order contiguous physical page frames.
    ///
    /// No `nid` parameter needed — the physical address itself
    /// determines which node it belongs to.
    fn free_pages(&self, pa: PhysAddr, order: u8) -> KResult<()>;

    /// Allocate on the current node, allow fallback.
    fn alloc_pages_local(
        &self,
        order: u8,
        flags: AllocFlags,
    ) -> KResult<PageBox<'_>> {
        let nid = numa::current_node();
        self.alloc_pages(order, flags, nid, NumaPolicy::Preferred)
    }

    /// Allocate strictly on the specified node. No fallback.
    fn alloc_pages_exact(
        &self,
        order: u8,
        flags: AllocFlags,
        nid: numa::NId,
    ) -> KResult<PageBox<'_>> {
        self.alloc_pages(order, flags, nid, NumaPolicy::Strict)
    }

    /// Single page, current node, allow fallback.
    fn alloc_page(&self, flags: AllocFlags) -> KResult<PageBox<'_>> {
        self.alloc_pages_local(0, flags)
    }

    /// Single page, strictly on specified node.
    fn alloc_page_exact(
        &self,
        flags: AllocFlags,
        nid: numa::NId,
    ) -> KResult<PageBox<'_>> {
        self.alloc_pages_exact(0, flags, nid)
    }
}

/// Owns 2^order contiguous physical page frames.
///
/// - Automatically calls `free_pages` on `Drop`
/// - Use `.phys_addr()` to borrow the address for page table mapping
pub struct PageBox<'a> {
    base: PhysAddr,
    order: u8,
    allocator: &'a (dyn PageAllocator + Sync),
}

impl<'a> PageBox<'a> {
    /// Only constructable by `PageAllocator` implementations.
    pub(crate) fn new(
        base: PhysAddr,
        order: u8,
        allocator: &'a (dyn PageAllocator + Sync),
    ) -> Self {
        Self {
            base,
            order,
            allocator,
        }
    }

    /// Borrow the starting physical address.
    #[inline(always)]
    pub fn phys_addr(&self) -> PhysAddr {
        self.base
    }

    /// Allocation order.
    #[inline(always)]
    pub fn order(&self) -> u8 {
        self.order
    }

    /// Number of pages (2^order).
    #[inline(always)]
    pub fn page_count(&self) -> usize {
        1 << self.order
    }

    /// Consume self, extract raw `(PhysAddr, order)`.
    ///
    /// Prevents `Drop` from running. Caller takes responsibility
    /// for eventually calling `free_pages`.
    pub fn into_raw(self) -> (PhysAddr, u8) {
        let base = self.base;
        let order = self.order;
        core::mem::forget(self);
        (base, order)
    }
}

impl fmt::Debug for PageBox<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageBox")
            .field("base", &self.base)
            .field("order", &self.order)
            .field("page_count", &self.page_count())
            .finish()
    }
}

impl Drop for PageBox<'_> {
    fn drop(&mut self) {
        if let Err(e) =
            self.allocator.free_pages(self.base, self.order)
        {
            panic!(
                "PageBox::drop: free_pages({:?}, order={}) failed: {:?}",
                self.base, self.order, e
            );
        }
    }
}

unsafe impl<'a> Send for PageBox<'a> {}
unsafe impl<'a> Sync for PageBox<'a> {}

/// Kernel object allocator
///
/// Same `&self` convention as `PageAllocator` (interior mutability).
pub trait KAllocator {
    /// Allocate raw memory with the given layout.
    fn alloc_raw(
        &self,
        layout: Layout,
        flags: AllocFlags,
        nid: numa::NId,
        policy: NumaPolicy,
    ) -> KResult<NonNull<u8>>;

    /// Free raw memory.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been returned by `alloc_raw` with the same
    ///   layout.
    /// - `ptr` current_nodemust not have been freed before.
    unsafe fn free_raw(
        &self,
        ptr: NonNull<u8>,
        layout: Layout,
    ) -> KResult<()>;

    /// Full-parameter: allocate memory, write `val`, return `ObjBox`.
    fn alloc_with<T>(
        &self,
        val: T,
        flags: AllocFlags,
        nid: numa::NId,
        policy: NumaPolicy,
    ) -> KResult<KBox<'_, T>>
    where
        Self: Sized,
        Self: Sync,
    {
        let ptr = self
            .alloc_raw(Layout::new::<T>(), flags, nid, policy)?
            .cast::<T>();
        unsafe { ptr.as_ptr().write(val) };
        Ok(KBox::new(ptr, self))
    }

    /// Allocate T on the current node, allow fallback.
    fn alloc<T>(
        &self,
        val: T,
        flags: AllocFlags,
    ) -> KResult<KBox<'_, T>>
    where
        Self: Sized,
        Self: Sync,
    {
        let nid = numa::current_node();
        self.alloc_with(val, flags, nid, NumaPolicy::Preferred)
    }

    /// Allocate T strictly on the specified node.
    fn alloc_exact<T>(
        &self,
        val: T,
        flags: AllocFlags,
        nid: numa::NId,
    ) -> KResult<KBox<'_, T>>
    where
        Self: Sized,
        Self: Sync,
    {
        self.alloc_with(val, flags, nid, NumaPolicy::Strict)
    }
}

/// Owns a heap-allocated `T`. Kernel equivalent of `Box<T>`.
///
/// - Not `Clone`, not `Copy` — ownership is unique
/// - Implements `Deref`/`DerefMut` for transparent access
/// - `Drop` calls `drop_in_place(T)` then `free_raw`
pub struct KBox<'a, T> {
    ptr: NonNull<T>,
    allocator: &'a (dyn KAllocator + Sync),
}

impl<'a, T> KBox<'a, T> {
    /// Only constructable by `ObjectAllocator` implementations.
    pub(crate) fn new(
        ptr: NonNull<T>,
        allocator: &'a (dyn KAllocator + Sync),
    ) -> Self {
        Self { ptr, allocator }
    }

    /// Raw const pointer to the inner value.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Raw mut pointer to the inner value.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Consume self, extract raw `NonNull<T>`.
    ///
    /// Prevents `Drop` from running. Caller takes responsibility
    /// for eventually dropping T and calling `free_raw`.
    pub fn into_raw(self) -> NonNull<T> {
        let ptr = self.ptr;
        core::mem::forget(self);
        ptr
    }
}

impl<T> Deref for KBox<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for KBox<'_, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: fmt::Debug> fmt::Debug for KBox<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.deref(), f)
    }
}

impl<T: fmt::Display> fmt::Display for KBox<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

impl<T> Drop for KBox<'_, T> {
    fn drop(&mut self) {
        unsafe {
            // 1. Drop the inner value
            core::ptr::drop_in_place(self.ptr.as_ptr());

            // 2. Free the memory
            if let Err(e) = self
                .allocator
                .free_raw(self.ptr.cast(), Layout::new::<T>())
            {
                panic!(
                    "ObjBox::drop: free_raw({}) failed: {:?}",
                    core::any::type_name::<T>(),
                    e
                );
            }
        }
    }
}

unsafe impl<'a, T: Send> Send for KBox<'a, T> {}
unsafe impl<'a, T: Sync> Sync for KBox<'a, T> {}
