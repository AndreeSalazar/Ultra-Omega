# 🖥️ Templates CPU - x86_64 Machine Code

## Estructura de Archivos

Cada template CPU tiene 3 archivos:

1. **`*.json`** - Metadata con información de parámetros
2. **`*.hex`** - Representación hexadecimal comentada
3. **`*.bin`** - Bytecode binario puro (generado)

## Templates Disponibles

### 🌟 hello_world
Programa completo que imprime "Hello World!" y termina.
- **Tamaño**: 55 bytes
- **Parámetros**: Ninguno
- **Syscalls**: write(1), exit(60)

### 📝 syscall_write
Escribe datos a un file descriptor.
- **Tamaño**: 33 bytes
- **Parámetros**:
  - `fd` @ offset 10 (4 bytes)
  - `buffer_addr` @ offset 17 (8 bytes)
  - `length` @ offset 27 (4 bytes)

### 🚪 syscall_exit
Termina el proceso con código de salida.
- **Tamaño**: 14 bytes
- **Parámetros**:
  - `exit_code` @ offset 10 (4 bytes)

### 💾 memory_alloc
Aloca memoria usando mmap.
- **Tamaño**: 46 bytes
- **Parámetros**:
  - `size` @ offset 12 (8 bytes)
- **Flags**: MAP_PRIVATE | MAP_ANONYMOUS, PROT_READ | PROT_WRITE | PROT_EXEC

### ➕ math_add64
Suma dos valores de 64 bits.
- **Tamaño**: 7 bytes
- **Entrada**: rdi=a, rsi=b
- **Salida**: rax=a+b

### ➖ math_sub64
Resta dos valores de 64 bits.
- **Tamaño**: 7 bytes
- **Entrada**: rdi=a, rsi=b
- **Salida**: rax=a-b

### ✖️ math_mul64
Multiplica dos valores de 64 bits.
- **Tamaño**: 8 bytes
- **Entrada**: rdi=a, rsi=b
- **Salida**: rax=a*b

### ➗ math_div64
Divide dos valores de 64 bits.
- **Tamaño**: 11 bytes
- **Entrada**: rdi=a, rsi=b
- **Salida**: rax=a/b, rdx=a%b

## Inyección de Parámetros

Los templates con parámetros usan `0xFF` como placeholder:
- `0xFFFFFFFF` para uint32 (4 bytes)
- `0xFFFFFFFFFFFFFFFF` para uint64 (8 bytes)

El compilador reemplaza estos valores en los offsets especificados en el JSON.

## Ejemplo de Compilación Manual

```rust
// Crear una copia del template
let mut code = binary::cpu::SYSCALL_WRITE.to_vec();

// Inyectar fd = 1 (stdout) en offset 10
code[10..14].copy_from_slice(&1u32.to_le_bytes());

// Inyectar buffer_addr en offset 17
code[17..25].copy_from_slice(&buffer_addr.to_le_bytes());

// Inyectar length en offset 27
code[27..31].copy_from_slice(&length.to_le_bytes());
```

## Referencia de Instrucciones x86_64

| Hex | Instrucción | Descripción |
|-----|-------------|-------------|
| `48 C7 C0 XX XX XX XX` | mov rax, imm32 | Cargar valor en rax |
| `48 89 F8` | mov rax, rdi | Copiar rdi a rax |
| `48 01 F0` | add rax, rsi | Sumar rsi a rax |
| `48 29 F0` | sub rax, rsi | Restar rsi de rax |
| `48 0F AF C6` | imul rax, rsi | Multiplicar rax por rsi |
| `48 F7 F6` | div rsi | Dividir rdx:rax por rsi |
| `0F 05` | syscall | Ejecutar syscall |
| `C3` | ret | Retornar |

