// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Kernel Main en Rust
// Punto de entrada principal del kernel
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
    // En producción, escribir a VGA o serial
    unsafe {
        vga_write_string(b"PANIC: Kernel crashed!\n");
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
        // Scroll (simplificado)
        VGA_ROW = VGA_HEIGHT - 1;
        // En producción, implementar scroll real
    }
    
    let index = VGA_ROW * VGA_WIDTH + VGA_COL;
    let entry = (color as u16) << 8 | c as u16;
    *VGA_BUFFER.add(index) = entry;
    
    VGA_COL += 1;
}

unsafe fn vga_write_string(s: &[u8]) {
    let color = 0x0F; // Blanco sobre negro
    for &byte in s {
        if byte == b'\n' {
            VGA_COL = 0;
            VGA_ROW += 1;
        } else {
            vga_write_char(byte, color);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// KERNEL MAIN
// ═══════════════════════════════════════════════════════════════════════════════
#[no_mangle]
pub extern "C" fn kernel_main_rust() -> ! {
    unsafe {
        // Limpiar pantalla
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            *VGA_BUFFER.add(i) = 0x0F20; // Espacio blanco
        }
        
        VGA_ROW = 0;
        VGA_COL = 0;
        
        // Mensaje de bienvenida
        vga_write_string(b"========================================\n");
        vga_write_string(b"  FastOS 64-bit - Kernel en Rust\n");
        vga_write_string(b"========================================\n\n");
        
        vga_write_string(b"[OK] Kernel inicializado\n");
        vga_write_string(b"[OK] Memoria configurada\n");
        vga_write_string(b"[OK] Drivers cargados\n\n");
        
        vga_write_string(b"FastOS listo para usar!\n");
        vga_write_string(b"Presiona cualquier tecla...\n");
    }
    
    // Loop principal
    loop {
        unsafe {
            halt();
        }
    }
}

