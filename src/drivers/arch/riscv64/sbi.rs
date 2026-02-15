use crate::mm::addr::PhysAddr;
use crate::mm::addr::VirtAddr;

/// SBI return value
#[derive(Debug)]
pub struct SbiRet {
    /// Error code
    pub error: isize,
    /// Return value
    pub value: usize,
}

impl SbiRet {
    pub fn is_ok(&self) -> bool {
        self.error == 0
    }

    pub fn is_err(&self) -> bool {
        self.error != 0
    }
}

/// Generic SBI ecall
///
/// RISC-V SBI calling convention:
///   a7 = Extension ID (EID)
///   a6 = Function ID (FID)
///   a0-a5 = arguments
///   return: a0 = error, a1 = value
#[inline]
fn sbi_call(eid: usize, fid: usize, args: [usize; 6]) -> SbiRet {
    let error: isize;
    let value: usize;

    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") args[0] => error,
            inlateout("a1") args[1] => value,
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a6") fid,
            in("a7") eid,
        );
    }

    SbiRet { error, value }
}

// Helper macros for different argument counts
#[inline]
#[allow(dead_code)]
fn sbi_call_0(eid: usize, fid: usize) -> SbiRet {
    sbi_call(eid, fid, [0; 6])
}

#[inline]
fn sbi_call_1(eid: usize, fid: usize, a0: usize) -> SbiRet {
    sbi_call(eid, fid, [a0, 0, 0, 0, 0, 0])
}

#[inline]
fn sbi_call_2(eid: usize, fid: usize, a0: usize, a1: usize) -> SbiRet {
    sbi_call(eid, fid, [a0, a1, 0, 0, 0, 0])
}

#[inline]
fn sbi_call_3(
    eid: usize,
    fid: usize,
    a0: usize,
    a1: usize,
    a2: usize,
) -> SbiRet {
    sbi_call(eid, fid, [a0, a1, a2, 0, 0, 0])
}

pub use debug_console::*;
mod debug_console {
    use super::*;

    const EID_DBCN: usize = 0x4442434E; // Debug Console

    /// DBCN: Write bytes (non-blocking), returns bytes written
    fn console_write(num_bytes: usize, phys_addr: PhysAddr) -> SbiRet {
        sbi_call_3(EID_DBCN, 0, num_bytes, phys_addr.as_usize(), 0)
    }

    /// DBCN: Write single byte (blocking)
    fn console_write_byte(byte: u8) -> SbiRet {
        sbi_call_1(EID_DBCN, 2, byte as usize)
    }

    pub fn put_char(c: u8) {
        console_write_byte(c);
    }

    pub fn put_str(s: &str) {
        let bytes = s.as_bytes();
        let pa = VirtAddr::from_ptr(bytes.as_ptr()).to_phys();
        let mut offset = 0;

        while offset < bytes.len() {
            let pa = pa + offset;
            let ret = console_write(bytes.len() - offset, pa);
            if ret.is_err() || ret.value == 0 {
                // Fallback to byte-by-byte
                for &b in &bytes[offset..] {
                    put_char(b);
                }
                return;
            }
            offset += ret.value;
        }
    }
}

pub use system::*;
mod system {
    use super::*;

    const EID_SRST: usize = 0x53525354; // System Reset
    const RESET_SHUTDOWN: usize = 0;

    /// System shutdown reasons
    pub const REASON_NONE: usize = 0;
    /// System shutdown reasons
    pub const REASON_FAILURE: usize = 1;

    /// System shutdown
    pub fn shutdown(failure: bool) -> ! {
        let reason = if failure { REASON_FAILURE } else { REASON_NONE };
        sbi_call_2(EID_SRST, 0, RESET_SHUTDOWN, reason);
        unreachable!()
    }
}
