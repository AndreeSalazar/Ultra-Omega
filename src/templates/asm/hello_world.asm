default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    mov rcx, msg
    call printf
    add rsp, 40
    ret

section .data
    msg db 'Hola NASM!', 10, 0
