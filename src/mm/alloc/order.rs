use crate::mm::page::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order(u8);

impl Order {
    /// Maximum order supported by the system
    pub const MAX: u8 = 10;

    #[inline]
    pub const fn new(order: u8) -> Self {
        assert!(order <= Self::MAX, "order exceeds maximum");
        Self(order)
    }

    #[inline]
    pub const fn try_new(order: u8) -> Option<Self> {
        if order <= Self::MAX {
            Some(Order(order))
        } else {
            None
        }
    }

    pub const fn from_size(size: usize) -> Self {
        let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let order = pages.next_power_of_two().trailing_zeros() as u8;
        Self::new(order)
    }

    #[inline(always)]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Number of pages in this order
    #[inline(always)]
    pub const fn page_count(self) -> usize {
        1 << self.0
    }

    /// Total number of bytes in this order
    #[inline(always)]
    pub const fn byte_size(self) -> usize {
        self.page_count() * PAGE_SIZE
    }
}

impl core::ops::Shl<Order> for usize {
    type Output = usize;
    fn shl(self, rhs: Order) -> usize {
        self << rhs.0
    }
}
