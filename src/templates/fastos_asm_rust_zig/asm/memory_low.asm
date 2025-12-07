; ═══════════════════════════════════════════════════════════════════════════════
; FastOS: Funciones de Memoria de Bajo Nivel
; NASM x86_64 - Operaciones de memoria optimizadas
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; MEMCPY - Copiar memoria (optimizado)
; rdi = destino, rsi = origen, rdx = tamaño
; ═══════════════════════════════════════════════════════════════════════════════
global memcpy_low
memcpy_low:
    mov rcx, rdx        ; Contador
    mov rax, rdi        ; Retornar destino
    
    ; Alineación para performance
    test rcx, rcx
    jz .done
    
    ; Copiar 8 bytes a la vez si es posible
.loop:
    mov r8, [rsi]
    mov [rdi], r8
    add rsi, 8
    add rdi, 8
    sub rcx, 8
    jnz .loop
    
.done:
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; MEMSET - Llenar memoria con valor
; rdi = destino, rsi = valor, rdx = tamaño
; ═══════════════════════════════════════════════════════════════════════════════
global memset_low
memset_low:
    mov rcx, rdx        ; Contador
    mov rax, rdi        ; Retornar destino
    mov r8b, sil        ; Valor
    
    test rcx, rcx
    jz .done
    
.loop:
    mov [rdi], r8b
    inc rdi
    dec rcx
    jnz .loop
    
.done:
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; MEMCMP - Comparar memoria
; rdi = ptr1, rsi = ptr2, rdx = tamaño
; Retorna: 0 si igual, <0 si ptr1 < ptr2, >0 si ptr1 > ptr2
; ═══════════════════════════════════════════════════════════════════════════════
global memcmp_low
memcmp_low:
    mov rcx, rdx
    test rcx, rcx
    jz .equal
    
.loop:
    mov al, [rdi]
    mov bl, [rsi]
    cmp al, bl
    jl .less
    jg .greater
    
    inc rdi
    inc rsi
    dec rcx
    jnz .loop
    
.equal:
    xor eax, eax
    ret
    
.less:
    mov eax, -1
    ret
    
.greater:
    mov eax, 1
    ret

