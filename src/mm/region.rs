use super::numa::NId;
use crate::mm::addr::PhysAddr;

/// Describe a block of physical memory
#[derive(Clone, Copy, Debug)]
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

impl MemRegion {
    pub fn new(
        base: PhysAddr,
        size: usize,
        nid: NId,
        kind: MemRegionKind,
    ) -> MemRegion {
        Self {
            base,
            size,
            nid,
            kind,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.size = 0
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub fn end(&self) -> PhysAddr {
        self.base + self.size
    }
}

use crate::libs::dtb::DtbMemKind;
use crate::libs::dtb::DtbMemRegion;

impl From<DtbMemKind> for MemRegionKind {
    #[inline]
    fn from(kind: DtbMemKind) -> Self {
        match kind {
            DtbMemKind::Ram => MemRegionKind::Usable,
            DtbMemKind::Reserved => MemRegionKind::Reserved,
            DtbMemKind::Mmio => MemRegionKind::Mmio,
        }
    }
}

impl From<DtbMemRegion> for MemRegion {
    #[inline]
    fn from(r: DtbMemRegion) -> Self {
        Self {
            base: PhysAddr::new(r.base),
            size: r.size,
            nid: r.numa_id.map(NId::new).unwrap_or(NId::new(0)),
            kind: r.kind.into(),
        }
    }
}

impl From<&DtbMemRegion> for MemRegion {
    #[inline]
    fn from(r: &DtbMemRegion) -> Self {
        Self {
            base: PhysAddr::new(r.base),
            size: r.size,
            nid: r.numa_id.map(NId::new).unwrap_or(NId::new(0)),
            kind: r.kind.into(),
        }
    }
}
