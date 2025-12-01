; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Kernel Entry Point
; Nivel: Avanzado - Punto de entrada del kernel
; ═══════════════════════════════════════════════════════════════════════════
; Este archivo es el punto de entrada del kernel. Prepara el entorno y
; llama a la función main() del kernel escrita en C.
; ═══════════════════════════════════════════════════════════════════════════

[BITS 32]
[GLOBAL _start]
[EXTERN kernel_main]

section .text

; ═══════════════════════════════════════════════════════════════════════════
; PUNTO DE ENTRADA
; ═══════════════════════════════════════════════════════════════════════════

_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Limpiar registros
    xor eax, eax
    xor ebx, ebx
    xor ecx, ecx
    xor edx, edx
    
    ; Llamar al kernel en C
    call kernel_main
    
    ; Si el kernel retorna, entrar en halt loop
.halt:
    cli
    hlt
    jmp .halt

; ═══════════════════════════════════════════════════════════════════════════
; FUNCIONES DE BAJO NIVEL PARA EL KERNEL
; ═══════════════════════════════════════════════════════════════════════════

[GLOBAL outb]
[GLOBAL inb]
[GLOBAL outw]
[GLOBAL inw]
[GLOBAL io_wait]

; Escribir byte a puerto I/O
; void outb(uint16_t port, uint8_t value)
outb:
    mov dx, [esp + 4]        ; port
    mov al, [esp + 8]        ; value
    out dx, al
    ret

; Leer byte de puerto I/O
; uint8_t inb(uint16_t port)
inb:
    mov dx, [esp + 4]        ; port
    xor eax, eax
    in al, dx
    ret

; Escribir word a puerto I/O
; void outw(uint16_t port, uint16_t value)
outw:
    mov dx, [esp + 4]
    mov ax, [esp + 8]
    out dx, ax
    ret

; Leer word de puerto I/O
; uint16_t inw(uint16_t port)
inw:
    mov dx, [esp + 4]
    xor eax, eax
    in ax, dx
    ret

; Espera I/O (para timing)
io_wait:
    out 0x80, al
    ret

; ═══════════════════════════════════════════════════════════════════════════
; MANEJO DE INTERRUPCIONES
; ═══════════════════════════════════════════════════════════════════════════

[GLOBAL load_idt]
[GLOBAL enable_interrupts]
[GLOBAL disable_interrupts]

; Cargar IDT
; void load_idt(void* idt_ptr)
load_idt:
    mov eax, [esp + 4]
    lidt [eax]
    ret

; Habilitar interrupciones
enable_interrupts:
    sti
    ret

; Deshabilitar interrupciones
disable_interrupts:
    cli
    ret

; ═══════════════════════════════════════════════════════════════════════════
; HANDLERS DE INTERRUPCIÓN
; ═══════════════════════════════════════════════════════════════════════════

[EXTERN isr_handler]
[EXTERN irq_handler]

; Macro para ISR sin código de error
%macro ISR_NOERRCODE 1
[GLOBAL isr%1]
isr%1:
    push dword 0             ; Código de error dummy
    push dword %1            ; Número de interrupción
    jmp isr_common
%endmacro

; Macro para ISR con código de error
%macro ISR_ERRCODE 1
[GLOBAL isr%1]
isr%1:
    push dword %1            ; Número de interrupción
    jmp isr_common
%endmacro

; Macro para IRQ
%macro IRQ 2
[GLOBAL irq%1]
irq%1:
    push dword 0
    push dword %2
    jmp irq_common
%endmacro

; Definir ISRs (0-31: excepciones del CPU)
ISR_NOERRCODE 0              ; Division by zero
ISR_NOERRCODE 1              ; Debug
ISR_NOERRCODE 2              ; NMI
ISR_NOERRCODE 3              ; Breakpoint
ISR_NOERRCODE 4              ; Overflow
ISR_NOERRCODE 5              ; Bound Range Exceeded
ISR_NOERRCODE 6              ; Invalid Opcode
ISR_NOERRCODE 7              ; Device Not Available
ISR_ERRCODE   8              ; Double Fault
ISR_NOERRCODE 9              ; Coprocessor Segment Overrun
ISR_ERRCODE   10             ; Invalid TSS
ISR_ERRCODE   11             ; Segment Not Present
ISR_ERRCODE   12             ; Stack-Segment Fault
ISR_ERRCODE   13             ; General Protection Fault
ISR_ERRCODE   14             ; Page Fault
ISR_NOERRCODE 15             ; Reserved
ISR_NOERRCODE 16             ; x87 FPU Error
ISR_ERRCODE   17             ; Alignment Check
ISR_NOERRCODE 18             ; Machine Check
ISR_NOERRCODE 19             ; SIMD FPU Exception

; Definir IRQs (32-47)
IRQ 0, 32                    ; Timer
IRQ 1, 33                    ; Keyboard
IRQ 2, 34                    ; Cascade
IRQ 3, 35                    ; COM2
IRQ 4, 36                    ; COM1
IRQ 5, 37                    ; LPT2
IRQ 6, 38                    ; Floppy
IRQ 7, 39                    ; LPT1
IRQ 8, 40                    ; RTC
IRQ 9, 41                    ; Free
IRQ 10, 42                   ; Free
IRQ 11, 43                   ; Free
IRQ 12, 44                   ; Mouse
IRQ 13, 45                   ; Coprocessor
IRQ 14, 46                   ; Primary ATA
IRQ 15, 47                   ; Secondary ATA

; Código común para ISRs
isr_common:
    pusha                    ; Guardar registros
    push ds
    push es
    push fs
    push gs
    
    mov ax, 0x10             ; Segmento de datos del kernel
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    push esp                 ; Pasar puntero a registros
    call isr_handler
    add esp, 4
    
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8               ; Limpiar código de error y número
    iret

; Código común para IRQs
irq_common:
    pusha
    push ds
    push es
    push fs
    push gs
    
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    push esp
    call irq_handler
    add esp, 4
    
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8
    iret

; ═══════════════════════════════════════════════════════════════════════════
; SECCIÓN BSS (datos no inicializados)
; ═══════════════════════════════════════════════════════════════════════════

section .bss
align 16

stack_bottom:
    resb 16384               ; 16 KB de stack
stack_top:

