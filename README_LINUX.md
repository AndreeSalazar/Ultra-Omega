# 🐧 Configuración para Linux

## Dependencias Requeridas

Ultra Omega Node Lab necesita las siguientes herramientas para compilar y ejecutar código:

### 1. **NASM** (Netwide Assembler)
Para compilar código Assembly (NASM).

```bash
# Debian/Ubuntu
sudo apt-get install nasm

# Fedora/RHEL
sudo dnf install nasm

# Arch Linux
sudo pacman -S nasm
```

### 2. **GCC** (GNU Compiler Collection)
Para compilar código C y C++.

```bash
# Debian/Ubuntu
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc gcc-c++ make

# Arch Linux
sudo pacman -S gcc
```

### 3. **Rust** (Opcional, para compilar código Rust)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Instalación Automática

Ejecuta el script de instalación:

```bash
chmod +x install_dependencies.sh
./install_dependencies.sh
```

## Verificación

Verifica que todas las herramientas estén instaladas:

```bash
nasm -v
gcc --version
g++ --version
rustc --version  # Si instalaste Rust
```

## Diferencias con Windows

El código ahora detecta automáticamente el sistema operativo y usa:

- **Linux**: Formato `elf64`, extensión `.o` para objetos, sin extensión para ejecutables
- **Windows**: Formato `win64`, extensión `.obj` para objetos, `.exe` para ejecutables

## Solución de Problemas

### Error: "NASM no está instalado"
1. Instala NASM con el comando apropiado para tu distribución
2. Verifica que esté en tu PATH: `which nasm`

### Error: "Error de ensamblado"
- Verifica que el código NASM sea compatible con Linux (usa `elf64` en lugar de `win64`)
- Asegúrate de que las llamadas al sistema sean compatibles con Linux

### Error: "Error linkeando"
- Verifica que GCC esté instalado: `gcc --version`
- En algunos casos, puede necesitar la opción `-no-pie` (ya está incluida en el código)

## Compilación del Proyecto

```bash
# Compilar normalmente
cargo build

# Compilar con optimizaciones
cargo build --release

# Compilar con soporte Mojo (opcional)
cargo build --features mojo
```

## Ejecución

```bash
cargo run
# o
./target/debug/ultra-omega
```

