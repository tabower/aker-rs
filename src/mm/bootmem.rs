use core::cell::UnsafeCell;

use crate::libs::unsafe_static::UnsafeStatic;
use crate::mm::numa::NId;
use crate::mm::page::PAGE_SIZE;
use crate::prelude::*;

use super::addr::PhysAddr;
use super::align::AlignOps;
use super::allocator::*;
use super::region::MemRegion;
use super::region::MemRegionKind;

const MAX_USABLE_REGIONS: usize = 128;
const MAX_RESERVED_REGIONS: usize = 32;

static BOOTMEM: UnsafeStatic<BootMem> = UnsafeStatic::uninit();

static BOOTMEM_REGIONS: UnsafeStatic<
    [BootMemRegion; MAX_USABLE_REGIONS],
> = UnsafeStatic::zeroed();

struct BootMemInner {
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
            pr_warn!("[BootMem] too many usable regions\n");
            return;
        }

        pr_info!(
            "[BootMem] add usable   [{:#018x} - {:#018x}] node {}\n",
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
            pr_warn!("[BootMem] too many reserved regions\n");
            return;
        }

        pr_info!(
            "[BootMem] add reserved [{:#018x} - {:#018x}] node {}\n",
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
            "[BootMem] finalizing {} usable, {} reserved regions\n",
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

pub struct BootMem {
    inner: UnsafeCell<BootMemInner>,
}

unsafe impl Sync for BootMem {}
impl BootMem {
    const fn new(inner: BootMemInner) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    /// Obtain a static reference to the global BootMem    #[inline]
    pub fn get() -> &'static BootMem {
        unsafe { BOOTMEM.get() }
    }

    #[inline(always)]
    pub fn add_region(&self, region: MemRegion) {
        let inner = unsafe { &mut *self.inner.get() };
        inner.add_region(region);
    }

    #[inline(always)]
    pub fn finalize(&self) {
        let inner = unsafe { &mut *self.inner.get() };
        inner.finalize();
    }
}

impl PageAllocator for BootMem {
    fn alloc_pages(
        &self,
        order: u8,
        _flags: AllocFlags,
        nid: NId,
        policy: NumaPolicy,
    ) -> KResult<PageBox<'_>> {
        let me = unsafe { &mut *self.inner.get() };

        debug_assert!(me.finalized, "BootMem not yet finalized");
        let count: usize = 1usize << order;
        let size = count * PAGE_SIZE;
        let align = size;

        // Pass 1: try the preferred / strict node
        for i in 0..me.nr_regions {
            if me.regions[i].region.nid != nid {
                continue;
            }
            if let Some(pa) = me.regions[i].alloc(size, align) {
                return Ok(PageBox::new(pa, order, self));
            }
        }

        // If strict, we must not fall back to other nodes.
        if matches!(policy, NumaPolicy::Strict) {
            return KErr!(
                KErrNo::ENOMEM,
                "BootMem: no free pages on strict node"
            );
        }

        // Pass 2 (Preferred): try any node
        for i in 0..me.nr_regions {
            if me.regions[i].region.nid == nid {
                continue; // already tried
            }
            if let Some(pa) = me.regions[i].alloc(size, align) {
                return Ok(PageBox::new(pa, order, self));
            }
        }

        KErr!(KErrNo::ENOMEM, "BootMem: out of memory")
    }

    fn free_pages(&self, _pa: PhysAddr, _order: u8) -> KResult<()> {
        // Boot memory allocator is a bump allocator
        // We are not considering memory recycling at this time.
        Ok(())
    }
}

pub fn global_bootmem_init() {
    unsafe {
        let regions = BOOTMEM_REGIONS.get_mut();

        BOOTMEM.init(BootMem::new(BootMemInner {
            regions,
            nr_regions: 0,
            reserved: core::mem::zeroed(),
            nr_reserved: 0,
            finalized: false,
        }));
    };
}
