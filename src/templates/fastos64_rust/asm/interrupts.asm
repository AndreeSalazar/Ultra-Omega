; ═══════════════════════════════════════════════════════════════════════════════
; FastOS 64-bit: Manejo de Interrupciones
; Funciones ASM para interrupciones (llamables desde Rust)
; ═══════════════════════════════════════════════════════════════════════════════

bits 64
default rel

section .text

; ═══════════════════════════════════════════════════════════════════════════════
; SEND_EOI(irq) - Enviar End of Interrupt al PIC
; ═══════════════════════════════════════════════════════════════════════════════
global send_eoi
send_eoi:
    mov al, 0x20    ; EOI command
    cmp dil, 8      ; irq >= 8?
    jl .master_only
    
    ; EOI al slave PIC
    out 0xA0, al
.master_only:
    ; EOI al master PIC
    out 0x20, al
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; CLI - Deshabilitar interrupciones
; ═══════════════════════════════════════════════════════════════════════════════
global cli_asm
cli_asm:
    cli
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; STI - Habilitar interrupciones
; ═══════════════════════════════════════════════════════════════════════════════
global sti_asm
sti_asm:
    sti
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; GET_FLAGS() -> flags - Obtener flags del CPU
; ═══════════════════════════════════════════════════════════════════════════════
global get_flags
get_flags:
    pushfq
    pop rax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; CPUID - Obtener información del CPU
; ═══════════════════════════════════════════════════════════════════════════════
global cpuid_asm
cpuid_asm:
    ; rdi = eax input, rsi = puntero a resultado (4 dwords)
    mov rax, rdi
    cpuid
    mov [rsi], eax
    mov [rsi+4], ebx
    mov [rsi+8], ecx
    mov [rsi+12], edx
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; RDMSR(msr) -> edx:eax - Leer Model-Specific Register
; ═══════════════════════════════════════════════════════════════════════════════
global rdmsr_asm
rdmsr_asm:
    mov ecx, edi    ; MSR number
    rdmsr
    shl rdx, 32
    or rax, rdx     ; Combinar edx:eax en rax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; WRMSR(msr, value) - Escribir Model-Specific Register
; ═══════════════════════════════════════════════════════════════════════════════
global wrmsr_asm
wrmsr_asm:
    mov ecx, edi    ; MSR number
    mov eax, esi    ; Low 32 bits
    mov edx, edx    ; High 32 bits (de rsi >> 32)
    shr rsi, 32
    mov edx, esi
    wrmsr
    ret

