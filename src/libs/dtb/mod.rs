mod dtb;

pub(super) mod cpu;
pub(super) mod mem;
pub(super) mod numa;
pub(super) mod props;
pub(super) mod raw;

pub use cpu::DtbCpu;
pub use dtb::*;
pub use mem::DtbMemKind;
pub use mem::DtbMemRegion;
