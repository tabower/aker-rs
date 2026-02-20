use crate::arch::mm::PAGE_SHIFT;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::PhysPageNum;
use crate::mm::addr::VirtAddr;
use crate::mm::align::AlignOps;

use super::pagetable::PT_LEVEL_BITS;
use super::pagetable::PageTable;

/// Offset Start Positions for Each Address Level
///
/// ```
/// 63       39 38    30 29    21 20    12 11       0
/// +----------+--------+--------+--------+-------------+
/// | Reserved | VPN[2] | VPN[1] | VPN[0] | Page Offset |
/// +----------+--------+--------+--------+-------------+
///            | 9 bits | 9 bits | 9 bits |   12 bits   |
///              level=2  level=1  level=0
/// ```
pub const fn level_shift(level: usize) -> usize {
    PAGE_SHIFT + level * PT_LEVEL_BITS
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct PTEFlags: u64 {
        const V = 1 << 0;      // Valid
        const R = 1 << 1;      // Readable
        const W = 1 << 2;      // Writable
        const X = 1 << 3;      // Executable
        const U = 1 << 4;      // User accessible
        const G = 1 << 5;      // Global
        const A = 1 << 6;      // Accessed
        const D = 1 << 7;      // Dirty

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

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct PTE(u64);

const PPN_SHIFT: u8 = 10;
const PPN_BITS: u8 = 44;
const PPN_MASK: u64 = ((1u64 << PPN_BITS) - 1) << PPN_SHIFT;
const FLAGS_MASK: u64 = (1u64 << PPN_SHIFT) - 1;

impl PTE {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub const fn new(pa: PhysAddr, flags: PTEFlags) -> Self {
        Self(
            (pa.to_ppn().as_usize() << PPN_SHIFT) as u64
                | flags.bits(),
        )
    }

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.0 & PTEFlags::V.bits() != 0
    }

    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        // Leaf node: V=1 and (R=1 or X=1)
        self.is_valid()
            && (self.0 & (PTEFlags::R.bits() | PTEFlags::X.bits()))
                != 0
    }

    #[inline(always)]
    pub const fn is_table(&self) -> bool {
        self.is_valid() && !self.is_leaf()
    }

    #[inline(always)]
    pub const fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }

    #[inline(always)]
    pub const fn ppn(&self) -> PhysPageNum {
        PhysPageNum::new(((self.0 & PPN_MASK) >> PPN_SHIFT) as usize)
    }

    #[inline(always)]
    pub const fn pa(&self) -> PhysAddr {
        self.ppn().to_addr()
    }

    #[inline(always)]
    pub fn set_flags(&mut self, flags: PTEFlags) {
        self.0 = (self.0 & !FLAGS_MASK) | flags.bits();
    }

    #[inline(always)]
    pub const fn clear(&mut self) {
        self.0 = 0;
    }

    /// Derive the starting address of the page table from the PTE
    #[inline(always)]
    fn get_table_addr(&self) -> VirtAddr {
        let pte_addr = self as *const Self as usize;
        VirtAddr::new(pte_addr).page_floor()
    }

    #[inline(always)]
    pub fn get_table(&self) -> &PageTable {
        let table_addr = self.get_table_addr().as_usize();
        unsafe { &*(table_addr as *const PageTable) }
    }

    #[inline(always)]
    pub fn get_table_mut(&mut self) -> &mut PageTable {
        let table_addr = self.get_table_addr().as_usize();
        unsafe { &mut *(table_addr as *mut PageTable) }
    }
}

impl core::fmt::Display for PTE {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        f.debug_struct("PTE")
            .field("raw", &format_args!("{:#x}", self.0))
            .field("ppn", &format_args!("{}", self.ppn()))
            .field("flags", &self.flags())
            .field("valid", &self.is_valid())
            .field("leaf", &self.is_leaf())
            .finish()
    }
}
