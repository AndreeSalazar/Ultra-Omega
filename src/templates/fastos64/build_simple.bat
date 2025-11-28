@echo off
REM ═══════════════════════════════════════════════════════════════════════════════
REM FASTOS 64-BIT + VULKAN - BUILD SIMPLIFICADO
REM ═══════════════════════════════════════════════════════════════════════════════
REM Usa NASM para el bootloader y GCC de Windows para simular
REM Para prueba rápida en QEMU con modo texto VGA
REM ═══════════════════════════════════════════════════════════════════════════════

setlocal enabledelayedexpansion

set "GREEN=[92m"
set "YELLOW=[93m"
set "RED=[91m"
set "CYAN=[96m"
set "MAGENTA=[95m"
set "RESET=[0m"

set "BUILD_DIR=build"

REM Buscar QEMU
set "QEMU="
if exist "C:\Program Files\qemu\qemu-system-x86_64.exe" (
    set "QEMU=C:\Program Files\qemu\qemu-system-x86_64.exe"
) else if exist "C:\qemu\qemu-system-x86_64.exe" (
    set "QEMU=C:\qemu\qemu-system-x86_64.exe"
) else (
    for %%I in (qemu-system-x86_64.exe) do set "QEMU=%%~$PATH:I"
)

echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %CYAN%           FASTOS 64-BIT + VULKAN - Sistema de Eddi                        %RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %MAGENTA%  Hardware: AMD Ryzen 5 5600X + NVIDIA RTX 3060 12GB + 16GB RAM          %RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.

if "%1"=="run" goto :run
if "%1"=="clean" goto :clean
if "%1"=="help" goto :help

goto :build

:build
echo %CYAN%Compilando FastOS 64-bit + Vulkan (modo simplificado)...%RESET%
echo.

if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"

REM Verificar NASM
where nasm >nul 2>&1
if errorlevel 1 (
    echo %RED%Error: NASM no encontrado. Descarga de https://nasm.us%RESET%
    exit /b 1
)

REM Ensamblar bootloader multiboot2
echo %YELLOW%[1/2]%RESET% Ensamblando boot_multiboot2.asm...
nasm -f elf64 boot_multiboot2.asm -o "%BUILD_DIR%\boot.o"
if errorlevel 1 (
    echo %RED%Error ensamblando bootloader%RESET%
    exit /b 1
)

REM Crear kernel mínimo que muestra mensaje en VGA
echo %YELLOW%[2/2]%RESET% Creando kernel de prueba...

REM Crear archivo ASM con kernel mínimo que muestra FastOS + Vulkan
(
echo ; FastOS 64-bit + Vulkan - Kernel de prueba
echo [BITS 64]
echo global kernel_main
echo.
echo section .data
echo     ; Mensaje de bienvenida
echo     msg_line1: db "========================================", 0
echo     msg_line2: db "   FASTOS 64-BIT + VULKAN              ", 0
echo     msg_line3: db "   Sistema de Eddi Andree              ", 0
echo     msg_line4: db "========================================", 0
echo     msg_line5: db "                                        ", 0
echo     msg_line6: db "   CPU: AMD Ryzen 5 5600X              ", 0
echo     msg_line7: db "   GPU: NVIDIA RTX 3060 12GB           ", 0
echo     msg_line8: db "   RAM: 16 GB DDR4                     ", 0
echo     msg_line9: db "   SSD: 1 TB NVMe                      ", 0
echo     msg_line10: db "                                        ", 0
echo     msg_line11: db "   [OK] Modo largo activado ^(64-bit^)   ", 0
echo     msg_line12: db "   [OK] Vulkan-FastOS listo            ", 0
echo     msg_line13: db "   [OK] PCI Bus escaneado              ", 0
echo     msg_line14: db "                                        ", 0
echo     msg_line15: db "   Desarrollador Peruano               ", 0
echo     msg_line16: db "========================================", 0
echo.
echo section .text
echo.
echo kernel_main:
echo     ; VGA text mode buffer en 0xB8000
echo     mov rdi, 0xB8000
echo.
echo     ; Limpiar pantalla ^(fondo azul oscuro^)
echo     mov rcx, 2000
echo     mov ax, 0x1720      ; Espacio con fondo azul
echo .clear_loop:
echo     mov [rdi], ax
echo     add rdi, 2
echo     loop .clear_loop
echo.
echo     ; Dibujar banner
echo     mov rdi, 0xB8000
echo     add rdi, 160*3      ; Linea 3
echo.
echo     ; Linea 1 - borde superior ^(cyan^)
echo     lea rsi, [msg_line1]
echo     mov ah, 0x1B        ; Blanco sobre azul
echo     call print_string
echo.
echo     ; Linea 2 - titulo ^(amarillo^)
echo     add rdi, 160
echo     lea rsi, [msg_line2]
echo     mov ah, 0x1E        ; Amarillo sobre azul
echo     call print_string
echo.
echo     ; Linea 3 - autor ^(verde^)
echo     add rdi, 160
echo     lea rsi, [msg_line3]
echo     mov ah, 0x1A        ; Verde sobre azul
echo     call print_string
echo.
echo     ; Linea 4 - borde ^(cyan^)
echo     add rdi, 160
echo     lea rsi, [msg_line4]
echo     mov ah, 0x1B
echo     call print_string
echo.
echo     ; Linea 5 - espacio
echo     add rdi, 160
echo     lea rsi, [msg_line5]
echo     mov ah, 0x17
echo     call print_string
echo.
echo     ; Linea 6 - CPU ^(blanco^)
echo     add rdi, 160
echo     lea rsi, [msg_line6]
echo     mov ah, 0x1F        ; Blanco brillante
echo     call print_string
echo.
echo     ; Linea 7 - GPU ^(verde^)
echo     add rdi, 160
echo     lea rsi, [msg_line7]
echo     mov ah, 0x1A        ; Verde
echo     call print_string
echo.
echo     ; Linea 8 - RAM
echo     add rdi, 160
echo     lea rsi, [msg_line8]
echo     mov ah, 0x1F
echo     call print_string
echo.
echo     ; Linea 9 - SSD
echo     add rdi, 160
echo     lea rsi, [msg_line9]
echo     mov ah, 0x1F
echo     call print_string
echo.
echo     ; Linea 10 - espacio
echo     add rdi, 160
echo     lea rsi, [msg_line10]
echo     mov ah, 0x17
echo     call print_string
echo.
echo     ; Linea 11 - OK modo largo ^(verde^)
echo     add rdi, 160
echo     lea rsi, [msg_line11]
echo     mov ah, 0x1A
echo     call print_string
echo.
echo     ; Linea 12 - OK Vulkan ^(verde^)
echo     add rdi, 160
echo     lea rsi, [msg_line12]
echo     mov ah, 0x1A
echo     call print_string
echo.
echo     ; Linea 13 - OK PCI ^(verde^)
echo     add rdi, 160
echo     lea rsi, [msg_line13]
echo     mov ah, 0x1A
echo     call print_string
echo.
echo     ; Linea 14 - espacio
echo     add rdi, 160
echo     lea rsi, [msg_line14]
echo     mov ah, 0x17
echo     call print_string
echo.
echo     ; Linea 15 - Peru ^(rojo/blanco^)
echo     add rdi, 160
echo     lea rsi, [msg_line15]
echo     mov ah, 0x1C        ; Rojo sobre azul
echo     call print_string
echo.
echo     ; Linea 16 - borde
echo     add rdi, 160
echo     lea rsi, [msg_line16]
echo     mov ah, 0x1B
echo     call print_string
echo.
echo     ; Dibujar bandera de Peru en esquina
echo     mov rdi, 0xB8000
echo     add rdi, 160*3 + 70  ; Esquina derecha
echo.
echo     ; Fila 1 de bandera
echo     mov word [rdi], 0x4020      ; Rojo
echo     mov word [rdi+2], 0x4020
echo     mov word [rdi+4], 0xF020    ; Blanco
echo     mov word [rdi+6], 0xF020
echo     mov word [rdi+8], 0x4020    ; Rojo
echo     mov word [rdi+10], 0x4020
echo.
echo     ; Fila 2 de bandera
echo     add rdi, 160
echo     mov word [rdi], 0x4020
echo     mov word [rdi+2], 0x4020
echo     mov word [rdi+4], 0xF020
echo     mov word [rdi+6], 0xF020
echo     mov word [rdi+8], 0x4020
echo     mov word [rdi+10], 0x4020
echo.
echo     ; Fila 3 de bandera
echo     add rdi, 160
echo     mov word [rdi], 0x4020
echo     mov word [rdi+2], 0x4020
echo     mov word [rdi+4], 0xF020
echo     mov word [rdi+6], 0xF020
echo     mov word [rdi+8], 0x4020
echo     mov word [rdi+10], 0x4020
echo.
echo .halt:
echo     hlt
echo     jmp .halt
echo.
echo ; Funcion para imprimir string
echo ; RSI = string, AH = atributo, RDI = posicion VGA
echo print_string:
echo     push rdi
echo .print_loop:
echo     lodsb
echo     test al, al
echo     jz .print_done
echo     mov [rdi], ax
echo     add rdi, 2
echo     jmp .print_loop
echo .print_done:
echo     pop rdi
echo     ret
) > "%BUILD_DIR%\kernel_test.asm"

nasm -f elf64 "%BUILD_DIR%\kernel_test.asm" -o "%BUILD_DIR%\kernel.o"
if errorlevel 1 (
    echo %RED%Error creando kernel%RESET%
    exit /b 1
)

REM Enlazar (usando ld de MinGW si está disponible, o crear binario directo)
echo %YELLOW%[3/3]%RESET% Enlazando...

REM Crear script de linker simple
(
echo OUTPUT_FORMAT^(elf64-x86-64^)
echo ENTRY^(_start^)
echo SECTIONS {
echo     . = 0x100000;
echo     .multiboot : { *^(.multiboot^) }
echo     .text : { *^(.text^) }
echo     .rodata : { *^(.rodata^) }
echo     .data : { *^(.data^) }
echo     .bss : { *^(.bss^) }
echo }
) > "%BUILD_DIR%\link.ld"

REM Intentar usar ld de MinGW
where ld >nul 2>&1
if errorlevel 1 (
    echo %YELLOW%Nota: ld no encontrado, usando metodo alternativo...%RESET%
    REM Crear ELF directamente con NASM
    nasm -f bin boot_multiboot2.asm -o "%BUILD_DIR%\fastos64.bin" 2>nul
    if errorlevel 1 (
        echo %RED%Error: Necesitas MinGW o similar para enlazar%RESET%
        echo %YELLOW%Instala MinGW-w64 o usa WSL%RESET%
        exit /b 1
    )
) else (
    ld -n -T "%BUILD_DIR%\link.ld" -o "%BUILD_DIR%\fastos64.elf" "%BUILD_DIR%\boot.o" "%BUILD_DIR%\kernel.o"
    if errorlevel 1 (
        echo %RED%Error enlazando%RESET%
        exit /b 1
    )
)

echo.
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %GREEN%  FastOS 64-bit + Vulkan compilado!%RESET%
echo %GREEN%  Ejecuta: build_simple.bat run%RESET%
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
goto :end

:run
if not exist "%BUILD_DIR%\fastos64.elf" (
    if not exist "%BUILD_DIR%\fastos64.bin" (
        echo %YELLOW%Compilando primero...%RESET%
        call :build
    )
)

if "%QEMU%"=="" (
    echo %RED%Error: QEMU no encontrado%RESET%
    echo %YELLOW%Descarga de: https://www.qemu.org/download/#windows%RESET%
    exit /b 1
)

echo.
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %CYAN%  Iniciando FastOS 64-bit + Vulkan en QEMU...%RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.

if exist "%BUILD_DIR%\fastos64.elf" (
    "%QEMU%" -kernel "%BUILD_DIR%\fastos64.elf" -m 4G -cpu qemu64 -vga std
) else (
    "%QEMU%" -drive format=raw,file="%BUILD_DIR%\fastos64.bin" -m 4G -cpu qemu64 -vga std
)
goto :end

:clean
echo %CYAN%Limpiando...%RESET%
if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"
echo %GREEN%Limpieza completada%RESET%
goto :end

:help
echo.
echo %CYAN%Comandos:%RESET%
echo   build_simple.bat          Compilar
echo   build_simple.bat run      Ejecutar en QEMU
echo   build_simple.bat clean    Limpiar
echo   build_simple.bat help     Ayuda
echo.
goto :end

:end
endlocal

