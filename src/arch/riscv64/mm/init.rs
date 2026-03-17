use core::ptr::NonNull;

use crate::config::MAX_CPUS;
use crate::prelude::*;

use crate::arch::boot;

use crate::cpu::numa::NId;

use crate::kernel::dev::dtb;
use crate::kernel::sched::cpu;

use crate::libs::unsafe_static::UnsafeStatic;

use crate::mm::addr::PhysAddr;
use crate::mm::align::AlignOps;
use crate::mm::region::MemRegion;
use crate::mm::region::MemRegionKind;

use super::PAGE_SIZE;

fn ceil_log2(n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    // bits - leading_zeros - 1 = floor(log2(n))
    let floor =
        usize::BITS as usize - 1 - (n.leading_zeros() as usize);
    // If n is not a power of 2, add 1
    if n & (n - 1) != 0 { floor + 1 } else { floor }
}

/// Round-up division
fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

static PERCPU_POINTERS: UnsafeStatic<[NonNull<cpu::Cpu>; MAX_CPUS]> =
    UnsafeStatic::zeroed();

pub fn percpu_setup() {
    // let bm = BootMem::get();
    // let cpu_count = cpu::get_nr_cpus();

    // pr_info!(
    //     "[PERCPU] Allocate percpu structures for {} CPUs\n",
    //     cpu_count
    // );

    // for cpu_id in 0..cpu_count {
    //     let percpu_pages = div_ceil(size_of::<cpu::Cpu>(),
    // PAGE_SIZE);     let percpu_size_order =
    // ceil_log2(percpu_pages);     let percpu_base = bm
    //         .alloc_pages_exact(
    //             percpu_size_order as u8,
    //             AllocFlags::KERNEL,
    //             cpu_to_node(CpuId::new(cpu_id)),
    //         )
    //         .unwrap();

    //     let (pa, _order) = percpu_base.into_raw();

    //     unsafe {
    //         PERCPU_POINTERS.get_mut()[cpu_id] =
    // NonNull::new_unchecked(             pa.as_mut_ptr() as *mut
    // cpu::Cpu,         )
    //     };

    //     pr_info!(
    //         "[PERCPU] Allocated percpu area for CPU {} at address
    // {} in node {}\n",         cpu_id,
    //         pa,
    //         cpu_to_node(CpuId::new(cpu_id))
    //     );
    // }
}

pub fn pages_setup() {
    // let bm = BootMem::get();

    //

    // let total_pages = bm.total_pages();
    // crate::arch::mm::page::init(total_pages);
}

pub fn bootmem_setup() {
    let alloc_end = boot::alloc_end();
    pr_info!(
        "[BOOTMEM] Initial boot memory end at address {:#x}\n",
        alloc_end
    );
    let alloc_end = PhysAddr::new(alloc_end).align_up(PAGE_SIZE);
    pr_info!(
        "[BOOTMEM] Aligned boot memory end to {:#x}\n",
        alloc_end
    );

    let pa_base = boot::linker::phy_base();
    let size = alloc_end.as_usize() - pa_base;

    let boot_reserved_mem = MemRegion::new(
        PhysAddr::new(pa_base),
        size,
        NId::new(0),
        MemRegionKind::Reserved,
    );

    use crate::mm::alloc::bootmem as BM;

    BM::global_bootmem_init();
    let bm = BM::BootMem::get_mut();
    bm.add_region(boot_reserved_mem);

    dtb::kernel_dtb().for_each_mem(
        |region: crate::libs::dtb::DtbMemRegion| {
            bm.add_region(region.into());
        },
    );

    bm.finalize();
}
