use crate::arch::cpu as arch_cpu;

use crate::cpu::CpuId;

use super::cpu::BOOT_CPU;

pub fn boot_cpu_init() {
    let id;
    unsafe {
        id = CpuId::new(arch_cpu::cpuid_raw());
        BOOT_CPU.get_mut().set_id(id);
        arch_cpu::set_this_cpu(BOOT_CPU.as_mut_ptr());
    }
}
