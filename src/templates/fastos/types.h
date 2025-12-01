/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Types Header
 * Nivel: Básico - Definiciones de tipos básicos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef TYPES_H
#define TYPES_H

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

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS ESPECIALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef uint32_t            size_t;
typedef int32_t             ssize_t;
typedef int32_t             ptrdiff_t;
typedef uint32_t            uintptr_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define NULL                ((void*)0)
#define TRUE                1
#define FALSE               0

/* Límites */
#define UINT8_MAX           255
#define UINT16_MAX          65535
#define UINT32_MAX          4294967295U
#define INT8_MAX            127
#define INT8_MIN            (-128)
#define INT16_MAX           32767
#define INT16_MIN           (-32768)
#define INT32_MAX           2147483647
#define INT32_MIN           (-2147483648)

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS ÚTILES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define MIN(a, b)           ((a) < (b) ? (a) : (b))
#define MAX(a, b)           ((a) > (b) ? (a) : (b))
#define ABS(x)              ((x) < 0 ? -(x) : (x))
#define CLAMP(x, lo, hi)    MIN(MAX(x, lo), hi)

#define ARRAY_SIZE(arr)     (sizeof(arr) / sizeof((arr)[0]))
#define ALIGN_UP(x, align)  (((x) + (align) - 1) & ~((align) - 1))
#define ALIGN_DOWN(x, align) ((x) & ~((align) - 1))

#define BIT(n)              (1U << (n))
#define SET_BIT(x, n)       ((x) |= BIT(n))
#define CLEAR_BIT(x, n)     ((x) &= ~BIT(n))
#define TOGGLE_BIT(x, n)    ((x) ^= BIT(n))
#define TEST_BIT(x, n)      (((x) & BIT(n)) != 0)

/* ═══════════════════════════════════════════════════════════════════════════
 * ATRIBUTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PACKED              __attribute__((packed))
#define ALIGNED(n)          __attribute__((aligned(n)))
#define NORETURN            __attribute__((noreturn))
#define UNUSED              __attribute__((unused))
#define ALWAYS_INLINE       __attribute__((always_inline)) inline

#endif /* TYPES_H */

