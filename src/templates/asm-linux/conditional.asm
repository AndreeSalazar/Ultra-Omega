; ==========================================
; Condicional If/Else en NASM (Linux x64)
; ==========================================

default rel
section .text
global main
extern printf

main:
    sub rsp, 8
    mov rax, 50
    cmp rax, 100
    jl is_less
    mov rdi, msg_ge     ; Linux: primer argumento en rdi
    jmp print

is_less:
    mov rdi, msg_less

print:
    xor rax, rax
    call printf
    add rsp, 8
    xor rax, rax
    ret

section .data
    msg_ge db 'El valor es mayor o igual a 100', 10, 0
    msg_less db 'El valor es menor a 100', 10, 0

