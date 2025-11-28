; ═══════════════════════════════════════════════════════════════
; LIBRERÍA: Gestión de Memoria (NASM x64 Windows)
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
    push rcx
    mov rcx, %2
    imul rcx, %3
    sub rsp, 32
    call malloc
    add rsp, 32
    mov %1, rax
    pop rcx
%endmacro

; ═══════════════════════════════════════════════════════════════
; Macro: FREE_MEM - Liberar memoria
; ═══════════════════════════════════════════════════════════════
%macro FREE_MEM 1
    mov rcx, %1
    sub rsp, 32
    call free
    add rsp, 32
%endmacro

; ═══════════════════════════════════════════════════════════════
; Función: mem_set - Llenar memoria con un valor
; RCX = destino, RDX = valor, R8 = cantidad de bytes
; ═══════════════════════════════════════════════════════════════
mem_set:
    push rdi
    mov rdi, rcx
    mov rax, rdx
    mov rcx, r8
    rep stosb
    pop rdi
    ret

; ═══════════════════════════════════════════════════════════════
; Función: mem_copy - Copiar bloque de memoria
; RCX = destino, RDX = origen, R8 = cantidad
; ═══════════════════════════════════════════════════════════════
mem_copy:
    push rsi
    push rdi
    mov rdi, rcx
    mov rsi, rdx
    mov rcx, r8
    rep movsb
    pop rdi
    pop rsi
    ret

main:
    ; Ejemplo: reservar array de 10 enteros
    ALLOC_ARRAY rbx, 10, 4    ; rbx = puntero al array
    
    ; Llenar con ceros
    mov rcx, rbx
    xor rdx, rdx
    mov r8, 40                ; 10 * 4 bytes
    call mem_set
    
    ; Imprimir confirmación
    sub rsp, 40
    lea rcx, [msg]
    call printf
    add rsp, 40
    
    ; Liberar memoria
    FREE_MEM rbx
    
    xor eax, eax
    ret

section .data
    msg db 'Memoria gestionada correctamente', 10, 0

