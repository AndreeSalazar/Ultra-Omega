; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Manipulación de Strings (NASM x64 Linux)
; Nivel: Intermedio - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Funciones para trabajar con cadenas de texto.

default rel
section .text
global main
extern printf
extern strlen

; ═══════════════════════════════════════════════════════════════
; Función: str_length - Obtener longitud de string
; Entrada: RDI = puntero al string (Linux)
; Salida: RAX = longitud
; ═══════════════════════════════════════════════════════════════
str_length:
    push rbx
    xor rax, rax
.loop:
    mov bl, [rdi + rax]
    test bl, bl
    jz .done
    inc rax
    jmp .loop
.done:
    pop rbx
    ret

; ═══════════════════════════════════════════════════════════════
; Función: str_copy - Copiar string
; Entrada: RDI = destino, RSI = origen (Linux)
; ═══════════════════════════════════════════════════════════════
str_copy:
    push rsi
    push rdi
    mov rdi, rdi              ; Ya está en rdi
    mov rsi, rsi              ; Ya está en rsi
.copy_loop:
    lodsb
    stosb
    test al, al
    jnz .copy_loop
    pop rdi
    pop rsi
    ret

main:
    ; Ejemplo: obtener longitud de un string (Linux: rdi)
    lea rdi, [test_string]
    call str_length
    mov rbx, rax              ; Guardar longitud
    
    ; Imprimir resultado (Linux: rdi, rsi)
    sub rsp, 8
    lea rdi, [fmt]
    mov rsi, rbx
    xor rax, rax
    call printf
    add rsp, 8
    
    xor rax, rax
    ret

section .data
    test_string db 'Hola Mundo', 0
    fmt db 'Longitud: %d caracteres', 10, 0

