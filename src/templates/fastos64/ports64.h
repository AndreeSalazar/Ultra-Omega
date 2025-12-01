/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - PORT I/O
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: ports64.h
 * Descripción: Funciones de entrada/salida de puertos para x86_64
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef PORTS64_H
#define PORTS64_H

#include "types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * SALIDA DE PUERTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline void outb(uint16_t port, uint8_t value) {
    __asm__ volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline void outw(uint16_t port, uint16_t value) {
    __asm__ volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

static inline void outl(uint16_t port, uint32_t value) {
    __asm__ volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ENTRADA DE PUERTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline uint8_t inb(uint16_t port) {
    uint8_t value;
    __asm__ volatile("inb %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static inline uint16_t inw(uint16_t port) {
    uint16_t value;
    __asm__ volatile("inw %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

static inline uint32_t inl(uint16_t port) {
    uint32_t value;
    __asm__ volatile("inl %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ESPERA DE I/O (para dispositivos lentos)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline void io_wait(void) {
    outb(0x80, 0);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * REGISTROS DE CONTROL
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline uint64_t read_cr0(void) {
    uint64_t value;
    __asm__ volatile("mov %%cr0, %0" : "=r"(value));
    return value;
}

static inline void write_cr0(uint64_t value) {
    __asm__ volatile("mov %0, %%cr0" : : "r"(value));
}

static inline uint64_t read_cr2(void) {
    uint64_t value;
    __asm__ volatile("mov %%cr2, %0" : "=r"(value));
    return value;
}

static inline uint64_t read_cr3(void) {
    uint64_t value;
    __asm__ volatile("mov %%cr3, %0" : "=r"(value));
    return value;
}

static inline void write_cr3(uint64_t value) {
    __asm__ volatile("mov %0, %%cr3" : : "r"(value));
}

static inline uint64_t read_cr4(void) {
    uint64_t value;
    __asm__ volatile("mov %%cr4, %0" : "=r"(value));
    return value;
}

static inline void write_cr4(uint64_t value) {
    __asm__ volatile("mov %0, %%cr4" : : "r"(value));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MSR (Model Specific Registers)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline uint64_t rdmsr(uint32_t msr) {
    uint32_t low, high;
    __asm__ volatile("rdmsr" : "=a"(low), "=d"(high) : "c"(msr));
    return ((uint64_t)high << 32) | low;
}

static inline void wrmsr(uint32_t msr, uint64_t value) {
    uint32_t low = value & 0xFFFFFFFF;
    uint32_t high = value >> 32;
    __asm__ volatile("wrmsr" : : "c"(msr), "a"(low), "d"(high));
}

/* MSRs comunes */
#define MSR_EFER            0xC0000080
#define MSR_STAR            0xC0000081
#define MSR_LSTAR           0xC0000082
#define MSR_CSTAR           0xC0000083
#define MSR_SFMASK          0xC0000084
#define MSR_FS_BASE         0xC0000100
#define MSR_GS_BASE         0xC0000101
#define MSR_KERNEL_GS_BASE  0xC0000102

/* ═══════════════════════════════════════════════════════════════════════════
 * INTERRUPCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline void cli(void) {
    __asm__ volatile("cli");
}

static inline void sti(void) {
    __asm__ volatile("sti");
}

static inline void hlt(void) {
    __asm__ volatile("hlt");
}

static inline uint64_t read_rflags(void) {
    uint64_t value;
    __asm__ volatile("pushfq; pop %0" : "=r"(value));
    return value;
}

static inline bool interrupts_enabled(void) {
    return (read_rflags() & (1 << 9)) != 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CPUID
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline void cpuid(uint32_t leaf, uint32_t* eax, uint32_t* ebx, 
                         uint32_t* ecx, uint32_t* edx) {
    __asm__ volatile("cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf), "c"(0));
}

/* ═══════════════════════════════════════════════════════════════════════════
 * TIMESTAMP COUNTER
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline uint64_t rdtsc(void) {
    uint32_t low, high;
    __asm__ volatile("rdtsc" : "=a"(low), "=d"(high));
    return ((uint64_t)high << 32) | low;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MEMORY BARRIERS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static inline void mfence(void) {
    __asm__ volatile("mfence" ::: "memory");
}

static inline void lfence(void) {
    __asm__ volatile("lfence" ::: "memory");
}

static inline void sfence(void) {
    __asm__ volatile("sfence" ::: "memory");
}

#endif /* PORTS64_H */

