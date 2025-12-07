// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Allocator Personalizado (Zig)
// Allocator de memoria de alto rendimiento para el kernel
// ═══════════════════════════════════════════════════════════════════════════════

const std = @import("std");

// ═══════════════════════════════════════════════════════════════════════════════
// ALLOCATOR SIMPLE PARA EL KERNEL
// ═══════════════════════════════════════════════════════════════════════════════
const HEAP_SIZE = 1024 * 1024 * 16; // 16 MB
var heap: [HEAP_SIZE]u8 align(16) = undefined;
var heap_pos: usize = 0;

/// Allocator simple de pila (stack-based)
pub const KernelAllocator = struct {
    pub fn alloc(self: *KernelAllocator, len: usize, alignment: u29, len_align: u29, ret_addr: usize) ?[*]u8 {
        _ = self;
        _ = len_align;
        _ = ret_addr;
        
        // Alinear
        const aligned_pos = std.mem.alignForward(usize, heap_pos, alignment);
        
        if (aligned_pos + len > HEAP_SIZE) {
            return null;
        }
        
        heap_pos = aligned_pos + len;
        return heap[aligned_pos..aligned_pos + len].ptr;
    }
    
    pub fn resize(self: *KernelAllocator, buf: []u8, buf_align: u29, new_len: usize, ret_addr: usize) bool {
        _ = self;
        _ = buf_align;
        _ = ret_addr;
        
        // Para un allocator simple, no permitimos resize
        return new_len <= buf.len;
    }
    
    pub fn free(self: *KernelAllocator, buf: []u8, buf_align: u29, ret_addr: usize) void {
        _ = self;
        _ = buf;
        _ = buf_align;
        _ = ret_addr;
        
        // En un allocator simple de pila, no hacemos nada
    }
};

/// Función exportada para ser llamada desde Rust
export fn zig_alloc(size: usize) ?[*]u8 {
    var allocator = KernelAllocator{};
    return allocator.alloc(size, 8, 0, @returnAddress());
}

/// Función exportada para liberar memoria
export fn zig_free(ptr: ?[*]u8) void {
    _ = ptr;
    // En un allocator simple, no hacemos nada
}

