; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Manipulación de Strings (NASM x64 Windows)
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
; Entrada: RCX = puntero al string
; Salida: RAX = longitud
; ═══════════════════════════════════════════════════════════════
str_length:
    push rbx
    xor rax, rax
.loop:
    mov bl, [rcx + rax]
    test bl, bl
    jz .done
    inc rax
    jmp .loop
.done:
    pop rbx
    ret

; ═══════════════════════════════════════════════════════════════
; Función: str_copy - Copiar string
; Entrada: RCX = destino, RDX = origen
; ═══════════════════════════════════════════════════════════════
str_copy:
    push rsi
    push rdi
    mov rdi, rcx
    mov rsi, rdx
.copy_loop:
    lodsb
    stosb
    test al, al
    jnz .copy_loop
    pop rdi
    pop rsi
    ret

main:
    ; Ejemplo: obtener longitud de un string
    lea rcx, [test_string]
    call str_length
    mov rbx, rax              ; Guardar longitud
    
    ; Imprimir resultado
    sub rsp, 40
    lea rcx, [fmt]
    mov rdx, rbx
    call printf
    add rsp, 40
    
    xor eax, eax
    ret

section .data
    test_string db 'Hola Mundo', 0
    fmt db 'Longitud: %d caracteres', 10, 0

