; ═══════════════════════════════════════
; Funciones en NASM (Linux x64)
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
; Parámetros: RDI = a, RSI = b (Linux)
; Retorno: RAX = a + b
; ─────────────────────────────────────
sumar:
    mov rax, rdi
    add rax, rsi
    ret

; ─────────────────────────────────────
; Función: multiplicar
; Parámetros: RDI = a, RSI = b (Linux)
; Retorno: RAX = a * b
; ─────────────────────────────────────
multiplicar:
    mov rax, rdi
    imul rax, rsi
    ret

; ─────────────────────────────────────
; Función: factorial (recursiva)
; Parámetros: RDI = n (Linux)
; Retorno: RAX = n!
; ─────────────────────────────────────
factorial:
    push rbx
    sub rsp, 8
    
    cmp rdi, 1
    jle .base_case
    
    mov rbx, rdi
    dec rdi
    call factorial
    imul rax, rbx
    jmp .done
    
.base_case:
    mov rax, 1
    
.done:
    add rsp, 8
    pop rbx
    ret

; ─────────────────────────────────────
; Main
; ─────────────────────────────────────
main:
    sub rsp, 8
    
    ; Llamar sumar(5, 3) - Linux: rdi, rsi
    mov rdi, 5
    mov rsi, 3
    call sumar
    
    ; Imprimir resultado (Linux: rdi, rsi)
    mov rdi, fmt_sum
    mov rsi, rax
    xor rax, rax
    call printf
    
    ; Llamar factorial(5) - Linux: rdi
    mov rdi, 5
    call factorial
    
    mov rdi, fmt_result
    mov rsi, rax
    xor rax, rax
    call printf
    
    add rsp, 8
    xor rax, rax
    ret

