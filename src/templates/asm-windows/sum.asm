; ==========================================
; Suma Básica en NASM
; ==========================================

default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    
    ; Cargar valores en registros
    mov rax, 15     ; Primer numero
    mov rbx, 27     ; Segundo numero
    
    ; Sumar RBX a RAX (el resultado se guarda en RAX)
    add rax, rbx
    
    ; Preparar argumentos para printf
    mov rcx, fmt    ; Primer argumento: formato
    mov rdx, rax    ; Segundo argumento: el valor a imprimir
    
    call printf
    
    add rsp, 40
    ret

section .data
    fmt db 'La suma de 15 + 27 es: %d', 10, 0