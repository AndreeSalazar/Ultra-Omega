// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Drivers de Hardware (Rust)
// Drivers seguros para dispositivos de hardware
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

use crate::ports;

/// Driver VGA básico
pub struct VgaDriver;

impl VgaDriver {
    pub fn new() -> Self {
        VgaDriver
    }
    
    pub fn write_char(&self, c: u8, x: usize, y: usize) {
        unsafe {
            let vga_buffer: *mut u16 = 0xB8000 as *mut u16;
            let idx = y * 80 + x;
            let entry = ((0x0F as u16) << 8) | (c as u16);
            *vga_buffer.add(idx) = entry;
        }
    }
}

/// Driver de teclado básico (PS/2)
pub struct KeyboardDriver;

impl KeyboardDriver {
    pub fn new() -> Self {
        KeyboardDriver
    }
    
    pub fn read_key(&self) -> Option<u8> {
        unsafe {
            // Leer del puerto 0x60 (PS/2 data port)
            let status = ports::port_inb(0x64);
            if status & 0x01 != 0 {
                Some(ports::port_inb(0x60))
            } else {
                None
            }
        }
    }
}

