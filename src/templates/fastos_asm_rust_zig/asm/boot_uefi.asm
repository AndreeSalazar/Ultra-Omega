; ═══════════════════════════════════════════════════════════════════════════════
; FastOS: Bootloader UEFI
; NASM x86_64 - Punto de entrada desde firmware UEFI
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; ENTRY POINT UEFI
; ═══════════════════════════════════════════════════════════════════════════════
global _start
_start:
    ; Guardar contexto UEFI
    push rbp
    mov rbp, rsp
    
    ; Limpiar flags
    cli
    cld
    
    ; Configurar stack
    mov rsp, stack_top
    
    ; Configurar segmentos (flat memory model en long mode)
    mov ax, 0
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Llamar a kernel_main_rust (definido en Rust)
    extern kernel_main_rust
    call kernel_main_rust
    
    ; Si retorna, entrar en loop infinito
.halt:
    hlt
    jmp .halt

; ═══════════════════════════════════════════════════════════════════════════════
; FUNCIONES DE BAJO NIVEL (Llamables desde Rust/Zig)
; ═══════════════════════════════════════════════════════════════════════════════

; ─────────────────────────────────────────────────────────────────────────────
; outb(port, value) - Escribir byte a puerto
; ─────────────────────────────────────────────────────────────────────────────
global outb
outb:
    mov dx, di      ; port (rdi)
    mov al, sil     ; value (rsi)
    out dx, al
    ret

; ─────────────────────────────────────────────────────────────────────────────
; inb(port) - Leer byte de puerto
; ─────────────────────────────────────────────────────────────────────────────
global inb
inb:
    mov dx, di      ; port (rdi)
    in al, dx
    movzx eax, al
    ret

; ─────────────────────────────────────────────────────────────────────────────
; enable_interrupts() - Habilitar interrupciones
; ─────────────────────────────────────────────────────────────────────────────
global enable_interrupts
enable_interrupts:
    sti
    ret

; ─────────────────────────────────────────────────────────────────────────────
; disable_interrupts() - Deshabilitar interrupciones
; ─────────────────────────────────────────────────────────────────────────────
global disable_interrupts
disable_interrupts:
    cli
    ret

; ─────────────────────────────────────────────────────────────────────────────
; halt() - Detener CPU
; ─────────────────────────────────────────────────────────────────────────────
global halt
halt:
    hlt
    ret

section .bss
align 16
stack_bottom:
    resb 8192       ; Stack de 8KB
stack_top:

