use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AllocFlags: u32 {
        // Zone Selection

        /// Require DMA-capable low memory (typically < 16MB)
        const DMA       = 1 << 0;
        /// Require 32-bit addressable memory (< 4GB)
        const DMA32     = 1 << 1;
        /// Allow high memory (if architecture supports)
        const HIGHMEM   = 1 << 2;

        // Behavior Control

        /// May sleep/block waiting for memory
        const WAIT      = 1 << 8;
        /// May trigger memory reclaim (page cache, swap, etc.)
        const RECLAIM   = 1 << 9;
        /// May perform I/O for reclaim
        const IO        = 1 << 10;
        /// May enter filesystem for reclaim
        const FS        = 1 << 11;
        /// Zero the memory before returning
        const ZERO      = 1 << 12;
        /// Never fail (keep retrying, use with caution!)
        const NOFAIL    = 1 << 13;
        /// Don't trigger OOM killer
        const NOOOM     = 1 << 14;
        /// Allocation is for kernel stack (special handling)
        const KSTACK    = 1 << 15;

        // Composite Flags

        /// Normal kernel allocation: may sleep, reclaim, do I/O
        const KERNEL = Self::WAIT.bits()
                     | Self::RECLAIM.bits()
                     | Self::IO.bits()
                     | Self::FS.bits();

        /// Atomic context: cannot sleep, no reclaim
        const ATOMIC = 0;

        /// Kernel + zero memory
        const KERNEL_ZERO = Self::KERNEL.bits() | Self::ZERO.bits();

        /// User space allocation: may use highmem
        const USER = Self::KERNEL.bits() | Self::HIGHMEM.bits();

        /// User + zero (for security)
        const USER_ZERO = Self::USER.bits() | Self::ZERO.bits();

        /// DMA allocation
        const DMA_KERNEL = Self::DMA.bits() | Self::KERNEL.bits();
    }
}

impl Default for AllocFlags {
    fn default() -> Self {
        Self::KERNEL
    }
}

impl AllocFlags {
    /// Can this allocation sleep/block?
    #[inline(always)]
    pub const fn may_sleep(&self) -> bool {
        self.contains(Self::WAIT)
    }

    /// Should memory be zeroed?
    #[inline(always)]
    pub const fn should_zero(&self) -> bool {
        self.contains(Self::ZERO)
    }

    /// Can trigger reclaim?
    #[inline(always)]
    pub const fn may_reclaim(&self) -> bool {
        self.contains(Self::RECLAIM)
    }

    /// Is this a DMA allocation?
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        self.intersects(Self::DMA | Self::DMA32)
    }
}
