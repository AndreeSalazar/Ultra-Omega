// ═══════════════════════════════════════════════════════════════════════════════
// FastOS: Sistema de Interrupciones (Rust)
// Manejo seguro de interrupciones con llamadas a ASM
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

extern "C" {
    fn enable_interrupts();
    fn disable_interrupts();
}

/// Estructura del stack frame de interrupción
#[repr(C)]
pub struct InterruptStackFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Habilitar interrupciones globalmente
pub unsafe fn interrupts_enable() {
    enable_interrupts();
}

/// Deshabilitar interrupciones globalmente
pub unsafe fn interrupts_disable() {
    disable_interrupts();
}

/// Handler de interrupción genérico (llamado desde ASM)
#[no_mangle]
pub extern "C" fn isr_handler_rust(stack_frame: *const InterruptStackFrame) {
    // Aquí se puede manejar la interrupción
    // Por ahora, solo ignoramos
}

