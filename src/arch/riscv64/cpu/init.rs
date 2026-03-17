use crate::prelude::*;

use crate::cpu;
use crate::cpu::numa::NId;
use crate::cpu::numa::cpu_node_map_mut;

use crate::kernel::dev::dtb;

pub fn cpu_node_map_setup() {
    let cnm = unsafe { cpu_node_map_mut() };

    dtb::kernel_dtb().for_each_cpu(|dtb_cpu| {
        let cpuid = cpu::CpuId::new(dtb_cpu.id);
        let numaid = NId::new(dtb_cpu.numa_id.unwrap());

        pr_info!("[NUMA] CPU {} is in NUMA node {}\n", cpuid, numaid);

        cnm.cpu_to_node[dtb_cpu.id] = numaid;
        cnm.node_to_cpu[numaid.get()].set(cpuid);
    });
}

pub fn set_nr_cpus() {
    let cpu_count = dtb::kernel_dtb().cpu_count();
    assert!(cpu_count > 0, "No CPUs found in the device tree.");
    pr_info!(
        "[CPU] Detected {} CPU(s) from the device tree.\n",
        cpu_count
    );
    unsafe {
        cpu::set_nr_cpus(cpu_count);
    }
}
