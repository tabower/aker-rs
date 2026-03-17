use crate::arch::boot::linker::KernelSection;
use crate::kernel::dev::dtb::kernel_dtb;
use crate::libs::dtb::DtbMemKind;
use crate::mm::align::AlignOps;
use crate::prelude::*;

use crate::arch::boot::linker;
use crate::arch::mm::PAGE_SHIFT;
use crate::arch::mm::PAGE_SIZE;
use crate::arch::vm::pte::PTEFlags;

use crate::libs::unsafe_static::UnsafeStatic;

use crate::mm::addr::PhysAddr;
use crate::mm::addr::VirtAddr;
use crate::mm::alloc::AllocFlags;
use crate::mm::alloc::NumaPolicy;
use crate::mm::alloc::PageBox;
use crate::mm::alloc::bootmem::BootMem;
use crate::mm::vm::level::PageLevel;
use crate::mm::vm::pagetable::PageTableRoot;

use super::sv::SvPageConfig;

static KERNEL_PGTABE_BOOT: UnsafeStatic<
    PageTableRoot<SvPageConfig, BootMem, BootMem>,
> = UnsafeStatic::zeroed();

pub fn vm_setup() {
    // alloc root page
    let root_page = PageBox::<BootMem>::new_numa_page(
        NumaPolicy::Local,
        AllocFlags::KERNEL_ZERO,
    )
    .unwrap();

    pr_info!(
        "[VM] Alloc Kernel Root Page PPN: {}, addr: {}\n",
        root_page.ppn(),
        root_page.ppn().to_addr()
    );

    unsafe {
        KERNEL_PGTABE_BOOT
            .init(PageTableRoot::new(root_page, BootMem));
    }

    let pmd_align = PageLevel::PMD.pages_per_entry() * PAGE_SIZE;

    kernel_dtb().for_each_mem(|region| {
        let pa_start =
            PhysAddr::new(region.base).align_down(pmd_align);
        let pa_end = PhysAddr::new(region.base + region.size)
            .align_up(pmd_align);

        let (name, flags) = match region.kind {
            DtbMemKind::Ram => ("RAM", PTEFlags::KERNEL_RW),
            DtbMemKind::Mmio => ("MMIO", PTEFlags::KERNEL_RW),
            DtbMemKind::Reserved => return,
        };

        let seg = KernelSection::from_pa(
            pa_start.as_usize(),
            pa_end.as_usize(),
        );
        map_mem_seg(&seg, name, flags);
    });

    let sbi_start =
        PhysAddr::new(linker::phy_base()).align_down(pmd_align);
    let sbi_end = VirtAddr::new(linker::_stext())
        .to_phys()
        .align_up(pmd_align);

    if sbi_end > sbi_start {
        let sbi_seg = KernelSection::from_pa(
            sbi_start.as_usize(),
            sbi_end.as_usize(),
        );
        unmap_mem_seg(&sbi_seg, "SBI_RESERVED");
    }

    // .text - R-X
    let text = linker::text_section();
    unmap_mem_seg(&text, "TEXT");
    map_mem_seg(&text, "TEXT", PTEFlags::KERNEL_RX);

    // .rodata - R--
    let rodata = linker::rodata_section();
    if rodata.size > 0 {
        unmap_mem_seg(&rodata, "RODATA");
        map_mem_seg(&rodata, "RODATA", PTEFlags::KERNEL_RO);
    }

    // .data + .bss - RW-
    let data_bss = linker::data_bss_section();
    if data_bss.size > 0 {
        unmap_mem_seg(&data_bss, "DATA_BSS");
        map_mem_seg(&data_bss, "DATA_BSS", PTEFlags::KERNEL_RW);
    }

    // JUST FOR TEST
    let kernel_pgtable = unsafe { KERNEL_PGTABE_BOOT.get_mut() };
    let root_pa = kernel_pgtable.pa();

    use crate::arch::cpu::regs;
    let mode = regs::satp::SATP_MODE << regs::satp::SATP_MODE_SHIFT;
    let satp_val = mode | (root_pa.as_usize() >> PAGE_SHIFT);

    pr_info!(
        "[VM] Switching page table, satp={:#x}, table={:#x}\n",
        satp_val,
        root_pa
    );

    use crate::arch::cpu::regs::satp;
    satp::write(satp_val);
    unsafe {
        core::arch::asm!("sfence.vma", options(nostack));
    }
}

fn unmap_mem_seg(seg: &KernelSection, name: &str) {
    let kernel_pgtable = unsafe { KERNEL_PGTABE_BOOT.get_mut() };
    pr_info!(
        "[VM] UnMap {} PA [{:#x}, {:#x}),\tsize {:#x},\tVA [{:#x}, {:#x})\n",
        name,
        seg.pa_start,
        seg.pa_start + seg.size,
        seg.size,
        seg.va_start,
        seg.va_start + seg.size,
    );

    kernel_pgtable
        .unmap_range(
            VirtAddr::new(seg.va_start).to_ppn(),
            seg.size / PAGE_SIZE,
            PageLevel::PMD,
        )
        .expect("unmap_mem_seg unmap panic");
}

fn map_mem_seg(seg: &KernelSection, name: &str, flags: PTEFlags) {
    pr_info!(
        "[VM] Map   {} PA [{:#x}, {:#x}),\tsize {:#x},\tVA [{:#x}, {:#x})\n",
        name,
        seg.pa_start,
        seg.pa_start + seg.size,
        seg.size,
        seg.va_start,
        seg.va_start + seg.size,
    );

    let kernel_pgtable = unsafe { KERNEL_PGTABE_BOOT.get_mut() };

    kernel_pgtable
        .map_range(
            VirtAddr::new(seg.va_start).to_ppn(),
            PhysAddr::new(seg.pa_start).to_ppn(),
            seg.size / PAGE_SIZE,
            flags,
            PageLevel::PMD,
        )
        .expect("Failed to map kernel segment");
}
