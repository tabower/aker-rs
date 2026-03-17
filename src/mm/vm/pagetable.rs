//! Page Table structures and configuration
use alloc::vec::Vec;
use core::alloc::Allocator as RustAllocator;
use core::marker::PhantomData;

use crate::mm::addr::PhysAddr;
use crate::mm::addr::PhysPageNum;
use crate::mm::addr::VirtAddr;
use crate::mm::alloc::PageAllocator;
use crate::mm::alloc::PageBox;
use crate::mm::arch::PT_ENTRIES;

use super::config::PageTableConfig;
use super::pte::PTE;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PTE; PT_ENTRIES],
}

impl PageTable {
    /// Get mutable reference to entry
    #[inline(always)]
    pub fn entries_mut(&mut self) -> &mut [PTE; PT_ENTRIES] {
        &mut self.entries
    }

    /// Get reference to entry at index
    #[inline(always)]
    pub fn get(&self, index: usize) -> &PTE {
        &self.entries[index]
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.fill(PTE::empty());
    }

    /// Get physical address of this page table
    #[inline(always)]
    pub fn as_pa(&self) -> PhysAddr {
        VirtAddr::new(self as *const _ as usize).to_phys()
    }
}

/// Root of a page table hierarchy
pub struct PageTableRoot<C, A, B>
where
    C: PageTableConfig,
    A: PageAllocator + Send,
    B: RustAllocator + Send,
{
    pub(super) root: PhysPageNum,

    /// All pages in the collection, excluding the root
    /// [-TODO-]
    /// For now, we are only concerned with increasing the number of
    /// pages and not with reclamation. This will obviously prevent
    /// timely reclamation when the user program calls `unmap`. We
    /// will address this in a future optimization; for now, our
    /// priority is to get the kernel up and running.
    pub(super) pages: Vec<PageBox<A>, B>,
    _marker: PhantomData<C>,
}

impl<C, B, A> PageTableRoot<C, A, B>
where
    C: PageTableConfig,
    A: PageAllocator + Send,
    B: RustAllocator + Send,
{
    pub fn new(root_page: PageBox<A>, alloc: B) -> Self {
        let mut root = PageTableRoot {
            root: root_page.ppn(),
            pages: Vec::new_in(alloc),
            _marker: PhantomData,
        };
        root.pages.push(root_page);
        root
    }

    /// Physical address of root table
    #[inline]
    pub fn pa(&self) -> PhysAddr {
        self.root.to_addr()
    }
}

unsafe impl<C, B, A> Send for PageTableRoot<C, A, B>
where
    C: PageTableConfig,
    A: PageAllocator + Send,
    B: RustAllocator + Send,
{
}
