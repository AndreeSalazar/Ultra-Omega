; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Stage 2 Bootloader
; ═══════════════════════════════════════════════════════════════════════════
; Configura modo protegido y salta al kernel
; ═══════════════════════════════════════════════════════════════════════════

[BITS 16]
[ORG 0x7E00]

KERNEL_ADDR equ 0x10000

stage2_start:
    mov si, msg_stage2
    call print16
    
    ; Habilitar A20
    call enable_a20
    
    mov si, msg_pmode
    call print16
    
    ; Cargar GDT y entrar en modo protegido
    cli
    lgdt [gdt_descriptor]
    
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    jmp CODE_SEG:pm_start

; === Habilitar A20 ===
enable_a20:
    ; Método BIOS
    mov ax, 0x2401
    int 0x15
    jnc .done
    
    ; Método Fast A20
    in al, 0x92
    or al, 2
    out 0x92, al
    
.done:
    mov si, msg_a20
    call print16
    ret

; === Print 16-bit ===
print16:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print16
.done:
    ret

; === GDT ===
gdt_start:
    dq 0                        ; Null descriptor

gdt_code:
    dw 0xFFFF                   ; Limit
    dw 0x0000                   ; Base low
    db 0x00                     ; Base mid
    db 10011010b                ; Access: Present, Ring0, Code, Exec, Read
    db 11001111b                ; Flags: 4K granularity, 32-bit
    db 0x00                     ; Base high

gdt_data:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b                ; Access: Present, Ring0, Data, Write
    db 11001111b
    db 0x00

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

; === Mensajes 16-bit ===
msg_stage2: db 'Stage2 loaded', 13, 10, 0
msg_a20:    db 'A20 enabled', 13, 10, 0
msg_pmode:  db 'Entering Protected Mode...', 13, 10, 0

; ═══════════════════════════════════════════════════════════════════════════
; MODO PROTEGIDO 32-bit
; ═══════════════════════════════════════════════════════════════════════════

[BITS 32]

pm_start:
    ; Configurar segmentos
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000
    
    ; Limpiar pantalla
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0F20          ; Espacio blanco sobre negro
    rep stosw
    
    ; Mostrar mensaje
    mov esi, msg_pm
    mov edi, 0xB8000
    call print32
    
    ; Mostrar "Jumping to kernel..."
    mov esi, msg_jump
    mov edi, 0xB8000 + 160  ; Segunda línea
    call print32
    
    ; Saltar al kernel
    jmp KERNEL_ADDR

; === Print 32-bit ===
print32:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0F            ; Blanco sobre negro
    stosw
    jmp print32
.done:
    ret

; === Mensajes 32-bit ===
msg_pm:   db 'Protected Mode OK!', 0
msg_jump: db 'Jumping to kernel...', 0

; Padding a 2KB (4 sectores)
times 2048 - ($ - $$) db 0
