; ═══════════════════════════════════════════════════════════════════════════════
; FastOS 64-bit: Kernel Entry Point
; Punto de entrada desde bootloader hacia kernel Rust
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; KERNEL ENTRY
; Llamado desde bootloader, salta a Rust
; ═══════════════════════════════════════════════════════════════════════════════
global kernel_entry
kernel_entry:
    ; Configurar stack
    mov rsp, stack_top
    
    ; Limpiar flags
    cli
    cld
    
    ; Llamar a kernel_main_rust
    extern kernel_main_rust
    call kernel_entry_rust
    
    ; Loop infinito si retorna
.halt:
    hlt
    jmp .halt

; ═══════════════════════════════════════════════════════════════════════════════
; INTERRUPT HANDLERS (Stubs que llaman a Rust)
; ═══════════════════════════════════════════════════════════════════════════════

; ─────────────────────────────────────────────────────────────────────────────
; ISR común - Guarda contexto y llama a handler Rust
; ─────────────────────────────────────────────────────────────────────────────
%macro ISR_NOERRCODE 1
global isr%1
isr%1:
    push 0          ; Error code dummy
    push %1         ; Interrupt number
    jmp isr_common_stub
%endmacro

%macro ISR_ERRCODE 1
global isr%1
isr%1:
    push %1         ; Interrupt number (error code ya está en stack)
    jmp isr_common_stub
%endmacro

; Crear todos los ISRs
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
ISR_ERRCODE 30
ISR_NOERRCODE 31

; ISR común - Guarda todos los registros
extern interrupt_handler_rust
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
    
    ; Cargar puntero a estructura de interrupción
    mov rdi, rsp    ; rsp apunta a InterruptFrame
    
    ; Llamar a handler Rust
    call interrupt_handler_rust
    
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
    
    ; Limpiar error code e interrupt number
    add rsp, 16
    
    iretq

; ─────────────────────────────────────────────────────────────────────────────
; IRQ Handlers (Interrupciones de hardware)
; ─────────────────────────────────────────────────────────────────────────────
%macro IRQ 2
global irq%1
irq%1:
    push 0
    push %2
    jmp irq_common_stub
%endmacro

IRQ 0, 32
IRQ 1, 33
IRQ 2, 34
IRQ 3, 35
IRQ 4, 36
IRQ 5, 37
IRQ 6, 38
IRQ 7, 39
IRQ 8, 40
IRQ 9, 41
IRQ 10, 42
IRQ 11, 43
IRQ 12, 44
IRQ 13, 45
IRQ 14, 46
IRQ 15, 47

extern irq_handler_rust
irq_common_stub:
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
    
    mov rdi, rsp
    call irq_handler_rust
    
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
    
    add rsp, 16
    iretq

; ═══════════════════════════════════════════════════════════════════════════════
; FUNCIONES DE MEMORIA
; ═══════════════════════════════════════════════════════════════════════════════

; ─────────────────────────────────────────────────────────────────────────────
; memcpy(dest, src, count) - Copiar memoria
; ─────────────────────────────────────────────────────────────────────────────
global memcpy_asm
memcpy_asm:
    mov rcx, rdx    ; count
    mov rdi, rdi    ; dest
    mov rsi, rsi    ; src
    rep movsb
    mov rax, rdi    ; retornar dest
    ret

; ─────────────────────────────────────────────────────────────────────────────
; memset(dest, value, count) - Llenar memoria
; ─────────────────────────────────────────────────────────────────────────────
global memset_asm
memset_asm:
    mov rcx, rdx    ; count
    mov al, sil     ; value
    mov rdi, rdi    ; dest
    rep stosb
    mov rax, rdi    ; retornar dest
    ret

; ─────────────────────────────────────────────────────────────────────────────
; memcmp(ptr1, ptr2, count) -> diff - Comparar memoria
; ─────────────────────────────────────────────────────────────────────────────
global memcmp_asm
memcmp_asm:
    mov rcx, rdx
    mov rdi, rdi
    mov rsi, rsi
    repe cmpsb
    jz .equal
    movzx rax, byte [rdi-1]
    movzx rdx, byte [rsi-1]
    sub rax, rdx
    ret
.equal:
    xor rax, rax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; STACK
; ═══════════════════════════════════════════════════════════════════════════════
section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

