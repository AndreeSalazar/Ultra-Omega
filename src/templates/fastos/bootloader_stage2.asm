; ═══════════════════════════════════════════════════════════════════════════
; FastOS - Bootloader Stage 2
; Nivel: Intermedio - Preparación para modo protegido
; ═══════════════════════════════════════════════════════════════════════════
; Este código se carga después del boot sector y prepara el sistema para
; entrar en modo protegido de 32 bits.
;
; Funciones:
; - Habilitar línea A20
; - Configurar GDT (Global Descriptor Table)
; - Cambiar a modo protegido
; - Saltar al kernel de 32 bits
; ═══════════════════════════════════════════════════════════════════════════

[BITS 16]
[ORG 0x7E00]

stage2_start:
    ; Mostrar mensaje
    mov si, msg_stage2
    call print_string_16
    
    ; Habilitar línea A20
    call enable_a20
    
    ; Cargar kernel en memoria
    call load_kernel
    
    ; Cargar GDT
    cli
    lgdt [gdt_descriptor]
    
    ; Entrar en modo protegido
    mov eax, cr0
    or eax, 1                ; Establecer bit PE (Protection Enable)
    mov cr0, eax
    
    ; Salto lejano para limpiar pipeline y cargar CS
    jmp CODE_SEG:protected_mode_start

; ═══════════════════════════════════════════════════════════════════════════
; HABILITAR LÍNEA A20
; ═══════════════════════════════════════════════════════════════════════════
; La línea A20 permite acceder a memoria por encima de 1MB

enable_a20:
    push ax
    
    ; Método 1: Usando BIOS
    mov ax, 0x2401
    int 0x15
    jnc .done
    
    ; Método 2: Usando controlador de teclado
    call .wait_input
    mov al, 0xAD             ; Deshabilitar teclado
    out 0x64, al
    
    call .wait_input
    mov al, 0xD0             ; Leer output port
    out 0x64, al
    
    call .wait_output
    in al, 0x60
    push ax
    
    call .wait_input
    mov al, 0xD1             ; Escribir output port
    out 0x64, al
    
    call .wait_input
    pop ax
    or al, 2                 ; Establecer bit A20
    out 0x60, al
    
    call .wait_input
    mov al, 0xAE             ; Habilitar teclado
    out 0x64, al
    
    call .wait_input
    
.done:
    mov si, msg_a20_ok
    call print_string_16
    pop ax
    ret

.wait_input:
    in al, 0x64
    test al, 2
    jnz .wait_input
    ret

.wait_output:
    in al, 0x64
    test al, 1
    jz .wait_output
    ret

; ═══════════════════════════════════════════════════════════════════════════
; CARGAR KERNEL
; ═══════════════════════════════════════════════════════════════════════════

KERNEL_OFFSET   equ 0x10000  ; Dirección donde cargar el kernel
KERNEL_SECTORS  equ 32       ; Sectores del kernel

load_kernel:
    push ax
    push bx
    push cx
    push dx
    push es
    
    mov si, msg_loading_kernel
    call print_string_16
    
    ; Configurar ES:BX para destino
    mov ax, KERNEL_OFFSET >> 4
    mov es, ax
    xor bx, bx
    
    ; Leer sectores
    mov ah, 0x02
    mov al, KERNEL_SECTORS
    mov ch, 0                ; Cilindro 0
    mov cl, 6                ; Sector 6 (después de stage 2)
    mov dh, 0                ; Cabeza 0
    mov dl, 0x80             ; Disco duro
    
    int 0x13
    jc .error
    
    mov si, msg_ok_16
    call print_string_16
    
    pop es
    pop dx
    pop cx
    pop bx
    pop ax
    ret

.error:
    mov si, msg_kernel_error
    call print_string_16
    jmp $

; ═══════════════════════════════════════════════════════════════════════════
; FUNCIONES DE IMPRESIÓN (16 bits)
; ═══════════════════════════════════════════════════════════════════════════

print_string_16:
    push ax
    push bx
    push si
.loop:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0E
    mov bh, 0
    int 0x10
    jmp .loop
.done:
    pop si
    pop bx
    pop ax
    ret

; ═══════════════════════════════════════════════════════════════════════════
; GDT - GLOBAL DESCRIPTOR TABLE
; ═══════════════════════════════════════════════════════════════════════════

gdt_start:

; Descriptor nulo (requerido)
gdt_null:
    dd 0x0
    dd 0x0

; Descriptor de código
gdt_code:
    dw 0xFFFF                ; Límite (bits 0-15)
    dw 0x0000                ; Base (bits 0-15)
    db 0x00                  ; Base (bits 16-23)
    db 10011010b             ; Flags: Present, Ring 0, Code, Executable, Readable
    db 11001111b             ; Flags: Granularity 4K, 32-bit, Límite (bits 16-19)
    db 0x00                  ; Base (bits 24-31)

; Descriptor de datos
gdt_data:
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 10010010b             ; Flags: Present, Ring 0, Data, Writable
    db 11001111b
    db 0x00

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1   ; Tamaño de GDT
    dd gdt_start                  ; Dirección de GDT

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

; ═══════════════════════════════════════════════════════════════════════════
; MODO PROTEGIDO (32 bits)
; ═══════════════════════════════════════════════════════════════════════════

[BITS 32]

protected_mode_start:
    ; Configurar segmentos de datos
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Configurar stack
    mov esp, 0x90000
    
    ; Limpiar pantalla en modo protegido
    call clear_screen_32
    
    ; Mostrar mensaje
    mov esi, msg_protected
    mov edi, 0xB8000
    call print_string_32
    
    ; Saltar al kernel
    jmp KERNEL_OFFSET

; Limpiar pantalla (modo protegido)
clear_screen_32:
    push eax
    push ecx
    push edi
    
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0720           ; Espacio con atributo gris
    rep stosw
    
    pop edi
    pop ecx
    pop eax
    ret

; Imprimir string (modo protegido)
; ESI = string, EDI = posición en video memory
print_string_32:
    push eax
    push esi
    push edi
    
.loop:
    lodsb
    test al, al
    jz .done
    mov ah, 0x0F             ; Atributo: blanco sobre negro
    stosw
    jmp .loop
    
.done:
    pop edi
    pop esi
    pop eax
    ret

; ═══════════════════════════════════════════════════════════════════════════
; MENSAJES
; ═══════════════════════════════════════════════════════════════════════════

msg_stage2:         db 'Stage 2 cargado', 13, 10, 0
msg_a20_ok:         db 'A20 habilitada', 13, 10, 0
msg_loading_kernel: db 'Cargando kernel... ', 0
msg_ok_16:          db '[OK]', 13, 10, 0
msg_kernel_error:   db '[ERROR]', 13, 10, 0
msg_protected:      db 'FastOS - Modo Protegido 32-bit activado!', 0

; Rellenar hasta 2KB (4 sectores)
times 2048 - ($ - $$) db 0

