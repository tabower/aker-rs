OUTPUT_ARCH( "riscv" )
ENTRY( _entry )

/*
 * SV48 High Address Space
 * KERNEL_VMA_BASE Must be 2MB-aligned to support large page mapping
 */

KERNEL_VMA_BASE = 0xFFFF800000200000;
KERNEL_LMA_BASE = 0x80200000;
PHYSICAL_BASE   = 0x80000000;

/* 2MB alignment constant */
PAGE_SIZE_2M = 0x200000;

PHDRS
{
    text   PT_LOAD FLAGS(0x5); /* R-X */
    rodata PT_LOAD FLAGS(0x4); /* R-- */
    data   PT_LOAD FLAGS(0x6); /* RW- */
}

SECTIONS
{
    . = KERNEL_VMA_BASE;
    PROVIDE(skernel = .);

    /* (.text) - RX */
    PROVIDE(stext = .);
    .text : AT(KERNEL_LMA_BASE) {
        KEEP(*(.text._entry))
        *(.text .text.*)
    } :text
    . = ALIGN(PAGE_SIZE_2M);
    PROVIDE(etext = .);

    /* (.rodata) - R */
    PROVIDE(srodata = .);
    .rodata : {
        *(.srodata .srodata.*)
        *(.rodata .rodata.*)
    } :rodata
    . = ALIGN(PAGE_SIZE_2M);
    PROVIDE(erodata = .);

    /* (.data + .bss) - RW, single segment */
    PROVIDE(sdata = .);
    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
        . = ALIGN(8);
        *(.got .got.* .got.plt)
    } :data
    PROVIDE(edata = .);

    . = ALIGN(8);
    PROVIDE(sbss = .);
    .bss : {
        *(.sbss .sbss.*)
        *(.bss .bss.*)
    } :data
    PROVIDE(ebss = .);

    . = ALIGN(PAGE_SIZE_2M);
    PROVIDE(ekernel = .);

    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
        *(.note*)
    }
}