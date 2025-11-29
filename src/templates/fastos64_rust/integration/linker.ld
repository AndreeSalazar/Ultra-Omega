/* ═══════════════════════════════════════════════════════════════════════════════
 * FastOS 64-bit: Linker Script
 * Combina código ASM y Rust en un solo kernel
 * ═══════════════════════════════════════════════════════════════════════════════ */

ENTRY(_start)

SECTIONS
{
    /* Kernel comienza en 1MB (dirección física) */
    . = 0x100000;
    
    /* Código del bootloader (ASM) */
    .boot : {
        *(.boot)
        boot_uefi.o(.text)
    }
    
    /* Código del kernel */
    .text : ALIGN(4K) {
        *(.text)
        kernel_entry.o(.text)
        interrupts.o(.text)
        memory.o(.text)
    }
    
    /* Datos de solo lectura */
    .rodata : ALIGN(4K) {
        *(.rodata)
    }
    
    /* Datos inicializados */
    .data : ALIGN(4K) {
        *(.data)
    }
    
    /* Datos no inicializados */
    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss)
        *(.stack)
    }
    
    /* Símbolos para el kernel */
    kernel_start = .;
    kernel_end = .;
}

