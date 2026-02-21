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
        let result: usize;
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
        let result: usize;
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
        let result: u64;
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
        let result: u64;
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

macro_rules! _csrs {
    ($csr_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrs ", $csr_name, ", {0}"),
                in(reg) $val,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _csrc {
    ($csr_name:literal, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrc ", $csr_name, ", {0}"),
                in(reg) $val,
                options(nomem, nostack),
            );
        }
    };
}

macro_rules! _csrrc {
    ($csr_name:literal, $val:expr) => {{
        let result: usize;
        unsafe {
            core::arch::asm!(
                concat!("csrrc {0}, ", $csr_name, ", {1}"),
                out(reg) result,
                in(reg) $val,
                options(nomem, nostack),
            );
        }
        result
    }};
}

macro_rules! _csrrw {
    ($csr_name:literal, $val:expr) => {{
        let result: usize;
        unsafe {
            core::arch::asm!(
                concat!("csrrw {0}, ", $csr_name, ", {1}"),
                out(reg) result,
                in(reg) $val,
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
    use core::sync::atomic::Ordering;
    use core::sync::atomic::compiler_fence;

    pub const SSTATUS_SIE: usize = 1 << 1;
    pub const SSTATUS_SPIE: usize = 1 << 5;
    pub const SSTATUS_SPP: usize = 1 << 8;

    #[inline(always)]
    pub fn read() -> usize {
        compiler_fence(Ordering::SeqCst);
        _csrr!("sstatus")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        compiler_fence(Ordering::SeqCst);
        _csrw!("sstatus", x);
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub fn read_sie() -> bool {
        (read() & SSTATUS_SIE) != 0
    }

    #[inline(always)]
    pub fn set_sie() {
        compiler_fence(Ordering::SeqCst);
        _csrs!("sstatus", SSTATUS_SIE);
    }

    #[inline(always)]
    pub fn clear_sie() {
        _csrc!("sstatus", SSTATUS_SIE);
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub fn irq_save() -> usize {
        let flags = _csrrc!("sstatus", SSTATUS_SIE);
        compiler_fence(Ordering::SeqCst);
        flags
    }

    #[inline(always)]
    pub fn irq_restore(flags: usize) {
        compiler_fence(Ordering::SeqCst);
        _csrs!("sstatus", flags & SSTATUS_SIE);
    }
}

pub mod sie {
    pub const SIE_SSIE: usize = 1 << 1;
    pub const SIE_STIE: usize = 1 << 5;
    pub const SIE_SEIE: usize = 1 << 9;

    #[inline(always)]
    pub fn read() -> usize {
        _csrr!("sie")
    }
    #[inline(always)]
    pub fn write(x: usize) {
        _csrw!("sie", x);
    }

    #[inline(always)]
    pub fn set_stie() {
        _csrs!("sie", SIE_STIE);
    }
    #[inline(always)]
    pub fn clear_stie() {
        _csrc!("sie", SIE_STIE);
    }

    #[inline(always)]
    pub fn set_ssie() {
        _csrs!("sie", SIE_SSIE);
    }
    #[inline(always)]
    pub fn clear_ssie() {
        _csrc!("sie", SIE_SSIE);
    }

    #[inline(always)]
    pub fn set_seie() {
        _csrs!("sie", SIE_SEIE);
    }
    #[inline(always)]
    pub fn clear_seie() {
        _csrc!("sie", SIE_SEIE);
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
    use core::sync::atomic::Ordering;
    use core::sync::atomic::compiler_fence;

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
        compiler_fence(Ordering::SeqCst);
        _csrr!("satp")
    }

    #[inline(always)]
    pub fn write(x: usize) {
        compiler_fence(Ordering::SeqCst);
        _csrw!("satp", x);
        compiler_fence(Ordering::SeqCst);
    }

    #[inline(always)]
    pub fn swap(x: usize) -> usize {
        compiler_fence(Ordering::SeqCst);
        let old = _csrrw!("satp", x);
        compiler_fence(Ordering::SeqCst);
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

pub mod scause {
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

#[macro_export]
macro_rules! amoadd_reg_w {
    ($base_reg:literal, $value:expr, $rd_output:ident) => {
        unsafe {
            core::arch::asm!(
                concat!("amoadd.w {rd}, {rs2}, (", $base_reg, ")"),
                rd = out(reg) $rd_output,
                rs2 = in(reg) $value,
                options(nostack),
            );
        }
    };

    ($base_reg:literal, $value:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("amoadd.w {rd}, {rs2}, (", $base_reg, ")"),
                rd = out(reg) _,
                rs2 = in(reg) $value,
                options(nostack),
            );
        }
    };
}
