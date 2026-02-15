/// The CPU ID is switched along with every task during a
/// CPU context switch.
///
/// We assume that the CPU ID is always valid at any point
/// in time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CpuId(usize);

impl CpuId {
    #[inline(always)]
    pub fn new(cid: usize) -> Self {
        Self(cid)
    }

    /// Returns the raw CPU id.
    #[inline(always)]
    pub fn get(&self) -> usize {
        self.0
    }
}
