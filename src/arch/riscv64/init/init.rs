use crate::prelude::*;

use crate::arch::mm;
use crate::arch::trap;

use super::cpu_count;
use super::dtb;

pub fn init() {
    trap::init::early_trap_init();
    dtb::dtb_init();
    cpu_count::set_nr_cpus();
    mm::init::mm_init();

    pr_info!("[ARCH] hello world\n");
}
