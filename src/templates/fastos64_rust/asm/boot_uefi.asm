; ═══════════════════════════════════════════════════════════════════════════════
; FastOS 64-bit: Bootloader UEFI
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
; FUNCIONES DE BAJO NIVEL (Llamables desde Rust)
; ═══════════════════════════════════════════════════════════════════════════════

; ─────────────────────────────────────────────────────────────────────────────
; outb(port, value) - Escribir byte a puerto
; ─────────────────────────────────────────────────────────────────────────────
global outb
outb:
    mov dx, di      ; puerto (rdi en System V ABI)
    mov al, sil     ; valor (rsi en System V ABI)
    out dx, al
    ret

; ─────────────────────────────────────────────────────────────────────────────
; inb(port) -> value - Leer byte de puerto
; ─────────────────────────────────────────────────────────────────────────────
global inb
inb:
    mov dx, di      ; puerto
    xor rax, rax
    in al, dx
    ret

; ─────────────────────────────────────────────────────────────────────────────
; outw(port, value) - Escribir word a puerto
; ─────────────────────────────────────────────────────────────────────────────
global outw
outw:
    mov dx, di
    mov ax, si
    out dx, ax
    ret

; ─────────────────────────────────────────────────────────────────────────────
; inw(port) -> value - Leer word de puerto
; ─────────────────────────────────────────────────────────────────────────────
global inw
inw:
    mov dx, di
    xor rax, rax
    in ax, dx
    ret

; ─────────────────────────────────────────────────────────────────────────────
; outd(port, value) - Escribir dword a puerto
; ─────────────────────────────────────────────────────────────────────────────
global outd
outd:
    mov dx, di
    mov eax, esi
    out dx, eax
    ret

; ─────────────────────────────────────────────────────────────────────────────
; ind(port) -> value - Leer dword de puerto
; ─────────────────────────────────────────────────────────────────────────────
global ind
ind:
    mov dx, di
    xor rax, rax
    in eax, dx
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
; halt() - Entrar en estado halt
; ─────────────────────────────────────────────────────────────────────────────
global halt
halt:
    hlt
    ret

; ─────────────────────────────────────────────────────────────────────────────
; load_idt(idt_ptr) - Cargar IDT
; ─────────────────────────────────────────────────────────────────────────────
global load_idt
load_idt:
    lidt [rdi]  ; rdi contiene el puntero a la estructura IDT
    ret

; ─────────────────────────────────────────────────────────────────────────────
; load_gdt(gdt_ptr) - Cargar GDT
; ─────────────────────────────────────────────────────────────────────────────
global load_gdt
load_gdt:
    lgdt [rdi]  ; rdi contiene el puntero a la estructura GDT
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; STACK
; ═══════════════════════════════════════════════════════════════════════════════
section .bss
align 16
stack_bottom:
    resb 16384  ; 16 KB de stack
stack_top:

