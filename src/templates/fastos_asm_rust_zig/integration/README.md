# FastOS: Integración ASM + Rust + Zig

## Estructura

```
fastos_asm_rust_zig/
├── asm/              # NASM - Bootloader y bajo nivel
├── rust/             # Rust - Kernel y drivers
├── zig/              # Zig - Sistema y allocator
└── integration/      # Build scripts y configuración
```

## Compilación

### Linux/Mac
```bash
cd integration
chmod +x build.sh
./build.sh
```

### Windows
```batch
cd integration
build.bat
```

## Requisitos

- NASM (para ASM)
- Rust (para kernel)
- Zig (para sistema)
- Linker (ld o link.exe)

## Resultado

El build genera un kernel ejecutable que combina los tres lenguajes.

