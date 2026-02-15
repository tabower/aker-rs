macro_rules! _csrw {
    ($csr_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", $csr_name, ", {0}"),
                in(reg) $val,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _csrr {
    ($csr_name:literal) => {{
        let mut result: usize;
        unsafe {
            core::arch::asm!(
                concat!("csrr {0}, ", $csr_name),
                out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }};
}

macro_rules! _mv_write {
    ($reg_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("mv ", $reg_name, ", {0}"),
                in(reg) $val,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _mv_read {
    ($reg_name:literal) => {{
        let mut result: usize;
        unsafe {
            core::arch::asm!(
                concat!("mv {0}, ", $reg_name),
                out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }};
}

macro_rules! _csrw_u64 {
    ($csr_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", $csr_name, ", {0}"),
                in(reg) $val as u64,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _csrr_u64 {
    ($csr_name:literal) => {{
        let mut result: u64;
        unsafe {
            core::arch::asm!(
                concat!("csrr {0}, ", $csr_name),
                out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }};
}

macro_rules! _mv_write_u64 {
    ($reg_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("mv ", $reg_name, ", {0}"),
                in(reg) $val as u64,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _mv_read_u64 {
    ($reg_name:literal) => {{
        let mut result: u64;
        unsafe {
            core::arch::asm!(
                concat!("mv {0}, ", $reg_name),
                out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }};
}

pub mod tp {
    #[inline(always)]
    pub fn read() -> usize {
        _mv_read!("tp")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _mv_write!("tp", x);
    }
}

pub mod sstatus {
    pub const SSTATUS_SIE: usize = 1 << 1;
    pub const SSTATUS_SPIE: usize = 1 << 5;
    pub const SSTATUS_SPP: usize = 1 << 8;

    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("sstatus")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("sstatus", x);
    }

    #[inline(always)]
    pub fn read_ssie() -> usize {
        read() & SSTATUS_SIE
    }
    #[inline(always)]

    pub fn set_ssie() {
        write(read() | SSTATUS_SIE);
    }

    #[inline(always)]
    pub fn clear_ssie() {
        write(read() & !SSTATUS_SIE);
    }
}

pub mod stvec {
    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("stvec")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("stvec", x);
    }
}

/// satp Format：
/// ```text
/// 63    60 59        44 43                    0
/// +-------+------------+----------------------+
/// | MODE  |    ASID    |        PPN           |
/// +-------+------------+----------------------+
/// ```
pub mod satp {
    /// satp MODE field offset
    pub const MODE_SHIFT: usize = 60;
    /// satp MODE field mask
    pub const MODE_MASK: usize = 0xF;

    /// satp ASID field offset
    pub const ASID_SHIFT: usize = 44;
    /// satp ASID field mask (16 bits)
    pub const ASID_MASK: usize = 0xFFFF;

    /// satp PPN field offset
    pub const PPN_SHIFT: usize = 0;
    /// satp PPN field mask (44 bits)
    pub const PPN_MASK: usize = (1 << 44) - 1;

    /// Paging Mode:
    pub const MODE_BARE: usize = 0;
    /// Paging Mode: Sv39
    pub const MODE_SV39: usize = 8;
    /// Paging Mode: Sv48
    pub const MODE_SV48: usize = 9;
    /// Paging Mode: Sv57
    pub const MODE_SV57: usize = 10;

    /// Current Paging Mode Configuration
    #[cfg(feature = "sv39")]
    pub const SATP_MODE: usize = MODE_SV39;
    #[cfg(feature = "sv48")]
    pub const SATP_MODE: usize = MODE_SV48;
    #[cfg(feature = "sv57")]
    pub const SATP_MODE: usize = MODE_SV57;

    /// Value of the MODE field after left shift (used for fast
    /// construction of satp)
    pub const SATP_MODE_SHIFT: usize = MODE_SHIFT;

    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("satp")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("satp", x);
    }

    #[inline(always)]
    pub fn swap(x: usize) -> usize {
        let old: usize;
        unsafe {
            core::arch::asm!(
                "csrrw {old}, satp, {new}",
                old = out(reg) old,
                new = in(reg) x,
            );
        }
        old
    }
}

pub mod sepc {
    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("sepc")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("sepc", x);
    }
}

pub mod stval {
    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("stval")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("stval", x);
    }
}

pub mod sscause {
    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("scause")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("scause", x);
    }
}

pub mod stimecmp {
    #[inline(always)]
    pub fn read() -> u64 {
        _csrr_u64!("0x14d")
    }

    #[inline(always)]
    pub fn write(x: u64) {
        _csrw_u64!("0x14d", x);
    }
}

pub mod sie {
    // Supervisor Interrupt Enable
    pub const SIE_SSIE: usize = 1 << 1; // software
    pub const SIE_STIE: usize = 1 << 5; // Timer
    pub const SIE_SEIE: usize = 1 << 9; // external

    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("sie")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("sie", x);
    }

    #[inline(always)]
    pub fn read_stie() -> bool {
        (read() & SIE_STIE) != 0
    }

    #[inline(always)]
    pub fn set_stie() {
        write(read() | SIE_STIE);
    }

    #[inline(always)]
    pub fn clear_stie() {
        write(read() & !SIE_STIE);
    }

    #[inline(always)]
    pub fn set_ssie() {
        write(read() | SIE_SSIE);
    }

    #[inline(always)]
    pub fn clear_ssie() {
        write(read() & !SIE_SSIE);
    }

    #[inline(always)]
    pub fn set_seie() {
        write(read() | SIE_SEIE);
    }

    #[inline(always)]
    pub fn clear_seie() {
        write(read() & !SIE_SEIE);
    }
}
