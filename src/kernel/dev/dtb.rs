use crate::libs::dtb::Dtb;
use crate::libs::unsafe_static::UnsafeStatic;

static GLOBAL_DTB: UnsafeStatic<Dtb> = UnsafeStatic::uninit();

pub fn kernel_dtb() -> &'static Dtb<'static> {
    unsafe { GLOBAL_DTB.get() }
}

pub fn setup_kernel_dtb(dtb_addr: usize) {
    let dtb = Dtb::new(dtb_addr)
        .expect("Failed to initialize the device tree.");

    unsafe {
        GLOBAL_DTB.init(dtb);
    }
}
