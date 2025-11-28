# 🔥 ULTRA-OMEGA: Sistema de Templates Binarios

## Arquitectura

Este módulo proporciona **templates binarios ejecutables** para el backend de nodos de Ultra-Omega.

```
binary/
├── cpu/                    # Templates CPU (x86_64 machine code)
│   ├── *.json             # Metadata del template
│   ├── *.hex              # Representación hexadecimal legible
│   └── *.bin              # Bytecode binario puro
├── gpu/                    # Templates GPU (SPIR-V/Vulkan)
│   ├── *.json             # Metadata del shader
│   ├── *.glsl             # Código fuente GLSL
│   ├── *.spv.hex          # SPIR-V en hexadecimal
│   └── *.spv              # SPIR-V compilado
├── mod.rs                  # Módulo Rust con bytecode embebido
└── README.md               # Este archivo
```

## Templates CPU Disponibles

| Template | Tamaño | Descripción |
|----------|--------|-------------|
| `hello_world` | 55 bytes | Programa completo "Hello World!" |
| `syscall_write` | 33 bytes | Escribir a file descriptor |
| `syscall_exit` | 14 bytes | Terminar proceso |
| `memory_alloc` | 46 bytes | Alocar memoria (mmap) |
| `math_add64` | 7 bytes | Suma 64-bit |
| `math_sub64` | 7 bytes | Resta 64-bit |
| `math_mul64` | 8 bytes | Multiplicación 64-bit |
| `math_div64` | 11 bytes | División 64-bit |

## Templates GPU Disponibles

| Template | Stage | Descripción |
|----------|-------|-------------|
| `vertex_passthrough` | Vertex | Pass-through sin transformación |
| `vertex_mvp` | Vertex | Con matrices MVP |
| `fragment_color` | Fragment | Color interpolado |

## Uso en Rust

```rust
use crate::templates::binary;

// Acceder al bytecode directamente
let hello = binary::cpu::HELLO_WORLD;

// Obtener información de todos los templates
let templates = binary::get_cpu_templates();

// Compilar un template con parámetros
let node = binary::compile_template(
    binary::cpu::SYSCALL_WRITE,
    1,  // node_id
    "write_stdout",
    &[
        (10, 1, 4),      // fd = 1 (stdout)
        (17, 0x1000, 8), // buffer_addr
        (27, 13, 4),     // length = 13
    ],
)?;
```

## Formato de Archivos

### JSON Metadata
```json
{
  "template_name": "syscall_write",
  "template_type": "cpu",
  "architecture": "x86_64",
  "size_bytes": 33,
  "parameters": [
    {
      "name": "fd",
      "type": "uint32",
      "offset": 10,
      "size": 4
    }
  ]
}
```

### Archivo .hex
```hex
0x00: 48 C7 C0 01 00 00 00    ; mov rax, 1
0x07: 48 C7 C7 [FD 4 BYTES]   ; mov rdi, FD
...
```

## Convención de Llamada (System V AMD64 ABI)

- **Argumentos**: `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`
- **Retorno**: `rax` (enteros), `xmm0` (floats)
- **Preservar**: `rbx`, `rbp`, `r12-r15`
- **Scratch**: `rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8-r11`

## Syscalls Linux x86_64

| Número | Nombre | rdi | rsi | rdx |
|--------|--------|-----|-----|-----|
| 0 | read | fd | buf | count |
| 1 | write | fd | buf | count |
| 9 | mmap | addr | len | prot |
| 60 | exit | code | - | - |

## Compilar Shaders GLSL a SPIR-V

```bash
# Instalar Vulkan SDK primero
glslc vertex_passthrough.glsl -o vertex_passthrough.spv
glslc fragment_color.glsl -o fragment_color.spv
```

## Licencia

Parte del proyecto Ultra-Omega. Todos los derechos reservados.

