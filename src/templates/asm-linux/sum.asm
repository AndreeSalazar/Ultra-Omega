; ==========================================
; Suma Básica en NASM (Linux x64)
; ==========================================

default rel
section .text
global main
extern printf

main:
    sub rsp, 8
    
    ; Cargar valores en registros
    mov rax, 15     ; Primer numero
    mov rbx, 27     ; Segundo numero
    
    ; Sumar RBX a RAX (el resultado se guarda en RAX)
    add rax, rbx
    
    ; Preparar argumentos para printf (Linux: rdi, rsi, rdx, rcx)
    mov rdi, fmt    ; Primer argumento: formato
    mov rsi, rax    ; Segundo argumento: el valor a imprimir
    
    ; RAX = 0 para variadic functions
    xor rax, rax
    call printf
    
    add rsp, 8
    xor rax, rax
    ret

section .data
    fmt db 'La suma de 15 + 27 es: %d', 10, 0

