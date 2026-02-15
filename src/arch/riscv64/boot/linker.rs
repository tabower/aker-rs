use crate::build_gen::linker_const::KERNEL_LMA_BASE;
use crate::build_gen::linker_const::KERNEL_VMA_BASE;
use crate::build_gen::linker_const::PHYSICAL_BASE;

pub const KERNEL_OFFSET: usize = KERNEL_VMA_BASE - KERNEL_LMA_BASE;

#[inline(always)]
pub const fn vma_base() -> usize {
    KERNEL_VMA_BASE
}

#[inline(always)]
pub const fn lma_base() -> usize {
    KERNEL_LMA_BASE
}

#[inline(always)]
pub const fn phy_base() -> usize {
    PHYSICAL_BASE
}

#[inline(always)]
pub const fn offset() -> usize {
    KERNEL_OFFSET
}

// Address Translation

#[inline(always)]
pub const fn p2v_linear(paddr: usize) -> usize {
    paddr + KERNEL_OFFSET
}

#[inline(always)]
pub const fn v2p_linear(vaddr: usize) -> usize {
    vaddr - KERNEL_OFFSET
}
