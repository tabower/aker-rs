pub use crate::mm::arch::PTE;

use crate::mm::addr::PhysAddr;

/// Page Table Entry
///
/// Additional generic methods are implemented below.
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
