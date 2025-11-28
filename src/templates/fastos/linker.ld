/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Linker Script
 * ═══════════════════════════════════════════════════════════════════════════
 */

ENTRY(_start)

SECTIONS
{
    /* Kernel se carga en 0x10000 (64KB) */
    . = 0x10000;
    
    .text : {
        *(.text)
        *(.text.*)
    }
    
    .rodata : {
        *(.rodata)
        *(.rodata.*)
    }
    
    .data : {
        *(.data)
        *(.data.*)
    }
    
    .bss : {
        _bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        _bss_end = .;
    }
    
    _kernel_end = .;
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
