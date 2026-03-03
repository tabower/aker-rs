//! RISC-V Page Table Entry
//!
//! ```text
//! 63       54 53      28 27      19 18      10 9  8 7 6 5 4 3 2 1 0
//! +----------+----------+----------+----------+----+-+-+-+-+-+-+-+-+
//! | Reserved |  PPN[2]  |  PPN[1]  |  PPN[0]  |RSW |D|A|G|U|X|W|R|V|
//! +----------+----------+----------+----------+----+-+-+-+-+-+-+-+-+
//!   10 bits    26 bits    9 bits     9 bits   2bit
//! ```

use crate::mm::addr::PhysPageNum;

bitflags::bitflags! {
    /// RISC-V PTE flag bits
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PTEFlags: u64 {
        /// Valid
        const V = 1 << 0;
        /// Readable
        const R = 1 << 1;
        /// Writable
        const W = 1 << 2;
        /// Executable
        const X = 1 << 3;
        /// User accessible
        const U = 1 << 4;
        /// Global mapping
        const G = 1 << 5;
        /// Accessed
        const A = 1 << 6;
        /// Dirty
        const D = 1 << 7;

        // Common Flags
        const RW  = Self::R.bits() | Self::W.bits();
        const RX  = Self::R.bits() | Self::X.bits();
        const RWX = Self::R.bits() | Self::W.bits() | Self::X.bits();

        // BOOT Boot Phase Flag
        const BOOT = Self::V.bits() | Self::R.bits() | Self::W.bits() |
                     Self::X.bits();

        // Kernel Pages
        const KERNEL_RO = Self::V.bits() | Self::R.bits() | Self::G.bits();
        const KERNEL_RW = Self::V.bits() | Self::R.bits() | Self::W.bits() | Self::G.bits();
        const KERNEL_RX = Self::V.bits() | Self::R.bits() | Self::X.bits() | Self::G.bits();

        // User Page
        const USER_RO = Self::V.bits() | Self::R.bits() | Self::U.bits();
        const USER_RW = Self::V.bits() | Self::R.bits() | Self::W.bits() | Self::U.bits();
        const USER_RX = Self::V.bits() | Self::R.bits() | Self::X.bits() | Self::U.bits();
    }
}

const PPN_SHIFT: u32 = 10;
const PPN_MASK: u64 = 0x003F_FFFF_FFFF_FC00; // bits [53:10]
const FLAGS_MASK: u64 = 0x3FF; // bits [9:0]

/// RISC-V Page Table Entry
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PTE(u64);

impl PTE {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create PTE from raw bits
    #[inline]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Create a leaf PTE (maps to physical page)
    #[inline]
    pub const fn new_leaf(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self(
            ((ppn.as_usize() as u64) << PPN_SHIFT)
                | flags.bits()
                | PTEFlags::V.bits(),
        )
    }

    /// Create a non-leaf PTE (points to next level table)
    #[inline]
    pub const fn new_table(ppn: PhysPageNum) -> Self {
        Self(
            ((ppn.as_usize() as u64) << PPN_SHIFT)
                | PTEFlags::V.bits(),
        )
    }

    // ---- Predicates ----

    /// Check if PTE is valid
    #[inline]
    pub const fn is_valid(&self) -> bool {
        (self.0 & PTEFlags::V.bits()) != 0
    }

    /// Check if PTE is a leaf (has R, W, or X set)
    #[inline]
    pub const fn is_leaf(&self) -> bool {
        self.is_valid()
            && (self.0
                & (PTEFlags::R.bits()
                    | PTEFlags::W.bits()
                    | PTEFlags::X.bits()))
                != 0
    }

    // ---- Field access ----

    /// Get raw PTE value
    #[inline]
    pub const fn raw(&self) -> u64 {
        self.0
    }

    /// Get flags
    #[inline]
    pub const fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }

    /// Get physical page number
    #[inline]
    pub const fn ppn(&self) -> PhysPageNum {
        PhysPageNum::new(((self.0 & PPN_MASK) >> PPN_SHIFT) as usize)
    }

    // ---- Mutation ----

    /// Set flags (preserves PPN)
    #[inline]
    pub fn set_flags(&mut self, flags: PTEFlags) {
        self.0 = (self.0 & !FLAGS_MASK) | flags.bits();
    }

    /// Set physical page number (preserves flags)
    #[inline]
    pub fn set_ppn(&mut self, ppn: PhysPageNum) {
        self.0 = (self.0 & FLAGS_MASK)
            | ((ppn.as_usize() as u64) << PPN_SHIFT);
    }

    /// Clear to invalid
    #[inline]
    pub const fn clear(&mut self) {
        self.0 = 0;
    }
}
