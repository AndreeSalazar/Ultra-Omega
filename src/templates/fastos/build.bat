@echo off
REM ═══════════════════════════════════════════════════════════════════════════
REM FastOS - Script de compilación para Windows
REM ═══════════════════════════════════════════════════════════════════════════

setlocal enabledelayedexpansion

REM Configuración de rutas (usar comillas para espacios)
set "FASTOS_ROOT=C:\Users\Andre\Documents\Mis Programas Poderosos\FastOS"
set "TOOLCHAIN=%FASTOS_ROOT%\bin"

REM Obtener directorio actual del script
set "PROJECT_DIR=%~dp0"
set "BUILD_DIR=%PROJECT_DIR%build"

REM Colores
set "GREEN=[92m"
set "RED=[91m"
set "YELLOW=[93m"
set "CYAN=[96m"
set "RESET=[0m"

echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %CYAN%                    FastOS - Sistema de Compilacion                        %RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.

REM Verificar argumento
if "%1"=="" goto build
if "%1"=="clean" goto clean
if "%1"=="run" goto run
if "%1"=="check" goto check
if "%1"=="help" goto help
goto build

:check
echo %YELLOW%Verificando herramientas...%RESET%
echo.

where nasm >nul 2>&1
if %errorlevel%==0 (
    echo %GREEN%[OK]%RESET% NASM encontrado
) else (
    echo %RED%[ERROR]%RESET% NASM no encontrado
    echo        Descargar de: https://nasm.us
)

if exist "%TOOLCHAIN%\i686-elf-gcc.exe" (
    echo %GREEN%[OK]%RESET% i686-elf-gcc encontrado
) else (
    echo %RED%[ERROR]%RESET% i686-elf-gcc no encontrado
)

if exist "%TOOLCHAIN%\i686-elf-ld.exe" (
    echo %GREEN%[OK]%RESET% i686-elf-ld encontrado
) else (
    echo %RED%[ERROR]%RESET% i686-elf-ld no encontrado
)

where qemu-system-i386 >nul 2>&1
if %errorlevel%==0 (
    echo %GREEN%[OK]%RESET% QEMU encontrado
) else (
    echo %YELLOW%[WARN]%RESET% QEMU no encontrado ^(necesario para ejecutar^)
)
echo.
goto end

:clean
echo %YELLOW%Limpiando archivos generados...%RESET%
if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"
echo %GREEN%Listo.%RESET%
goto end

:build
echo %CYAN%Compilando FastOS...%RESET%
echo.

REM Crear directorio build
if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"

REM Cambiar al directorio del proyecto
pushd "%PROJECT_DIR%"

REM 1. Ensamblar boot sector
echo %YELLOW%[1/6]%RESET% Ensamblando boot_sector.asm...
nasm -f bin boot_sector.asm -o "%BUILD_DIR%\boot_sector.bin"
if %errorlevel% neq 0 (
    echo %RED%[ERROR]%RESET% Fallo al ensamblar boot_sector.asm
    goto error
)

REM 2. Ensamblar stage 2
echo %YELLOW%[2/6]%RESET% Ensamblando bootloader_stage2.asm...
nasm -f bin bootloader_stage2.asm -o "%BUILD_DIR%\bootloader_stage2.bin"
if %errorlevel% neq 0 (
    echo %RED%[ERROR]%RESET% Fallo al ensamblar bootloader_stage2.asm
    goto error
)

REM 3. Ensamblar kernel entry
echo %YELLOW%[3/6]%RESET% Ensamblando kernel_entry.asm...
nasm -f elf32 kernel_entry.asm -o "%BUILD_DIR%\kernel_entry.o"
if %errorlevel% neq 0 (
    echo %RED%[ERROR]%RESET% Fallo al ensamblar kernel_entry.asm
    goto error
)

REM 4. Compilar archivos C
echo %YELLOW%[4/6]%RESET% Compilando archivos C...

echo        Compilando kernel_main.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c kernel_main.c -o "%BUILD_DIR%\kernel_main.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando vga_driver.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c vga_driver.c -o "%BUILD_DIR%\vga_driver.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando keyboard_driver.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c keyboard_driver.c -o "%BUILD_DIR%\keyboard_driver.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando idt.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c idt.c -o "%BUILD_DIR%\idt.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando timer.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c timer.c -o "%BUILD_DIR%\timer.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando memory.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c memory.c -o "%BUILD_DIR%\memory.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando shell.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c shell.c -o "%BUILD_DIR%\shell.o"
if %errorlevel% neq 0 goto compile_error

echo        Compilando string.c...
"%TOOLCHAIN%\i686-elf-gcc.exe" -m32 -ffreestanding -fno-builtin -fno-stack-protector -nostdlib -Wall -O2 -c string.c -o "%BUILD_DIR%\string.o"
if %errorlevel% neq 0 goto compile_error

REM 5. Enlazar kernel (usando GCC para incluir libgcc)
echo %YELLOW%[5/6]%RESET% Enlazando kernel...
"%TOOLCHAIN%\i686-elf-gcc.exe" -T linker.ld -o "%BUILD_DIR%\kernel.elf" -ffreestanding -nostdlib "%BUILD_DIR%\kernel_entry.o" "%BUILD_DIR%\kernel_main.o" "%BUILD_DIR%\vga_driver.o" "%BUILD_DIR%\keyboard_driver.o" "%BUILD_DIR%\idt.o" "%BUILD_DIR%\timer.o" "%BUILD_DIR%\memory.o" "%BUILD_DIR%\shell.o" "%BUILD_DIR%\string.o" -lgcc
if %errorlevel% neq 0 (
    echo %RED%[ERROR]%RESET% Fallo al enlazar kernel
    goto error
)

REM Extraer binario
"%TOOLCHAIN%\i686-elf-objcopy.exe" -O binary "%BUILD_DIR%\kernel.elf" "%BUILD_DIR%\kernel.bin"

REM 6. Crear imagen final
echo %YELLOW%[6/6]%RESET% Creando imagen del sistema...
copy /b "%BUILD_DIR%\boot_sector.bin" + "%BUILD_DIR%\bootloader_stage2.bin" + "%BUILD_DIR%\kernel.bin" "%BUILD_DIR%\os-image.bin" >nul

popd

echo.
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %GREEN%  FastOS compilado exitosamente!%RESET%
echo %GREEN%  Imagen: %BUILD_DIR%\os-image.bin%RESET%
echo %GREEN%  Ejecuta: build.bat run%RESET%
echo %GREEN%═══════════════════════════════════════════════════════════════════════════%RESET%
goto end

:compile_error
echo %RED%[ERROR]%RESET% Fallo en compilacion C
goto error

:run
if not exist "%BUILD_DIR%\os-image.bin" (
    echo %RED%[ERROR]%RESET% No se encontro os-image.bin. Ejecuta primero: build.bat
    goto end
)

REM Buscar QEMU en rutas comunes
set "QEMU="
REM MSYS2/MinGW (pacman -S mingw-w64-x86_64-qemu)
if exist "C:\msys64\mingw64\bin\qemu-system-i386.exe" set "QEMU=C:\msys64\mingw64\bin\qemu-system-i386.exe"
if exist "C:\msys64\ucrt64\bin\qemu-system-i386.exe" set "QEMU=C:\msys64\ucrt64\bin\qemu-system-i386.exe"
REM Instalación estándar de QEMU
if exist "C:\Program Files\qemu\qemu-system-i386.exe" set "QEMU=C:\Program Files\qemu\qemu-system-i386.exe"
if exist "C:\Program Files (x86)\qemu\qemu-system-i386.exe" set "QEMU=C:\Program Files (x86)\qemu\qemu-system-i386.exe"
if exist "%USERPROFILE%\qemu\qemu-system-i386.exe" set "QEMU=%USERPROFILE%\qemu\qemu-system-i386.exe"

REM Intentar en PATH
where qemu-system-i386 >nul 2>&1
if %errorlevel%==0 set "QEMU=qemu-system-i386"

if "%QEMU%"=="" (
    echo %RED%[ERROR]%RESET% QEMU no encontrado.
    echo.
    echo %YELLOW%Instala QEMU desde:%RESET%
    echo   https://www.qemu.org/download/#windows
    echo   https://qemu.weilnetz.de/w64/
    echo.
    echo %YELLOW%Luego agrega al PATH:%RESET%
    echo   C:\Program Files\qemu
    echo.
    goto end
)

echo %CYAN%Ejecutando FastOS en QEMU...%RESET%
echo %YELLOW%Presiona Ctrl+Alt+G para liberar el mouse%RESET%
echo.
REM Usar floppy para bootear (más compatible con bootloaders simples)
"%QEMU%" -fda "%BUILD_DIR%\os-image.bin" -m 32M -boot a
goto end

:help
echo.
echo %CYAN%Uso: build.bat [comando]%RESET%
echo.
echo   build.bat         Compilar FastOS
echo   build.bat clean   Limpiar archivos generados
echo   build.bat run     Ejecutar en QEMU
echo   build.bat check   Verificar herramientas
echo   build.bat help    Mostrar esta ayuda
echo.
goto end

:error
popd
echo.
echo %RED%La compilacion fallo.%RESET%
exit /b 1

:end
endlocal
