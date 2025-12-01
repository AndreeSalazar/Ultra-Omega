/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT + VULKAN - LINKER SCRIPT
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: linker64.ld
 * Descripción: Script de enlace para kernel de 64 bits
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Punto de entrada */
ENTRY(_start)

/* Dirección base del kernel (1MB) */
KERNEL_OFFSET = 0x100000;

SECTIONS
{
    /* Comenzar en 1MB */
    . = KERNEL_OFFSET;
    
    /* Cabecera Multiboot2 */
    .multiboot ALIGN(4K) :
    {
        KEEP(*(.multiboot))
    }
    
    /* Código */
    .text ALIGN(4K) :
    {
        *(.text)
        *(.text.*)
    }
    
    /* Datos de solo lectura */
    .rodata ALIGN(4K) :
    {
        *(.rodata)
        *(.rodata.*)
    }
    
    /* Datos inicializados */
    .data ALIGN(4K) :
    {
        *(.data)
        *(.data.*)
    }
    
    /* Datos no inicializados (BSS) */
    .bss ALIGN(4K) :
    {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    }
    
    /* Calcular tamaño de BSS */
    __bss_size = __bss_end - __bss_start;
    
    /* Final del kernel */
    __kernel_end = .;
    
    /* Descartar secciones innecesarias */
    /DISCARD/ :
    {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

