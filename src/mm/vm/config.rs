use super::level::PageLevel;

/// Page table configuration for a specific virtual address mode
///
/// Defines the mapping between logical levels (PGD/PUD/PMD/PTE)
/// and physical levels (0, 1, 2, ...).
pub trait PageTableConfig: 'static + Copy {
    /// Number of physical page table levels
    /// for example, Riscv Sv39 = 3, Sv48 = 4
    const PHYSICAL_LEVELS: usize;

    /// Map logical level to physical level index
    ///
    /// Returns `None` if the level is folded (not used).
    fn logical_to_physical(level: PageLevel) -> Option<usize>;

    /// Map physical level index to logical level
    fn physical_to_logical(phys_level: usize) -> PageLevel;

    /// Check if a logical level is folded (skipped)
    fn is_folded(level: PageLevel) -> bool {
        Self::logical_to_physical(level).is_none()
    }
}
