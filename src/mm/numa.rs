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
