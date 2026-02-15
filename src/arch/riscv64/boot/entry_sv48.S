# Kernel entry point for 64-bit RISC-V using Sv48 paging mode.
# ** This file requires location-independent code implementation **

#include "linker_const.h"


# RISC-V64 uses SBI to jump here after M-mode boot completes. 
# The default physical address for this symbol is 0x80200000.
.section .text._entry
.globl _entry
_entry:
    j start_kernel
