//! Page Table Entry - Generic wrapper over architecture PTE

use crate::arch::vm::consts::PT_ENTRIES;
use crate::arch::vm::consts::level_shift;
use crate::arch::vm::pte::PTE as ArchPte;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::VirtAddr;

/// Page Table Entry
///
/// Re-exports the architecture-specific PTE type.
/// Additional generic methods are implemented below.
pub type PTE = ArchPte;

impl PTE {
    /// Check if this is a non-leaf (table pointer) entry
    #[inline]
    pub const fn is_table(&self) -> bool {
        self.is_valid() && !self.is_leaf()
    }

    /// Get the physical address this PTE points to
    #[inline]
    pub const fn pa(&self) -> PhysAddr {
        self.ppn().to_addr()
    }

    /// Get the PageTable index for a VA at given physical level
    #[inline]
    pub fn index_of(va: VirtAddr, phys_level: usize) -> usize {
        (va.as_usize() >> level_shift(phys_level)) & (PT_ENTRIES - 1)
    }
}

impl core::fmt::Display for PTE {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(
            f,
            "PTE {{ ppn: {}, flags: {:?}, valid: {}, leaf: {} }}",
            self.ppn(),
            self.flags(),
            self.is_valid(),
            self.is_leaf()
        )
    }
}
