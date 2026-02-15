#![no_std]
#![no_main]

pub mod rust;

/// After booting from the architecture, enter this entry.
#[unsafe(no_mangle)]
pub extern "C" fn start_kernel() -> ! {
    loop {}
}
