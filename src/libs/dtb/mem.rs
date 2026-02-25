#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DtbMemKind {
    Ram,      // /memory node
    Reserved, // /reserved-memory node
    Mmio,     // Device-mapped I/O
}

pub struct DtbMemRegion {
    pub base: usize,
    pub size: usize,
    pub numa_id: Option<usize>, /* The DTB may not contain NUMA
                                 * information */
    pub kind: DtbMemKind,
    pub hotpluggable: bool,
}
