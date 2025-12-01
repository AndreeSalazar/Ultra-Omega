# 🔍 Análisis: Segmentation Fault en NASM

## Problema Identificado

El código NASM está escrito para **Windows x64**, pero se está compilando y ejecutando en **Linux**. Esto causa un **segmentation fault (SIGSEGV)** porque las convenciones de llamadas son diferentes.

## Causa Raíz

### Convención de Llamadas Windows x64
```nasm
mov rcx, msg    ; Primer argumento en RCX
mov rdx, arg2   ; Segundo argumento en RDX
mov r8, arg3    ; Tercer argumento en R8
mov r9, arg4    ; Cuarto argumento en R9
call printf
```

### Convención de Llamadas Linux x64 (System V ABI)
```nasm
mov rdi, msg    ; Primer argumento en RDI
mov rsi, arg2   ; Segundo argumento en RSI
mov rdx, arg3   ; Tercer argumento en RDX
mov rcx, arg4   ; Cuarto argumento en RCX
call printf
```

## Código Problemático

El código actual usa:
```nasm
mov rcx, msg    ; ❌ Windows style
call printf
```

En Linux, `printf` espera el primer argumento en `rdi`, pero recibe `rcx` (que contiene basura o un valor incorrecto), causando el segmentation fault.

## Solución

### Código Adaptado para Linux

```nasm
default rel
section .text
global main
extern printf

main:
    sub rsp, 8          ; Alinear pila (múltiplo de 16)
    mov rdi, msg        ; ✅ Linux: primer argumento en RDI
    xor rax, rax        ; RAX = 0 (número de registros vectoriales usados)
    call printf
    add rsp, 8          ; Restaurar pila
    xor rax, rax        ; Return 0
    ret

section .data
    msg db 'Hola NASM Linux!', 10, 0
```

## Diferencias Clave

| Aspecto | Windows x64 | Linux x64 |
|---------|-------------|-----------|
| **Primer argumento** | `rcx` | `rdi` |
| **Segundo argumento** | `rdx` | `rsi` |
| **Tercer argumento** | `r8` | `rdx` |
| **Cuarto argumento** | `r9` | `rcx` |
| **Alineación pila** | 16 bytes (antes de call) | 16 bytes (antes de call) |
| **Shadow space** | 32 bytes reservados | No necesario |
| **RAX antes de call** | No usado | Debe ser 0 (o número de registros vectoriales) |

## Detección Automática

El código ahora detecta automáticamente cuando:
- El código contiene `mov rcx` (patrón Windows)
- El código menciona "win64" pero se compila en Linux
- Muestra advertencias claras antes de compilar

## Mensajes de Error Mejorados

Cuando ocurre un segmentation fault, el sistema ahora muestra:
- ⚠️ Advertencia de código Windows en Linux
- Explicación de las diferencias
- Sugerencias de cómo adaptar el código

## Próximos Pasos

1. **Adaptar el código manualmente**: Cambiar `rcx` → `rdi`, `rdx` → `rsi`, etc.
2. **Usar templates Linux**: Los templates de NASM deberían tener versión Linux
3. **Conversión automática** (futuro): Implementar conversión automática Windows → Linux

## Ejemplo Completo: Windows vs Linux

### Windows (NO funciona en Linux)
```nasm
main:
    sub rsp, 40         ; Shadow space (32) + alineación (8)
    mov rcx, msg        ; Primer argumento
    xor eax, eax
    call printf
    add rsp, 40
    ret
```

### Linux (Funciona correctamente)
```nasm
main:
    sub rsp, 8          ; Solo alineación
    mov rdi, msg        ; Primer argumento
    xor rax, rax        ; RAX = 0
    call printf
    add rsp, 8
    ret
```

## Verificación

Para verificar que el código funciona:
```bash
nasm -f elf64 test.asm -o test.o
gcc test.o -o test -no-pie
./test
```

Si funciona, deberías ver el mensaje sin segmentation fault.

