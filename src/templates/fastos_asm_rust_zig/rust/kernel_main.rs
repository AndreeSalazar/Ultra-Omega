// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Kernel Main en Rust
// Punto de entrada principal del kernel con integración ASM + Rust + Zig
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

// ═══════════════════════════════════════════════════════════════════════════════
// PANIC HANDLER
// ═══════════════════════════════════════════════════════════════════════════════
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        vga_write_string(b"PANIC: Kernel crashed!\n");
        if let Some(location) = info.location() {
            vga_write_string(b"Location: ");
            vga_write_string(location.file().as_bytes());
            vga_write_string(b"\n");
        }
    }
    loop {}
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES EXTERNAS (Definidas en ASM)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
    fn enable_interrupts();
    fn disable_interrupts();
    fn halt();
    fn memcpy_low(dest: *mut u8, src: *const u8, size: usize) -> *mut u8;
    fn memset_low(dest: *mut u8, value: u8, size: usize) -> *mut u8;
}

// ═══════════════════════════════════════════════════════════════════════════════
// VGA DRIVER SIMPLE
// ═══════════════════════════════════════════════════════════════════════════════
const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

static mut VGA_ROW: usize = 0;
static mut VGA_COL: usize = 0;

unsafe fn vga_write_char(c: u8, color: u8) {
    if VGA_COL >= VGA_WIDTH {
        VGA_COL = 0;
        VGA_ROW += 1;
    }
    
    if VGA_ROW >= VGA_HEIGHT {
        // Scroll
        VGA_ROW = VGA_HEIGHT - 1;
    }
    
    let idx = VGA_ROW * VGA_WIDTH + VGA_COL;
    let entry = ((color as u16) << 8) | (c as u16);
    *VGA_BUFFER.add(idx) = entry;
    
    VGA_COL += 1;
}

unsafe fn vga_write_string(s: &[u8]) {
    for &c in s {
        if c == b'\n' {
            VGA_COL = 0;
            VGA_ROW += 1;
        } else {
            vga_write_char(c, 0x0F); // Blanco sobre negro
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// KERNEL MAIN - Punto de entrada principal
// ═══════════════════════════════════════════════════════════════════════════════
#[no_mangle]
pub extern "C" fn kernel_main_rust() -> ! {
    unsafe {
        // Limpiar pantalla
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *VGA_BUFFER.add(i) = 0x0720; // Espacio en blanco
        }
        
        VGA_ROW = 0;
        VGA_COL = 0;
        
        // Mensaje de bienvenida
        vga_write_string(b"=== FastOS ASM + Rust + Zig ===\n");
        vga_write_string(b"Kernel iniciado correctamente!\n");
        vga_write_string(b"\n");
        
        // Mostrar información del sistema
        vga_write_string(b"Lenguajes integrados:\n");
        vga_write_string(b"  - ASM (NASM): Bootloader y bajo nivel\n");
        vga_write_string(b"  - Rust: Kernel y drivers seguros\n");
        vga_write_string(b"  - Zig: Sistema y allocator\n");
        vga_write_string(b"\n");
        
        // Aquí se puede inicializar Zig cuando esté listo
        // extern "C" { fn zig_system_init(); }
        // zig_system_init();
        
        vga_write_string(b"Sistema operativo listo!\n");
    }
    
    // Loop infinito
    loop {
        unsafe {
            halt();
        }
    }
}

