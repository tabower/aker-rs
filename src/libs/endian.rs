/// Read big-endian u32 from pointer (for DTB, network data, etc.)
#[inline]
unsafe fn read_be_u32(ptr: *const u8) -> u32 {
    let bytes =
        unsafe { core::ptr::read_unaligned(ptr as *const [u8; 4]) };
    u32::from_be_bytes(bytes)
}

/// Reading Little-Endian u32 from a Pointer (for x86 Memory, Most
/// File Formats)
#[inline]
unsafe fn read_le_u32(ptr: *const u8) -> u32 {
    let bytes =
        unsafe { core::ptr::read_unaligned(ptr as *const [u8; 4]) };
    u32::from_le_bytes(bytes)
}

/// Read big-endian u32 from base address + offset
/// # Safety
///
/// `base + offset` must point to at least 4 bytes of valid memory
#[inline]
pub fn read_be_u32_at(base: usize, offset: usize) -> u32 {
    unsafe {
        let ptr = (base as *const u8).add(offset);
        read_be_u32(ptr)
    }
}

/// Read a little-endian u32 from base address + offset
/// # Safety
///
/// `base + offset` must point to at least 4 bytes of valid memory
#[inline]
pub fn read_le_u32_at(base: usize, offset: usize) -> u32 {
    unsafe {
        let ptr = (base as *const u8).add(offset);
        read_le_u32(ptr)
    }
}
