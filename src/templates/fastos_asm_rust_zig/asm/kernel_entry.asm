; ═══════════════════════════════════════════════════════════════════════════════
; FastOS: Kernel Entry Point
; NASM x86_64 - Entry point y manejo de interrupciones básico
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; ENTRY POINT DEL KERNEL
; ═══════════════════════════════════════════════════════════════════════════════
global kernel_entry
kernel_entry:
    ; Guardar registros
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    
    ; Llamar al kernel principal en Rust
    extern kernel_main_rust
    call kernel_main_rust
    
    ; Restaurar registros
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; HANDLERS DE INTERRUPCIÓN BÁSICOS
; ═══════════════════════════════════════════════════════════════════════════════

; Interrupción genérica (para pruebas)
global isr_default
isr_default:
    ; Aquí se manejará en Rust/Zig
    iretq

