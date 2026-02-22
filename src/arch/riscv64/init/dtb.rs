use crate::arch::boot;
use crate::kernel::dev::dtb;
pub(super) fn dtb_init() {
    let dtb_addr = boot::dtb_addr();
    dtb::set_global(dtb_addr);
}
