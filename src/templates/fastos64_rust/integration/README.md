# FastOS 64-bit: ASM + Rust Integration

## Arquitectura

FastOS 64-bit combina lo mejor de ambos mundos:
- **ASM (NASM)**: Bootloader, interrupciones, funciones de bajo nivel
- **Rust**: Kernel principal, drivers, gestión de memoria, seguridad

## Estructura del Proyecto

```
fastos64_rust/
├── asm/                    # Código NASM x86_64
│   ├── boot_uefi.asm      # Bootloader UEFI
│   ├── kernel_entry.asm   # Entry point + ISRs
│   ├── interrupts.asm     # Manejo de interrupciones
│   └── memory.asm         # Funciones de memoria optimizadas
│
├── rust/                   # Código Rust del kernel
│   ├── kernel_main.rs     # Punto de entrada Rust
│   ├── ports.rs           # Port I/O wrapper
│   ├── interrupts.rs      # Sistema de interrupciones
│   ├── memory.rs          # Gestión de memoria
│   ├── drivers.rs         # Drivers de hardware
│   └── ffi.rs             # Foreign Function Interface
│
├── integration/            # Templates de integración
│   ├── Cargo.toml         # Configuración Rust
│   ├── linker.ld          # Linker script
│   ├── build.sh           # Script de compilación
│   └── README.md          # Este archivo
```

## Flujo de Ejecución

```
1. UEFI Firmware
   └─> boot_uefi.asm (ASM)
       └─> kernel_entry.asm (ASM)
           └─> kernel_main_rust() (Rust)
               └─> Sistema completo en Rust
```

## Cómo Compilar

### Requisitos
- NASM (https://nasm.us)
- Rust (nightly) con target `x86_64-unknown-none`
- LLVM toolchain

### Pasos

1. **Configurar Rust target:**
```bash
rustup target add x86_64-unknown-none
rustup component add rust-src
```

2. **Compilar ASM:**
```bash
nasm -f elf64 asm/boot_uefi.asm -o boot_uefi.o
nasm -f elf64 asm/kernel_entry.asm -o kernel_entry.o
nasm -f elf64 asm/interrupts.asm -o interrupts.o
nasm -f elf64 asm/memory.asm -o memory.o
```

3. **Compilar Rust:**
```bash
cargo build --target x86_64-unknown-none --release
```

4. **Linkear todo:**
```bash
ld -T linker.ld -o kernel.elf boot_uefi.o kernel_entry.o interrupts.o memory.o target/x86_64-unknown-none/release/libkernel.a
```

## Interfaz ASM ↔ Rust

### Llamar funciones ASM desde Rust:
```rust
extern "C" {
    fn outb(port: u16, value: u8);
    fn inb(port: u16) -> u8;
}
```

### Llamar funciones Rust desde ASM:
```asm
extern kernel_main_rust
call kernel_main_rust
```

## Ventajas de esta Arquitectura

✅ **ASM**: Control total sobre bootloader e interrupciones
✅ **Rust**: Seguridad de memoria, sin data races
✅ **Rendimiento**: Lo mejor de ambos mundos
✅ **Mantenibilidad**: Código kernel en Rust es más seguro

