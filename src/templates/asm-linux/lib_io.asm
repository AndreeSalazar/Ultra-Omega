; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Entrada/Salida (NASM x64 Linux)
; Nivel: Intermedio - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Funciones para entrada y salida de datos.

default rel
section .text
global main
extern printf
extern scanf

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_INT - Imprimir entero
; ═══════════════════════════════════════════════════════════════
%macro PRINT_INT 1
    sub rsp, 8
    lea rdi, [int_fmt]      ; Linux: primer argumento
    mov rsi, %1            ; Linux: segundo argumento
    xor rax, rax
    call printf
    add rsp, 8
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_STR - Imprimir string
; ═══════════════════════════════════════════════════════════════
%macro PRINT_STR 1
    sub rsp, 8
    lea rdi, [%1]           ; Linux: primer argumento
    xor rax, rax
    call printf
    add rsp, 8
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_NEWLINE - Imprimir nueva línea
; ═══════════════════════════════════════════════════════════════
%macro PRINT_NEWLINE 0
    sub rsp, 8
    lea rdi, [newline]      ; Linux: primer argumento
    xor rax, rax
    call printf
    add rsp, 8
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: READ_INT - Leer entero desde teclado
; Uso: READ_INT variable
; ═══════════════════════════════════════════════════════════════
%macro READ_INT 1
    sub rsp, 8
    lea rdi, [int_scan_fmt] ; Linux: primer argumento
    lea rsi, [%1]           ; Linux: segundo argumento
    xor rax, rax
    call scanf
    add rsp, 8
%endmacro

main:
    ; Ejemplo de uso
    PRINT_STR prompt
    READ_INT user_input
    
    PRINT_STR result_msg
    PRINT_INT qword [user_input]
    PRINT_NEWLINE
    
    xor rax, rax
    ret

section .data
    int_fmt db '%d', 0
    int_scan_fmt db '%d', 0
    newline db 10, 0
    prompt db 'Ingrese un numero: ', 0
    result_msg db 'Usted ingreso: ', 0

section .bss
    user_input resq 1

