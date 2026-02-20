use crate::build_gen::linker_const::KERNEL_LMA_BASE;
use crate::build_gen::linker_const::KERNEL_VMA_BASE;
use crate::build_gen::linker_const::PHYSICAL_BASE;

pub const KERNEL_OFFSET: usize = KERNEL_VMA_BASE - KERNEL_LMA_BASE;

unsafe extern "C" {
    // Kernel boundaries
    static skernel: u8;
    static ekernel: u8;

    // Code segment
    static stext: u8;
    static etext: u8;

    // Read-only data segment
    static srodata: u8;
    static erodata: u8;

    // Data segment
    static sdata: u8;
    static edata: u8;

    // BSS section
    static sbss: u8;
    static ebss: u8;

    // Kernel module section
    static skmod: u8;
    static ekmod: u8;
}

macro_rules! define_symbol_getter {
    ($name:ident, $symbol:ident) => {
        #[inline(always)]
        pub fn $name() -> usize {
            unsafe { &$symbol as *const _ as usize }
        }
    };
}

define_symbol_getter!(_skernel, skernel);
define_symbol_getter!(_ekernel, ekernel);
define_symbol_getter!(_stext, stext);
define_symbol_getter!(_etext, etext);
define_symbol_getter!(_srodata, srodata);
define_symbol_getter!(_erodata, erodata);
define_symbol_getter!(_sdata, sdata);
define_symbol_getter!(_edata, edata);
define_symbol_getter!(_sbss, sbss);
define_symbol_getter!(_ebss, ebss);
define_symbol_getter!(_skmod, skmod);
define_symbol_getter!(_ekmod, ekmod);

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
