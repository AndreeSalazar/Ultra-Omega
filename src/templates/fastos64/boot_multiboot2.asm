; ═══════════════════════════════════════════════════════════════════════════════
; FASTOS 64-BIT + VULKAN - MULTIBOOT2 BOOTLOADER
; ═══════════════════════════════════════════════════════════════════════════════
; Archivo: boot_multiboot2.asm
; Descripción: Bootloader Multiboot2 que configura modo largo (64-bit)
; Compatible con QEMU -kernel
; ═══════════════════════════════════════════════════════════════════════════════

[BITS 32]

; ═══════════════════════════════════════════════════════════════════════════════
; MULTIBOOT2 HEADER
; ═══════════════════════════════════════════════════════════════════════════════

section .multiboot
align 8

MULTIBOOT2_MAGIC        equ 0xE85250D6
MULTIBOOT2_ARCH_I386    equ 0
MULTIBOOT2_HEADER_LEN   equ multiboot_header_end - multiboot_header
MULTIBOOT2_CHECKSUM     equ -(MULTIBOOT2_MAGIC + MULTIBOOT2_ARCH_I386 + MULTIBOOT2_HEADER_LEN)

multiboot_header:
    dd MULTIBOOT2_MAGIC
    dd MULTIBOOT2_ARCH_I386
    dd MULTIBOOT2_HEADER_LEN
    dd MULTIBOOT2_CHECKSUM
    
    ; Tag de framebuffer
    align 8
    dw 5                    ; Tipo: framebuffer
    dw 0                    ; Flags
    dd 20                   ; Tamaño
    dd 1280                 ; Ancho preferido
    dd 720                  ; Alto preferido
    dd 32                   ; BPP preferido
    
    ; Tag final
    align 8
    dw 0                    ; Tipo: fin
    dw 0                    ; Flags
    dd 8                    ; Tamaño
multiboot_header_end:

; ═══════════════════════════════════════════════════════════════════════════════
; CÓDIGO DE INICIO (32-bit)
; ═══════════════════════════════════════════════════════════════════════════════

section .text
global _start
extern kernel_main

_start:
    ; Deshabilitar interrupciones
    cli
    
    ; Guardar puntero a Multiboot info
    mov edi, ebx
    
    ; Configurar stack temporal
    mov esp, stack_top
    
    ; Verificar que tenemos Multiboot2
    cmp eax, 0x36D76289
    jne .no_multiboot
    
    ; Verificar soporte de CPUID
    call check_cpuid
    test eax, eax
    jz .no_cpuid
    
    ; Verificar soporte de modo largo
    call check_long_mode
    test eax, eax
    jz .no_long_mode
    
    ; Configurar paginación para modo largo
    call setup_page_tables
    call enable_paging
    
    ; Cargar GDT de 64 bits
    lgdt [gdt64_pointer]
    
    ; Saltar a modo largo
    jmp gdt64_code:long_mode_start

.no_multiboot:
    mov al, 'M'
    jmp .error

.no_cpuid:
    mov al, 'C'
    jmp .error

.no_long_mode:
    mov al, 'L'
    jmp .error

.error:
    ; Mostrar error en VGA
    mov dword [0xB8000], 0x4F524F45  ; "ER"
    mov dword [0xB8004], 0x4F3A4F52  ; "R:"
    mov byte  [0xB8008], al
    mov byte  [0xB8009], 0x4F
    hlt

; ═══════════════════════════════════════════════════════════════════════════════
; VERIFICAR CPUID
; ═══════════════════════════════════════════════════════════════════════════════

check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    mov eax, 1
    ret
.no_cpuid:
    xor eax, eax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; VERIFICAR MODO LARGO
; ═══════════════════════════════════════════════════════════════════════════════

check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode
    
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    
    mov eax, 1
    ret
.no_long_mode:
    xor eax, eax
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; CONFIGURAR TABLAS DE PÁGINA
; ═══════════════════════════════════════════════════════════════════════════════

setup_page_tables:
    ; Limpiar tablas
    mov edi, pml4_table
    mov ecx, 4096 * 4
    xor eax, eax
    rep stosb
    
    ; PML4[0] -> PDPT
    mov eax, pdpt_table
    or eax, 0x03            ; Present + Writable
    mov [pml4_table], eax
    
    ; PDPT[0] -> PD
    mov eax, pd_table
    or eax, 0x03
    mov [pdpt_table], eax
    
    ; PD[0] -> PT
    mov eax, pt_table
    or eax, 0x03
    mov [pd_table], eax
    
    ; Mapear primeros 2MB (512 páginas de 4KB)
    mov edi, pt_table
    mov eax, 0x03           ; Present + Writable
    mov ecx, 512
.map_pt:
    mov [edi], eax
    add eax, 0x1000
    add edi, 8
    loop .map_pt
    
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; HABILITAR PAGINACIÓN Y MODO LARGO
; ═══════════════════════════════════════════════════════════════════════════════

enable_paging:
    ; Cargar PML4 en CR3
    mov eax, pml4_table
    mov cr3, eax
    
    ; Habilitar PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    
    ; Habilitar modo largo en EFER MSR
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    
    ; Habilitar paginación
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; CÓDIGO DE 64 BITS
; ═══════════════════════════════════════════════════════════════════════════════

[BITS 64]

long_mode_start:
    ; Cargar segmentos de datos
    mov ax, gdt64_data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Configurar stack de 64 bits
    mov rsp, stack_top
    
    ; Limpiar registros superiores
    xor rax, rax
    xor rbx, rbx
    xor rcx, rcx
    xor rdx, rdx
    
    ; RDI ya contiene el puntero a Multiboot info (de 32-bit)
    ; Extender a 64 bits
    mov edi, edi
    
    ; Llamar al kernel
    call kernel_main
    
    ; Halt infinito si retorna
.halt:
    cli
    hlt
    jmp .halt

; ═══════════════════════════════════════════════════════════════════════════════
; GDT DE 64 BITS
; ═══════════════════════════════════════════════════════════════════════════════

section .rodata
align 16

gdt64:
    dq 0                        ; Null descriptor
gdt64_code_entry:
    dq 0x00AF9A000000FFFF       ; Code: 64-bit, present, executable
gdt64_data_entry:
    dq 0x00CF92000000FFFF       ; Data: present, writable
gdt64_end:

gdt64_pointer:
    dw gdt64_end - gdt64 - 1
    dq gdt64

gdt64_code equ gdt64_code_entry - gdt64
gdt64_data equ gdt64_data_entry - gdt64

; ═══════════════════════════════════════════════════════════════════════════════
; BSS - TABLAS DE PÁGINA Y STACK
; ═══════════════════════════════════════════════════════════════════════════════

section .bss
align 4096

pml4_table:
    resb 4096
pdpt_table:
    resb 4096
pd_table:
    resb 4096
pt_table:
    resb 4096

stack_bottom:
    resb 65536              ; 64KB stack
stack_top:

