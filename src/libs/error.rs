/// Standard kernel error codes.
///
/// Represented as negative `isize` values, compatible with Linux
/// errno. Use `as isize` to get the numeric value for syscall
/// returns.
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KErrNo {
    /// Operation not permitted
    EPERM = -1,
    /// No such file or directory
    ENOENT = -2,
    /// No such process
    ESRCH = -3,
    /// Interrupted system call
    EINTR = -4,
    /// I/O error
    EIO = -5,
    /// No such device or address
    ENXIO = -6,
    /// Argument list too long
    E2BIG = -7,
    /// Exec format error
    ENOEXEC = -8,
    /// Bad file number
    EBADF = -9,
    /// Try again
    EAGAIN = -11,
    /// Out of memory
    ENOMEM = -12,
    /// Permission denied
    EACCES = -13,
    /// Bad address
    EFAULT = -14,
    /// Device or resource busy
    EBUSY = -16,
    /// File exists
    EEXIST = -17,
    /// No such device
    ENODEV = -19,
    /// Not a directory
    ENOTDIR = -20,
    /// Is a directory
    EISDIR = -21,
    /// Invalid argument
    EINVAL = -22,
    /// Too many open files
    ENFILE = -24,
    /// No space left on device
    ENOSPC = -28,
    /// Read-only file system
    EROFS = -30,
}

/// Kernel error type combining error code and context message.
///KErr { no: self, msg }

/// Designed for internal kernel error propagation with zero heap
/// allocation.
///
/// # Fields
///
/// * `no` - The error code ([`KErrNo`])
/// * `msg` - Static context message describing the error cause
///
/// # Examples
///
/// ```
/// let err = KErr::new(KErrNo::ENOMEM, "buddy allocator exhausted");
/// println!("{}", err);  // ENOMEM(-12): buddy allocator exhausted
/// ```
#[derive(Debug, Clone, Copy)]
pub struct KErr {
    /// Error code
    pub no: KErrNo,
    /// Context message (static, zero-cost)
    pub msg: &'static str,
}

impl KErr {
    /// Creates a new kernel error.
    ///
    /// # Arguments
    ///
    /// * `no` - Error code
    /// * `msg` - Static string describing the error context
    ///
    /// # Examples
    ///
    /// ```
    /// let err = KErr::new(KErrNo::EINVAL, "page size must be 4096");
    /// ```
    pub const fn new(no: KErrNo, msg: &'static str) -> Self {
        KErr { no, msg }
    }

    /// Returns the numeric errno value for syscall returns.
    ///
    /// # Examples
    ///
    /// ```
    /// let err = KErr::new(KErrNo::ENOENT, "file not found");
    /// assert_eq!(err.errno(), -2);
    /// ```
    pub const fn errno(&self) -> isize {
        self.no as isize
    }
}

/// Creates a `KResult::Err` with the given error code and message.
///
/// # Usage
///
/// ```
/// fn do_something() -> KResult<()> {
///     KErr!(KErrNo::EINVAL, "invalid argument")
/// }
/// ```
///
/// # Note
///
/// Returns `Err(KErr)`, not `KErr` itself.
#[macro_export]
macro_rules! KErr {
    ($no:expr, $msg:expr) => {
        Err($crate::libs::errors::KErr::new($no, $msg))
    };
}

/// Standard result type for kernel operations.
///
/// Alias for `Result<T, KErr>`.
///
/// # Examples
///
/// ```
/// fn read_sector(lba: u64) -> KResult<[u8; 512]> {
///     // ...
/// }
/// ```
pub type KResult<T> = core::result::Result<T, KErr>;

impl core::fmt::Display for KErr {
    /// Formats as `ERRNO(code): message`.
    ///
    /// Example output: `ENOMEM(-12): buddy allocator exhausted`
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(f, "{:?}({}): {}", self.no, self.no as isize, self.msg)
    }
}
