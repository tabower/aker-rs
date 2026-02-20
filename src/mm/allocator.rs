use crate::libs::error::KResult;
use crate::mm::addr::PhysAddr;

/// TODO
pub trait Allocator {
    /// Allocate 2^order contiguous pages
    fn alloc_pages(&mut self, order: u8) -> Option<PhysAddr>;

    /// Free 2^order contiguous pages
    fn free_pages(&mut self, pa: PhysAddr, order: u8) -> KResult<()>;

    /// Convenience: allocate a single page (default impl)
    fn alloc_page(&mut self) -> Option<PhysAddr> {
        self.alloc_pages(0)
    }

    /// Convenience: free a single page (default impl)
    fn free_page(&mut self, pa: PhysAddr) -> KResult<()> {
        self.free_pages(pa, 0)
    }
}
