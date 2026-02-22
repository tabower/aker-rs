use super::regs;
use crate::kernel::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub ra: usize,
    pub sp: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
}

impl Context {
    pub const fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }
}

#[inline(always)]
pub fn amoinc_preempt() {
    crate::amoadd_reg_w!("tp", 1);
}

#[inline(always)]
pub fn amodec_preempt() {
    crate::amoadd_reg_w!("tp", -1);
}

#[inline(always)]
pub fn this_cpu_raw() -> *mut Cpu {
    regs::tp::read() as *mut Cpu
}

#[inline(always)]
pub unsafe fn set_this_cpu(cpu_instance: *mut Cpu) {
    regs::tp::write(cpu_instance as usize);
}

#[inline(always)]
pub fn irq_get() -> bool {
    regs::sstatus::read_sie()
}

#[inline(always)]
pub fn irq_save() -> usize {
    regs::sstatus::irq_save()
}

#[inline(always)]
pub fn irq_restore(flags: usize) {
    regs::sstatus::irq_restore(flags);
}

/// Used for early ID acquisition;
///
/// after switching to per-CPU mode, this interface is no longer used.
#[inline(always)]
pub unsafe fn cpuid_raw() -> usize {
    regs::tp::read()
}
