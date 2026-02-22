use crate::arch::trap;
use crate::prelude::*;

pub fn init() {
    trap::init::early_trap_init();

    pr_info!("[ARCH] hello world\n");
}
