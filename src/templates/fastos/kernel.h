/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Kernel Header
 * Nivel: Avanzado - Definiciones principales del kernel
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef KERNEL_H
#define KERNEL_H

#include "types.h"
#include "ports.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN DEL KERNEL
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define KERNEL_NAME         "FastOS"
#define KERNEL_VERSION      "1.0.0"
#define KERNEL_AUTHOR       "Tu Nombre"

/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECCIONES DE MEMORIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define KERNEL_LOAD_ADDR    0x10000     /* Dirección de carga del kernel */
#define KERNEL_STACK_TOP    0x90000     /* Tope del stack del kernel */
#define VIDEO_MEMORY        0xB8000     /* Memoria de video VGA */
#define HEAP_START          0x100000    /* Inicio del heap (1 MB) */
#define HEAP_SIZE           0x400000    /* Tamaño del heap (4 MB) */

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DEL KERNEL
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Punto de entrada del kernel */
void kernel_main(void);

/* Control de interrupciones */
extern void enable_interrupts(void);
extern void disable_interrupts(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS DE DEBUG
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifdef DEBUG
    #define KERNEL_DEBUG(msg) vga_print("[DEBUG] " msg "\n")
#else
    #define KERNEL_DEBUG(msg)
#endif

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS DE ASSERT
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define KERNEL_ASSERT(condition, msg) do { \
    if (!(condition)) { \
        vga_print("[ASSERT FAILED] " msg "\n"); \
        while(1) { __asm__ volatile("cli; hlt"); } \
    } \
} while(0)

#endif /* KERNEL_H */

