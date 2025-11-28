; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Boot Sector (512 bytes)
; ═══════════════════════════════════════════════════════════════════════════

[BITS 16]
[ORG 0x7C00]

start:
    ; Configurar segmentos
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    ; Guardar drive de boot
    mov [boot_drive], dl
    
    ; Limpiar pantalla
    mov ax, 0x0003
    int 0x10
    
    ; Mensaje de inicio
    mov si, msg_boot
    call print
    
    ; === Cargar Stage 2 (sectores 2-5, 4 sectores = 2KB) ===
    mov si, msg_stage2
    call print
    
    ; Reset del disco
    xor ax, ax
    mov dl, [boot_drive]
    int 0x13
    
    ; Leer stage2
    mov ah, 0x02        ; Función: leer sectores
    mov al, 4           ; 4 sectores
    mov ch, 0           ; Cilindro 0
    mov cl, 2           ; Sector 2 (1-indexed)
    mov dh, 0           ; Cabeza 0
    mov dl, [boot_drive]
    xor bx, bx
    mov es, bx
    mov bx, 0x7E00      ; Destino: 0x7E00
    int 0x13
    jc disk_error
    
    mov si, msg_ok
    call print
    
    ; === Cargar Kernel (sectores 6-69, 64 sectores = 32KB) ===
    mov si, msg_kernel
    call print
    
    mov ax, 0x1000      ; Segmento destino
    mov es, ax
    xor bx, bx          ; Offset 0 -> ES:BX = 0x1000:0 = 0x10000
    
    mov ah, 0x02
    mov al, 64          ; 64 sectores (32KB, más que suficiente)
    mov ch, 0
    mov cl, 6           ; Sector 6
    mov dh, 0
    mov dl, [boot_drive]
    int 0x13
    jc disk_error
    
    mov si, msg_ok
    call print
    
    ; Saltar a Stage 2
    mov si, msg_jump
    call print
    
    jmp 0x0000:0x7E00

disk_error:
    mov si, msg_err
    call print
    mov ah, 0x00        ; Mostrar código de error
    add ah, '0'
    mov al, ah
    mov ah, 0x0E
    int 0x10
.halt:
    cli
    hlt
    jmp .halt

print:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    mov bx, 0x0007
    int 0x10
    jmp print
.done:
    ret

; Datos
boot_drive: db 0
msg_boot:   db 'FastOS v1.0', 13, 10, 0
msg_stage2: db 'Stage2... ', 0
msg_kernel: db 'Kernel... ', 0
msg_jump:   db 'Jump!', 13, 10, 0
msg_ok:     db 'OK', 13, 10, 0
msg_err:    db 'ERR:', 0

; Padding y firma
times 510 - ($ - $$) db 0
dw 0xAA55
