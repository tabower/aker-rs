use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

/// Kernel global static variable wrapper
///
/// Supports two usage patterns:
/// - `UnsafeStatic::new(val)` — Compile-time initialization
/// - `UnsafeStatic::uninit()` — Runtime delayed initialization
pub struct UnsafeStatic<T> {
    inner: UnsafeCell<MaybeUninit<T>>,
}

// Safety: Kernel code ensures no data races occur during access.
unsafe impl<T> Sync for UnsafeStatic<T> {}

impl<T> UnsafeStatic<T> {
    pub const fn new(val: T) -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::new(val)),
        }
    }

    pub const fn uninit() -> Self {
        Self {
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Write values at runtime (for uninit scenarios)
    ///
    /// # Safety
    /// - Should only be called once
    /// - No read operations may precede this call
    /// - No concurrent calls
    #[inline(always)]
    pub unsafe fn init(&self, val: T) {
        unsafe {
            (*self.inner.get()).write(val);
        }
    }

    /// Obtain immutable reference
    ///
    /// # Safety
    /// - Must be called after `init()` if constructed with `uninit`
    /// - Cannot coexist with `get_mut`
    #[inline(always)]
    pub unsafe fn get(&self) -> &T {
        unsafe { (*self.inner.get()).assume_init_ref() }
    }

    /// Obtain a mutable reference
    ///
    /// # Safety
    /// - Must be called after init() if constructed with uninit
    /// - No concurrent access
    #[inline(always)]
    pub unsafe fn get_mut(&self) -> &mut T {
        unsafe { (*self.inner.get()).assume_init_mut() }
    }

    /// not required to be initialized
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        unsafe { (*self.inner.get()).as_mut_ptr() }
    }

    /// not required to be initialized
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        unsafe { (*self.inner.get()).as_ptr() }
    }
}
