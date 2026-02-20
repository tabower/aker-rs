use super::props::*;
use fdt::node::FdtNode;

const PROP_NUMA_NODE_ID: &str = "numa-node-id";

pub(super) fn read_numa_id(node: &FdtNode) -> Option<usize> {
    prop_u32(node, PROP_NUMA_NODE_ID).map(|id| id as usize)
}
