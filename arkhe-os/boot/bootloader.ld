ENTRY(_start)

SECTIONS {
    . = 0x7C00; /* Endereço de carga do bootloader */

    .text : {
        *(.text)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }

    /* Assinatura de boot (55 AA) */
    . = 0x7DFE;
    .boot_sig : {
        SHORT(0xAA55)
    }
}
