; ═══════════════════════════════════════════════════════════════════════════════
; FastOS: Interrupt Service Routines
; NASM x86_64 - Handlers de interrupciones hardware y software
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; Macro para crear ISR sin código de error
%macro ISR_NOERRCODE 1
global isr%1
isr%1:
    push 0          ; Código de error dummy
    push %1         ; Número de interrupción
    jmp isr_common_stub
%endmacro

; Macro para crear ISR con código de error
%macro ISR_ERRCODE 1
global isr%1
isr%1:
    push %1         ; Número de interrupción (ya tiene error code en stack)
    jmp isr_common_stub
%endmacro

; ISRs básicos (0-31: Excepciones CPU)
ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7
ISR_ERRCODE 8
ISR_NOERRCODE 9
ISR_ERRCODE 10
ISR_ERRCODE 11
ISR_ERRCODE 12
ISR_ERRCODE 13
ISR_ERRCODE 14
ISR_NOERRCODE 15
ISR_NOERRCODE 16
ISR_ERRCODE 17
ISR_NOERRCODE 18
ISR_NOERRCODE 19
ISR_NOERRCODE 20
ISR_NOERRCODE 21
ISR_NOERRCODE 22
ISR_NOERRCODE 23
ISR_NOERRCODE 24
ISR_NOERRCODE 25
ISR_NOERRCODE 26
ISR_NOERRCODE 27
ISR_NOERRCODE 28
ISR_NOERRCODE 29
ISR_NOERRCODE 30
ISR_NOERRCODE 31

; Stub común que llama a la función en Rust
extern isr_handler_rust
isr_common_stub:
    ; Guardar todos los registros
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
    
    ; Llamar a la función en Rust
    mov rdi, rsp    ; Pasar puntero al stack frame
    call isr_handler_rust
    
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
    
    ; Limpiar código de error e ID
    add rsp, 16
    iretq

