#!/bin/bash
# Build script para FastOS ASM + Rust + Zig
# Linux/Mac

set -e

echo "=== Building FastOS ASM + Rust + Zig ==="

# Variables
KERNEL_NAME="fastos"
NASM_FLAGS="-f elf64"
RUST_FLAGS="--target x86_64-unknown-none"
ZIG_FLAGS="-target x86_64-freestanding"

# Compilar ASM
echo "Compilando ASM (NASM)..."
nasm $NASM_FLAGS -o boot_uefi.o ../asm/boot_uefi.asm
nasm $NASM_FLAGS -o kernel_entry.o ../asm/kernel_entry.asm
nasm $NASM_FLAGS -o interrupts.o ../asm/interrupts.asm
nasm $NASM_FLAGS -o memory_low.o ../asm/memory_low.asm

# Compilar Rust
echo "Compilando Rust..."
cargo build --release $RUST_FLAGS

# Compilar Zig
echo "Compilando Zig..."
zig build-lib $ZIG_FLAGS -static ../zig/system.zig ../zig/allocator.zig

# Linkear todo
echo "Linkeando..."
ld -T linker.ld -o $KERNEL_NAME boot_uefi.o kernel_entry.o interrupts.o memory_low.o \
   target/x86_64-unknown-none/release/libfastos_kernel.a \
   system.o allocator.o

echo "Build completado: $KERNEL_NAME"

