; ==========================================
; Bucle Simple en NASM (Linux x64)
; ==========================================

default rel
section .text
global main
extern printf

main:
    sub rsp, 8
    mov rbx, 1

cycle:
    cmp rbx, 5
    jg end_cycle
    mov rdi, fmt        ; Linux: primer argumento en rdi
    mov rsi, rbx        ; Linux: segundo argumento en rsi
    xor rax, rax
    call printf
    inc rbx
    jmp cycle

end_cycle:
    add rsp, 8
    xor rax, rax
    ret

section .data
    fmt db 'Iteracion numero: %d', 10, 0

