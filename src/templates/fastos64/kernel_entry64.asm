; ═══════════════════════════════════════════════════════════════════════════════
; FASTOS 64-BIT - KERNEL ENTRY POINT
; ═══════════════════════════════════════════════════════════════════════════════
; Archivo: kernel_entry64.asm
; Descripción: Punto de entrada del kernel en modo largo (64-bit)
; ═══════════════════════════════════════════════════════════════════════════════

[BITS 64]
[GLOBAL _start]
[GLOBAL kernel_entry]
[EXTERN kernel_main]

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; PUNTO DE ENTRADA DEL KERNEL 64-BIT
; ═══════════════════════════════════════════════════════════════════════════════
; RDI = puntero a BootInfo (framebuffer, mapa de memoria, etc.)
; ═══════════════════════════════════════════════════════════════════════════════

_start:
kernel_entry:
    ; Deshabilitar interrupciones mientras configuramos
    cli
    
    ; Configurar nuevo stack de 64 bits
    mov rsp, stack_top
    mov rbp, rsp
    
    ; Limpiar registros de segmento (no usados en modo largo, pero por limpieza)
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    ; Guardar puntero a BootInfo
    push rdi
    
    ; Limpiar BSS
    mov rdi, __bss_start
    mov rcx, __bss_size
    xor rax, rax
    rep stosb
    
    ; Restaurar puntero a BootInfo
    pop rdi
    
    ; Llamar al kernel principal en C
    ; RDI ya contiene el puntero a BootInfo (System V AMD64 ABI)
    call kernel_main
    
    ; Si kernel_main retorna, halt infinito
.hang:
    cli
    hlt
    jmp .hang

; ═══════════════════════════════════════════════════════════════════════════════
; SECCIÓN BSS - Variables no inicializadas
; ═══════════════════════════════════════════════════════════════════════════════

section .bss
align 16

__bss_start:
    ; Stack de 64KB
    resb 65536
stack_top:

__bss_end:

; Calcular tamaño de BSS
__bss_size equ __bss_end - __bss_start

