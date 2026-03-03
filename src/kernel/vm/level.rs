//! Page table level abstraction

use crate::arch::vm::consts::PT_LEVEL_BITS;
use crate::mm::page::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PageLevel {
    /// 4 KiB pages (always physical level 0)
    PTE = 0,
    /// 2 MiB pages
    PMD = 1,
    /// 1 GiB pages
    PUD = 2,
    /// 512 GiB
    PGD = 3,
}

impl PageLevel {
    /// Number of supported levels
    pub const MAX_LEVELS: usize = 5;

    /// Create from numeric index
    #[inline]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::PTE),
            1 => Some(Self::PMD),
            2 => Some(Self::PUD),
            3 => Some(Self::PGD),
            _ => None,
        }
    }

    /// Convert to numeric index
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }

    /// Page size at this level
    #[inline]
    pub const fn page_size(self) -> usize {
        PAGE_SIZE << (self.as_index() * PT_LEVEL_BITS)
    }

    /// Page offset mask at this level
    #[inline]
    pub const fn page_mask(self) -> usize {
        self.page_size() - 1
    }

    /// Next lower level (towards PTE)
    #[inline]
    pub const fn down(self) -> Option<Self> {
        match self {
            Self::PGD => Some(Self::PUD),
            Self::PUD => Some(Self::PMD),
            Self::PMD => Some(Self::PTE),
            Self::PTE => None,
        }
    }

    /// Next higher level (towards root)
    #[inline]
    pub const fn up(self) -> Option<Self> {
        match self {
            Self::PTE => Some(Self::PMD),
            Self::PMD => Some(Self::PUD),
            Self::PUD => Some(Self::PGD),
            Self::PGD => None,
        }
    }
}
