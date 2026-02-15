#![no_std]
#![no_main]

pub mod arch;
pub mod build_gen;
pub mod drivers;
pub mod mm;
pub mod rust;

/// After booting from the architecture, enter this entry.
#[unsafe(no_mangle)]
pub extern "C" fn start_kernel() -> ! {
    arch::init::init();
    loop {}
}
