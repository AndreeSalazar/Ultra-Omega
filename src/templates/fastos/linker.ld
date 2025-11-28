/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Linker Script
 * Nivel: Avanzado - Script de enlazado para el kernel
 * ═══════════════════════════════════════════════════════════════════════════
 * Este script define cómo se organizan las secciones del kernel en memoria.
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Punto de entrada del kernel */
ENTRY(_start)

/* Dirección base del kernel (1 MB) */
KERNEL_OFFSET = 0x10000;

SECTIONS
{
    /* Establecer ubicación inicial */
    . = KERNEL_OFFSET;
    
    /* ═══════════════════════════════════════════════════════════════════════
     * SECCIÓN DE CÓDIGO (.text)
     * ═══════════════════════════════════════════════════════════════════════
     */
    .text BLOCK(4K) : ALIGN(4K)
    {
        /* Primero el código de entrada */
        *(.multiboot)
        *kernel_entry*.o(.text)
        
        /* Después el resto del código */
        *(.text)
        *(.text.*)
    }
    
    /* Guardar fin de código */
    _text_end = .;
    
    /* ═══════════════════════════════════════════════════════════════════════
     * SECCIÓN DE SOLO LECTURA (.rodata)
     * ═══════════════════════════════════════════════════════════════════════
     */
    .rodata BLOCK(4K) : ALIGN(4K)
    {
        *(.rodata)
        *(.rodata.*)
    }
    
    /* ═══════════════════════════════════════════════════════════════════════
     * SECCIÓN DE DATOS INICIALIZADOS (.data)
     * ═══════════════════════════════════════════════════════════════════════
     */
    .data BLOCK(4K) : ALIGN(4K)
    {
        *(.data)
        *(.data.*)
    }
    
    /* ═══════════════════════════════════════════════════════════════════════
     * SECCIÓN DE DATOS NO INICIALIZADOS (.bss)
     * ═══════════════════════════════════════════════════════════════════════
     */
    .bss BLOCK(4K) : ALIGN(4K)
    {
        _bss_start = .;
        
        *(COMMON)
        *(.bss)
        *(.bss.*)
        
        _bss_end = .;
    }
    
    /* ═══════════════════════════════════════════════════════════════════════
     * FIN DEL KERNEL
     * ═══════════════════════════════════════════════════════════════════════
     */
    _kernel_end = .;
    
    /* ═══════════════════════════════════════════════════════════════════════
     * DESCARTAR SECCIONES NO NECESARIAS
     * ═══════════════════════════════════════════════════════════════════════
     */
    /DISCARD/ :
    {
        *(.comment)
        *(.note)
        *(.note.*)
        *(.eh_frame)
        *(.eh_frame_hdr)
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * SÍMBOLOS EXPORTADOS
 * ═══════════════════════════════════════════════════════════════════════════
 * Estos símbolos pueden ser usados desde C:
 * 
 * extern uint32_t _text_end;
 * extern uint32_t _bss_start;
 * extern uint32_t _bss_end;
 * extern uint32_t _kernel_end;
 * ═══════════════════════════════════════════════════════════════════════════
 */

