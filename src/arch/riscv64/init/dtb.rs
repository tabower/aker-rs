use crate::prelude::*;

use crate::arch::boot;

use crate::kernel::dev::dtb::kernel_dtb;
use crate::kernel::dev::dtb::{self};

pub(super) fn dtb_init() {
    let pa_dtb_addr = boot::dtb_addr();
    pr_info!(
        "[DTB] Device tree physical address: {:#x}\n",
        pa_dtb_addr
    );

    let dtb_addr = boot::linker::p2v_linear(pa_dtb_addr);
    dtb::setup_kernel_dtb(dtb_addr);
    let size = kernel_dtb().total_size();

    pr_info!("[DTB] Virtual address : {:#x}\n", dtb_addr);
    pr_info!("[DTB] Total size      : {} (B)\n", size,);
    pr_info!(
        "[DTB] Page range      : [{:#x} - {:#x})\n",
        dtb_addr,
        dtb_addr + size
    );
    pr_info!("[DTB] !! Do NOT occupy DTB pages!\n");
}
