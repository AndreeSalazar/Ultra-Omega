; ═══════════════════════════════════════
; Manipulación de Strings en NASM (Linux x64)
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
; RDI = puntero (Linux)
; ─────────────────────────────────────
mi_strlen:
    xor rax, rax
.loop:
    cmp byte [rdi + rax], 0
    je .done
    inc rax
    jmp .loop
.done:
    ret

main:
    sub rsp, 8
    
    ; Copiar msg1 a buffer (Linux: rdi, rsi)
    lea rdi, [buffer]
    lea rsi, [msg1]
    call strcpy
    
    ; Concatenar msg2 (Linux: rdi, rsi)
    lea rdi, [buffer]
    lea rsi, [msg2]
    call strcat
    
    ; Imprimir resultado (Linux: rdi)
    lea rdi, [buffer]
    xor rax, rax
    call printf
    
    ; Calcular longitud con nuestra función (Linux: rdi)
    lea rdi, [buffer]
    call mi_strlen
    
    ; Imprimir longitud (Linux: rdi, rsi)
    mov rdi, fmt_len
    mov rsi, rax
    xor rax, rax
    call printf
    
    add rsp, 8
    xor rax, rax
    ret

