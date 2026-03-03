//! Page Table structures and configuration

use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::arch::vm::consts::PT_ENTRIES;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::PhysPageNum;
use crate::mm::addr::VirtAddr;
use crate::mm::align::AlignOps;
use crate::mm::allocator::PageAllocator;

use super::level::PageLevel;
use super::pte::PTE;

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PTE; PT_ENTRIES],
}

impl PageTable {
    /// Create an empty page table
    pub const fn empty() -> Self {
        Self {
            entries: [PTE::empty(); PT_ENTRIES],
        }
    }

    /// Get mutable reference to entry at index
    #[inline(always)]
    pub fn get(&self, index: usize) -> &PTE {
        &self.entries[index]
    }

    /// Get mutable reference to entry at index
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut PTE {
        &mut self.entries[index]
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.fill(PTE::empty());
    }

    /// Check if all entries are invalid
    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|pte| !pte.is_valid())
    }

    /// Get physical address of this page table
    #[inline(always)]
    pub fn as_pa(&self) -> PhysAddr {
        VirtAddr::new(self as *const _ as usize).to_phys()
    }

    /// Get physical page number of this page table
    #[inline(always)]
    pub fn as_ppn(&self) -> PhysPageNum {
        self.as_pa().to_ppn()
    }

    /// Recover PageTable reference from one of its entries
    ///
    /// # Safety
    /// `pte` must be an entry within a valid PageTable.
    #[inline(always)]
    pub unsafe fn from_entry(pte: &PTE) -> &PageTable {
        let addr =
            VirtAddr::new(pte as *const PTE as usize).page_floor();
        unsafe { &*(addr.as_usize() as *const PageTable) }
    }

    /// Recover mutable PageTable reference from one of its entries
    ///
    /// # Safety
    /// `pte` must be an entry within a valid PageTable.
    #[inline]
    pub unsafe fn from_entry_mut(pte: &mut PTE) -> &mut PageTable {
        let addr =
            VirtAddr::new(pte as *mut PTE as usize).page_floor();
        unsafe { &mut *(addr.as_usize() as *mut PageTable) }
    }
}

/// Page table configuration for a specific virtual address mode
///
/// Defines the mapping between logical levels (PGD/PUD/PMD/PTE)
/// and physical levels (0, 1, 2, ...).
pub trait PageTableConfig: 'static + Copy {
    /// Number of physical page table levels
    const PHYSICAL_LEVELS: usize;

    /// Root logical level
    fn root_level() -> PageLevel {
        PageLevel::PGD
    }

    /// Map logical level to physical level index
    ///
    /// Returns `None` if the level is folded (not used).
    fn logical_to_physical(level: PageLevel) -> Option<usize>;

    /// Map physical level index to logical level
    fn physical_to_logical(phys_level: usize) -> PageLevel;

    /// Check if a logical level is folded (skipped)
    #[inline]
    fn is_folded(level: PageLevel) -> bool {
        Self::logical_to_physical(level).is_none()
    }
}

/// Root of a page table hierarchy
pub struct PageTableRoot<C, A>
where
    C: PageTableConfig,
    A: PageAllocator,
{
    root: NonNull<PageTable>,
    _marker: PhantomData<(C, A)>,
}

impl<C: PageTableConfig, A: PageAllocator> PageTableRoot<C, A> {
    /// Create from physical address of root table
    ///
    /// # Safety
    /// `pa` must point to a valid, aligned PageTable.
    pub unsafe fn from_pa(pa: PhysAddr) -> Self {
        debug_assert!(pa.is_page_aligned());
        Self {
            root: unsafe {
                NonNull::new_unchecked(
                    pa.to_virt().as_mut_ptr::<PageTable>(),
                )
            },
            _marker: PhantomData,
        }
    }

    /// Physical address of root table
    #[inline]
    pub fn root_pa(&self) -> PhysAddr {
        unsafe { self.root.as_ref() }.as_pa()
    }

    /// Physical page number of root table
    #[inline]
    pub fn root_ppn(&self) -> PhysPageNum {
        self.root_pa().to_ppn()
    }

    /// Get reference to root table
    #[inline]
    pub fn root_table(&self) -> &PageTable {
        unsafe { self.root.as_ref() }
    }

    /// Get mutable reference to root table
    #[inline]
    pub fn root_table_mut(&mut self) -> &mut PageTable {
        unsafe { self.root.as_mut() }
    }
}

unsafe impl<C: PageTableConfig, A: PageAllocator> Send
    for PageTableRoot<C, A>
{
}
unsafe impl<C: PageTableConfig, A: PageAllocator> Sync
    for PageTableRoot<C, A>
{
}
