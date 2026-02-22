use crate::config;
use crate::kernel::cpu::CpuId;
use crate::kernel::cpu::cpumask::CpuMask;
use crate::libs::unsafe_static::UnsafeStatic;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NId(usize);

impl NId {
    #[inline(always)]
    pub const fn new(nid: usize) -> Self {
        Self(nid)
    }

    /// Returns the raw NUMA id.
    #[inline(always)]
    pub const fn get(&self) -> usize {
        self.0
    }
}

impl core::fmt::Display for NId {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// nid <- map -> cid

#[derive(Debug)]
pub struct CpuNodeMap {
    pub cpu_to_node: [NId; config::MAX_CPUS],
    pub node_to_cpu: [CpuMask; config::MAX_NODES],
}

static GLOBAL_CPU_NODE_MAP: UnsafeStatic<CpuNodeMap> =
    UnsafeStatic::uninit();

pub unsafe fn cpu_node_map_mut() -> &'static mut CpuNodeMap {
    unsafe { GLOBAL_CPU_NODE_MAP.get_mut() }
}

pub fn cpu_node_map() -> &'static CpuNodeMap {
    unsafe { GLOBAL_CPU_NODE_MAP.get() }
}

pub fn cpu_to_node(cid: CpuId) -> NId {
    unsafe { GLOBAL_CPU_NODE_MAP.get() }.cpu_to_node[cid.get()]
}

pub fn node_cpumask(nid: NId) -> &'static CpuMask {
    &unsafe { GLOBAL_CPU_NODE_MAP.get() }.node_to_cpu[nid.get()]
}
