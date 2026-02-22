use crate::kernel::cpu;
use crate::kernel::dev::dtb;

pub(super) fn set_nr_cpus() {
    let nr = dtb::dtb().cpu_count();
    unsafe {
        cpu::set_nr_cpus(nr);
    }
}
