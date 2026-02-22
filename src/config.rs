pub const MAX_CPUS: usize = 64;
pub const MAX_NODES: usize = 64;

pub const CPUMASK_BITMAP_LEN: usize =
    (MAX_CPUS + usize::BITS as usize - 1) / usize::BITS as usize;
