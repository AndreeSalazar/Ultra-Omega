default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    mov rbx, 1

cycle:
    cmp rbx, 5
    jg end_cycle
    mov rcx, fmt
    mov rdx, rbx
    call printf
    inc rbx
    jmp cycle

end_cycle:
    add rsp, 40
    ret

section .data
    fmt db 'Iteracion numero: %d', 10, 0
