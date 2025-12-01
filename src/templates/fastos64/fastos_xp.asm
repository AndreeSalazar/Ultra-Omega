; ═══════════════════════════════════════════════════════════════════════════════
; FASTOS XP EDITION - Desktop estilo Windows XP
; ═══════════════════════════════════════════════════════════════════════════════
; Autor: Eddi Andreé Salazar Matos - Desarrollador Peruano 🇵🇪
; Hardware: RTX 3060 12GB + Ryzen 5 5600X + 16GB RAM
; 
; Características:
; - Desktop con degradado azul estilo XP
; - Taskbar con degradado y botón Start verde
; - Ventanas con titlebar azul degradado
; - Botones de ventana (minimizar, maximizar, cerrar)
; - Iconos de escritorio
; - Bandera de Perú
; - Cursor del mouse
; - Información del sistema y Vulkan
; ═══════════════════════════════════════════════════════════════════════════════

[BITS 16]
[ORG 0x7C00]

; ═══════════════════════════════════════════════════════════════════════════════
; BOOT SECTOR - Carga Stage 2
; ═══════════════════════════════════════════════════════════════════════════════
boot_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    sti
    
    mov [boot_drive], dl
    
    ; Mostrar logo de arranque
    mov ax, 0x0003          ; Modo texto 80x25
    int 0x10
    
    ; Ocultar cursor
    mov ah, 0x01
    mov cx, 0x2607
    int 0x10
    
    ; Mostrar mensaje de boot
    mov si, boot_msg
    call print16
    
    ; Cargar Stage 2 (sectores 2-40)
    mov ah, 0x02
    mov al, 39              ; 39 sectores
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drive]
    mov bx, 0x7E00
    int 0x13
    jc boot_error
    
    ; Pequeña pausa para ver el mensaje
    mov cx, 0x0008
    mov dx, 0x0000
    mov ah, 0x86
    int 0x15
    
    jmp stage2_start

boot_error:
    mov si, err_msg
    call print16
    jmp $

print16:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0E
    mov bx, 0x000F
    int 0x10
    jmp print16
.done:
    ret

boot_msg:   db 13, 10
            db "  FastOS XP Edition", 13, 10
            db "  Loading kernel...", 13, 10, 0
err_msg:    db "  Disk error!", 0
boot_drive: db 0

times 510-($-$$) db 0
dw 0xAA55

; ═══════════════════════════════════════════════════════════════════════════════
; STAGE 2 - Configuración y Desktop
; ═══════════════════════════════════════════════════════════════════════════════
stage2_start:
    ; Configurar modo VGA 320x200x256
    mov ax, 0x0013
    int 0x10
    
    ; Configurar paleta XP
    call setup_xp_palette
    
    ; Segmento de video
    mov ax, 0xA000
    mov es, ax
    
    ; Dibujar desktop completo
    call draw_xp_desktop
    call draw_xp_taskbar
    call draw_xp_start_button
    call draw_xp_systray
    call draw_window_mycomputer
    call draw_window_vulkan
    call draw_desktop_icons
    call draw_peru_flag
    call draw_cursor

    ; Mostrar textos
    call draw_all_texts

main_loop:
    ; Leer teclado (non-blocking)
    mov ah, 0x01
    int 0x16
    jz .no_key
    
    mov ah, 0x00
    int 0x16
    
    ; ESC = salir
    cmp al, 27
    je shutdown
    
.no_key:
    ; Actualizar cursor animado
    call animate_cursor
    
    ; Pequeño delay
    mov cx, 0x0001
    mov dx, 0x0000
    mov ah, 0x86
    int 0x15
    
    jmp main_loop

shutdown:
    ; Modo texto
    mov ax, 0x0003
    int 0x10
    
    mov si, shutdown_msg
    call print16
    
    cli
    hlt

shutdown_msg: db 13, 10, "  FastOS XP shutting down...", 13, 10
              db "  Thank you for using FastOS!", 13, 10
              db "  - Eddi Salazar, Peru", 13, 10, 0

; ═══════════════════════════════════════════════════════════════════════════════
; SETUP XP PALETTE - Colores estilo Windows XP
; ═══════════════════════════════════════════════════════════════════════════════
setup_xp_palette:
    mov dx, 0x03C8
    
    ; 0: Negro
    xor al, al
    out dx, al
    inc dx
    out dx, al
    out dx, al
    out dx, al
    dec dx
    
    ; 1: Azul XP cielo (fondo)
    mov al, 1
    out dx, al
    inc dx
    mov al, 0       ; R
    out dx, al
    mov al, 30      ; G
    out dx, al
    mov al, 50      ; B
    out dx, al
    dec dx
    
    ; 2: Verde XP (Start button)
    mov al, 2
    out dx, al
    inc dx
    mov al, 20
    out dx, al
    mov al, 50
    out dx, al
    mov al, 10
    out dx, al
    dec dx
    
    ; 3: Azul XP oscuro (taskbar)
    mov al, 3
    out dx, al
    inc dx
    mov al, 0
    out dx, al
    mov al, 20
    out dx, al
    mov al, 45
    out dx, al
    dec dx
    
    ; 4: Rojo
    mov al, 4
    out dx, al
    inc dx
    mov al, 63
    out dx, al
    mov al, 0
    out dx, al
    mov al, 0
    out dx, al
    dec dx
    
    ; 5: Azul título ventana XP
    mov al, 5
    out dx, al
    inc dx
    mov al, 0
    out dx, al
    mov al, 32
    out dx, al
    mov al, 63
    out dx, al
    dec dx
    
    ; 6: Gris claro (fondo ventana)
    mov al, 6
    out dx, al
    inc dx
    mov al, 58
    out dx, al
    mov al, 58
    out dx, al
    mov al, 56
    out dx, al
    dec dx
    
    ; 7: Blanco
    mov al, 7
    out dx, al
    inc dx
    mov al, 63
    out dx, al
    mov al, 63
    out dx, al
    mov al, 63
    out dx, al
    dec dx
    
    ; 8: Gris oscuro
    mov al, 8
    out dx, al
    inc dx
    mov al, 32
    out dx, al
    mov al, 32
    out dx, al
    mov al, 32
    out dx, al
    dec dx
    
    ; 9: Verde NVIDIA
    mov al, 9
    out dx, al
    inc dx
    mov al, 30
    out dx, al
    mov al, 48
    out dx, al
    mov al, 0
    out dx, al
    dec dx
    
    ; 10: Amarillo
    mov al, 10
    out dx, al
    inc dx
    mov al, 63
    out dx, al
    mov al, 63
    out dx, al
    mov al, 0
    out dx, al
    dec dx
    
    ; 11: Cyan
    mov al, 11
    out dx, al
    inc dx
    mov al, 0
    out dx, al
    mov al, 50
    out dx, al
    mov al, 63
    out dx, al
    dec dx
    
    ; 12: Naranja (botón cerrar)
    mov al, 12
    out dx, al
    inc dx
    mov al, 63
    out dx, al
    mov al, 25
    out dx, al
    mov al, 0
    out dx, al
    dec dx
    
    ; 13: Azul claro taskbar
    mov al, 13
    out dx, al
    inc dx
    mov al, 10
    out dx, al
    mov al, 35
    out dx, al
    mov al, 55
    out dx, al
    dec dx
    
    ; 14: Verde Start hover
    mov al, 14
    out dx, al
    inc dx
    mov al, 25
    out dx, al
    mov al, 55
    out dx, al
    mov al, 15
    out dx, al
    dec dx
    
    ; 15: Blanco puro
    mov al, 15
    out dx, al
    inc dx
    mov al, 63
    out dx, al
    mov al, 63
    out dx, al
    mov al, 63
    out dx, al
    dec dx
    
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW XP DESKTOP
; ═══════════════════════════════════════════════════════════════════════════════
draw_xp_desktop:
    xor di, di
    mov bx, 172
    
.row:
    cmp bx, 0
    je .done
    mov al, 1
    cmp bx, 140
    jl .use_color
    mov al, 13
.use_color:
    mov cx, 320
    rep stosb
    dec bx
    jmp .row
.done:
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW XP TASKBAR
; ═══════════════════════════════════════════════════════════════════════════════
draw_xp_taskbar:
    mov di, 320*172
    mov bx, 28
.tb_row:
    cmp bx, 0
    je .tb_done
    mov al, 3
    cmp bx, 20
    jl .tb_color
    mov al, 13
.tb_color:
    mov cx, 320
    rep stosb
    dec bx
    jmp .tb_row
.tb_done:
    mov di, 320*172
    mov cx, 320
    mov al, 11
    rep stosb
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW XP START BUTTON
; ═══════════════════════════════════════════════════════════════════════════════
draw_xp_start_button:
    mov di, 320*174 + 3
    mov bx, 22
.start_row:
    cmp bx, 0
    je .start_done
    mov al, 2
    cmp bx, 15
    jl .start_color
    mov al, 14
.start_color:
    push di
    mov cx, 50
    rep stosb
    pop di
    add di, 320
    dec bx
    jmp .start_row
.start_done:
    mov di, 320*174 + 3
    mov cx, 50
    mov al, 7
    rep stosb
    mov di, 320*174 + 3
    mov bx, 22
.left_border:
    mov byte [es:di], 7
    add di, 320
    dec bx
    jnz .left_border
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW XP SYSTRAY
; ═══════════════════════════════════════════════════════════════════════════════
draw_xp_systray:
    mov di, 320*175 + 260
    mov bx, 18
.tray_row:
    cmp bx, 0
    je .tray_done
    push di
    mov cx, 55
    mov al, 3
    rep stosb
    pop di
    add di, 320
    dec bx
    jmp .tray_row
.tray_done:
    mov di, 320*175 + 260
    mov cx, 55
    mov al, 8
    rep stosb
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW WINDOW MY COMPUTER
; ═══════════════════════════════════════════════════════════════════════════════
draw_window_mycomputer:
    mov di, 320*25 + 18
    mov bx, 82
.shadow1:
    push di
    mov cx, 142
    mov al, 0
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .shadow1
    
    mov di, 320*22 + 15
    mov bx, 80
.win1_bg:
    push di
    mov cx, 140
    mov al, 6
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .win1_bg
    
    mov di, 320*22 + 15
    mov bx, 18
.title1:
    push di
    mov cx, 140
    mov al, 5
    cmp bx, 12
    jl .t1_color
    mov al, 11
.t1_color:
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .title1
    
    mov di, 320*24 + 135
    mov bx, 12
.close1:
    push di
    mov cx, 16
    mov al, 12
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .close1
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW WINDOW VULKAN
; ═══════════════════════════════════════════════════════════════════════════════
draw_window_vulkan:
    mov di, 320*45 + 168
    mov bx, 72
.shadow2:
    push di
    mov cx, 142
    mov al, 0
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .shadow2
    
    mov di, 320*42 + 165
    mov bx, 70
.win2_bg:
    push di
    mov cx, 140
    mov al, 6
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .win2_bg
    
    mov di, 320*42 + 165
    mov bx, 18
.title2:
    push di
    mov cx, 140
    mov al, 5
    cmp bx, 12
    jl .t2_color
    mov al, 11
.t2_color:
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .title2
    
    mov di, 320*44 + 285
    mov bx, 12
.close2:
    push di
    mov cx, 16
    mov al, 12
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .close2
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW DESKTOP ICONS
; ═══════════════════════════════════════════════════════════════════════════════
draw_desktop_icons:
    mov di, 320*125 + 20
    mov bx, 20
.icon1:
    push di
    mov cx, 24
    mov al, 5
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .icon1
    
    mov di, 320*125 + 55
    mov bx, 20
.icon2:
    push di
    mov cx, 24
    mov al, 8
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .icon2
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW PERU FLAG
; ═══════════════════════════════════════════════════════════════════════════════
draw_peru_flag:
    mov di, 320*75 + 110
    mov bx, 18
.peru_r1:
    push di
    mov cx, 8
    mov al, 4
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .peru_r1
    
    mov di, 320*75 + 118
    mov bx, 18
.peru_w:
    push di
    mov cx, 8
    mov al, 7
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .peru_w
    
    mov di, 320*75 + 126
    mov bx, 18
.peru_r2:
    push di
    mov cx, 8
    mov al, 4
    rep stosb
    pop di
    add di, 320
    dec bx
    jnz .peru_r2
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW CURSOR
; ═══════════════════════════════════════════════════════════════════════════════
draw_cursor:
    mov word [cursor_x], 160
    mov word [cursor_y], 100
    
animate_cursor:
    mov ax, [cursor_y]
    mov bx, 320
    mul bx
    add ax, [cursor_x]
    mov di, ax
    
    mov bx, 10
.cursor_line:
    push bx
    push di
    mov byte [es:di], 0
    inc di
    mov cx, bx
    dec cx
    jz .skip_white
    mov al, 7
    rep stosb
.skip_white:
    pop di
    add di, 320
    pop bx
    dec bx
    jnz .cursor_line
    ret

cursor_x: dw 160
cursor_y: dw 100

; ═══════════════════════════════════════════════════════════════════════════════
; DRAW ALL TEXTS
; ═══════════════════════════════════════════════════════════════════════════════
draw_all_texts:
    mov dh, 3
    mov dl, 3
    call set_cursor
    mov si, txt_sysinfo
    mov bl, 15
    call print_attr
    
    mov dh, 5
    mov dl, 3
    call set_cursor
    mov si, txt_fastos
    mov bl, 11
    call print_attr
    
    mov dh, 6
    mov dl, 3
    call set_cursor
    mov si, txt_author
    mov bl, 15
    call print_attr
    
    mov dh, 8
    mov dl, 3
    call set_cursor
    mov si, txt_cpu
    mov bl, 10
    call print_attr
    
    mov dh, 9
    mov dl, 3
    call set_cursor
    mov si, txt_gpu
    mov bl, 9
    call print_attr
    
    mov dh, 10
    mov dl, 3
    call set_cursor
    mov si, txt_ram
    mov bl, 11
    call print_attr
    
    mov dh, 11
    mov dl, 3
    call set_cursor
    mov si, txt_storage
    mov bl, 14
    call print_attr
    
    mov dh, 6
    mov dl, 22
    call set_cursor
    mov si, txt_vulkan
    mov bl, 15
    call print_attr
    
    mov dh, 8
    mov dl, 22
    call set_cursor
    mov si, txt_vk_ver
    mov bl, 15
    call print_attr
    
    mov dh, 9
    mov dl, 22
    call set_cursor
    mov si, txt_vk_dev
    mov bl, 9
    call print_attr
    
    mov dh, 10
    mov dl, 22
    call set_cursor
    mov si, txt_vk_vram
    mov bl, 11
    call print_attr
    
    mov dh, 11
    mov dl, 22
    call set_cursor
    mov si, txt_vk_rtx
    mov bl, 14
    call print_attr
    
    mov dh, 12
    mov dl, 22
    call set_cursor
    mov si, txt_vk_status
    mov bl, 10
    call print_attr
    
    mov dh, 22
    mov dl, 1
    call set_cursor
    mov si, txt_start
    mov bl, 15
    call print_attr
    
    mov dh, 22
    mov dl, 33
    call set_cursor
    mov si, txt_clock
    mov bl, 15
    call print_attr
    
    mov dh, 18
    mov dl, 3
    call set_cursor
    mov si, txt_mypc
    mov bl, 15
    call print_attr
    
    mov dh, 18
    mov dl, 8
    call set_cursor
    mov si, txt_trash
    mov bl, 15
    call print_attr
    ret

set_cursor:
    mov ah, 0x02
    xor bh, bh
    int 0x10
    ret

print_attr:
    lodsb
    or al, al
    jz .done
    mov ah, 0x09
    mov cx, 1
    xor bh, bh
    int 0x10
    mov ah, 0x03
    int 0x10
    inc dl
    mov ah, 0x02
    int 0x10
    jmp print_attr
.done:
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; STRINGS
; ═══════════════════════════════════════════════════════════════════════════════
txt_sysinfo:    db "System Info", 0
txt_fastos:     db "FastOS XP", 0
txt_author:     db "Eddi Salazar", 0
txt_cpu:        db "Ryzen 5 5600X", 0
txt_gpu:        db "RTX 3060 12GB", 0
txt_ram:        db "16 GB DDR4", 0
txt_storage:    db "1 TB NVMe", 0

txt_vulkan:     db "Vulkan API", 0
txt_vk_ver:     db "Version 1.3", 0
txt_vk_dev:     db "RTX 3060", 0
txt_vk_vram:    db "VRAM: 12GB", 0
txt_vk_rtx:     db "RTX: ON", 0
txt_vk_status:  db "READY", 0

txt_start:      db "Start", 0
txt_clock:      db "23:45", 0
txt_mypc:       db "PC", 0
txt_trash:      db "Bin", 0

; Padding
times 20480-($-$$) db 0

