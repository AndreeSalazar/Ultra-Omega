// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Drivers en Rust
// Drivers de hardware con integración ASM
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

use crate::ports::{Port, ports};

// ═══════════════════════════════════════════════════════════════════════════════
// VGA DRIVER
// ═══════════════════════════════════════════════════════════════════════════════
pub struct VgaDriver {
    buffer: *mut u16,
    row: usize,
    col: usize,
}

impl VgaDriver {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;
    const BUFFER: *mut u16 = 0xB8000 as *mut u16;
    
    pub fn new() -> Self {
        VgaDriver {
            buffer: Self::BUFFER,
            row: 0,
            col: 0,
        }
    }
    
    pub unsafe fn clear(&mut self) {
        for i in 0..(Self::WIDTH * Self::HEIGHT) {
            *self.buffer.add(i) = 0x0F20; // Espacio blanco
        }
        self.row = 0;
        self.col = 0;
    }
    
    pub unsafe fn write_char(&mut self, c: u8, color: u8) {
        if self.col >= Self::WIDTH {
            self.col = 0;
            self.row += 1;
        }
        
        if self.row >= Self::HEIGHT {
            self.scroll();
        }
        
        let index = self.row * Self::WIDTH + self.col;
        let entry = (color as u16) << 8 | c as u16;
        *self.buffer.add(index) = entry;
        
        self.col += 1;
    }
    
    pub unsafe fn write_string(&mut self, s: &[u8], color: u8) {
        for &byte in s {
            if byte == b'\n' {
                self.col = 0;
                self.row += 1;
            } else {
                self.write_char(byte, color);
            }
        }
    }
    
    fn scroll(&mut self) {
        unsafe {
            // Mover todas las líneas hacia arriba
            for y in 1..Self::HEIGHT {
                for x in 0..Self::WIDTH {
                    let src_idx = y * Self::WIDTH + x;
                    let dst_idx = (y - 1) * Self::WIDTH + x;
                    *self.buffer.add(dst_idx) = *self.buffer.add(src_idx);
                }
            }
            
            // Limpiar última línea
            for x in 0..Self::WIDTH {
                let idx = (Self::HEIGHT - 1) * Self::WIDTH + x;
                *self.buffer.add(idx) = 0x0F20;
            }
            
            self.row = Self::HEIGHT - 1;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEYBOARD DRIVER
// ═══════════════════════════════════════════════════════════════════════════════
pub struct KeyboardDriver {
    data_port: Port,
    status_port: Port,
}

impl KeyboardDriver {
    pub fn new() -> Self {
        KeyboardDriver {
            data_port: ports::KEYBOARD_DATA,
            status_port: ports::KEYBOARD_STATUS,
        }
    }
    
    pub unsafe fn is_data_ready(&self) -> bool {
        (self.status_port.read_u8() & 0x01) != 0
    }
    
    pub unsafe fn read_scancode(&self) -> Option<u8> {
        if self.is_data_ready() {
            Some(self.data_port.read_u8())
        } else {
            None
        }
    }
    
    pub fn scancode_to_char(scancode: u8) -> Option<char> {
        // Tabla de scancodes simplificada
        match scancode {
            0x02 => Some('1'),
            0x03 => Some('2'),
            0x04 => Some('3'),
            0x05 => Some('4'),
            0x06 => Some('5'),
            0x07 => Some('6'),
            0x08 => Some('7'),
            0x09 => Some('8'),
            0x0A => Some('9'),
            0x0B => Some('0'),
            0x10 => Some('q'),
            0x11 => Some('w'),
            0x12 => Some('e'),
            0x13 => Some('r'),
            0x14 => Some('t'),
            0x15 => Some('y'),
            0x16 => Some('u'),
            0x17 => Some('i'),
            0x18 => Some('o'),
            0x19 => Some('p'),
            0x1E => Some('a'),
            0x1F => Some('s'),
            0x20 => Some('d'),
            0x21 => Some('f'),
            0x22 => Some('g'),
            0x23 => Some('h'),
            0x24 => Some('j'),
            0x25 => Some('k'),
            0x26 => Some('l'),
            0x2C => Some('z'),
            0x2D => Some('x'),
            0x2E => Some('c'),
            0x2F => Some('v'),
            0x30 => Some('b'),
            0x31 => Some('n'),
            0x32 => Some('m'),
            0x1C => Some('\n'),
            0x39 => Some(' '),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TIMER DRIVER
// ═══════════════════════════════════════════════════════════════════════════════
pub struct TimerDriver {
    ticks: u64,
}

impl TimerDriver {
    pub fn new() -> Self {
        TimerDriver { ticks: 0 }
    }
    
    pub fn tick(&mut self) {
        self.ticks += 1;
    }
    
    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }
    
    pub fn sleep_ms(&mut self, ms: u64) {
        let target = self.ticks + ms;
        while self.ticks < target {
            // Esperar (en producción: usar halt)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SERIAL DRIVER (Para debugging)
// ═══════════════════════════════════════════════════════════════════════════════
pub struct SerialDriver {
    port: Port,
}

impl SerialDriver {
    pub fn new(port: u16) -> Self {
        SerialDriver {
            port: Port::new(port),
        }
    }
    
    pub unsafe fn init(&self) {
        // Configurar baud rate (115200)
        self.port.write_u8(0x80); // Habilitar DLAB
        self.port.write_u8(0x01);  // Divisor low
        self.port.write_u8(0x00);  // Divisor high
        
        // Configurar línea
        self.port.write_u8(0x03); // 8 bits, no parity, 1 stop bit
        
        // Habilitar FIFO
        self.port.write_u8(0xC7);
        
        // Habilitar interrupciones
        self.port.write_u8(0x0B);
    }
    
    pub unsafe fn write_byte(&self, byte: u8) {
        // Esperar a que el buffer esté vacío
        while (self.port.read_u8() & 0x20) == 0 {}
        self.port.write_u8(byte);
    }
    
    pub unsafe fn write_string(&self, s: &[u8]) {
        for &byte in s {
            self.write_byte(byte);
        }
    }
}

