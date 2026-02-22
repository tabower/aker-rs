use crate::arch::mm::PAGE_SIZE;
use crate::libs::dtb;

use super::linker;

#[inline(always)]
pub(super) fn copy_dtb_to_ekernel(dtb_addr: usize) -> (usize, usize) {
    let dtb = dtb::Dtb::new(dtb_addr)
        .expect("[Boot] Device tree information is incorrect.");

    let size = dtb.total_size();

    // Calculate DTB target address: Immediately following the kernel,
    // page-aligned
    let dtb_phys_dst = linker::_ekernel().wrapping_add(PAGE_SIZE - 1)
        & !(PAGE_SIZE - 1);

    unsafe {
        core::ptr::copy_nonoverlapping(
            dtb_addr as *const u8,
            dtb_phys_dst as *mut u8,
            size,
        );
    }

    // Returns the physical memory boundary after copying
    (dtb_phys_dst, dtb_phys_dst.wrapping_add(size))
}
