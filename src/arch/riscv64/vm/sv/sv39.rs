use crate::kernel::vm::level::PageLevel;
use crate::kernel::vm::pagetable::PageTableConfig;

#[derive(Clone, Copy)]
pub struct Sv39Config;

impl PageTableConfig for Sv39Config {
    const PHYSICAL_LEVELS: usize = 3;

    fn logical_to_physical(level: PageLevel) -> Option<usize> {
        match level {
            PageLevel::PGD => Some(2), // PGD = Level 2
            PageLevel::PUD => Some(2), /* PUD = Level 2 (collapsed */
            // to PGD)
            PageLevel::PMD => Some(1), // PMD = Level 1
            PageLevel::PTE => Some(0), // PTE = Level 0
        }
    }

    fn physical_to_logical(phys_level: usize) -> PageLevel {
        match phys_level {
            2 => PageLevel::PGD,
            1 => PageLevel::PMD,
            0 => PageLevel::PTE,
            _ => panic!("Sv39 Invalid phyical level: {}", phys_level),
        }
    }

    fn is_folded(level: PageLevel) -> bool {
        level == PageLevel::PUD
    }
}
