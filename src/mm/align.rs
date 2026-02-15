use super::page::PAGE_MASK;
use super::page::PAGE_SIZE;

/// Common alignment and page operations
pub trait AlignOps: Sized {
    fn raw(&self) -> usize;
    fn from_raw(v: usize) -> Self;

    #[inline]
    fn align_down(&self, align: usize) -> Self {
        debug_assert!(align.is_power_of_two());
        Self::from_raw(self.raw() & !(align - 1))
    }

    #[inline]
    fn align_up(&self, align: usize) -> Self {
        debug_assert!(align.is_power_of_two());
        Self::from_raw((self.raw() + align - 1) & !(align - 1))
    }

    #[inline]
    fn is_aligned(&self, align: usize) -> bool {
        debug_assert!(align.is_power_of_two());
        self.raw() & (align - 1) == 0
    }

    #[inline]
    fn page_floor(&self) -> Self {
        self.align_down(PAGE_SIZE)
    }

    #[inline]
    fn page_ceil(&self) -> Self {
        self.align_up(PAGE_SIZE)
    }

    #[inline]
    fn page_offset(&self) -> usize {
        self.raw() & PAGE_MASK
    }

    #[inline]
    fn is_page_aligned(&self) -> bool {
        self.is_aligned(PAGE_SIZE)
    }

    #[inline]
    fn checked_add(&self, rhs: usize) -> Option<Self> {
        self.raw().checked_add(rhs).map(Self::from_raw)
    }

    #[inline]
    fn checked_sub(&self, rhs: usize) -> Option<Self> {
        self.raw().checked_sub(rhs).map(Self::from_raw)
    }
}
