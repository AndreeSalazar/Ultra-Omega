; ═══════════════════════════════════════════════════════════════════════════════
; FastOS 64-bit: Funciones de Memoria en ASM
; Operaciones de memoria optimizadas (llamables desde Rust)
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; MEMCPY_OPTIMIZED(dest, src, count) - Copia optimizada
; ═══════════════════════════════════════════════════════════════════════════════
global memcpy_optimized
memcpy_optimized:
    mov rcx, rdx    ; count
    mov rdi, rdi    ; dest
    mov rsi, rsi    ; src
    
    ; Copiar en bloques de 8 bytes si es posible
    cmp rcx, 8
    jl .byte_copy
    
    ; Alinear a 8 bytes
    test rdi, 7
    jz .aligned
    
    ; Copiar bytes hasta alineación
.align_loop:
    test rdi, 7
    jz .aligned
    movsb
    loop .align_loop
    
.aligned:
    ; Copiar en bloques de 8 bytes
    mov r8, rcx
    shr r8, 3       ; r8 = count / 8
    jz .byte_copy
    
.qword_copy:
    movsq
    dec r8
    jnz .qword_copy
    
    ; Copiar bytes restantes
    and rcx, 7
    jz .done
    
.byte_copy:
    rep movsb
    
.done:
    mov rax, rdi
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; MEMSET_OPTIMIZED(dest, value, count) - Llenado optimizado
; ═══════════════════════════════════════════════════════════════════════════════
global memset_optimized
memset_optimized:
    mov rcx, rdx    ; count
    mov al, sil     ; value
    mov rdi, rdi    ; dest
    
    ; Expandir byte a qword
    mov ah, al
    movzx rax, ax
    shl rax, 16
    mov ax, ax
    mov r8, rax
    shl r8, 32
    or rax, r8      ; rax = value repetido 8 veces
    
    ; Llenar en bloques de 8 bytes
    cmp rcx, 8
    jl .byte_fill
    
    ; Alinear
    test rdi, 7
    jz .aligned_fill
    
.align_fill_loop:
    test rdi, 7
    jz .aligned_fill
    stosb
    loop .align_fill_loop
    
.aligned_fill:
    mov r8, rcx
    shr r8, 3
    jz .byte_fill
    
.qword_fill:
    stosq
    dec r8
    jnz .qword_fill
    
    and rcx, 7
    jz .done_fill
    
.byte_fill:
    rep stosb
    
.done_fill:
    mov rax, rdi
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; MEMCMP_OPTIMIZED(ptr1, ptr2, count) -> diff
; ═══════════════════════════════════════════════════════════════════════════════
global memcmp_optimized
memcmp_optimized:
    mov rcx, rdx
    mov rdi, rdi
    mov rsi, rsi
    
    ; Comparar en bloques de 8 bytes
    cmp rcx, 8
    jl .byte_compare
    
    ; Alinear
    test rdi, 7
    jz .aligned_compare
    
.align_compare_loop:
    test rdi, 7
    jz .aligned_compare
    cmpsb
    jne .diff
    loop .align_compare_loop
    
.aligned_compare:
    mov r8, rcx
    shr r8, 3
    jz .byte_compare
    
.qword_compare:
    cmpsq
    jne .diff
    dec r8
    jnz .qword_compare
    
    and rcx, 7
    jz .equal
    
.byte_compare:
    repe cmpsb
    jz .equal
    
.diff:
    movzx rax, byte [rdi-1]
    movzx rdx, byte [rsi-1]
    sub rax, rdx
    ret
    
.equal:
    xor rax, rax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; PAGE_FAULT_HANDLER - Handler de page fault (llamado desde Rust)
; ═══════════════════════════════════════════════════════════════════════════════
global page_fault_handler_asm
page_fault_handler_asm:
    ; rdi = error_code, rsi = cr2 (dirección que causó el fault)
    ; Esta función es llamada desde el ISR 14
    extern page_fault_handler_rust
    call page_fault_handler_rust
    ret

