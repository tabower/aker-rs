// use core::alloc::Layout;
// use core::marker::PhantomData;
// use core::mem::MaybeUninit;
// use core::ops::Deref;
// use core::ops::DerefMut;
// use core::ptr::NonNull;
//
// use crate::mm::allocator::PageAllocator;
// use crate::prelude::*;
//
// use super::flags::AllocFlags;
// use super::numa_policy::NumaPolicy;
//
// /// Kernel object allocator
// pub unsafe trait KAllocator {
//     /// Allocate raw memory with the given layout.
//     fn alloc(
//         layout: Layout,
//         policy: NumaPolicy,
//         flags: AllocFlags,
//     ) -> KResult<NonNull<u8>>;
//
//     /// Free raw memory.
//     ///
//     /// # Safety
//     ///
//     /// - `ptr` must have been returned by `alloc_raw` with the
// same     ///   layout.
//     /// - `ptr` must have been previously deallocated and must no
//     ///   longer
//     /// be in use
//     unsafe fn dealloc(ptr: NonNull<u8>, layout: Layout);
// }
//
// pub struct KBox<T, A: KAllocator> {
//     ptr: NonNull<T>,
//     _alloc: PhantomData<A>,
// }
//
// impl<T, A: KAllocator> KBox<T, A> {
//     pub fn new_with<F>(f: F, flags: AllocFlags) -> KResult<Self>
//     where
//         F: FnOnce(&mut MaybeUninit<T>) -> KResult<()>,
//     {
//         let layout = Layout::new::<T>();
//         let raw = A::alloc(layout, NumaPolicy::Local, flags)?;
//         let ptr = raw.cast::<T>();
//
//         let uninit =
//             unsafe { &mut *(ptr.as_ptr() as *mut MaybeUninit<T>) };
//
//         match f(uninit) {
//             Ok(()) => Ok(Self {
//                 ptr,
//                 _alloc: PhantomData,
//             }),
//             Err(e) => {
//                 unsafe {
//                     A::dealloc(raw, layout);
//                 }
//                 Err(e)
//             }
//         }
//     }
//
//     /// In the worst case, `val` is first constructed on the stack
// and     /// then copied to the heap. For large objects (such as
//     /// structs containing large arrays), this may cause the kernel
//     /// stack to overflow. For large objects, please use
//     /// [`new_with`](Self::new_with) to construct them in-place.
//     pub fn new(val: T, flags: AllocFlags) -> KResult<Self> {
//         let layout = Layout::new::<T>();
//         let raw = A::alloc(layout, NumaPolicy::Local, flags)?;
//         let ptr = raw.cast::<T>();
//         unsafe {
//             ptr.as_ptr().write(val);
//         }
//         Ok(Self {
//             ptr,
//             _alloc: PhantomData,
//         })
//     }
// }
//
// impl<T, A: KAllocator> Drop for KBox<T, A> {
//     fn drop(&mut self) {
//         unsafe {
//             core::ptr::drop_in_place(self.ptr.as_ptr());
//             A::dealloc(self.ptr.cast(), Layout::new::<T>());
//         }
//     }
// }
//
// impl<T, A: KAllocator> Deref for KBox<T, A> {
//     type Target = T;
//
//     #[inline(always)]
//     fn deref(&self) -> &T {
//         unsafe { self.ptr.as_ref() }
//     }
// }
//
// impl<T, A: KAllocator> DerefMut for KBox<T, A> {
//     #[inline(always)]
//     fn deref_mut(&mut self) -> &mut T {
//         unsafe { self.ptr.as_mut() }
//     }
// }
//
// impl<T: core::fmt::Display, A: KAllocator> core::fmt::Display
//     for KBox<T, A>
// {
//     fn fmt(
//         &self,
//         f: &mut core::fmt::Formatter<'_>,
//     ) -> core::fmt::Result {
//         core::fmt::Display::fmt(self.deref(), f)
//     }
// }
//
//
//
//
//
//
//
//
//
//
//
//
//
// pub struct Kmalloc {}
//
// unsafe impl KAllocator for Kmalloc {
//     fn alloc(
//             layout: Layout,
//             policy: NumaPolicy,
//             flags: AllocFlags,
//         ) -> KResult<NonNull<u8>> {
//         // current cpu
//         todo!()
//     }
//
//     unsafe fn dealloc(ptr: NonNull<u8>, layout: Layout) {
//         todo!()
//     }
// }
