; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Operaciones Matemáticas (NASM x64 Windows)
; Nivel: Básico - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Proporciona macros y funciones para operaciones matemáticas.
; Hereda a otros nodos para añadir capacidades de cálculo.

default rel
section .text
global main
extern printf

; ═══════════════════════════════════════════════════════════════
; Macros de operaciones básicas
; ═══════════════════════════════════════════════════════════════

; Sumar dos valores: ADD_VALUES dest, src1, src2
%macro ADD_VALUES 3
    mov %1, %2
    add %1, %3
%endmacro

; Restar: SUB_VALUES dest, src1, src2
%macro SUB_VALUES 3
    mov %1, %2
    sub %1, %3
%endmacro

; Multiplicar: MUL_VALUES dest, src1, src2
%macro MUL_VALUES 3
    mov rax, %2
    imul rax, %3
    mov %1, rax
%endmacro

; Dividir: DIV_VALUES dest, dividendo, divisor
%macro DIV_VALUES 3
    xor rdx, rdx
    mov rax, %2
    mov rcx, %3
    idiv rcx
    mov %1, rax
%endmacro

main:
    ; Ejemplo: calcular (10 + 5) * 2
    ADD_VALUES rax, 10, 5      ; rax = 15
    MUL_VALUES rbx, rax, 2     ; rbx = 30
    
    ; Imprimir resultado
    sub rsp, 40
    lea rcx, [fmt]
    mov rdx, rbx
    call printf
    add rsp, 40
    
    xor eax, eax
    ret

section .data
    fmt db 'Resultado: %d', 10, 0

