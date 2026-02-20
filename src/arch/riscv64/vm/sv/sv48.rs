use super::super::pagetable::PageLevel;
use super::super::pagetable::PageTableConfig;

/// RISC-V Sv48 Config
#[derive(Clone, Copy)]
pub struct Sv48Config;

impl PageTableConfig for Sv48Config {
    const PHYSICAL_LEVELS: usize = 4;

    fn logical_to_physical(level: PageLevel) -> Option<usize> {
        Some(level as usize)
    }

    fn physical_to_logical(phys_level: usize) -> PageLevel {
        match phys_level {
            3 => PageLevel::PGD,
            2 => PageLevel::PUD,
            1 => PageLevel::PMD,
            0 => PageLevel::PTE,
            _ => panic!("Sv48 Invalid phyical level: {}", phys_level),
        }
    }

    fn is_folded(_level: PageLevel) -> bool {
        false
    }
}
