// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Foreign Function Interface (FFI)
// Interfaces para comunicación entre ASM, Rust y Zig
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

/// Función exportada para ser llamada desde Zig
#[no_mangle]
pub extern "C" fn rust_get_system_info() -> u64 {
    // Retornar información del sistema
    0x1234567890ABCDEF
}

/// Función para llamar a Zig desde Rust
extern "C" {
    pub fn zig_system_init();
    pub fn zig_alloc(size: usize) -> *mut u8;
    pub fn zig_free(ptr: *mut u8);
}

