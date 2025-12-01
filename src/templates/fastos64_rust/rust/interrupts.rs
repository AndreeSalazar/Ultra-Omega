// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: Sistema de Interrupciones en Rust
// Manejo de interrupciones con integración ASM
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]
#![feature(abi_x86_interrupt)]

use core::arch::asm;

// ═══════════════════════════════════════════════════════════════════════════════
// ESTRUCTURA DE INTERRUPCIÓN
// ═══════════════════════════════════════════════════════════════════════════════
#[repr(C, packed)]
pub struct InterruptFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub interrupt_number: u64,
    pub error_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// HANDLER DE INTERRUPCIONES
// ═══════════════════════════════════════════════════════════════════════════════
#[no_mangle]
pub extern "C" fn interrupt_handler_rust(frame: *const InterruptFrame) {
    unsafe {
        let frame = &*frame;
        let int_num = frame.interrupt_number;
        
        // Manejar diferentes interrupciones
        match int_num {
            0 => handle_divide_by_zero(frame),
            14 => handle_page_fault(frame),
            _ => handle_generic_interrupt(int_num, frame),
        }
    }
}

#[no_mangle]
pub extern "C" fn irq_handler_rust(frame: *const InterruptFrame) {
    unsafe {
        let frame = &*frame;
        let irq_num = frame.interrupt_number - 32; // IRQs empiezan en 32
        
        match irq_num {
            0 => handle_timer(),
            1 => handle_keyboard(),
            _ => handle_generic_irq(irq_num),
        }
        
        // Enviar EOI
        send_eoi(irq_num as u8);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HANDLERS ESPECÍFICOS
// ═══════════════════════════════════════════════════════════════════════════════
fn handle_divide_by_zero(_frame: &InterruptFrame) {
    // En producción: escribir a VGA o serial
    unsafe {
        vga_write_string(b"ERROR: Division by zero!\n");
    }
}

fn handle_page_fault(frame: &InterruptFrame) {
    unsafe {
        let cr2: u64;
        asm!("mov {}, cr2", out(reg) cr2);
        
        vga_write_string(b"ERROR: Page fault at address: ");
        // En producción: imprimir dirección
    }
}

fn handle_generic_interrupt(num: u64, _frame: &InterruptFrame) {
    unsafe {
        vga_write_string(b"Interrupt: ");
        // En producción: imprimir número
    }
}

fn handle_timer() {
    static mut TICKS: u64 = 0;
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            // Cada 100 ticks, actualizar algo
        }
    }
}

fn handle_keyboard() {
    unsafe {
        // Leer scancode del puerto 0x60
        let scancode = inb(0x60);
        // Procesar tecla
    }
}

fn handle_generic_irq(irq: u8) {
    unsafe {
        vga_write_string(b"IRQ: ");
        // En producción: imprimir número
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES AUXILIARES
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    fn send_eoi(irq: u8);
    fn inb(port: u16) -> u8;
    fn vga_write_string(s: &[u8]);
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURAR IDT
// ═══════════════════════════════════════════════════════════════════════════════
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    _reserved: u32,
}

#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u64,
}

pub unsafe fn setup_idt() {
    // En producción: configurar todas las entradas IDT
    // apuntando a los ISRs definidos en kernel_entry.asm
    
    extern "C" {
        fn isr0();
        fn load_idt(ptr: *const IdtPtr);
    }
    
    // Crear IDT
    static mut IDT: [IdtEntry; 256] = [IdtEntry {
        offset_low: 0,
        selector: 0x08,
        ist: 0,
        flags: 0x8E,
        offset_mid: 0,
        offset_high: 0,
        _reserved: 0,
    }; 256];
    
    // Configurar entradas (simplificado)
    // En producción: configurar todas las 256 entradas
    
    let idt_ptr = IdtPtr {
        limit: (core::mem::size_of::<IdtEntry>() * 256 - 1) as u16,
        base: IDT.as_ptr() as u64,
    };
    
    load_idt(&idt_ptr);
}

