; ═══════════════════════════════════════════════════════════════════════════════
; FASTOS 64-BIT - UEFI BOOTLOADER
; ═══════════════════════════════════════════════════════════════════════════════
; Archivo: boot_uefi.asm
; Descripción: Bootloader UEFI para sistemas de 64 bits
; Formato: PE32+ ejecutable para UEFI
; ═══════════════════════════════════════════════════════════════════════════════

format PE64 EFI
entry efi_main

; ═══════════════════════════════════════════════════════════════════════════════
; ESTRUCTURAS UEFI
; ═══════════════════════════════════════════════════════════════════════════════

struct EFI_TABLE_HEADER
    Signature       dq ?
    Revision        dd ?
    HeaderSize      dd ?
    CRC32           dd ?
    Reserved        dd ?
ends

struct EFI_SYSTEM_TABLE
    Hdr             EFI_TABLE_HEADER
    FirmwareVendor  dq ?
    FirmwareRevision dd ?
    Padding1        dd ?
    ConsoleInHandle dq ?
    ConIn           dq ?
    ConsoleOutHandle dq ?
    ConOut          dq ?
    StandardErrorHandle dq ?
    StdErr          dq ?
    RuntimeServices dq ?
    BootServices    dq ?
    NumberOfTableEntries dq ?
    ConfigurationTable dq ?
ends

struct SIMPLE_TEXT_OUTPUT_PROTOCOL
    Reset           dq ?
    OutputString    dq ?
    TestString      dq ?
    QueryMode       dq ?
    SetMode         dq ?
    SetAttribute    dq ?
    ClearScreen     dq ?
    SetCursorPosition dq ?
    EnableCursor    dq ?
    Mode            dq ?
ends

; ═══════════════════════════════════════════════════════════════════════════════
; SECCIÓN DE DATOS
; ═══════════════════════════════════════════════════════════════════════════════

section '.data' data readable writeable

SystemTable     dq ?
ConOut          dq ?

; Strings en formato Unicode (UCS-2)
msg_welcome     du 13, 10
                du '╔═══════════════════════════════════════════════════════════════╗', 13, 10
                du '║                                                               ║', 13, 10
                du '║   ███████╗ █████╗ ███████╗████████╗ ██████╗ ███████╗         ║', 13, 10
                du '║   ██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔════╝         ║', 13, 10
                du '║   █████╗  ███████║███████╗   ██║   ██║   ██║███████╗         ║', 13, 10
                du '║   ██╔══╝  ██╔══██║╚════██║   ██║   ██║   ██║╚════██║         ║', 13, 10
                du '║   ██║     ██║  ██║███████║   ██║   ╚██████╔╝███████║         ║', 13, 10
                du '║   ╚═╝     ╚═╝  ╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚══════╝         ║', 13, 10
                du '║                                                               ║', 13, 10
                du '║                    64-BIT EDITION                             ║', 13, 10
                du '║                                                               ║', 13, 10
                du '╠═══════════════════════════════════════════════════════════════╣', 13, 10
                du '║  Desarrollado por: Eddi Andree Salazar Matos                  ║', 13, 10
                du '║  Desarrollador Peruano                                        ║', 13, 10
                du '╚═══════════════════════════════════════════════════════════════╝', 13, 10
                du 13, 10, 0

msg_loading     du '[*] Cargando FastOS 64-bit...', 13, 10, 0
msg_memory      du '[*] Configurando memoria...', 13, 10, 0
msg_gop         du '[*] Inicializando GOP (Graphics Output Protocol)...', 13, 10, 0
msg_kernel      du '[*] Saltando al kernel de 64 bits...', 13, 10, 0
msg_ok          du ' [OK]', 13, 10, 0

; ═══════════════════════════════════════════════════════════════════════════════
; SECCIÓN DE CÓDIGO
; ═══════════════════════════════════════════════════════════════════════════════

section '.text' code executable readable

; ═══════════════════════════════════════════════════════════════════════════════
; PUNTO DE ENTRADA UEFI
; ═══════════════════════════════════════════════════════════════════════════════
; RCX = ImageHandle
; RDX = SystemTable
; ═══════════════════════════════════════════════════════════════════════════════

efi_main:
    ; Guardar parámetros
    push rbx
    push rsi
    push rdi
    sub rsp, 40                     ; Shadow space para llamadas
    
    ; Guardar SystemTable
    mov [SystemTable], rdx
    
    ; Obtener ConOut
    mov rax, [rdx + EFI_SYSTEM_TABLE.ConOut]
    mov [ConOut], rax
    
    ; Limpiar pantalla
    mov rcx, [ConOut]
    mov rax, [rcx + SIMPLE_TEXT_OUTPUT_PROTOCOL.ClearScreen]
    call rax
    
    ; Mostrar banner de bienvenida
    lea rdx, [msg_welcome]
    call print_string
    
    ; Mostrar mensajes de carga
    lea rdx, [msg_loading]
    call print_string
    
    lea rdx, [msg_memory]
    call print_string
    
    lea rdx, [msg_gop]
    call print_string
    
    lea rdx, [msg_kernel]
    call print_string
    
    ; TODO: Aquí iría el código para:
    ; 1. Obtener el mapa de memoria
    ; 2. Inicializar GOP para framebuffer
    ; 3. Cargar el kernel desde disco
    ; 4. Saltar al kernel
    
    ; Por ahora, loop infinito
.hang:
    hlt
    jmp .hang

; ═══════════════════════════════════════════════════════════════════════════════
; FUNCIÓN: print_string
; Imprime una cadena Unicode usando UEFI ConOut
; RDX = puntero a la cadena
; ═══════════════════════════════════════════════════════════════════════════════

print_string:
    push rcx
    push rax
    
    mov rcx, [ConOut]
    mov rax, [rcx + SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString]
    call rax
    
    pop rax
    pop rcx
    ret

; ═══════════════════════════════════════════════════════════════════════════════
; SECCIÓN DE RELOCALIZACIÓN (requerida por PE)
; ═══════════════════════════════════════════════════════════════════════════════

section '.reloc' fixups data discardable

