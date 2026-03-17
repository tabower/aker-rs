//! RISC-V page table constants

/// Bits per page table level (VPN width)
pub const PT_LEVEL_BITS: usize = 9;

/// Entries per page table
pub const PT_ENTRIES: usize = 1 << PT_LEVEL_BITS;
