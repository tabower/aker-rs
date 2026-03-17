use super::consts::PT_LEVEL_BITS;

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

/// Number of supported levels
pub const MAX_LEVELS: usize = 4;

impl PageLevel {
    #[inline(always)]
    pub const fn pages_per_entry(&self) -> usize {
        1usize << (*self as usize * PT_LEVEL_BITS)
    }

    #[inline(always)]
    pub const fn form_usize(level: usize) -> Option<Self> {
        match level {
            0 => Some(Self::PTE),
            1 => Some(Self::PMD),
            2 => Some(Self::PUD),
            3 => Some(Self::PGD),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_usize(self) -> usize {
        self as usize
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
