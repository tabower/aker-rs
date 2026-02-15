#[cfg(target_arch = "riscv64")]
#[path = "riscv64/mod.rs"]
pub mod current;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
pub mod current;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64/mod.rs"]
pub mod current;

// Re-export for external use
pub use current::boot;
pub use current::init;
pub use current::mm;
pub use current::io;

#[cfg(target_arch = "riscv64")]
pub const ARCH_NAME: &str = "RISCV64";

#[cfg(target_arch = "x86_64")]
pub const ARCH_NAME: &str = "X86_64";
