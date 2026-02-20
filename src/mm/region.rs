use super::numa::NId;
use crate::mm::addr::PhysAddr;

/// Describe a block of physical memory
#[derive(Clone, Copy)]
pub struct MemRegion {
    pub base: PhysAddr,
    pub size: usize,
    pub nid: NId,
    pub kind: MemRegionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemRegionKind {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    Mmio,
}
