#[cfg(target_arch = "riscv64")]
#[path = "riscv64/mod.rs"]
pub mod current;

// Re-export for external use
pub use current::boot;
pub use current::init;
pub use current::io;
pub use current::mm;

#[cfg(target_arch = "riscv64")]
pub const ARCH_NAME: &str = "RISCV64";
