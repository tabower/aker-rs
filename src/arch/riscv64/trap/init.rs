use super::trap;
use crate::arch::cpu;

pub fn trap_init() {
    trap::trap_set_vec(trap::kernelvec as *const () as usize);
    cpu::sie_trap_on();
}

pub fn early_trap_init() {
    trap::trap_set_vec(trap::early_kernelvec as *const () as usize);
    cpu::sie_trap_on();
}
