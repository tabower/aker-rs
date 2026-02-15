use crate::mm::addr::PhysAddr;

use super::numa::NId;

/// Describe a block of physical memory
#[derive(Clone, Copy)]
pub struct MemRegion {
    pub start: PhysAddr,
    pub size: usize,
    pub nid: NId,
}
