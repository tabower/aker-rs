use crate::arch::vm::pagetable::PageTable;
use crate::arch::vm::pagetable::PageTableConfig;
use crate::arch::vm::pte::PTE;
use crate::arch::vm::pte::PTEFlags;
use crate::arch::vm::sv::SvPageConfig;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::VirtAddr;
use crate::mm::align::AlignOps;

use super::alloc::BootAllocator;
use super::linker;

/// Physical level corresponding to 1GB large pages
const GIGAPAGE_PHYS_LEVEL: usize = 2;
const GIGABYTE: usize = 1024 * 1024 * 1024;
const BOOT_FLAGS: PTEFlags = PTEFlags::BOOT;

/// Generic 1GB large page mapping function
///
/// Supports SV39 (3-level) and SV48 (4-level) page tables
#[inline(always)]
unsafe fn map_1gb_entry(
    root_table: *mut PageTable,
    allocator: &mut BootAllocator,
    virt: VirtAddr,
    phys: PhysAddr,
) {
    // Physical level of root page table = Total levels - 1
    // SV39: PHYSICAL_LEVELS=3, root_phys_level=2
    // SV48: PHYSICAL_LEVELS=4, root_phys_level=3

    let root_phys_level = SvPageConfig::PHYSICAL_LEVELS - 1;

    let gigapage_table: *mut PageTable = match root_phys_level {
        // SV39: The root page table is at the 1GB (PUD) level and is
        // used directly.
        2 => root_table,

        // SV48: The root page table is at the PGD level and requires
        // indexing to PUD first.
        3 => {
            let pgd_idx = PageTable::index_of(virt, root_phys_level);
            let pgd_entry = unsafe { (*root_table).get_mut(pgd_idx) };

            if pgd_entry.is_valid() {
                pgd_entry.pa().to_virt().as_mut_ptr::<PageTable>()
            } else {
                let new_table = allocator.alloc() as *mut PageTable;

                unsafe {
                    (*new_table).clear();
                }

                *pgd_entry = PTE::new(
                    PhysAddr::new(new_table as usize),
                    PTEFlags::V,
                );
                new_table
            }
        }

        _ => panic!(
            "Unsupported page table levels: {}",
            SvPageConfig::PHYSICAL_LEVELS
        ),
    };

    // Fill in entries at the 1GB level (physical level 2)
    let entry_idx = PageTable::index_of(virt, GIGAPAGE_PHYS_LEVEL);
    let entry = unsafe { (*gigapage_table).get_mut(entry_idx) };

    debug_assert!(
        !entry.is_valid(),
        "1GB entry already mapped: {:#x}",
        virt.as_usize()
    );
    *entry = PTE::new(phys, BOOT_FLAGS);
}

/// Create the startup page table
#[inline(always)]
pub(super) fn setup_boot_vm(
    allocator: &mut BootAllocator,
    mem_end: usize,
) -> usize {
    let root_table = allocator.alloc() as *mut PageTable;

    let map_start =
        PhysAddr::from(linker::phy_base()).align_down(GIGABYTE);
    let vmap_end = PhysAddr::from(mem_end).align_up(GIGABYTE);

    let mut curr_phys = map_start;
    while curr_phys < vmap_end {
        let cur_virt = curr_phys.to_virt();

        unsafe {
            // Identity mapping
            map_1gb_entry(
                root_table,
                allocator,
                // WARN: The `cur_virt` variable cannot be used here
                // because the preceding `curr_phys.to_virt()` method
                // converts it into a virtual address with linear
                // offset, whereas we require an identity mapping.
                VirtAddr::new(curr_phys.as_usize()),
                curr_phys,
            );

            // High-level mapping
            map_1gb_entry(root_table, allocator, cur_virt, curr_phys);
        }

        curr_phys = curr_phys + GIGABYTE;
    }

    root_table as usize
}
