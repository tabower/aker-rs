//! RISC-V page table constants

use crate::mm::page::PAGE_SHIFT;

/// Bits per page table level (VPN width)
pub const PT_LEVEL_BITS: usize = 9;

/// Entries per page table
pub const PT_ENTRIES: usize = 1 << PT_LEVEL_BITS;

/// Bit shift for a given physical level
///
/// ```text
/// Level 0: bits [20:12]  → shift 12
/// Level 1: bits [29:21]  → shift 21
/// Level 2: bits [38:30]  → shift 30
/// Level 3: bits [47:39]  → shift 39 (Sv48/Sv57)
/// Level 4: bits [56:48]  → shift 48 (Sv57)
/// ```
#[inline]
pub const fn level_shift(level: usize) -> usize {
    PAGE_SHIFT + level * PT_LEVEL_BITS
}

/// Page size at a given physical level
#[inline]
pub const fn level_page_size(level: usize) -> usize {
    1 << level_shift(level)
}
