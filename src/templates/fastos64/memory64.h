/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - MEMORY MANAGER
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef MEMORY64_H
#define MEMORY64_H

#include "types64.h"

void memory64_init(BootInfo* boot_info);
void* kmalloc(size_t size);
void kfree(void* ptr);
void* memset(void* dest, int val, size_t len);
void* memcpy(void* dest, const void* src, size_t len);
int memcmp(const void* s1, const void* s2, size_t len);

#endif

