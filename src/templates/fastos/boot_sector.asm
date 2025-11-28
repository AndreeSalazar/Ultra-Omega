; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Boot Sector (Sector de Arranque)
; Nivel: Básico - Primer código que ejecuta la BIOS
; ═══════════════════════════════════════════════════════════════════════════
; Este es el primer sector del disco (512 bytes) que la BIOS carga en memoria
; en la dirección 0x7C00 y comienza a ejecutar.
;
; Funciones:
; - Inicializar registros de segmento
; - Mostrar mensaje de bienvenida
; - Cargar el siguiente sector (Stage 2)
; - Saltar al kernel
; ═══════════════════════════════════════════════════════════════════════════

[BITS 16]                    ; Modo real de 16 bits
[ORG 0x7C00]                 ; Dirección donde la BIOS carga el boot sector

; ═══════════════════════════════════════════════════════════════════════════
; PUNTO DE ENTRADA
; ═══════════════════════════════════════════════════════════════════════════
start:
    ; Deshabilitar interrupciones durante la inicialización
    cli
    
    ; Configurar segmentos
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00           ; Stack crece hacia abajo desde 0x7C00
    
    ; Habilitar interrupciones
    sti
    
    ; Guardar número de drive de arranque
    mov [boot_drive], dl
    
    ; Limpiar pantalla
    call clear_screen
    
    ; Mostrar mensaje de bienvenida
    mov si, msg_welcome
    call print_string
    
    ; Mostrar mensaje de carga
    mov si, msg_loading
    call print_string
    
    ; Cargar Stage 2 del bootloader
    call load_stage2
    
    ; Saltar a Stage 2
    jmp 0x0000:STAGE2_OFFSET

; ═══════════════════════════════════════════════════════════════════════════
; FUNCIONES DE PANTALLA
; ═══════════════════════════════════════════════════════════════════════════

; Limpiar pantalla usando BIOS
clear_screen:
    push ax
    push bx
    push cx
    push dx
    
    mov ah, 0x00             ; Función: establecer modo de video
    mov al, 0x03             ; Modo texto 80x25, 16 colores
    int 0x10
    
    pop dx
    pop cx
    pop bx
    pop ax
    ret

; Imprimir string terminado en null
; SI = puntero al string
print_string:
    push ax
    push bx
    push si
    
.loop:
    lodsb                    ; Cargar byte de [SI] en AL, incrementar SI
    test al, al              ; ¿Es null?
    jz .done
    
    mov ah, 0x0E             ; Función: teletype output
    mov bh, 0x00             ; Página 0
    mov bl, 0x07             ; Color: gris claro
    int 0x10
    
    jmp .loop
    
.done:
    pop si
    pop bx
    pop ax
    ret

; Imprimir carácter
; AL = carácter a imprimir
print_char:
    push ax
    push bx
    
    mov ah, 0x0E
    mov bh, 0x00
    mov bl, 0x07
    int 0x10
    
    pop bx
    pop ax
    ret

; ═══════════════════════════════════════════════════════════════════════════
; FUNCIONES DE DISCO
; ═══════════════════════════════════════════════════════════════════════════

STAGE2_OFFSET   equ 0x7E00   ; Dirección donde cargar Stage 2
STAGE2_SECTORS  equ 4        ; Número de sectores a cargar

; Cargar Stage 2 desde disco
load_stage2:
    push ax
    push bx
    push cx
    push dx
    
    mov ah, 0x02             ; Función: leer sectores
    mov al, STAGE2_SECTORS   ; Número de sectores a leer
    mov ch, 0                ; Cilindro 0
    mov cl, 2                ; Sector 2 (los sectores empiezan en 1)
    mov dh, 0                ; Cabeza 0
    mov dl, [boot_drive]     ; Drive de arranque
    mov bx, STAGE2_OFFSET    ; Dirección de destino ES:BX
    
    int 0x13                 ; Llamar a BIOS
    
    jc .disk_error           ; Si hay error, mostrar mensaje
    
    ; Verificar que se leyeron los sectores correctos
    cmp al, STAGE2_SECTORS
    jne .disk_error
    
    mov si, msg_ok
    call print_string
    
    pop dx
    pop cx
    pop bx
    pop ax
    ret
    
.disk_error:
    mov si, msg_disk_error
    call print_string
    jmp $                    ; Loop infinito

; ═══════════════════════════════════════════════════════════════════════════
; DATOS
; ═══════════════════════════════════════════════════════════════════════════

boot_drive:     db 0

msg_welcome:    db '╔══════════════════════════════════════╗', 13, 10
                db '║     FastOS Bootloader v1.0           ║', 13, 10
                db '║     (c) 2024 - Tu Nombre             ║', 13, 10
                db '╚══════════════════════════════════════╝', 13, 10, 0

msg_loading:    db 'Cargando kernel... ', 0
msg_ok:         db '[OK]', 13, 10, 0
msg_disk_error: db '[ERROR DE DISCO]', 13, 10, 0

; ═══════════════════════════════════════════════════════════════════════════
; FIRMA DE BOOT SECTOR
; ═══════════════════════════════════════════════════════════════════════════

times 510 - ($ - $$) db 0    ; Rellenar con ceros hasta byte 510
dw 0xAA55                    ; Firma de boot sector (bytes 511-512)

