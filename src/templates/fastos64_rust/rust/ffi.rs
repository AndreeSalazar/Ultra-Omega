// ═══════════════════════════════════════════════════════════════════════════════
// FastOS 64-bit: FFI (Foreign Function Interface)
// Interfaz para llamar funciones ASM desde Rust
// ═══════════════════════════════════════════════════════════════════════════════

#![no_std]

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE PUERTOS (Definidas en boot_uefi.asm)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    pub fn outb(port: u16, value: u8);
    pub fn inb(port: u16) -> u8;
    pub fn outw(port: u16, value: u16);
    pub fn inw(port: u16) -> u16;
    pub fn outd(port: u16, value: u32);
    pub fn ind(port: u16) -> u32;
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE INTERRUPCIONES (Definidas en interrupts.asm)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    pub fn send_eoi(irq: u8);
    pub fn cli_asm();
    pub fn sti_asm();
    pub fn get_flags() -> u64;
    pub fn cpuid_asm(eax: u32, result: *mut u32);
    pub fn rdmsr_asm(msr: u32) -> u64;
    pub fn wrmsr_asm(msr: u32, value: u64);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE MEMORIA (Definidas en memory.asm)
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    pub fn memcpy_optimized(dest: *mut u8, src: *const u8, count: usize) -> *mut u8;
    pub fn memset_optimized(dest: *mut u8, value: u8, count: usize) -> *mut u8;
    pub fn memcmp_optimized(ptr1: *const u8, ptr2: *const u8, count: usize) -> i32;
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE SISTEMA
// ═══════════════════════════════════════════════════════════════════════════════
extern "C" {
    pub fn enable_interrupts();
    pub fn disable_interrupts();
    pub fn halt();
    pub fn load_idt(idt_ptr: *const u8);
    pub fn load_gdt(gdt_ptr: *const u8);
}

// ═══════════════════════════════════════════════════════════════════════════════
// WRAPPER SEGURO PARA CPUID
// ═══════════════════════════════════════════════════════════════════════════════
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub unsafe fn cpuid(eax: u32) -> CpuidResult {
    let mut result = [0u32; 4];
    cpuid_asm(eax, result.as_mut_ptr());
    
    CpuidResult {
        eax: result[0],
        ebx: result[1],
        ecx: result[2],
        edx: result[3],
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WRAPPER PARA MSR
// ═══════════════════════════════════════════════════════════════════════════════
pub unsafe fn rdmsr(msr: u32) -> u64 {
    rdmsr_asm(msr)
}

pub unsafe fn wrmsr(msr: u32, value: u64) {
    wrmsr_asm(msr, value);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE MEMORIA SEGURAS
// ═══════════════════════════════════════════════════════════════════════════════
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, count: usize) {
    memcpy_optimized(dest, src, count);
}

pub unsafe fn memset(dest: *mut u8, value: u8, count: usize) {
    memset_optimized(dest, value, count);
}

pub unsafe fn memcmp(ptr1: *const u8, ptr2: *const u8, count: usize) -> i32 {
    memcmp_optimized(ptr1, ptr2, count)
}

