; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Impresión de texto (NASM x64 Linux)
; Nivel: Básico - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Este módulo proporciona funcionalidad de impresión.
; Puede heredarse a otros nodos para añadir capacidad de output.

default rel
section .text
global main
extern printf

; ═══════════════════════════════════════════════════════════════
; Macro para imprimir string (uso: PRINT msg)
; ═══════════════════════════════════════════════════════════════
%macro PRINT 1
    sub rsp, 8
    lea rdi, [%1]       ; Linux: primer argumento en rdi
    xor rax, rax
    call printf
    add rsp, 8
%endmacro

main:
    ; Ejemplo de uso
    PRINT mensaje
    
    xor rax, rax
    ret

section .data
    mensaje db 'Librería de impresión cargada (Linux)', 10, 0

