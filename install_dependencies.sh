#!/bin/bash
# Script para instalar dependencias de Ultra Omega en Linux

echo "🔧 Instalando dependencias para Ultra Omega Node Lab..."
echo ""

# Detectar distribución
if [ -f /etc/debian_version ]; then
    echo "📦 Detectada distribución Debian/Ubuntu"
    echo "Instalando dependencias..."
    sudo apt-get update
    sudo apt-get install -y nasm gcc g++ build-essential
    echo "✅ Dependencias instaladas"
    
elif [ -f /etc/redhat-release ]; then
    echo "📦 Detectada distribución Red Hat/Fedora"
    echo "Instalando dependencias..."
    sudo dnf install -y nasm gcc gcc-c++ make
    echo "✅ Dependencias instaladas"
    
elif [ -f /etc/arch-release ]; then
    echo "📦 Detectada distribución Arch Linux"
    echo "Instalando dependencias..."
    sudo pacman -S --noconfirm nasm gcc
    echo "✅ Dependencias instaladas"
    
else
    echo "⚠️  Distribución no reconocida"
    echo "Por favor, instala manualmente:"
    echo "  - nasm (ensamblador)"
    echo "  - gcc (compilador C)"
    echo "  - g++ (compilador C++)"
    echo "  - rustc (compilador Rust - se instala con rustup)"
    exit 1
fi

echo ""
echo "🔍 Verificando instalación..."

# Verificar NASM
if command -v nasm &> /dev/null; then
    echo "✅ NASM: $(nasm -v | head -n1)"
else
    echo "❌ NASM no encontrado"
fi

# Verificar GCC
if command -v gcc &> /dev/null; then
    echo "✅ GCC: $(gcc --version | head -n1)"
else
    echo "❌ GCC no encontrado"
fi

# Verificar G++
if command -v g++ &> /dev/null; then
    echo "✅ G++: $(g++ --version | head -n1)"
else
    echo "❌ G++ no encontrado"
fi

# Verificar Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "⚠️  Rust no encontrado"
    echo "   Instala Rust con: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

echo ""
echo "🎉 ¡Listo! Ahora puedes compilar y ejecutar Ultra Omega."

