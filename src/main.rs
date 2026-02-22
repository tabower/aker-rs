#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "__tests_main"]

pub mod arch;
pub mod build_gen;
pub mod config;
pub mod drivers;
pub mod kernel;
pub mod libs;
pub mod mm;
pub mod prelude;
pub mod rust;

#[cfg(test)]
pub mod tests;

/// After booting from the architecture, enter this entry.
#[unsafe(no_mangle)]
pub extern "C" fn start_kernel() -> ! {
    kernel::cpu::init::boot_cpu_init();

    arch::init::init();

    #[cfg(test)]
    __tests_main();

    loop {}
}
