/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Memory Manager
 * Nivel: Avanzado - Gestor de memoria dinámica
 * ═══════════════════════════════════════════════════════════════════════════
 * Implementación de un gestor de memoria heap simple pero funcional.
 * Usa un algoritmo de first-fit con lista enlazada de bloques libres.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "memory.h"
#include "vga.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Dirección de inicio del heap (después del kernel) */
#define HEAP_START          0x100000    /* 1 MB */
#define HEAP_SIZE           0x400000    /* 4 MB */
#define HEAP_END            (HEAP_START + HEAP_SIZE)

/* Tamaño mínimo de bloque */
#define MIN_BLOCK_SIZE      32

/* Magic number para validación */
#define BLOCK_MAGIC         0xDEADBEEF

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Cabecera de bloque de memoria */
typedef struct block_header {
    uint32_t magic;                     /* Para detectar corrupción */
    uint32_t size;                      /* Tamaño del bloque (sin cabecera) */
    uint8_t is_free;                    /* ¿Está libre? */
    uint8_t padding[3];                 /* Alineamiento */
    struct block_header* next;          /* Siguiente bloque */
    struct block_header* prev;          /* Bloque anterior */
} block_header_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static block_header_t* heap_start = NULL;
static uint32_t total_memory = 0;
static uint32_t used_memory = 0;
static uint32_t free_memory = 0;
static uint32_t num_allocations = 0;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES INTERNAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Alinear tamaño a múltiplo de 8 */
static inline uint32_t align_size(uint32_t size) {
    return (size + 7) & ~7;
}

/* Dividir bloque si es lo suficientemente grande */
static void split_block(block_header_t* block, uint32_t size) {
    uint32_t remaining = block->size - size - sizeof(block_header_t);
    
    if (remaining >= MIN_BLOCK_SIZE) {
        /* Crear nuevo bloque con el espacio restante */
        block_header_t* new_block = (block_header_t*)((uint8_t*)block + 
                                    sizeof(block_header_t) + size);
        
        new_block->magic = BLOCK_MAGIC;
        new_block->size = remaining;
        new_block->is_free = 1;
        new_block->next = block->next;
        new_block->prev = block;
        
        if (block->next) {
            block->next->prev = new_block;
        }
        
        block->next = new_block;
        block->size = size;
        
        free_memory += remaining;
    }
}

/* Fusionar bloques libres adyacentes */
static void coalesce_blocks(block_header_t* block) {
    /* Fusionar con el siguiente si está libre */
    if (block->next && block->next->is_free) {
        block->size += sizeof(block_header_t) + block->next->size;
        block->next = block->next->next;
        
        if (block->next) {
            block->next->prev = block;
        }
    }
    
    /* Fusionar con el anterior si está libre */
    if (block->prev && block->prev->is_free) {
        block->prev->size += sizeof(block_header_t) + block->size;
        block->prev->next = block->next;
        
        if (block->next) {
            block->next->prev = block->prev;
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar gestor de memoria */
void memory_init(void) {
    /* Crear bloque inicial que abarca todo el heap */
    heap_start = (block_header_t*)HEAP_START;
    
    heap_start->magic = BLOCK_MAGIC;
    heap_start->size = HEAP_SIZE - sizeof(block_header_t);
    heap_start->is_free = 1;
    heap_start->next = NULL;
    heap_start->prev = NULL;
    
    total_memory = HEAP_SIZE;
    free_memory = heap_start->size;
    used_memory = 0;
    num_allocations = 0;
}

/* Asignar memoria */
void* kmalloc(uint32_t size) {
    if (size == 0) return NULL;
    
    size = align_size(size);
    
    /* Buscar bloque libre (first-fit) */
    block_header_t* current = heap_start;
    
    while (current) {
        if (current->magic != BLOCK_MAGIC) {
            /* Corrupción de memoria detectada */
            vga_print_color("\n[MEMORY] Heap corruption detected!\n", 
                           VGA_COLOR_WHITE, VGA_COLOR_RED);
            return NULL;
        }
        
        if (current->is_free && current->size >= size) {
            /* Bloque encontrado */
            split_block(current, size);
            
            current->is_free = 0;
            used_memory += current->size;
            free_memory -= current->size;
            num_allocations++;
            
            /* Retornar puntero después de la cabecera */
            return (void*)((uint8_t*)current + sizeof(block_header_t));
        }
        
        current = current->next;
    }
    
    /* No hay memoria disponible */
    return NULL;
}

/* Asignar memoria inicializada a cero */
void* kcalloc(uint32_t count, uint32_t size) {
    uint32_t total = count * size;
    void* ptr = kmalloc(total);
    
    if (ptr) {
        memset(ptr, 0, total);
    }
    
    return ptr;
}

/* Redimensionar bloque de memoria */
void* krealloc(void* ptr, uint32_t new_size) {
    if (ptr == NULL) {
        return kmalloc(new_size);
    }
    
    if (new_size == 0) {
        kfree(ptr);
        return NULL;
    }
    
    /* Obtener cabecera del bloque actual */
    block_header_t* block = (block_header_t*)((uint8_t*)ptr - 
                            sizeof(block_header_t));
    
    if (block->magic != BLOCK_MAGIC) {
        return NULL;
    }
    
    /* Si el nuevo tamaño cabe en el bloque actual */
    if (block->size >= new_size) {
        return ptr;
    }
    
    /* Asignar nuevo bloque y copiar datos */
    void* new_ptr = kmalloc(new_size);
    if (new_ptr) {
        memcpy(new_ptr, ptr, block->size);
        kfree(ptr);
    }
    
    return new_ptr;
}

/* Liberar memoria */
void kfree(void* ptr) {
    if (ptr == NULL) return;
    
    /* Obtener cabecera del bloque */
    block_header_t* block = (block_header_t*)((uint8_t*)ptr - 
                            sizeof(block_header_t));
    
    /* Validar magic number */
    if (block->magic != BLOCK_MAGIC) {
        vga_print_color("\n[MEMORY] Invalid free detected!\n", 
                       VGA_COLOR_WHITE, VGA_COLOR_RED);
        return;
    }
    
    /* Validar que no esté ya libre */
    if (block->is_free) {
        vga_print_color("\n[MEMORY] Double free detected!\n", 
                       VGA_COLOR_WHITE, VGA_COLOR_RED);
        return;
    }
    
    /* Marcar como libre */
    block->is_free = 1;
    used_memory -= block->size;
    free_memory += block->size;
    num_allocations--;
    
    /* Fusionar con bloques adyacentes */
    coalesce_blocks(block);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE UTILIDAD
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Copiar memoria */
void* memcpy(void* dest, const void* src, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    
    while (n--) {
        *d++ = *s++;
    }
    
    return dest;
}

/* Establecer memoria */
void* memset(void* ptr, int value, uint32_t n) {
    uint8_t* p = (uint8_t*)ptr;
    
    while (n--) {
        *p++ = (uint8_t)value;
    }
    
    return ptr;
}

/* Comparar memoria */
int memcmp(const void* s1, const void* s2, uint32_t n) {
    const uint8_t* p1 = (const uint8_t*)s1;
    const uint8_t* p2 = (const uint8_t*)s2;
    
    while (n--) {
        if (*p1 != *p2) {
            return *p1 - *p2;
        }
        p1++;
        p2++;
    }
    
    return 0;
}

/* Mover memoria (maneja solapamiento) */
void* memmove(void* dest, const void* src, uint32_t n) {
    uint8_t* d = (uint8_t*)dest;
    const uint8_t* s = (const uint8_t*)src;
    
    if (d < s) {
        while (n--) {
            *d++ = *s++;
        }
    } else {
        d += n;
        s += n;
        while (n--) {
            *--d = *--s;
        }
    }
    
    return dest;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTADÍSTICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Obtener memoria total */
uint32_t memory_get_total(void) {
    return total_memory;
}

/* Obtener memoria usada */
uint32_t memory_get_used(void) {
    return used_memory;
}

/* Obtener memoria libre */
uint32_t memory_get_free(void) {
    return free_memory;
}

/* Obtener número de asignaciones activas */
uint32_t memory_get_allocations(void) {
    return num_allocations;
}

/* Imprimir estadísticas de memoria */
void memory_print_stats(void) {
    vga_print("\n=== Estadisticas de Memoria ===\n");
    
    vga_print("Total:        ");
    vga_print_dec(total_memory / 1024);
    vga_print(" KB\n");
    
    vga_print("Usada:        ");
    vga_print_dec(used_memory / 1024);
    vga_print(" KB (");
    vga_print_dec((used_memory * 100) / total_memory);
    vga_print("%)\n");
    
    vga_print("Libre:        ");
    vga_print_dec(free_memory / 1024);
    vga_print(" KB (");
    vga_print_dec((free_memory * 100) / total_memory);
    vga_print("%)\n");
    
    vga_print("Asignaciones: ");
    vga_print_dec(num_allocations);
    vga_print("\n");
}

/* Verificar integridad del heap */
int memory_check_heap(void) {
    block_header_t* current = heap_start;
    int errors = 0;
    
    while (current) {
        if (current->magic != BLOCK_MAGIC) {
            vga_print_color("[MEMORY] Block corruption at ", 
                           VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
            vga_print_hex((uint32_t)current);
            vga_print("\n");
            errors++;
        }
        current = current->next;
    }
    
    return errors;
}

