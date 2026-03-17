// use core::marker::PhantomData;
// use core::ptr::NonNull;

// use crate::mm::alloc::PageAllocator;
// use crate::prelude::*;

// use super::kmem_cache::KMemCache;

// #[repr(transparent)]
// pub struct TypedCache<T, A: PageAllocator> {
//     inner: KMemCache<A>,
//     _type: PhantomData<T>,
// }

// unsafe impl<T, A: PageAllocator> Send for TypedCache<T, A> {}
// unsafe impl<T, A: PageAllocator> Sync for TypedCache<T, A> {}

// impl<T, A: PageAllocator> TypedCache<T, A> {
//     pub const fn new() -> Self {
//         Self {
//             inner: KMemCache::new(size_of::<T>(), align_of::<T>()),
//             _type: PhantomData,
//         }
//     }

//     #[inline(always)]
//     pub fn alloc(&mut self) -> KResult<NonNull<T>> {
//         self.inner.alloc_raw().map(|p| p.cast())
//     }

//     #[inline(always)]
//     pub unsafe fn free(&mut self, ptr: NonNull<T>) {
//         self.inner.free_raw(ptr.cast())
//     }

//     #[inline(always)]
//     pub unsafe fn destroy(&mut self) {
//         unsafe { self.inner.destroy() }
//     }
// }
