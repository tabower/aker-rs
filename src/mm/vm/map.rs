use alloc::vec::Vec;
use core::alloc::Allocator as RustAllocator;
use core::ptr::NonNull;

use crate::prelude::*;

use crate::arch::vm::consts::PT_ENTRIES;
use crate::arch::vm::pte::PTE;
use crate::arch::vm::pte::PTEFlags;

use crate::mm::addr::PhysPageNum;
use crate::mm::addr::VirtPageNum;
use crate::mm::alloc::AllocFlags;
use crate::mm::alloc::PageAllocator;
use crate::mm::alloc::PageBox;

use super::config::PageTableConfig;
use super::level::PageLevel;
use super::pagetable::PageTable;
use super::pagetable::PageTableRoot;

pub enum ChildAction {
    /// Continue recursing into the subtable
    Descend(NonNull<PageTable>),
    /// Skip this subtree
    Skip,
}

/// Operation callback during page table traversal
pub trait WalkHandler {
    /// Processing a batch of contiguous PTEs at leaf nodes
    fn on_leaf(&mut self, entries: &mut [PTE], level: PageLevel);

    /// Called before entering the subtree table
    /// - Returns Descend(ptr): the walker enters the subtree
    ///   recursively
    /// - Returns Skip: the walker skips this subtree
    /// - Returns Err: the walker terminates
    fn enter_child(
        &mut self,
        pte: &mut PTE,
        level: PageLevel,
    ) -> KResult<ChildAction>;

    /// Called after returning from the child page table
    fn leave_child(
        &mut self,
        pte: &mut PTE,
        level: PageLevel,
        child: NonNull<PageTable>,
    );
}

/// MapHandler
pub struct MapHandler<'a, A: PageAllocator, B: RustAllocator> {
    pages: &'a mut Vec<PageBox<A>, B>,
    next_ppn: PhysPageNum,
    flags: PTEFlags,
}

impl<A: PageAllocator, B: RustAllocator> WalkHandler
    for MapHandler<'_, A, B>
{
    fn on_leaf(&mut self, entries: &mut [PTE], level: PageLevel) {
        let ppn_interval = level.pages_per_entry();
        for pte in entries {
            debug_assert!(!pte.is_valid());
            *pte = PTE::new_leaf(self.next_ppn, self.flags);
            self.next_ppn += ppn_interval;
        }
    }

    fn enter_child(
        &mut self,
        pte: &mut PTE,
        _level: PageLevel,
    ) -> KResult<ChildAction> {
        if !pte.is_valid() {
            // alloc new page
            let page =
                PageBox::<A>::new_page(AllocFlags::KERNEL_ZERO)?;

            let ppn = page.ppn();
            // The push operation may fail, so write to the PTE after
            // the push.
            self.pages.push(page);
            *pte = PTE::new_table(ppn);

            let ptr = unsafe {
                NonNull::new_unchecked(
                    ppn.to_addr().to_virt().as_mut_ptr::<PageTable>(),
                )
            };

            Ok(ChildAction::Descend(ptr))
        } else if pte.is_leaf() {
            KErr!(KErrNo::EEXIST, "mapping already exists")
        } else {
            // Intermediate table already exists
            let ptr = unsafe {
                NonNull::new_unchecked(
                    pte.pa().to_virt().as_mut_ptr::<PageTable>(),
                )
            };

            Ok(ChildAction::Descend(ptr))
        }
    }

    fn leave_child(
        &mut self,
        _pte: &mut PTE,
        _level: PageLevel,
        _child: NonNull<PageTable>,
    ) {
    }
}

pub struct UnmapHandler;

impl WalkHandler for UnmapHandler {
    fn on_leaf(&mut self, entries: &mut [PTE], _level: PageLevel) {
        for pte in entries {
            *pte = PTE::empty();
        }
    }

    fn enter_child(
        &mut self,
        pte: &mut PTE,
        _level: PageLevel,
    ) -> KResult<ChildAction> {
        if pte.is_valid() && !pte.is_leaf() {
            let ptr = unsafe {
                NonNull::new_unchecked(
                    pte.pa().to_virt().as_mut_ptr::<PageTable>(),
                )
            };
            Ok(ChildAction::Descend(ptr))
        } else {
            // Does not exist or is already a leaf node
            // Skip, do not report an error
            Ok(ChildAction::Skip)
        }
    }

    fn leave_child(
        &mut self,
        _pte: &mut PTE,
        _level: PageLevel,
        _child: NonNull<PageTable>,
    ) {
        // [-TODO-] Do not recycle intermediate page tables for now
        // todo!()
    }
}

pub fn walk_range<C: PageTableConfig>(
    table: &mut PageTable,
    mut vpn: VirtPageNum,
    mut remaining: usize,
    current_level: PageLevel,
    target_level: PageLevel,
    handler: &mut impl WalkHandler,
) -> KResult<()> {
    if C::is_folded(current_level) {
        return walk_range::<C>(
            table,
            vpn,
            remaining,
            current_level.down().unwrap(),
            target_level,
            handler,
        );
    }

    let pages_per_entry = current_level.pages_per_entry();

    while remaining > 0 {
        let idx = vpn.level_index(current_level);
        if idx >= PT_ENTRIES {
            break;
        }

        if current_level == target_level {
            // Target Level: Batch Processing of Leaves
            let max_batch = PT_ENTRIES - idx;
            let need =
                (remaining + pages_per_entry - 1) / pages_per_entry;
            let batch = max_batch.min(need);

            handler.on_leaf(
                &mut table.entries_mut()[idx..idx + batch],
                current_level,
            );

            let consumed = batch * pages_per_entry;
            remaining = remaining.saturating_sub(consumed);
            vpn += consumed;
        } else {
            // Intermediate level: Recursion after consulting the
            // handler
            let pte = &mut table.entries_mut()[idx];
            let offset = vpn.offset_within(current_level);
            let can_cover = pages_per_entry - offset;
            let child_count = remaining.min(can_cover);

            match handler.enter_child(pte, current_level)? {
                ChildAction::Skip => {
                    // The handler says to skip it; the walker just
                    // moves the cursor forward
                }
                ChildAction::Descend(mut child_ptr) => {
                    walk_range::<C>(
                        unsafe { child_ptr.as_mut() },
                        vpn,
                        child_count,
                        current_level.down().unwrap(),
                        target_level,
                        handler,
                    )?;

                    // Retrieve the PTE again (the recursion may have
                    // modified other entries, but idx
                    // remains unchanged)
                    let pte = &mut table.entries_mut()[idx];
                    handler.leave_child(
                        pte,
                        current_level,
                        child_ptr,
                    );
                }
            }

            remaining -= child_count;
            vpn += child_count;
        }
    }

    Ok(())
}

impl<C, A, B> PageTableRoot<C, A, B>
where
    C: PageTableConfig,
    A: PageAllocator + Send,
    B: RustAllocator + Send,
{
    pub fn map_range(
        &mut self,
        vpn: VirtPageNum,
        ppn: PhysPageNum,
        count: usize,
        flags: PTEFlags,
        target_level: PageLevel,
    ) -> KResult<()> {
        let align = target_level.pages_per_entry();
        debug_assert!(
            vpn.as_usize() % align == 0,
            "vpn not aligned to target level"
        );
        debug_assert!(
            ppn.as_usize() % align == 0,
            "ppn not aligned to target level"
        );
        debug_assert!(
            count % align == 0,
            "count not aligned to target level"
        );

        let mut handler = MapHandler {
            pages: &mut self.pages,
            next_ppn: ppn,
            flags,
        };

        let table = unsafe {
            NonNull::new(
                self.root
                    .to_addr()
                    .to_virt()
                    .as_mut_ptr::<PageTable>(),
            )
            .unwrap()
            .as_mut()
        };

        walk_range::<C>(
            table,
            vpn,
            count,
            PageLevel::PGD,
            target_level,
            &mut handler,
        )
    }

    pub fn unmap_range(
        &mut self,
        vpn: VirtPageNum,
        count: usize,
        target_level: PageLevel,
    ) -> KResult<()> {
        let mut handler = UnmapHandler;
        let table = unsafe {
            NonNull::new(
                self.root
                    .to_addr()
                    .to_virt()
                    .as_mut_ptr::<PageTable>(),
            )
            .unwrap()
            .as_mut()
        };
        walk_range::<C>(
            table,
            vpn,
            count,
            PageLevel::PGD,
            target_level,
            &mut handler,
        )
    }
}
