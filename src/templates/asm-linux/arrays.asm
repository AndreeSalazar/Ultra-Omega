; ═══════════════════════════════════════
; Arrays y Memoria en NASM (Linux x64)
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
; RDI = puntero, RSI = longitud (Linux)
; ─────────────────────────────────────
suma_array:
    xor rax, rax
    xor r8, r8
.loop:
    cmp r8, rsi
    jge .done
    add eax, [rdi + r8*4]
    inc r8
    jmp .loop
.done:
    ret

; ─────────────────────────────────────
; Encontrar máximo
; RDI = puntero, RSI = longitud (Linux)
; ─────────────────────────────────────
max_array:
    mov eax, [rdi]
    mov r8, 1
.loop:
    cmp r8, rsi
    jge .done
    mov r9d, [rdi + r8*4]
    cmp r9d, eax
    jle .skip
    mov eax, r9d
.skip:
    inc r8
    jmp .loop
.done:
    ret

main:
    sub rsp, 8
    
    ; Imprimir cada elemento
    xor r12, r12
.print_loop:
    cmp r12, array_len
    jge .print_done
    
    mov rdi, fmt_elem       ; Linux: primer argumento
    mov rsi, r12            ; Linux: segundo argumento
    lea rax, [array]
    mov rdx, [rax + r12*4]  ; Linux: tercer argumento
    xor rax, rax
    call printf
    
    inc r12
    jmp .print_loop
    
.print_done:
    ; Calcular suma
    lea rdi, [array]        ; Linux: primer argumento
    mov rsi, array_len      ; Linux: segundo argumento
    call suma_array
    
    mov rdi, fmt_sum
    mov rsi, rax
    xor rax, rax
    call printf
    
    ; Encontrar máximo
    lea rdi, [array]
    mov rsi, array_len
    call max_array
    
    mov rdi, fmt_max
    mov rsi, rax
    xor rax, rax
    call printf
    
    add rsp, 8
    xor rax, rax
    ret

