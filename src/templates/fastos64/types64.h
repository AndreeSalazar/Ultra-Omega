/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - TIPOS DE DATOS
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: types64.h
 * Descripción: Tipos de datos fundamentales para arquitectura x86_64
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef TYPES64_H
#define TYPES64_H

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS ENTEROS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef unsigned char       uint8_t;
typedef unsigned short      uint16_t;
typedef unsigned int        uint32_t;
typedef unsigned long long  uint64_t;

typedef signed char         int8_t;
typedef signed short        int16_t;
typedef signed int          int32_t;
typedef signed long long    int64_t;

typedef uint64_t            size_t;
typedef int64_t             ssize_t;
typedef int64_t             ptrdiff_t;
typedef uint64_t            uintptr_t;
typedef int64_t             intptr_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS BOOLEANOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef uint8_t             bool;
#define true                1
#define false               0

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define NULL                ((void*)0)

#define UINT8_MAX           0xFF
#define UINT16_MAX          0xFFFF
#define UINT32_MAX          0xFFFFFFFF
#define UINT64_MAX          0xFFFFFFFFFFFFFFFFULL

#define INT8_MIN            (-128)
#define INT8_MAX            127
#define INT16_MIN           (-32768)
#define INT16_MAX           32767
#define INT32_MIN           (-2147483648)
#define INT32_MAX           2147483647
#define INT64_MIN           (-9223372036854775807LL - 1)
#define INT64_MAX           9223372036854775807LL

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS DE UTILIDAD
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define ALIGN_UP(x, align)      (((x) + ((align) - 1)) & ~((align) - 1))
#define ALIGN_DOWN(x, align)    ((x) & ~((align) - 1))
#define IS_ALIGNED(x, align)    (((x) & ((align) - 1)) == 0)

#define MIN(a, b)               ((a) < (b) ? (a) : (b))
#define MAX(a, b)               ((a) > (b) ? (a) : (b))
#define CLAMP(x, lo, hi)        MIN(MAX(x, lo), hi)

#define ARRAY_SIZE(arr)         (sizeof(arr) / sizeof((arr)[0]))

#define UNUSED(x)               ((void)(x))

#define PACKED                  __attribute__((packed))
#define NORETURN                __attribute__((noreturn))
#define ALWAYS_INLINE           __attribute__((always_inline)) inline

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURA DE INFORMACIÓN DE BOOT
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct PACKED {
    /* Framebuffer */
    uint64_t    framebuffer_addr;
    uint32_t    framebuffer_width;
    uint32_t    framebuffer_height;
    uint32_t    framebuffer_pitch;
    uint32_t    framebuffer_bpp;
    
    /* Mapa de memoria */
    uint64_t    memory_map_addr;
    uint64_t    memory_map_size;
    uint64_t    memory_map_desc_size;
    
    /* ACPI */
    uint64_t    acpi_rsdp_addr;
    
    /* Información del kernel */
    uint64_t    kernel_phys_addr;
    uint64_t    kernel_virt_addr;
    uint64_t    kernel_size;
    
    /* Total de memoria */
    uint64_t    total_memory;
    
} BootInfo;

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS DE MEMORIA UEFI
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef enum {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType
} EFI_MEMORY_TYPE;

typedef struct PACKED {
    uint32_t    Type;
    uint32_t    Pad;
    uint64_t    PhysicalStart;
    uint64_t    VirtualStart;
    uint64_t    NumberOfPages;
    uint64_t    Attribute;
} EFI_MEMORY_DESCRIPTOR;

#endif /* TYPES64_H */

