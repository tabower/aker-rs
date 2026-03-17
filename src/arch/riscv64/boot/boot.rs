use crate::arch::cpu::regs;
use crate::arch::mm::PAGE_SHIFT;

use super::alloc::BootAllocator;
use super::dtb::copy_dtb_to_ekernel;
use super::linker::p2v_linear;
use super::vm::setup_boot_vm;

// Record basic information during the boot phase to prepare for
// subsequent initialization
struct EarlyBootInfo {
    // The starting point for the next allocatable region in the boot
    // phase. The area preceding this point is reserved (typically
    // 0x80000000 to alloc_end) and must be excluded by the memory
    // allocator.
    pub alloc_end: usize,

    // Stores the device tree address for subsequent parsing of the
    // device tree.
    pub dtb_paddr: usize,
}

static mut EARLY_BOOT_INFO: EarlyBootInfo = EarlyBootInfo {
    alloc_end: 0,
    dtb_paddr: 0,
};

pub fn dtb_addr() -> usize {
    unsafe { EARLY_BOOT_INFO.dtb_paddr }
}

pub fn alloc_end() -> usize {
    unsafe { EARLY_BOOT_INFO.alloc_end }
}

/// Called during the assembly boot phase, it performs:
/// - Copies the device tree immediately adjacent to the kernel binary
///   for subsequent memory allocation
/// - Establish an extremely early memory allocation in 1-page units
/// - Perform an identity + linear virtual memory mapping
///
/// # Arguments
/// `dtb_paddr` - The physical address of the device tree
///
/// # Returns
/// Returning the value to STAP, performing a page table switch in
/// assembly
#[unsafe(no_mangle)]
pub(super) unsafe extern "C" fn early_boot_init(
    dtb_paddr: usize,
) -> usize {
    let (dtb, mem_end) = copy_dtb_to_ekernel(dtb_paddr);

    let mut allocator = BootAllocator::new(mem_end);

    let root = setup_boot_vm(&mut allocator, mem_end);

    let mode = regs::satp::SATP_MODE << regs::satp::SATP_MODE_SHIFT;

    unsafe {
        EARLY_BOOT_INFO.alloc_end = allocator.end();
        EARLY_BOOT_INFO.dtb_paddr = dtb;
    };

    mode | (root >> PAGE_SHIFT)
}
