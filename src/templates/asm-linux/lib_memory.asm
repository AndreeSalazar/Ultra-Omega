; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Gestión de Memoria (NASM x64 Linux)
; Nivel: Avanzado - Componente independiente
; ═══════════════════════════════════════════════════════════════
; Macros y funciones para manipulación de memoria y arrays.

default rel
section .text
global main
extern printf
extern malloc
extern free

; ═══════════════════════════════════════════════════════════════
; Macro: ALLOC_ARRAY - Reservar memoria para array
; Uso: ALLOC_ARRAY dest, num_elementos, tamaño_elemento
; ═══════════════════════════════════════════════════════════════
%macro ALLOC_ARRAY 3
    push rdi
    mov rdi, %2               ; Linux: primer argumento
    imul rdi, %3
    sub rsp, 8
    call malloc
    add rsp, 8
    mov %1, rax
    pop rdi
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: FREE_MEM - Liberar memoria
; ═══════════════════════════════════════════════════════════════
%macro FREE_MEM 1
    mov rdi, %1               ; Linux: primer argumento
    sub rsp, 8
    call free
    add rsp, 8
%endmacro

; ═══════════════════════════════════════════════════════════════
; Función: mem_set - Llenar memoria con un valor
; RDI = destino, RSI = valor, RDX = cantidad de bytes (Linux)
; ═══════════════════════════════════════════════════════════════
mem_set:
    push rdi
    mov rdi, rdi              ; Ya está en rdi
    mov rax, rsi
    mov rcx, rdx
    rep stosb
    pop rdi
    ret

; ═══════════════════════════════════════════════════════════════
; Función: mem_copy - Copiar bloque de memoria
; RDI = destino, RSI = origen, RDX = cantidad (Linux)
; ═══════════════════════════════════════════════════════════════
mem_copy:
    push rsi
    push rdi
    mov rdi, rdi              ; Ya está en rdi
    mov rsi, rsi              ; Ya está en rsi
    mov rcx, rdx
    rep movsb
    pop rdi
    pop rsi
    ret

main:
    ; Ejemplo: reservar array de 10 enteros
    ALLOC_ARRAY rbx, 10, 4    ; rbx = puntero al array
    
    ; Llenar con ceros
    mov rdi, rbx              ; Linux: primer argumento
    xor rsi, rsi              ; Linux: segundo argumento (valor 0)
    mov rdx, 40               ; Linux: tercer argumento (10 * 4 bytes)
    call mem_set
    
    ; Imprimir confirmación (Linux: rdi)
    sub rsp, 8
    lea rdi, [msg]
    xor rax, rax
    call printf
    add rsp, 8
    
    ; Liberar memoria
    FREE_MEM rbx
    
    xor rax, rax
    ret

section .data
    msg db 'Memoria gestionada correctamente (Linux)', 10, 0

