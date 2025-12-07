// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Port I/O Wrapper (Rust)
// Wrapper seguro para operaciones de puerto I/O
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
}

/// Escribir un byte a un puerto I/O (wrapper seguro)
pub unsafe fn port_outb(port: u16, value: u8) {
    outb(port, value);
}

/// Leer un byte de un puerto I/O (wrapper seguro)
pub unsafe fn port_inb(port: u16) -> u8 {
    inb(port)
}

/// Escribir un word (16 bits) a un puerto I/O
pub unsafe fn port_outw(port: u16, value: u16) {
    outb(port, (value & 0xFF) as u8);
    outb(port + 1, ((value >> 8) & 0xFF) as u8);
}

/// Leer un word (16 bits) de un puerto I/O
pub unsafe fn port_inw(port: u16) -> u16 {
    let low = inb(port) as u16;
    let high = (inb(port + 1) as u16) << 8;
    low | high
}

