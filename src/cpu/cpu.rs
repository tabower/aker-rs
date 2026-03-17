use crate::libs::unsafe_static::UnsafeStatic;

use super::numa::NId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CpuId(usize);

impl CpuId {
    #[inline(always)]
    pub const fn new(cid: usize) -> Self {
        Self(cid)
    }

    /// Returns the raw CPU id.
    #[inline(always)]
    pub const fn get(&self) -> usize {
        self.0
    }
}

impl core::fmt::Display for CpuId {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(f, "C{}", self.0)
    }
}

pub struct CpuDev {
    pub id: CpuId,
    pub nid: NId,
    pub ticks: u64,
}

impl CpuDev {
    pub const fn new(id: CpuId, nid: NId, ticks: u64) -> Self {
        CpuDev { id, nid, ticks }
    }
}

/// Number of CPUs detected in the system
/// This variable must be initialized before memory allocation is
/// established.
static NR_CPUS: UnsafeStatic<usize> = UnsafeStatic::uninit();
pub unsafe fn set_nr_cpus(nr: usize) {
    unsafe { NR_CPUS.init(nr) };
}

pub fn get_nr_cpus() -> usize {
    unsafe { *NR_CPUS.get() }
}
