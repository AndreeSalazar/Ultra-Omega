# 🚀 FastOS: ASM + Rust + Zig

Sistema operativo de 64 bits que combina **NASM x86_64**, **Rust** y **Zig** para aprovechar lo mejor de los tres mundos.

## 📁 Estructura

```
fastos_asm_rust_zig/
├── asm/                    # 🔴 NASM x86_64
│   ├── boot_uefi.asm      # Bootloader UEFI
│   ├── kernel_entry.asm   # Entry point
│   ├── interrupts.asm     # Manejo de interrupciones
│   └── memory_low.asm     # Funciones de memoria optimizadas
│
├── rust/                   # 🦀 Rust
│   ├── kernel_main.rs     # Punto de entrada principal
│   ├── ports.rs           # Port I/O wrapper
│   ├── interrupts.rs      # Sistema de interrupciones
│   ├── memory.rs          # Gestión de memoria
│   ├── drivers.rs         # Drivers de hardware
│   └── ffi.rs             # Foreign Function Interface
│
├── zig/                    # ⚡ Zig
│   ├── system.zig         # Sistema operativo
│   ├── allocator.zig      # Allocator de memoria
│   ├── filesystem.zig     # Sistema de archivos
│   └── scheduler.zig      # Scheduler de procesos
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
- ✅ Seguridad y prevención de bugs

### Zig se encarga de:
- ✅ Sistema de archivos
- ✅ Scheduler de procesos
- ✅ Allocators de alto rendimiento
- ✅ Componentes del sistema que requieren máximo rendimiento

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
│kernel_main.rs   │ ← Rust: Kernel principal
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│system.zig       │ ← Zig: Sistema operativo
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  FastOS OS      │
└─────────────────┘
```

## 🔌 Interfaz Multi-Lenguaje

### Llamar ASM desde Rust/Zig:
```rust
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
}
```

### Llamar Rust desde Zig:
```zig
extern fn rust_get_system_info() u64;
```

### Llamar Zig desde Rust:
```rust
extern "C" {
    fn zig_system_init();
    fn zig_alloc(size: usize) -> *mut u8;
}
```

## ✨ Ventajas de la Combinación

| Aspecto | ASM | Rust | Zig |
|---------|-----|------|-----|
| **Control** | ✅ Total | ⚠️ Limitado | ⚠️ Limitado |
| **Seguridad** | ❌ Manual | ✅ Garantizada | ✅ Garantizada |
| **Rendimiento** | ✅ Máximo | ✅ Excelente | ✅ Excelente |
| **Simplicidad** | ❌ Complejo | ⚠️ Media | ✅ Alta |
| **Compilación** | ⚠️ Manual | ✅ Buena | ✅ Excelente |

**Combinación**: Lo mejor de los tres mundos 🚀

