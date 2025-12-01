@echo off
REM ═══════════════════════════════════════════════════════════════════════════════
REM FastOS 64-bit: Build Script (Windows)
REM Compila ASM + Rust y genera kernel.elf
REM ═══════════════════════════════════════════════════════════════════════════════

echo 🔨 Compilando FastOS 64-bit (ASM + Rust)...

REM Directorios
set ASM_DIR=asm
set RUST_DIR=rust
set BUILD_DIR=build
set OBJ_DIR=%BUILD_DIR%\obj

REM Crear directorios
if not exist %BUILD_DIR% mkdir %BUILD_DIR%
if not exist %OBJ_DIR% mkdir %OBJ_DIR%

REM ═══════════════════════════════════════════════════════════════════════════════
REM PASO 1: Compilar ASM (NASM)
REM ═══════════════════════════════════════════════════════════════════════════════
echo 📝 Compilando código ASM...

nasm -f win64 %ASM_DIR%\boot_uefi.asm -o %OBJ_DIR%\boot_uefi.o
nasm -f win64 %ASM_DIR%\kernel_entry.asm -o %OBJ_DIR%\kernel_entry.o
nasm -f win64 %ASM_DIR%\interrupts.asm -o %OBJ_DIR%\interrupts.o
nasm -f win64 %ASM_DIR%\memory.asm -o %OBJ_DIR%\memory.o

if errorlevel 1 (
    echo ❌ Error compilando ASM
    exit /b 1
)

echo ✅ ASM compilado

REM ═══════════════════════════════════════════════════════════════════════════════
REM PASO 2: Compilar Rust
REM ═══════════════════════════════════════════════════════════════════════════════
echo 🦀 Compilando código Rust...

cd %RUST_DIR%
cargo build --target x86_64-unknown-none --release
if errorlevel 1 (
    echo ❌ Error compilando Rust
    exit /b 1
)
cd ..

echo ✅ Rust compilado

REM ═══════════════════════════════════════════════════════════════════════════════
REM PASO 3: Linkear todo
REM ═══════════════════════════════════════════════════════════════════════════════
echo 🔗 Linkeando kernel...

ld -T linker.ld ^
   -o %BUILD_DIR%\kernel.elf ^
   %OBJ_DIR%\boot_uefi.o ^
   %OBJ_DIR%\kernel_entry.o ^
   %OBJ_DIR%\interrupts.o ^
   %OBJ_DIR%\memory.o ^
   %RUST_DIR%\target\x86_64-unknown-none\release\libkernel.a

if errorlevel 1 (
    echo ❌ Error linkeando
    exit /b 1
)

echo ✅ Kernel linkeado: %BUILD_DIR%\kernel.elf

echo.
echo 🎉 ¡Compilación completada!
echo    Kernel: %BUILD_DIR%\kernel.elf
echo.
echo Para ejecutar en QEMU:
echo   qemu-system-x86_64 -kernel %BUILD_DIR%\kernel.elf

