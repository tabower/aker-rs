// for test only, once.
use crate::drivers::arch::riscv64::sbi;

pub fn init() {
    sbi::put_str("hello world\n");
    sbi::shutdown(false);
}
