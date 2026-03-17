use core::fmt;
use core::fmt::Write;

use crate::arch;

struct LogStdout;

impl fmt::Write for LogStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            arch::io::console::raw_putstr(s);
        }
        Ok(())
    }
}

// TODO log split;
#[doc(hidden)]
#[inline]
pub fn __log_print(args: fmt::Arguments) {
    LogStdout.write_fmt(args).unwrap();
}

/// no-lock
#[doc(hidden)]
pub fn __raw_print(args: fmt::Arguments) {
    LogStdout.write_fmt(args).unwrap();
}

/// INFO
#[macro_export]
macro_rules! pr_info {
    ($($arg:tt)*) => {
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[34m"));
        $crate::kernel::dev::log::__log_print(format_args!($($arg)*));
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[0m"));
    };
}

/// WARN
#[macro_export]
macro_rules! pr_warn {
    ($($arg:tt)*) => {
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[93m"));
        $crate::kernel::dev::log::__log_print(format_args!($($arg)*));
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[0m"));
    };
}

/// ERROR
#[macro_export]
macro_rules! pr_error {
    ($($arg:tt)*) => {
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[31m"));
        $crate::kernel::dev::log::__log_print(format_args!($($arg)*));
        $crate::kernel::dev::log::__log_print(format_args!("\x1b[0m"));
    };
}

#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)] {
            $crate::kernel::dev::log::__raw_print(format_args!("\x1b[32m"));
            $crate::kernel::dev::log::__raw_print(format_args!($($arg)*));
            $crate::kernel::dev::log::__raw_print(format_args!("\x1b[0m"));
        }
    };
}

#[macro_export]
macro_rules! pr_trace {
    ($cond:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        if $cond {
            $crate::kernel::dev::log::__raw_print(format_args!("\x1b[90m"));
            $crate::kernel::dev::log::__raw_print(format_args!($($arg)*));
            $crate::kernel::dev::log::__raw_print(format_args!("\x1b[0m"));
        }
    };
}
