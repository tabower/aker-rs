// use super::kmem_cache::KMemCache;
// use super::super::buddy::Buddy;
// use crate::prelude::*;
// use core::ptr::NonNull;
//
// const MIN_ORDER: u32 = 4; // 16 bytes
// const MAX_ORDER: u32 = 11; // 2048 bytes
// const CACHE_COUNT: usize = (MAX_ORDER - MIN_ORDER + 1) as usize;
//
// static mut KMALLOC_CACHES: [KMemCache<Buddy>; CACHE_COUNT] = [
//     KMemCache::new(16, 16),
//     KMemCache::new(32, 32),
//     KMemCache::new(64, 64),
//     KMemCache::new(128, 128),
//     KMemCache::new(256, 256),
//     KMemCache::new(512, 512),
//     KMemCache::new(1024, 1024),
//     KMemCache::new(2048, 2048),
// ];
//
//
// #[inline(always)]
// const fn size_to_index(size: usize) -> Option<usize> {
//     if size == 0 || size > (1 << MAX_ORDER) {
//         return None;
//     }
//     let order = size.next_power_of_two().trailing_zeros();
//     let order = if order < MIN_ORDER { MIN_ORDER } else { order };
//     Some((order - MIN_ORDER) as usize)
// }
//
// /// Allocate `size` bytes from the slab allocator.
// pub unsafe fn kmalloc(size: usize) -> KResult<NonNull<u8>> {
//     let idx = size_to_index(size).ok_or(KErrNo::NoMem)?;
//     unsafe {
//         KMALLOC_CACHES[idx]
//     }.alloc_raw()
// }
//
// /// Free a pointer previously returned by `kmalloc`.
// /// Caller must pass the same `size` used at allocation.
// pub unsafe fn kfree(ptr: NonNull<u8>, size: usize) {
//     if let Some(idx) = size_to_index(size) {
//         unsafe {
//             KMALLOC_CACHES[idx]
//         }.free_raw(ptr);
//     }
// }
