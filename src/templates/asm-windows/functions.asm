; ═══════════════════════════════════════
; Funciones en NASM (Windows x64)
; ═══════════════════════════════════════

default rel
section .data
    fmt_result  db 'Resultado: %d', 10, 0
    fmt_sum     db 'Suma: %d', 10, 0

section .text
global main
extern printf

; ─────────────────────────────────────
; Función: sumar
; Parámetros: RCX = a, RDX = b
; Retorno: RAX = a + b
; ─────────────────────────────────────
sumar:
    mov rax, rcx
    add rax, rdx
    ret

; ─────────────────────────────────────
; Función: multiplicar
; Parámetros: RCX = a, RDX = b
; Retorno: RAX = a * b
; ─────────────────────────────────────
multiplicar:
    mov rax, rcx
    imul rax, rdx
    ret

; ─────────────────────────────────────
; Función: factorial (recursiva)
; Parámetros: RCX = n
; Retorno: RAX = n!
; ─────────────────────────────────────
factorial:
    push rbx
    sub rsp, 32
    
    cmp rcx, 1
    jle .base_case
    
    mov rbx, rcx
    dec rcx
    call factorial
    imul rax, rbx
    jmp .done
    
.base_case:
    mov rax, 1
    
.done:
    add rsp, 32
    pop rbx
    ret

; ─────────────────────────────────────
; Main
; ─────────────────────────────────────
main:
    sub rsp, 40
    
    ; Llamar sumar(5, 3)
    mov rcx, 5
    mov rdx, 3
    call sumar
    
    ; Imprimir resultado
    mov rcx, fmt_sum
    mov rdx, rax
    xor eax, eax
    call printf
    
    ; Llamar factorial(5)
    mov rcx, 5
    call factorial
    
    mov rcx, fmt_result
    mov rdx, rax
    xor eax, eax
    call printf
    
    add rsp, 40
    xor eax, eax
    ret

