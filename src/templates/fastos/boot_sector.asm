; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Boot Sector (512 bytes)
; ═══════════════════════════════════════════════════════════════════════════
; Primer sector del disco que la BIOS carga en 0x7C00
; Carga Stage 2 y salta a él
; ═══════════════════════════════════════════════════════════════════════════

[BITS 16]
[ORG 0x7C00]

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    mov [boot_drive], dl
    
    ; Limpiar pantalla
    mov ah, 0x00
    mov al, 0x03
    int 0x10
    
    ; Mensaje de bienvenida
    mov si, msg_boot
    call print
    
    ; Cargar Stage 2
    mov si, msg_load
    call print
    
    mov ah, 0x02
    mov al, 4            ; 4 sectores
    mov ch, 0
    mov cl, 2            ; Sector 2
    mov dh, 0
    mov dl, [boot_drive]
    mov bx, 0x7E00       ; Destino
    int 0x13
    jc error
    
    mov si, msg_ok
    call print
    
    jmp 0x0000:0x7E00    ; Saltar a Stage 2

error:
    mov si, msg_err
    call print
    jmp $

print:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    int 0x10
    jmp print
.done:
    ret

; Datos
boot_drive: db 0
msg_boot:   db 'FastOS Boot v1.0', 13, 10, 0
msg_load:   db 'Loading... ', 0
msg_ok:     db 'OK', 13, 10, 0
msg_err:    db 'DISK ERROR', 13, 10, 0

; Padding y firma
times 510 - ($ - $$) db 0
dw 0xAA55
