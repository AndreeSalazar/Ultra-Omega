; ═══════════════════════════════════════
; Arrays y Memoria en NASM
; ═══════════════════════════════════════

default rel
section .data
    array       dd 10, 20, 30, 40, 50
    array_len   equ 5
    fmt_elem    db 'Elemento[%d] = %d', 10, 0
    fmt_sum     db 'Suma total: %d', 10, 0
    fmt_max     db 'Maximo: %d', 10, 0

section .text
global main
extern printf

; ─────────────────────────────────────
; Suma de array
; RCX = puntero, RDX = longitud
; ─────────────────────────────────────
suma_array:
    xor rax, rax
    xor r8, r8
.loop:
    cmp r8, rdx
    jge .done
    add eax, [rcx + r8*4]
    inc r8
    jmp .loop
.done:
    ret

; ─────────────────────────────────────
; Encontrar máximo
; RCX = puntero, RDX = longitud
; ─────────────────────────────────────
max_array:
    mov eax, [rcx]
    mov r8, 1
.loop:
    cmp r8, rdx
    jge .done
    mov r9d, [rcx + r8*4]
    cmp r9d, eax
    jle .skip
    mov eax, r9d
.skip:
    inc r8
    jmp .loop
.done:
    ret

main:
    sub rsp, 40
    
    ; Imprimir cada elemento
    xor r12, r12
.print_loop:
    cmp r12, array_len
    jge .print_done
    
    mov rcx, fmt_elem
    mov rdx, r12
    lea rax, [array]
    mov r8d, [rax + r12*4]
    xor eax, eax
    call printf
    
    inc r12
    jmp .print_loop
    
.print_done:
    ; Calcular suma
    lea rcx, [array]
    mov rdx, array_len
    call suma_array
    
    mov rcx, fmt_sum
    mov rdx, rax
    xor eax, eax
    call printf
    
    ; Encontrar máximo
    lea rcx, [array]
    mov rdx, array_len
    call max_array
    
    mov rcx, fmt_max
    mov rdx, rax
    xor eax, eax
    call printf
    
    add rsp, 40
    xor eax, eax
    ret

