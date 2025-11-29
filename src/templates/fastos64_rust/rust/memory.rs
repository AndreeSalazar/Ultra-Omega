// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Gestión de Memoria en Rust
// Allocator, page tables, memory management
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES EXTERNAS (Definidas en memory.asm)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn memcpy_optimized(dest: *mut u8, src: *const u8, count: usize) -> *mut u8;
    fn memset_optimized(dest: *mut u8, value: u8, count: usize) -> *mut u8;
    fn memcmp_optimized(ptr1: *const u8, ptr2: *const u8, count: usize) -> i32;
}

// ═══════════════════════════════════════════════════════════════════════════════
// WRAPPER SEGURO PARA FUNCIONES DE MEMORIA
// ═══════════════════════════════════════════════════════════════════════════════
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, count: usize) {
    memcpy_optimized(dest, src, count);
}

pub unsafe fn memset(dest: *mut u8, value: u8, count: usize) {
    memset_optimized(dest, value, count);
}

pub unsafe fn memcmp(ptr1: *const u8, ptr2: *const u8, count: usize) -> i32 {
    memcmp_optimized(ptr1, ptr2, count)
}

// ═══════════════════════════════════════════════════════════════════════════════
// ALLOCATOR SIMPLE (Bump Allocator)
// ═══════════════════════════════════════════════════════════════════════════════
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
        }
    }
    
    pub unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let align = layout.align();
        let size = layout.size();
        
        // Alinear next
        let aligned_next = (self.next + align - 1) & !(align - 1);
        
        if aligned_next + size > self.heap_end {
            return Err(());
        }
        
        let ptr = aligned_next as *mut u8;
        self.next = aligned_next + size;
        
        Ok(NonNull::new_unchecked(ptr))
    }
    
    pub unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator no hace nada en dealloc
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // En producción: usar mutex o lock
        // Por ahora, simplificado
        core::ptr::null_mut()
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op para bump allocator
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE TABLE STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════
#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[repr(C)]
pub struct PageTableEntry {
    value: u64,
}

impl PageTableEntry {
    pub fn new(addr: u64, flags: u64) -> Self {
        PageTableEntry {
            value: (addr & 0x000F_FFFF_FFFF_F000) | flags,
        }
    }
    
    pub fn present(&self) -> bool {
        (self.value & 1) != 0
    }
    
    pub fn address(&self) -> u64 {
        self.value & 0x000F_FFFF_FFFF_F000
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MEMORY MAP
// ═══════════════════════════════════════════════════════════════════════════════
pub struct MemoryMap {
    pub kernel_start: u64,
    pub kernel_end: u64,
    pub heap_start: u64,
    pub heap_end: u64,
}

impl MemoryMap {
    pub const fn new() -> Self {
        MemoryMap {
            kernel_start: 0x100000,      // 1 MB
            kernel_end: 0x400000,        // 4 MB
            heap_start: 0x400000,        // 4 MB
            heap_end: 0x1000000,         // 16 MB
        }
    }
}

