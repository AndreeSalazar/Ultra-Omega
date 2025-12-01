; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Impresión de texto (NASM x64 Windows)
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
    sub rsp, 40
    lea rcx, [%1]
    call printf
    add rsp, 40
%endmacro

main:
    ; Ejemplo de uso
    PRINT mensaje
    
    xor eax, eax
    ret

section .data
    mensaje db 'Librería de impresión cargada', 10, 0

