@echo off
REM ═══════════════════════════════════════════════════════════════════════════════
REM FASTOS 64-BIT + VULKAN - BUILD SCRIPT PARA WINDOWS
REM ═══════════════════════════════════════════════════════════════════════════════
REM Sistema: FastOS 64-bit con soporte Vulkan-like
REM Hardware: AMD Ryzen 5 5600X + NVIDIA RTX 3060 12GB
REM Autor: Eddi Andree Salazar Matos
REM ═══════════════════════════════════════════════════════════════════════════════

setlocal enabledelayedexpansion

REM Colores
set "GREEN=[92m"
set "YELLOW=[93m"
set "RED=[91m"
set "CYAN=[96m"
set "RESET=[0m"

REM Directorios
set "BUILD_DIR=build"
set "CURRENT_DIR=%CD%"

REM Cross-compiler (ajustar según tu instalación)
set "CROSS_PREFIX=%CURRENT_DIR%\x86_64-elf"
set "CC=%CROSS_PREFIX%\bin\x86_64-elf-gcc.exe"
set "LD=%CROSS_PREFIX%\bin\x86_64-elf-ld.exe"
set "OBJCOPY=%CROSS_PREFIX%\bin\x86_64-elf-objcopy.exe"

REM NASM y QEMU
set "NASM=nasm"
set "QEMU=qemu-system-x86_64"

REM Buscar QEMU en ubicaciones comunes
if not exist "%QEMU%.exe" (
    if exist "C:\Program Files\qemu\qemu-system-x86_64.exe" (
        set "QEMU=C:\Program Files\qemu\qemu-system-x86_64.exe"
    ) else if exist "C:\qemu\qemu-system-x86_64.exe" (
        set "QEMU=C:\qemu\qemu-system-x86_64.exe"
    )
)

REM Flags
set "CFLAGS=-ffreestanding -fno-stack-protector -fno-pic -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -mcmodel=kernel -Wall -Wextra -O2 -I."
set "LDFLAGS=-nostdlib -z max-page-size=0x1000"
set "ASFLAGS=-f elf64"

echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %CYAN%           FASTOS 64-BIT + VULKAN - Sistema de Eddi                        %RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.

REM Procesar argumentos
if "%1"=="clean" goto :clean
if "%1"=="run" goto :run
if "%1"=="debug" goto :debug
if "%1"=="check" goto :check
if "%1"=="help" goto :help

REM Compilación por defecto
goto :build

REM ═══════════════════════════════════════════════════════════════════════════════
REM BUILD
REM ═══════════════════════════════════════════════════════════════════════════════
:build
echo %CYAN%Compilando FastOS 64-bit + Vulkan...%RESET%
echo.

REM Crear directorio de build
if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"

REM Verificar herramientas
call :check_tools
if errorlevel 1 goto :error

REM Compilar kernel_entry64.asm
echo %YELLOW%[1/7]%RESET% Ensamblando kernel_entry64.asm...
"%NASM%" %ASFLAGS% kernel_entry64.asm -o "%BUILD_DIR%\kernel_entry64.o"
if errorlevel 1 goto :error

REM Compilar archivos C
echo %YELLOW%[2/7]%RESET% Compilando kernel_main64.c...
"%CC%" %CFLAGS% -c kernel_main64.c -o "%BUILD_DIR%\kernel_main64.o"
if errorlevel 1 goto :error

echo %YELLOW%[3/7]%RESET% Compilando framebuffer.c...
"%CC%" %CFLAGS% -c framebuffer.c -o "%BUILD_DIR%\framebuffer.o"
if errorlevel 1 goto :error

echo %YELLOW%[4/7]%RESET% Compilando pci.c...
"%CC%" %CFLAGS% -c pci.c -o "%BUILD_DIR%\pci.o"
if errorlevel 1 goto :error

echo %YELLOW%[5/7]%RESET% Compilando gpu_nvidia.c...
"%CC%" %CFLAGS% -c gpu_nvidia.c -o "%BUILD_DIR%\gpu_nvidia.o"
if errorlevel 1 goto :error

echo %YELLOW%[6/7]%RESET% Compilando vulkan_fastos.c...
"%CC%" %CFLAGS% -c vulkan_fastos.c -o "%BUILD_DIR%\vulkan_fastos.o"
if errorlevel 1 goto :error

REM Enlazar
echo %YELLOW%[7/7]%RESET% Enlazando kernel64.elf...
"%LD%" %LDFLAGS% -T linker64.ld -o "%BUILD_DIR%\kernel64.elf" ^
    "%BUILD_DIR%\kernel_entry64.o" ^
    "%BUILD_DIR%\kernel_main64.o" ^
    "%BUILD_DIR%\framebuffer.o" ^
    "%BUILD_DIR%\pci.o" ^
    "%BUILD_DIR%\gpu_nvidia.o" ^
    "%BUILD_DIR%\vulkan_fastos.o"
if errorlevel 1 goto :error

echo.
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %GREEN%  FastOS 64-bit + Vulkan compilado exitosamente!%RESET%
echo %GREEN%  Kernel: %BUILD_DIR%\kernel64.elf%RESET%
echo %GREEN%  Ejecuta: build64.bat run%RESET%
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM RUN
REM ═══════════════════════════════════════════════════════════════════════════════
:run
if not exist "%BUILD_DIR%\kernel64.elf" (
    echo %YELLOW%Kernel no encontrado, compilando...%RESET%
    call :build
    if errorlevel 1 goto :error
)

echo.
echo %CYAN%Iniciando QEMU con FastOS 64-bit + Vulkan...%RESET%
echo %CYAN%Hardware emulado: 4GB RAM, CPU x86_64%RESET%
echo.

REM Ejecutar QEMU
"%QEMU%" ^
    -kernel "%BUILD_DIR%\kernel64.elf" ^
    -m 4G ^
    -cpu qemu64,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2 ^
    -vga std ^
    -serial stdio ^
    -no-reboot ^
    -no-shutdown

goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM DEBUG
REM ═══════════════════════════════════════════════════════════════════════════════
:debug
if not exist "%BUILD_DIR%\kernel64.elf" (
    call :build
    if errorlevel 1 goto :error
)

echo %CYAN%Iniciando QEMU en modo debug (esperando GDB en puerto 1234)...%RESET%
"%QEMU%" ^
    -kernel "%BUILD_DIR%\kernel64.elf" ^
    -m 4G ^
    -cpu qemu64 ^
    -vga std ^
    -serial stdio ^
    -s -S

goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM CHECK
REM ═══════════════════════════════════════════════════════════════════════════════
:check
echo %CYAN%Verificando herramientas...%RESET%
echo.

echo Buscando NASM...
where nasm >nul 2>&1
if errorlevel 1 (
    echo %RED%  [X] NASM no encontrado%RESET%
    echo      Descarga: https://nasm.us
) else (
    echo %GREEN%  [OK] NASM encontrado%RESET%
)

echo Buscando x86_64-elf-gcc...
if exist "%CC%" (
    echo %GREEN%  [OK] x86_64-elf-gcc encontrado%RESET%
) else (
    echo %RED%  [X] x86_64-elf-gcc no encontrado%RESET%
    echo      Necesitas el cross-compiler para x86_64-elf
    echo      Descarga: https://wiki.osdev.org/GCC_Cross-Compiler
)

echo Buscando QEMU...
if exist "%QEMU%" (
    echo %GREEN%  [OK] QEMU encontrado%RESET%
) else (
    where qemu-system-x86_64 >nul 2>&1
    if errorlevel 1 (
        echo %RED%  [X] QEMU no encontrado%RESET%
        echo      Descarga: https://www.qemu.org
    ) else (
        echo %GREEN%  [OK] QEMU encontrado%RESET%
    )
)

echo.
goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM CHECK TOOLS (interno)
REM ═══════════════════════════════════════════════════════════════════════════════
:check_tools
where nasm >nul 2>&1
if errorlevel 1 (
    echo %RED%Error: NASM no encontrado%RESET%
    exit /b 1
)
if not exist "%CC%" (
    echo %RED%Error: x86_64-elf-gcc no encontrado en %CC%%RESET%
    echo %YELLOW%Descarga el cross-compiler de: https://wiki.osdev.org/GCC_Cross-Compiler%RESET%
    exit /b 1
)
exit /b 0

REM ═══════════════════════════════════════════════════════════════════════════════
REM CLEAN
REM ═══════════════════════════════════════════════════════════════════════════════
:clean
echo %CYAN%Limpiando...%RESET%
if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"
echo %GREEN%Limpieza completada%RESET%
goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM HELP
REM ═══════════════════════════════════════════════════════════════════════════════
:help
echo.
echo %CYAN%FASTOS 64-BIT + VULKAN - Comandos disponibles:%RESET%
echo.
echo   build64.bat          Compilar el kernel
echo   build64.bat run      Ejecutar en QEMU
echo   build64.bat debug    Ejecutar en modo debug (GDB)
echo   build64.bat clean    Limpiar archivos compilados
echo   build64.bat check    Verificar herramientas instaladas
echo   build64.bat help     Mostrar esta ayuda
echo.
echo %YELLOW%Requisitos:%RESET%
echo   - NASM (https://nasm.us)
echo   - x86_64-elf-gcc (Cross-compiler)
echo   - QEMU (https://www.qemu.org)
echo.
goto :end

REM ═══════════════════════════════════════════════════════════════════════════════
REM ERROR
REM ═══════════════════════════════════════════════════════════════════════════════
:error
echo.
echo %RED%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %RED%  Error durante la compilacion!%RESET%
echo %RED%  Ejecuta: build64.bat check%RESET%
echo %RED%═══════════════════════════════════════════════════════════════════════════%RESET%
exit /b 1

:end
endlocal

