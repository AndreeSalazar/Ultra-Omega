# 🚀 FastOS 64-bit - Sistema Operativo de 64 bits

## Descripción

**FastOS 64-bit** es una versión avanzada del sistema operativo educativo FastOS, diseñado para arquitectura x86_64 (modo largo). Incluye soporte para:

- ✅ UEFI Boot
- ✅ Modo Largo (64-bit)
- ✅ Framebuffer gráfico (GOP)
- ✅ Escaneo de bus PCI/PCIe
- ✅ Detección de GPU
- ✅ Preparado para drivers de GPU futuros

## 📁 Estructura de Archivos

```
fastos64/
├── boot_uefi.asm       # Bootloader UEFI (PE64)
├── kernel_entry64.asm  # Punto de entrada del kernel
├── kernel_main64.c     # Kernel principal
├── types64.h           # Tipos de datos de 64 bits
├── ports64.h           # I/O de puertos
├── framebuffer.h/c     # Gráficos de framebuffer
├── idt64.h             # Interrupt Descriptor Table
├── memory64.h          # Gestión de memoria
├── keyboard64.h        # Driver de teclado
├── pci.h/c             # Driver PCI (para detectar GPU)
├── shell64.h           # Shell interactivo
├── font8x16.h          # Fuente bitmap
├── linker64.ld         # Script de linker
└── Makefile            # Sistema de compilación
```

## 🔗 Mapa de Nodos (Ultra-Omega)

```
                    ┌─────────────────────┐
                    │   boot_uefi.asm     │ ← UEFI Bootloader
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ kernel_entry64.asm  │ ← Entrada 64-bit
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │     types64.h       │ ← Tipos base
                    └──────────┬──────────┘
                ┌──────────────┼──────────────┐
                │              │              │
     ┌──────────▼────┐  ┌──────▼──────┐  ┌────▼──────────┐
     │  ports64.h    │  │ framebuffer │  │   memory64    │
     └───────────────┘  └─────────────┘  └───────────────┘
                │              │              │
                └──────────────┼──────────────┘
                               │
                    ┌──────────▼──────────┐
                    │      idt64.h        │ ← Interrupciones
                    └──────────┬──────────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
     ┌──────────▼────┐  ┌──────▼──────┐  ┌────▼──────────┐
     │  keyboard64   │  │    pci.c    │  │   shell64     │
     └───────────────┘  └─────────────┘  └───────────────┘
                               │
                    ┌──────────▼──────────┐
                    │  kernel_main64.c    │ ← Kernel principal
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │  FASTOS 64-BIT      │
                    └─────────────────────┘
```

## 🛠️ Requisitos

### Cross-Compiler
```bash
# x86_64-elf-gcc (para Linux)
# O usar el cross-compiler de tu distribución
```

### Herramientas
- **FASM** o **NASM** para el bootloader UEFI
- **x86_64-elf-gcc** para el kernel
- **QEMU** con soporte UEFI (OVMF)

## 🔨 Compilación

```bash
# Compilar
make

# Ejecutar en QEMU con UEFI
make run

# Limpiar
make clean
```

## 🎮 Vulkan + FastOS 64-bit

### ¿Es posible?

**Técnicamente sí**, pero requiere:

1. **Driver de GPU completo** - Miles de líneas de código
2. **Implementación de Vulkan** - Muy complejo
3. **Memoria virtual** - Ya soportado en 64-bit

### Enfoque Realista

En lugar de implementar Vulkan completo, FastOS 64-bit prepara la base:

1. ✅ **Escaneo PCI** - Detecta GPUs
2. ✅ **Acceso a BARs** - Puede mapear memoria de GPU
3. ✅ **Framebuffer** - Gráficos básicos funcionan
4. 🔄 **Próximo paso** - Driver básico de GPU (modo de video)

### Arquitectura para GPU

```
┌─────────────────────────────────────────────────────────┐
│                    FASTOS 64-BIT                        │
├─────────────────────────────────────────────────────────┤
│  Aplicación                                             │
│       │                                                 │
│       ▼                                                 │
│  ┌─────────────────────────────────────────────────┐   │
│  │           Graphics API (futuro)                  │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────────────┐  │   │
│  │  │ Vulkan  │  │ OpenGL  │  │ Framebuffer API │  │   │
│  │  │ (futuro)│  │ (futuro)│  │    (actual)     │  │   │
│  │  └────┬────┘  └────┬────┘  └────────┬────────┘  │   │
│  └───────┼────────────┼────────────────┼───────────┘   │
│          │            │                │               │
│          ▼            ▼                ▼               │
│  ┌─────────────────────────────────────────────────┐   │
│  │              GPU Driver Layer                    │   │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────────────┐  │   │
│  │  │ NVIDIA  │  │   AMD   │  │  QEMU/VirtIO    │  │   │
│  │  │ (futuro)│  │ (futuro)│  │   (básico)      │  │   │
│  │  └────┬────┘  └────┬────┘  └────────┬────────┘  │   │
│  └───────┼────────────┼────────────────┼───────────┘   │
│          │            │                │               │
│          ▼            ▼                ▼               │
│  ┌─────────────────────────────────────────────────┐   │
│  │                   PCI Driver                     │   │
│  │              (Implementado ✅)                   │   │
│  └─────────────────────────────────────────────────┘   │
│                         │                              │
│                         ▼                              │
│  ┌─────────────────────────────────────────────────┐   │
│  │                    HARDWARE                      │   │
│  │         GPU (NVIDIA/AMD/Intel/QEMU)             │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## 📝 Comandos del Shell

```
help     - Mostrar ayuda
clear    - Limpiar pantalla
info     - Información del sistema
pci      - Listar dispositivos PCI
gpu      - Información de GPU detectada
mem      - Estado de memoria
reboot   - Reiniciar sistema
halt     - Detener sistema
```

## 👨‍💻 Autor

**Eddi Andreé Salazar Matos**  
Desarrollador Peruano 🇵🇪  
Ultra-Omega Project

## 📄 Licencia

MIT License - Libre para uso personal y comercial.

