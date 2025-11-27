default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    mov rax, 50
    cmp rax, 100
    jl is_less
    mov rcx, msg_ge
    jmp print

is_less:
    mov rcx, msg_less

print:
    call printf
    add rsp, 40
    ret

section .data
    msg_ge db 'El valor es mayor o igual a 100', 10, 0
    msg_less db 'El valor es menor a 100', 10, 0
