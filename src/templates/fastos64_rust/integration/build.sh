#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FastOS 64-bit: Build Script
# Compila ASM + Rust y genera kernel.elf
# ═══════════════════════════════════════════════════════════════════════════════

set -e

echo "🔨 Compilando FastOS 64-bit (ASM + Rust)..."

# Directorios
ASM_DIR="asm"
RUST_DIR="rust"
BUILD_DIR="build"
OBJ_DIR="$BUILD_DIR/obj"

# Crear directorios
mkdir -p $BUILD_DIR $OBJ_DIR

# ═══════════════════════════════════════════════════════════════════════════════
# PASO 1: Compilar ASM (NASM)
# ═══════════════════════════════════════════════════════════════════════════════
echo "📝 Compilando código ASM..."

nasm -f elf64 $ASM_DIR/boot_uefi.asm -o $OBJ_DIR/boot_uefi.o
nasm -f elf64 $ASM_DIR/kernel_entry.asm -o $OBJ_DIR/kernel_entry.o
nasm -f elf64 $ASM_DIR/interrupts.asm -o $OBJ_DIR/interrupts.o
nasm -f elf64 $ASM_DIR/memory.asm -o $OBJ_DIR/memory.o

echo "✅ ASM compilado"

# ═══════════════════════════════════════════════════════════════════════════════
# PASO 2: Compilar Rust
# ═══════════════════════════════════════════════════════════════════════════════
echo "🦀 Compilando código Rust..."

cd $RUST_DIR
cargo build --target x86_64-unknown-none --release
cd ..

echo "✅ Rust compilado"

# ═══════════════════════════════════════════════════════════════════════════════
# PASO 3: Linkear todo
# ═══════════════════════════════════════════════════════════════════════════════
echo "🔗 Linkeando kernel..."

ld -T linker.ld \
   -o $BUILD_DIR/kernel.elf \
   $OBJ_DIR/boot_uefi.o \
   $OBJ_DIR/kernel_entry.o \
   $OBJ_DIR/interrupts.o \
   $OBJ_DIR/memory.o \
   $RUST_DIR/target/x86_64-unknown-none/release/libkernel.a

echo "✅ Kernel linkeado: $BUILD_DIR/kernel.elf"

# ═══════════════════════════════════════════════════════════════════════════════
# PASO 4: Generar imagen booteable (opcional)
# ═══════════════════════════════════════════════════════════════════════════════
if command -v objcopy &> /dev/null; then
    echo "📦 Generando imagen binaria..."
    objcopy -O binary $BUILD_DIR/kernel.elf $BUILD_DIR/kernel.bin
    echo "✅ Imagen generada: $BUILD_DIR/kernel.bin"
fi

echo ""
echo "🎉 ¡Compilación completada!"
echo "   Kernel: $BUILD_DIR/kernel.elf"
echo ""
echo "Para ejecutar en QEMU:"
echo "  qemu-system-x86_64 -kernel $BUILD_DIR/kernel.elf"

