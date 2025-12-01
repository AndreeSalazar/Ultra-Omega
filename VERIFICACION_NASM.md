# ✅ Verificación de NASM en Linux

## Estado de la Instalación

### ✅ NASM Instalado
- **Versión**: 2.15.05
- **Ubicación**: `/usr/bin/nasm`
- **Formato soportado**: `elf64` (para Linux x86-64)

### ✅ GCC Instalado
- **Versión**: 11.4.0
- **Linker**: GNU ld 2.38

## Pruebas Realizadas

### 1. Compilación NASM
```bash
nasm -f elf64 test.asm -o test.o
```
✅ **Resultado**: Compilación exitosa

### 2. Linker
```bash
gcc test.o -o test -no-pie
```
✅ **Resultado**: Linker exitoso

### 3. Ejecución
```bash
./test
```
✅ **Resultado**: "Hola desde NASM Linux!"

## Configuración del Código

El código ahora:

1. **Detecta NASM automáticamente**
   - Verifica que NASM esté instalado
   - Muestra la versión detectada
   - Proporciona mensajes de error claros si no está instalado

2. **Usa el formato correcto para Linux**
   - Formato: `elf64` (en lugar de `win64` de Windows)
   - Extensión objeto: `.o` (en lugar de `.obj`)
   - Ejecutable: sin extensión (en lugar de `.exe`)

3. **Linker optimizado para Linux**
   - Usa `-no-pie` por defecto (evita problemas con Position Independent Executables)
   - Tiene fallback sin `-no-pie` si falla
   - Muestra mensajes de error detallados

## Notas Importantes

### Código NASM para Linux

El código NASM debe ser compatible con Linux. Diferencias principales:

**Windows:**
```nasm
default rel
section .text
global main
extern printf

main:
    sub rsp, 40
    mov rcx, msg    ; Windows: rcx es primer argumento
    xor eax, eax
    call printf
    add rsp, 40
    ret
```

**Linux:**
```nasm
default rel
section .text
global main
extern printf

main:
    sub rsp, 8
    mov rdi, msg    ; Linux: rdi es primer argumento
    xor rax, rax
    call printf
    add rsp, 8
    ret
```

### Convención de Llamadas

- **Windows x64**: `rcx, rdx, r8, r9` (primeros 4 argumentos)
- **Linux x64**: `rdi, rsi, rdx, rcx` (primeros 6 argumentos)

## Solución de Problemas

### Error: "NASM no está instalado"
```bash
sudo apt-get install nasm  # Debian/Ubuntu
sudo dnf install nasm      # Fedora
sudo pacman -S nasm         # Arch
```

### Error: "Error de ensamblado"
- Verifica que el código sea compatible con Linux
- Asegúrate de usar `rdi` en lugar de `rcx` para el primer argumento
- Verifica que uses `elf64` como formato

### Error: "Error linkeando"
- El código intenta automáticamente con y sin `-no-pie`
- Verifica que GCC esté instalado: `gcc --version`
- Revisa los mensajes de error del linker

## Próximos Pasos

1. ✅ NASM detectado y funcionando
2. ✅ Linker configurado correctamente
3. ✅ Código adaptado para Linux
4. 🎯 Listo para usar en Ultra Omega

