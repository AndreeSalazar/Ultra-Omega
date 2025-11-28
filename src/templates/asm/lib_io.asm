; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Entrada/Salida (NASM x64 Windows)
; Nivel: Intermedio - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Funciones para entrada y salida de datos.

default rel
section .text
global main
extern printf
extern scanf
extern gets

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_INT - Imprimir entero
; ═══════════════════════════════════════════════════════════════
%macro PRINT_INT 1
    sub rsp, 40
    lea rcx, [int_fmt]
    mov rdx, %1
    call printf
    add rsp, 40
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_STR - Imprimir string
; ═══════════════════════════════════════════════════════════════
%macro PRINT_STR 1
    sub rsp, 40
    lea rcx, [%1]
    call printf
    add rsp, 40
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: PRINT_NEWLINE - Imprimir nueva línea
; ═══════════════════════════════════════════════════════════════
%macro PRINT_NEWLINE 0
    sub rsp, 40
    lea rcx, [newline]
    call printf
    add rsp, 40
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: READ_INT - Leer entero desde teclado
; Uso: READ_INT variable
; ═══════════════════════════════════════════════════════════════
%macro READ_INT 1
    sub rsp, 40
    lea rcx, [int_scan_fmt]
    lea rdx, [%1]
    call scanf
    add rsp, 40
%endmacro

main:
    ; Ejemplo de uso
    PRINT_STR prompt
    READ_INT user_input
    
    PRINT_STR result_msg
    PRINT_INT qword [user_input]
    PRINT_NEWLINE
    
    xor eax, eax
    ret

section .data
    int_fmt db '%d', 0
    int_scan_fmt db '%d', 0
    newline db 10, 0
    prompt db 'Ingrese un numero: ', 0
    result_msg db 'Usted ingreso: ', 0

section .bss
    user_input resq 1

