// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Port I/O en Rust
// Wrapper seguro para funciones ASM de puertos
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES EXTERNAS (Definidas en boot_uefi.asm)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
    fn outw(port: u16, value: u16);
    fn inw(port: u16) -> u16;
    fn outd(port: u16, value: u32);
    fn ind(port: u16) -> u32;
}

// ═══════════════════════════════════════════════════════════════════════════════
// WRAPPER SEGURO PARA PORT I/O
// ═══════════════════════════════════════════════════════════════════════════════
pub struct Port {
    port: u16,
}

impl Port {
    pub const fn new(port: u16) -> Self {
        Port { port }
    }
    
    pub unsafe fn write_u8(&self, value: u8) {
        outb(self.port, value);
    }
    
    pub unsafe fn read_u8(&self) -> u8 {
        inb(self.port)
    }
    
    pub unsafe fn write_u16(&self, value: u16) {
        outw(self.port, value);
    }
    
    pub unsafe fn read_u16(&self) -> u16 {
        inw(self.port)
    }
    
    pub unsafe fn write_u32(&self, value: u32) {
        outd(self.port, value);
    }
    
    pub unsafe fn read_u32(&self) -> u32 {
        ind(self.port)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PORTS COMUNES
// ═══════════════════════════════════════════════════════════════════════════════
pub mod ports {
    use super::Port;
    
    // VGA
    pub const VGA_INDEX: Port = Port::new(0x3D4);
    pub const VGA_DATA: Port = Port::new(0x3D5);
    
    // PIC (Programmable Interrupt Controller)
    pub const PIC1_COMMAND: Port = Port::new(0x20);
    pub const PIC1_DATA: Port = Port::new(0x21);
    pub const PIC2_COMMAND: Port = Port::new(0xA0);
    pub const PIC2_DATA: Port = Port::new(0xA1);
    
    // PIT (Programmable Interval Timer)
    pub const PIT_CHANNEL0: Port = Port::new(0x40);
    pub const PIT_CHANNEL1: Port = Port::new(0x41);
    pub const PIT_CHANNEL2: Port = Port::new(0x42);
    pub const PIT_COMMAND: Port = Port::new(0x43);
    
    // Keyboard
    pub const KEYBOARD_DATA: Port = Port::new(0x60);
    pub const KEYBOARD_STATUS: Port = Port::new(0x64);
    pub const KEYBOARD_COMMAND: Port = Port::new(0x64);
}

// ═══════════════════════════════════════════════════════════════════════════════
// EJEMPLO: Configurar PIT
// ═══════════════════════════════════════════════════════════════════════════════
pub unsafe fn configure_pit(frequency: u32) {
    // Frecuencia base: 1193182 Hz
    let divisor = 1193182 / frequency;
    
    // Configurar PIT
    ports::PIT_COMMAND.write_u8(0x36); // Channel 0, mode 3, binary
    
    // Escribir divisor (low byte, high byte)
    ports::PIT_CHANNEL0.write_u8((divisor & 0xFF) as u8);
    ports::PIT_CHANNEL0.write_u8((divisor >> 8) as u8);
}

