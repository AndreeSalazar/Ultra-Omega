# FastOS - Sistema Operativo desde Cero

```
███████╗ █████╗ ███████╗████████╗ ██████╗ ███████╗
██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔════╝
█████╗  ███████║███████╗   ██║   ██║   ██║███████╗
██╔══╝  ██╔══██║╚════██║   ██║   ██║   ██║╚════██║
██║     ██║  ██║███████║   ██║   ╚██████╔╝███████║
╚═╝     ╚═╝  ╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚══════╝
```

## 📋 Descripción

FastOS es un sistema operativo educativo de 32 bits escrito desde cero en Assembly (NASM) y C. Está diseñado para aprender los conceptos fundamentales del desarrollo de sistemas operativos.

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                         SHELL                                │
├─────────────────────────────────────────────────────────────┤
│  VGA Driver  │  Keyboard  │  Timer  │  Memory Manager       │
├─────────────────────────────────────────────────────────────┤
│                    IDT / Interrupciones                      │
├─────────────────────────────────────────────────────────────┤
│                      KERNEL (32-bit)                         │
├─────────────────────────────────────────────────────────────┤
│                    Bootloader Stage 2                        │
├─────────────────────────────────────────────────────────────┤
│                      Boot Sector                             │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Estructura de Archivos

```
FastOS/
├── src/
│   ├── boot_sector.asm      # Sector de arranque (512 bytes)
│   ├── bootloader_stage2.asm # Preparación modo protegido
│   ├── kernel_entry.asm     # Punto de entrada del kernel
│   ├── kernel_main.c        # Función principal del kernel
│   ├── kernel.h             # Definiciones del kernel
│   ├── vga_driver.c         # Driver de video VGA
│   ├── vga.h
│   ├── keyboard_driver.c    # Driver de teclado PS/2
│   ├── keyboard.h
│   ├── idt.c                # Tabla de descriptores de interrupción
│   ├── idt.h
│   ├── timer.c              # Driver del timer (PIT)
│   ├── timer.h
│   ├── memory.c             # Gestor de memoria dinámica
│   ├── memory.h
│   ├── shell.c              # Intérprete de comandos
│   ├── shell.h
│   ├── string.c             # Funciones de strings
│   ├── string.h
│   ├── types.h              # Tipos de datos
│   ├── ports.h              # Funciones de E/S
│   └── linker.ld            # Script de enlazado
├── build/                   # Archivos compilados
├── Makefile                 # Sistema de compilación
└── README.md
```

## 🔧 Requisitos

### Herramientas necesarias:

- **NASM** - Ensamblador
- **GCC Cross-Compiler (i686-elf-gcc)** - Compilador C para x86
- **GNU Make** - Sistema de compilación
- **QEMU** - Emulador para pruebas

### Instalación en Linux/WSL:

```bash
# Instalar herramientas básicas
sudo apt update
sudo apt install nasm qemu-system-x86 make

# Instalar cross-compiler (ver instrucciones en osdev.org)
# O usar: sudo apt install gcc-i686-linux-gnu
```

## 🚀 Compilación y Ejecución

```bash
# Compilar todo
make

# Ejecutar en QEMU
make run

# Limpiar archivos generados
make clean

# Crear imagen ISO
make iso

# Debug con GDB
make debug
```

## 💻 Comandos del Shell

| Comando    | Descripción                          |
|------------|--------------------------------------|
| `help`     | Muestra ayuda                        |
| `clear`    | Limpia la pantalla                   |
| `echo`     | Muestra texto                        |
| `uptime`   | Tiempo de ejecución                  |
| `memory`   | Estadísticas de memoria              |
| `version`  | Versión del sistema                  |
| `date`     | Fecha y hora actual                  |
| `calc`     | Calculadora simple                   |
| `color`    | Cambiar color del texto              |
| `beep`     | Emitir sonido                        |
| `reboot`   | Reiniciar sistema                    |
| `shutdown` | Apagar sistema                       |

## 📚 Componentes

### 1. Boot Sector (`boot_sector.asm`)
- Primer código que ejecuta la BIOS
- Carga Stage 2 en memoria
- Muestra mensaje de bienvenida

### 2. Stage 2 Bootloader (`bootloader_stage2.asm`)
- Habilita línea A20
- Configura GDT
- Cambia a modo protegido 32-bit
- Carga el kernel

### 3. Kernel Entry (`kernel_entry.asm`)
- Punto de entrada del kernel
- Funciones de E/S de puertos
- Handlers de interrupción

### 4. VGA Driver (`vga_driver.c`)
- Modo texto 80x25
- Colores (16 foreground, 16 background)
- Cursor hardware
- Scroll automático

### 5. Keyboard Driver (`keyboard_driver.c`)
- Teclado PS/2
- Buffer circular
- Soporte Shift, Ctrl, Alt
- Caps Lock, Num Lock

### 6. IDT (`idt.c`)
- Manejo de excepciones (0-19)
- IRQs de hardware (32-47)
- Remapeo del PIC

### 7. Timer (`timer.c`)
- PIT 8254
- Interrupciones periódicas
- Funciones de espera
- Beep del speaker

### 8. Memory Manager (`memory.c`)
- Heap de 4 MB
- kmalloc/kfree
- Detección de corrupción
- Estadísticas

### 9. Shell (`shell.c`)
- Intérprete de comandos
- Historial de comandos
- Parser de argumentos

## 🎓 Conceptos Aprendidos

- **Modo Real vs Modo Protegido**
- **GDT (Global Descriptor Table)**
- **IDT (Interrupt Descriptor Table)**
- **PIC (Programmable Interrupt Controller)**
- **PIT (Programmable Interval Timer)**
- **Memoria VGA**
- **Controlador de teclado 8042**
- **Gestión de memoria dinámica**
- **Cross-compilation**

## 📖 Referencias

- [OSDev Wiki](https://wiki.osdev.org/)
- [Intel x86 Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [Writing a Simple Operating System from Scratch](https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)

## 📝 Licencia

Este proyecto es de código abierto y está disponible para fines educativos.

---

**FastOS** - Aprende creando tu propio sistema operativo 🚀

