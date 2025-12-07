// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Gestión de Memoria (Rust)
// Gestión segura de memoria con wrappers para funciones ASM
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

extern "C" {
    fn memcpy_low(dest: *mut u8, src: *const u8, size: usize) -> *mut u8;
    fn memset_low(dest: *mut u8, value: u8, size: usize) -> *mut u8;
}

/// Copiar memoria (wrapper seguro)
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    memcpy_low(dest, src, size)
}

/// Llenar memoria con un valor (wrapper seguro)
pub unsafe fn memset(dest: *mut u8, value: u8, size: usize) -> *mut u8 {
    memset_low(dest, value, size)
}

/// Estructura para gestión de memoria del kernel
pub struct MemoryManager {
    // Aquí se puede implementar un allocator personalizado
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {}
    }
    
    // Futuro: métodos para alloc/dealloc
}

