# 🚀 FastOS 64-bit: ASM (NASM) + Rust

Sistema operativo de 64 bits que combina **NASM x86_64** y **Rust** para aprovechar lo mejor de ambos mundos.

## 📁 Estructura

```
fastos64_rust/
├── asm/                    # 🔴 Código NASM x86_64
│   ├── boot_uefi.asm      # Bootloader UEFI
│   ├── kernel_entry.asm   # Entry point + ISRs
│   ├── interrupts.asm     # Manejo de interrupciones
│   └── memory.asm         # Funciones de memoria optimizadas
│
├── rust/                   # 🦀 Código Rust del kernel
│   ├── lib.rs             # Módulo principal
│   ├── kernel_main.rs     # Punto de entrada
│   ├── ports.rs           # Port I/O wrapper
│   ├── interrupts.rs      # Sistema de interrupciones
│   ├── memory.rs          # Gestión de memoria
│   ├── drivers.rs         # Drivers de hardware
│   └── ffi.rs             # Foreign Function Interface
│
└── integration/            # 🔗 Integración y Build
    ├── Cargo.toml         # Configuración Rust
    ├── linker.ld          # Linker script
    ├── build.sh           # Build script (Linux/Mac)
    ├── build.bat          # Build script (Windows)
    └── README.md          # Documentación de integración
```

## 🎯 Filosofía de Diseño

### ASM (NASM) se encarga de:
- ✅ Bootloader y arranque del sistema
- ✅ Interrupt Service Routines (ISRs)
- ✅ Funciones de bajo nivel (puertos, CPU)
- ✅ Operaciones de memoria optimizadas
- ✅ Entry points críticos

### Rust se encarga de:
- ✅ Kernel principal y lógica del sistema
- ✅ Drivers de hardware (con wrappers ASM)
- ✅ Gestión de memoria segura
- ✅ Sistema de archivos
- ✅ Procesos y scheduling

## 🔄 Flujo de Ejecución

```
┌─────────────────┐
│  UEFI Firmware  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ boot_uefi.asm   │ ← ASM: Bootloader
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│kernel_entry.asm │ ← ASM: Entry point + ISRs
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│kernel_main_rust │ ← Rust: Kernel principal
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Sistema OS     │ ← Rust: Drivers, memoria, etc.
└─────────────────┘
```

## 🔌 Interfaz ASM ↔ Rust

### Llamar ASM desde Rust:
```rust
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
}

unsafe {
    outb(0x3D4, 0x0E);
    let valor = inb(0x3D5);
}
```

### Llamar Rust desde ASM:
```asm
extern kernel_main_rust
call kernel_main_rust
```

## 📦 Templates Disponibles

### ASM (NASM):
- `boot_uefi.asm` - Bootloader UEFI completo
- `kernel_entry.asm` - Entry point + 48 ISRs
- `interrupts.asm` - Funciones de interrupciones
- `memory.asm` - Memcpy/memset/memcmp optimizados

### Rust:
- `kernel_main.rs` - Punto de entrada principal
- `ports.rs` - Wrapper seguro para Port I/O
- `interrupts.rs` - Sistema de interrupciones
- `memory.rs` - Gestión de memoria + allocator
- `drivers.rs` - VGA, Keyboard, Timer, Serial
- `ffi.rs` - Foreign Function Interface

### Integration:
- `Cargo.toml` - Configuración Rust
- `linker.ld` - Script de linkeo
- `build.sh` / `build.bat` - Scripts de compilación
- `example_integration.rs` - Ejemplos de uso

## 🛠️ Compilación

Ver `integration/README.md` para instrucciones detalladas.

## ✨ Ventajas

| Aspecto | ASM | Rust |
|---------|-----|------|
| **Control** | ✅ Total | ⚠️ Limitado |
| **Seguridad** | ❌ Manual | ✅ Garantizada |
| **Rendimiento** | ✅ Máximo | ✅ Excelente |
| **Mantenibilidad** | ❌ Difícil | ✅ Fácil |

**Combinación**: Lo mejor de ambos mundos 🚀

