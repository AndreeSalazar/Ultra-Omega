default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    mov rax, 15
    mov rbx, 27
    add rax, rbx
    mov rcx, fmt
    mov rdx, rax
    call printf
    add rsp, 40
    ret

section .data
    fmt db 'La suma es: %d', 10, 0
