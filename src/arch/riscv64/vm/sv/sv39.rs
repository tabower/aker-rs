use crate::mm::vm::config::PageTableConfig;
use crate::mm::vm::level::PageLevel;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Sv39Config;

impl PageTableConfig for Sv39Config {
    const PHYSICAL_LEVELS: usize = 3;

    fn logical_to_physical(level: PageLevel) -> Option<usize> {
        match level {
            PageLevel::PGD => Some(2), // PGD = Level 2
            // PageLevel::PUD => Some(2), // PUD = Level 2 (collapsed
            // to PGD)
            _ => Some(level as usize),
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
