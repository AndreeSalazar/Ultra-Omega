// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Ejemplo de Integración ASM + Rust
// Muestra cómo combinar ambos lenguajes
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]
#![no_main]

// ═══════════════════════════════════════════════════════════════════════════════
// EJEMPLO 1: Llamar función ASM desde Rust
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
    fn memcpy_optimized(dest: *mut u8, src: *const u8, count: usize) -> *mut u8;
}

fn ejemplo_port_io() {
    unsafe {
        // Escribir a puerto VGA
        outb(0x3D4, 0x0E);
        let valor = inb(0x3D5);
        // Usar valor...
    }
}

fn ejemplo_memoria() {
    let src = b"Hola desde ASM!";
    let mut dest = [0u8; 16];
    
    unsafe {
        memcpy_optimized(dest.as_mut_ptr(), src.as_ptr(), src.len());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EJEMPLO 2: Función Rust llamada desde ASM
// ═══════════════════════════════════════════════════════════════════════════════
#[no_mangle]
pub extern "C" fn kernel_main_rust() -> ! {
    // Esta función es llamada desde boot_uefi.asm
    
    // Inicializar sistema
    init_system();
    
    // Loop principal
    loop {
        unsafe {
            halt();
        }
    }
}

#[no_mangle]
pub extern "C" fn interrupt_handler_rust(frame: *const InterruptFrame) {
    // Esta función es llamada desde kernel_entry.asm
    
    unsafe {
        let frame = &*frame;
        // Procesar interrupción...
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EJEMPLO 3: Estructura compartida entre ASM y Rust
// ═══════════════════════════════════════════════════════════════════════════════
#[repr(C, packed)]
pub struct InterruptFrame {
    pub r15: u64,
    pub r14: u64,
    // ... resto de registros
    pub interrupt_number: u64,
    pub error_code: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// EJEMPLO 4: Inline Assembly en Rust
// ═══════════════════════════════════════════════════════════════════════════════
use core::arch::asm;

fn leer_cr2() -> u64 {
    unsafe {
        let cr2: u64;
        asm!("mov {}, cr2", out(reg) cr2);
        cr2
    }
}

fn escribir_cr3(value: u64) {
    unsafe {
        asm!("mov cr3, {}", in(reg) value);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES AUXILIARES
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn halt();
}

fn init_system() {
    // Inicializar drivers, memoria, etc.
}

