use crate::prelude::*;

use super::cpu;
use super::dtb;
use super::mm;
use super::trap;
use super::vm;

pub fn init() {
    trap::early_trap_init();
    dtb::dtb_init();
    cpu::set_nr_cpus();
    cpu::cpu_node_map_setup();
    mm::bootmem_setup();

    // cpu::numaid;
    vm::vm_setup();
    mm::percpu_setup();

    pr_info!("[ARCH] hello world\n");
}
