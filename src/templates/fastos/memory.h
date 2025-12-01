/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Memory Manager Header
 * Nivel: Avanzado - Definiciones del gestor de memoria
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef MEMORY_H
#define MEMORY_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Macro para asignación segura */
#define SAFE_MALLOC(ptr, size) do { \
    (ptr) = kmalloc(size); \
    if (!(ptr)) { \
        vga_print("[MEMORY] Allocation failed!\n"); \
    } \
} while(0)

/* Macro para liberación segura */
#define SAFE_FREE(ptr) do { \
    if (ptr) { \
        kfree(ptr); \
        (ptr) = NULL; \
    } \
} while(0)

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE ASIGNACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void memory_init(void);

/* Asignación dinámica */
void* kmalloc(uint32_t size);
void* kcalloc(uint32_t count, uint32_t size);
void* krealloc(void* ptr, uint32_t new_size);
void kfree(void* ptr);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE UTILIDAD
 * ═══════════════════════════════════════════════════════════════════════════
 */

void* memcpy(void* dest, const void* src, uint32_t n);
void* memset(void* ptr, int value, uint32_t n);
int memcmp(const void* s1, const void* s2, uint32_t n);
void* memmove(void* dest, const void* src, uint32_t n);

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTADÍSTICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t memory_get_total(void);
uint32_t memory_get_used(void);
uint32_t memory_get_free(void);
uint32_t memory_get_allocations(void);
void memory_print_stats(void);
int memory_check_heap(void);

#endif /* MEMORY_H */

