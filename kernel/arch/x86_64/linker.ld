ENTRY(_start)

SECTIONS
{
    /* Higher-half virtual base address */
    . = 0xFFFFFFFF80000000;

    /* Physical load address (Limine maps this for us) */
    KERNEL_PHYS_BASE = 0x100000;

    .text : AT(KERNEL_PHYS_BASE) {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
}
