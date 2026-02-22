use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::arch::mm::PAGE_SIZE;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::PhysPageNum;
use crate::mm::addr::VirtAddr;
use crate::mm::align::AlignOps;
use crate::mm::allocator::Allocator;

use super::pte;

/// Number of bits per page table level
pub const PT_LEVEL_BITS: usize = 9;

/// Number of entries per page table
pub const PT_ENTRIES: usize = 1 << PT_LEVEL_BITS;

/// Page Table Structure
#[repr(C, align(4096))]
pub struct PageTable {
    pub(super) entries: [pte::PTE; PT_ENTRIES],
}

impl PageTable {
    pub const fn empty() -> Self {
        Self {
            entries: [pte::PTE::empty(); PT_ENTRIES],
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut pte::PTE {
        &mut self.entries[index]
    }

    #[inline(always)]
    pub fn index_of(va: VirtAddr, phys_level: usize) -> usize {
        (va.as_usize() >> pte::level_shift(phys_level))
            & (PT_ENTRIES - 1)
    }

    pub fn clear(&mut self) {
        self.entries.fill(pte::PTE::empty());
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|pte| !pte.is_valid())
    }

    #[inline(always)]
    pub fn as_pa(&self) -> PhysAddr {
        VirtAddr::new(self as *const _ as usize).to_phys()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PageLevel {
    PTE = 0,
    PMD = 1,
    PUD = 2,
    PGD = 3,
}

impl PageLevel {
    #[inline(always)]
    pub const fn page_size(self) -> usize {
        PAGE_SIZE << (self as usize * PT_LEVEL_BITS)
    }

    #[inline(always)]
    pub const fn page_mask(self) -> usize {
        self.page_size() - 1
    }

    #[inline(always)]
    pub const fn next(self) -> Option<Self> {
        match self {
            PageLevel::PGD => Some(PageLevel::PUD),
            PageLevel::PUD => Some(PageLevel::PMD),
            PageLevel::PMD => Some(PageLevel::PTE),
            PageLevel::PTE => None,
        }
    }
}

///  PageTable Config Trait
pub trait PageTableConfig: 'static + Copy {
    const PHYSICAL_LEVELS: usize;

    fn root_level() -> PageLevel {
        PageLevel::PGD
    }

    fn logical_to_physical(level: PageLevel) -> Option<usize>;
    fn physical_to_logical(phys_level: usize) -> PageLevel;

    #[inline(always)]
    fn is_folded(level: PageLevel) -> bool {
        Self::logical_to_physical(level).is_none()
    }
}

/// Pagetable Root
pub struct PageTableRoot<C: PageTableConfig, A: Allocator> {
    pub(super) root: NonNull<PageTable>,
    _marker: PhantomData<(C, A)>,
}

impl<C: PageTableConfig, A: Allocator> PageTableRoot<C, A> {
    pub fn from_pa(pa: PhysAddr) -> Self {
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

    #[inline(always)]
    pub fn root_pa(&self) -> PhysAddr {
        unsafe { self.root.as_ref() }.as_pa()
    }

    #[inline(always)]
    pub fn root_ppn(&self) -> PhysPageNum {
        self.root_pa().to_ppn()
    }

    #[inline(always)]
    pub fn root_table(&mut self) -> &mut PageTable {
        unsafe { self.root.as_mut() }
    }
}

unsafe impl<C: PageTableConfig, A: Allocator> Send
    for PageTableRoot<C, A>
{
}
unsafe impl<C: PageTableConfig, A: Allocator> Sync
    for PageTableRoot<C, A>
{
}
