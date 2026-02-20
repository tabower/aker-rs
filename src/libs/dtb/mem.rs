#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemKind {
    Ram,      // /memory node
    Reserved, // /reserved-memory node
    Mmio,     // Device-mapped I/O
}

pub struct MemRegion {
    pub base: usize,
    pub size: usize,
    pub numa_id: Option<usize>, /* The DTB may not contain NUMA
                                 * information */
    pub kind: MemKind,
    pub hotpluggable: bool,
}
