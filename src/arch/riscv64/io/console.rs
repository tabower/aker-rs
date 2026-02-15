use crate::drivers::arch::riscv64::sbi;

// TODO
// Write it this way for now;
// we'll refactor it later after establishing the driver framework.

pub unsafe fn raw_putchar(c: usize) {
    sbi::put_char(c as u8);
}

pub unsafe fn raw_putstr(s: &str) {
    sbi::put_str(s);
}
