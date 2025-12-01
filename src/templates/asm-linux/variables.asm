; ═══════════════════════════════════════
; Variables y Tipos de Datos en NASM (Linux)
; ═══════════════════════════════════════

default rel
section .data
    ; Bytes (1 byte)
    byte_var    db 42
    
    ; Words (2 bytes)
    word_var    dw 1000
    
    ; Double words (4 bytes)
    dword_var   dd 100000
    
    ; Quad words (8 bytes)
    qword_var   dq 10000000000
    
    ; Strings
    msg         db 'Hola Variables Linux!', 10, 0
    
    ; Arrays
    array       dd 1, 2, 3, 4, 5

section .bss
    ; Variables no inicializadas
    buffer      resb 256    ; 256 bytes
    temp_word   resw 1      ; 1 word
    temp_dword  resd 1      ; 1 dword

section .text
global main
extern printf

main:
    sub rsp, 8
    
    ; Acceder a variables
    mov al, [byte_var]
    movzx rax, word [word_var]
    mov eax, [dword_var]
    mov rax, [qword_var]
    
    ; Imprimir mensaje (Linux: rdi)
    mov rdi, msg
    xor rax, rax
    call printf
    
    add rsp, 8
    xor rax, rax
    ret

