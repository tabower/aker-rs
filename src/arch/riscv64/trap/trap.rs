use crate::arch::cpu::regs;
use crate::prelude::*;
core::arch::global_asm!(include_str!("kernelvec.S"));

// Defined in the kernelvec.S file,
unsafe extern "C" {
    /// `early_kernelvec` serves as the entry point for trap handling
    /// during the early initialization phase, primarily addressing
    /// errors that may occur during initialization (often address
    /// access errors).
    pub(super) unsafe fn early_kernelvec();

    /// `kernelvec` is the entry point for S-mode trap handling.
    pub(super) unsafe fn kernelvec();
}

#[unsafe(no_mangle)]
pub(super) extern "C" fn kerneltrap() {
    pr_info!("kerneltrap");
}

#[inline(always)]
pub(super) fn trap_set_vec(vec: usize) {
    regs::stvec::write(vec);
}

#[unsafe(no_mangle)]
pub(super) extern "C" fn early_kerneltrap() -> ! {
    let cause = regs::scause::read();
    let epc = regs::sepc::read();
    let tval = regs::stval::read();

    pr_error!("=== EARLY KERNEL TRAP ===\n");
    pr_error!("scause = {:?}\n", cause);
    pr_error!("sepc   = {:#x}\n", epc);
    pr_error!("stval  = {:#x}\n", tval);

    loop {
        core::hint::spin_loop();
    }
}
