use core::alloc::AllocError;
use core::alloc::Allocator as RustAllocator;
use core::ptr;

use crate::prelude::*;

use crate::cpu::numa::NId;

use crate::libs::unsafe_static::UnsafeStatic;

use crate::kernel::sched::cpu::PreemptGuard;

use crate::mm::addr::PhysAddr;
use crate::mm::addr::PhysPageNum;
use crate::mm::align::AlignOps;
use crate::mm::page::PAGE_SIZE;
use crate::mm::region::MemRegion;
use crate::mm::region::MemRegionKind;

use super::AllocFlags;
use super::NumaPolicy;
use super::Order;
use super::PageAllocator;

const MAX_USABLE_REGIONS: usize = 128;
const MAX_RESERVED_REGIONS: usize = 32;

static BOOTMEM: UnsafeStatic<BootMemInner> = UnsafeStatic::uninit();

static BOOTMEM_REGIONS: UnsafeStatic<
    [BootMemRegion; MAX_USABLE_REGIONS],
> = UnsafeStatic::zeroed();

#[derive(Clone, Copy)]
pub struct BootMem;

impl BootMem {
    pub unsafe fn init(inner: BootMemInner) {
        unsafe {
            BOOTMEM.init(inner);
        }
    }

    pub fn get_mut() -> &'static mut BootMemInner {
        unsafe { BOOTMEM.get_mut() }
    }
}

pub struct BootMemInner {
    regions: &'static mut [BootMemRegion],
    nr_regions: usize, // Number of regions filled

    reserved: [MemRegion; MAX_RESERVED_REGIONS],
    nr_reserved: usize,

    finalized: bool,
}

struct BootMemRegion {
    region: MemRegion,

    cur: PhysAddr,
    end: PhysAddr,
}

impl BootMemRegion {
    pub fn new(base: PhysAddr, size: usize, nid: NId) -> Self {
        Self {
            region: MemRegion::new(
                base,
                size,
                nid,
                MemRegionKind::Usable,
            ),
            cur: base,
            end: base + size,
        }
    }

    /// Mark this area as vacant and available.
    pub fn clear(&mut self) {
        self.region.clear();
    }

    /// We use regions with an empty region (size == 0)
    /// to indicate that the location is vacant and available.
    #[inline]
    pub fn is_unused(&self) -> bool {
        self.region.is_empty()
    }

    /// Allocate a block of memory from the current BootMemRegion.
    pub fn alloc(
        &mut self,
        size: usize,
        align: usize,
    ) -> Option<PhysAddr> {
        if self.is_unused() {
            return None;
        }

        let aligned = self.cur.align_up(align);
        let new_cur = aligned.checked_add(size)?;

        if new_cur > self.end {
            return None;
        }

        self.cur = new_cur;

        Some(aligned)
    }
}

impl BootMemInner {
    /// Add a memory region.
    ///
    /// Currently, only memory regions of types
    /// `MemRegionKind::Usable` and `MemRegionKind::Reserved`are
    /// processed, and regions with size 0 are excluded.
    pub fn add_region(&mut self, region: MemRegion) {
        debug_assert!(
            !self.finalized,
            "cannot add regions after finalize\n"
        );

        if region.is_empty() {
            return;
        }

        match region.kind {
            MemRegionKind::Usable => self.add_usable(region),
            MemRegionKind::Reserved => self.add_reserved(region),
            _ => {
                pr_warn!(
                    "[BootMem] does not currently support \
                    this kind of MemRegion.{:#?}\n",
                    region
                );
            }
        }
    }

    fn add_usable(&mut self, region: MemRegion) {
        if self.nr_regions >= MAX_USABLE_REGIONS {
            pr_warn!("[BootMem] Too many usable regions\n");
            return;
        }

        pr_info!(
            "[BootMem] Add usable   [{:#018x} - {:#018x}] node {}\n",
            region.base.as_usize(),
            region.end().as_usize(),
            region.nid
        );

        self.regions[self.nr_regions] =
            BootMemRegion::new(region.base, region.size, region.nid);
        self.nr_regions += 1;
    }

    fn add_reserved(&mut self, region: MemRegion) {
        if self.nr_reserved >= MAX_RESERVED_REGIONS {
            pr_warn!("[BootMem] Too many reserved regions\n");
            return;
        }

        pr_info!(
            "[BootMem] Add reserved [{:#018x} - {:#018x}] node {}\n",
            region.base.as_usize(),
            region.end().as_usize(),
            region.nid
        );

        self.reserved[self.nr_reserved] = region;
        self.nr_reserved += 1;
    }

    pub fn finalize(&mut self) {
        debug_assert!(!self.finalized);

        pr_info!(
            "[BootMem] Finalizing {} usable, {} reserved regions\n",
            self.nr_regions,
            self.nr_reserved
        );

        self.sort_usable();
        self.merge_usable();
        self.subtract_reserved();
        self.compact();
        self.sort_usable(); // subtract may have appended split fragments at the end

        self.finalized = true;
    }

    fn sort_usable(&mut self) {
        // Insertion Sort
        for i in 1..self.nr_regions {
            let mut j = i;
            while j > 0
                && self.regions[j - 1].region.base
                    > self.regions[j].region.base
            {
                self.regions.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    /// Merge adjacent or overlapping regions that share the same NID.
    /// Assumes the array is already sorted by base address.
    fn merge_usable(&mut self) {
        if self.nr_regions <= 1 {
            return;
        }

        // write cursor – always points to the current region we try
        // to extend
        let mut w = 0;

        for r in 1..self.nr_regions {
            let w_base = self.regions[w].region.base;
            let w_end = w_base + self.regions[w].region.size;

            let r_base = self.regions[r].region.base;
            let r_end = r_base + self.regions[r].region.size;

            let same_nid = self.regions[w].region.nid
                == self.regions[r].region.nid;

            // Overlapping or adjacent, and same NUMA node → merge
            if same_nid && r_base <= w_end {
                if r_end > w_end {
                    self.regions[w].region.size = r_end - w_base;
                    self.regions[w].end = r_end;
                    // cur stays at w's base — correct since finalize
                    // runs before any alloc
                }
            } else {
                w += 1;
                if w != r {
                    self.regions.swap(w, r);
                }
            }
        }

        self.nr_regions = w + 1;
    }

    /// For every reserved region, carve it out of every usable region
    /// it overlaps with.
    fn subtract_reserved(&mut self) {
        for r in 0..self.nr_reserved {
            if self.reserved[r].is_empty() {
                continue;
            }

            let res_base = self.reserved[r].base;
            let res_end = res_base + self.reserved[r].size;

            let mut i = 0;
            while i < self.nr_regions {
                if self.regions[i].is_unused() {
                    i += 1;
                    continue;
                }

                let reg_base = self.regions[i].region.base;
                let reg_end = reg_base + self.regions[i].region.size;
                let nid = self.regions[i].region.nid;

                // ---- no overlap ----
                if res_base >= reg_end || res_end <= reg_base {
                    i += 1;
                    continue;
                }

                // ---- reserved fully covers usable ----
                if res_base <= reg_base && res_end >= reg_end {
                    self.regions[i].clear();
                    i += 1;
                    continue;
                }

                // ---- reserved sits in the middle → split ----
                if res_base > reg_base && res_end < reg_end {
                    // current region becomes the left fragment
                    self.regions[i] = BootMemRegion::new(
                        reg_base,
                        res_base - reg_base,
                        nid,
                    );

                    // append the right fragment
                    if self.nr_regions < MAX_USABLE_REGIONS {
                        self.regions[self.nr_regions] =
                            BootMemRegion::new(
                                res_end,
                                reg_end - res_end,
                                nid,
                            );
                        self.nr_regions += 1;
                    } else {
                        pr_warn!("BootMem: no room for split region");
                    }

                    i += 1;
                    continue;
                }

                // ---- partial overlap ----
                if res_base <= reg_base {
                    // reserved covers the left part → trim left
                    self.regions[i] = BootMemRegion::new(
                        res_end,
                        reg_end - res_end,
                        nid,
                    );
                } else {
                    // reserved covers the right part → trim right
                    self.regions[i] = BootMemRegion::new(
                        reg_base,
                        res_base - reg_base,
                        nid,
                    );
                }

                i += 1;
            }
        }
    }

    /// Remove vacant (cleared) entries and pack the array.
    fn compact(&mut self) {
        let mut w = 0;
        for r in 0..self.nr_regions {
            if !self.regions[r].is_unused() {
                if w != r {
                    self.regions.swap(w, r);
                }
                w += 1;
            }
        }
        self.nr_regions = w;
    }
}

unsafe impl Sync for BootMemInner {}

unsafe impl PageAllocator for BootMem {
    fn alloc_pages(
        order: Order,
        _flags: AllocFlags,
        policy: NumaPolicy,
    ) -> KResult<PhysPageNum> {
        let me = Self::get_mut();
        debug_assert!(me.finalized, "BootMem not yet finalized");

        let size = order.byte_size();
        let align = PAGE_SIZE;

        let (nid, allow_fallback) = {
            let guard = PreemptGuard::new();
            let cpu = guard.cpu();
            match policy {
                NumaPolicy::Local => (cpu.nid(), false),
                NumaPolicy::Strict(nid) => (nid, false),
                NumaPolicy::Preferred => (cpu.nid(), true),
            }
        };

        // Pass 1: try the preferred / strict node
        for i in 0..me.nr_regions {
            if me.regions[i].region.nid != nid {
                continue;
            }
            if let Some(pa) = me.regions[i].alloc(size, align) {
                return Ok(pa.to_ppn());
            }
        }

        if !allow_fallback {
            return KErr!(
                KErrNo::ENOMEM,
                "BootMem: no free pages on node {nid}"
            );
        }

        // Pass 2 (Preferred): try any node
        for i in 0..me.nr_regions {
            if me.regions[i].region.nid == nid {
                continue; // already tried
            }
            if let Some(pa) = me.regions[i].alloc(size, align) {
                return Ok(pa.to_ppn());
            }
        }

        KErr!(KErrNo::ENOMEM, "BootMem: out of memory")
    }

    // Boot memory allocator is a bump allocator
    // We are not considering memory recycling at this time.
    #[inline(always)]
    unsafe fn free_pages(_ppn: PhysPageNum, _order: Order) {}
}

unsafe impl RustAllocator for BootMem {
    fn allocate(
        &self,
        layout: core::alloc::Layout,
    ) -> Result<ptr::NonNull<[u8]>, AllocError> {
        let me = Self::get_mut();
        debug_assert!(me.finalized, "BootMem not yet finalized");

        let size = layout.size();
        let align = layout.align();

        let mut pa = None;
        me.regions.iter_mut().any(|r: &mut BootMemRegion| {
            pa = r.alloc(size, align);
            pa.is_some()
        });

        match pa {
            Some(addr) => {
                let ptr = addr.to_virt().as_mut_ptr::<u8>();
                let slice = unsafe {
                    core::ptr::NonNull::new_unchecked(
                        core::ptr::slice_from_raw_parts_mut(
                            ptr, size,
                        ),
                    )
                };
                Ok(slice)
            }
            None => Err(core::alloc::AllocError),
        }
    }

    unsafe fn deallocate(
        &self,
        _ptr: ptr::NonNull<u8>,
        _layout: core::alloc::Layout,
    ) {
    }
}

pub fn global_bootmem_init() {
    unsafe {
        let regions = BOOTMEM_REGIONS.get_mut();
        BOOTMEM.init(BootMemInner {
            regions,
            nr_regions: 0,
            reserved: core::mem::zeroed(),
            nr_reserved: 0,
            finalized: false,
        });
    };
}
