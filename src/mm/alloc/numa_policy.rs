use crate::cpu::numa::NId;

/// NUMA Allocation Policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumaPolicy {
    /// Only allocate on the local node.
    Local,

    /// Only allocate on the specified node.
    /// Fail immediately if that node has insufficient memory.
    Strict(NId),

    /// Try the local node first.
    /// If insufficient, fall back to other nodes by NUMA distance.
    Preferred,
}
