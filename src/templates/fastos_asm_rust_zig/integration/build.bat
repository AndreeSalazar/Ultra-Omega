@echo off
REM Build script para FastOS ASM + Rust + Zig
REM Windows

echo === Building FastOS ASM + Rust + Zig ===

REM Variables
set KERNEL_NAME=fastos
set NASM_FLAGS=-f win64

REM Compilar ASM
echo Compilando ASM (NASM)...
nasm %NASM_FLAGS% -o boot_uefi.obj ..\asm\boot_uefi.asm
nasm %NASM_FLAGS% -o kernel_entry.obj ..\asm\kernel_entry.asm
nasm %NASM_FLAGS% -o interrupts.obj ..\asm\interrupts.asm
nasm %NASM_FLAGS% -o memory_low.obj ..\asm\memory_low.asm

REM Compilar Rust
echo Compilando Rust...
cargo build --release --target x86_64-unknown-none

REM Compilar Zig
echo Compilando Zig...
zig build-lib -target x86_64-freestanding -static ..\zig\system.zig ..\zig\allocator.zig

REM Linkear todo
echo Linkeando...
REM Usar link.exe o lld.exe dependiendo de lo disponible
link /OUT:%KERNEL_NAME%.exe boot_uefi.obj kernel_entry.obj interrupts.obj memory_low.obj ^
    target\x86_64-unknown-none\release\libfastos_kernel.a system.obj allocator.obj

echo Build completado: %KERNEL_NAME%.exe

