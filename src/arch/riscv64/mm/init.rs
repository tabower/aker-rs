use crate::arch::boot;
use crate::kernel::dev::dtb;
use crate::mm::addr::PhysAddr;
use crate::mm::bootmem::BootMem;
use crate::mm::bootmem::global_bootmem_init;
use crate::mm::numa::NId;
use crate::mm::region::MemRegion;
use crate::mm::region::MemRegionKind;

pub fn mm_init() {
    bootmem_init();
}

fn bootmem_init() {
    let boot_end = boot::alloc_end();
    let pa_base = boot::linker::phy_base();
    let size = boot_end - pa_base;

    let boot_reserved_mem = MemRegion::new(
        PhysAddr::new(pa_base),
        size,
        NId::new(0),
        MemRegionKind::Reserved,
    );

    global_bootmem_init();
    let bm = BootMem::get();
    bm.add_region(boot_reserved_mem);

    dtb::dtb().for_each_mem(|region| {
        bm.add_region(region.into());
    });

    bm.finalize();
}
