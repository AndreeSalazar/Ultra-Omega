; ═══════════════════════════════════════
; Manipulación de Strings en NASM
; ═══════════════════════════════════════

default rel
section .data
    msg1        db 'Hola ', 0
    msg2        db 'Mundo!', 10, 0
    fmt_len     db 'Longitud: %d', 10, 0
    buffer      db 256 dup(0)

section .text
global main
extern printf
extern strlen
extern strcpy
extern strcat

; ─────────────────────────────────────
; Función: mi_strlen
; Cuenta caracteres hasta null
; ─────────────────────────────────────
mi_strlen:
    xor rax, rax
.loop:
    cmp byte [rcx + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret

main:
    sub rsp, 40
    
    ; Copiar msg1 a buffer
    lea rcx, [buffer]
    lea rdx, [msg1]
    call strcpy
    
    ; Concatenar msg2
    lea rcx, [buffer]
    lea rdx, [msg2]
    call strcat
    
    ; Imprimir resultado
    lea rcx, [buffer]
    xor eax, eax
    call printf
    
    ; Calcular longitud con nuestra función
    lea rcx, [buffer]
    call mi_strlen
    
    ; Imprimir longitud
    mov rcx, fmt_len
    mov rdx, rax
    xor eax, eax
    call printf
    
    add rsp, 40
    xor eax, eax
    ret

