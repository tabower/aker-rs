# Kernel entry point for 64-bit RISC-V using Sv48 paging mode.
# ** This file requires location-independent code implementation **

#include "koffset.h"


# RISC-V64 uses SBI to jump here after M-mode boot completes.
# The default physical address for this symbol is 0x80200000.

    .section .text._entry
    .globl   _entry
_entry:
# a0 = hartid
# a1 = dtb_physical_address

# Save hartid to tp (Thread Pointer)
    mv       tp, a0

    li       s0, KERNEL_OFFSET

# Clear Bss
bss_clear_start:
    la       t0, sbss
    sub      t0, t0, s0
    la       t1, ebss
    sub      t1, t1, s0

bss_clear_loop:
    bgeu     t0, t1, bss_clear_done
    sd       zero, 0(t0)
    addi     t0, t0, 8
    j        bss_clear_loop
bss_clear_done:

# Load the stack pointer for the kernel.
# We will not enable page table mapping for the time being.
# Sub KERNEL_OFFSET to get the physical address of the stack top.
    la       sp, boot_stack_top
    sub      sp, sp, s0


    j        start_kernel

    .section .bss
    .align   12                     # 4096-byte alignment for the stack
    .global  boot_stack
boot_stack:
    .space   4096 * 4               # 16KB Boot Stack
    .global  boot_stack_top
boot_stack_top:
