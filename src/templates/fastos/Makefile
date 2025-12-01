# ═══════════════════════════════════════════════════════════════════════════
# FastOS - Makefile para Windows
# Nivel: Avanzado - Sistema de compilación
# ═══════════════════════════════════════════════════════════════════════════
# Este Makefile compila todo el sistema operativo FastOS.
# Configurado para Windows con i686-elf cross-compiler
# ═══════════════════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN - RUTAS DEL CROSS-COMPILER
# ═══════════════════════════════════════════════════════════════════════════

# Ruta base del cross-compiler (MODIFICAR SI ES DIFERENTE)
TOOLCHAIN_PATH = C:/Users/Andre/Documents/Mis Programas Poderosos/FastOS/bin

# Nombre del proyecto
PROJECT = FastOS

# Directorios (relativo al proyecto)
BUILD_DIR = build

# ═══════════════════════════════════════════════════════════════════════════
# HERRAMIENTAS
# ═══════════════════════════════════════════════════════════════════════════

# NASM (debe estar en PATH o especificar ruta completa)
ASM = nasm

# Cross-compiler i686-elf
CC = "$(TOOLCHAIN_PATH)/i686-elf-gcc.exe"
LD = "$(TOOLCHAIN_PATH)/i686-elf-ld.exe"
OBJCOPY = "$(TOOLCHAIN_PATH)/i686-elf-objcopy.exe"

# ═══════════════════════════════════════════════════════════════════════════
# FLAGS DE COMPILACIÓN
# ═══════════════════════════════════════════════════════════════════════════

# Flags de ensamblador NASM
ASMFLAGS = -f elf32 -g

# Flags de compilador C (freestanding para kernel)
CFLAGS = -m32 \
         -ffreestanding \
         -fno-builtin \
         -fno-stack-protector \
         -nostdlib \
         -nodefaultlibs \
         -Wall \
         -Wextra \
         -O2 \
         -g \
         -I.

# Flags de enlazador
LDFLAGS = -T linker.ld -nostdlib

# ═══════════════════════════════════════════════════════════════════════════
# ARCHIVOS FUENTE
# ═══════════════════════════════════════════════════════════════════════════

# Archivos ASM del bootloader
ASM_BOOT = boot_sector.asm
ASM_STAGE2 = bootloader_stage2.asm
ASM_ENTRY = kernel_entry.asm

# Archivos C del kernel
C_SOURCES = kernel_main.c \
            vga_driver.c \
            keyboard_driver.c \
            idt.c \
            timer.c \
            memory.c \
            shell.c \
            string.c

# Objetos
C_OBJECTS = $(patsubst %.c,$(BUILD_DIR)/%.o,$(C_SOURCES))

# ═══════════════════════════════════════════════════════════════════════════
# REGLAS PRINCIPALES
# ═══════════════════════════════════════════════════════════════════════════

.PHONY: all clean run debug help check

# Compilar todo
all: $(BUILD_DIR) $(BUILD_DIR)/os-image.bin
	@echo ═══════════════════════════════════════════════════════════════
	@echo   FastOS compilado exitosamente!
	@echo   Imagen: $(BUILD_DIR)/os-image.bin
	@echo   Ejecuta: make run
	@echo ═══════════════════════════════════════════════════════════════

# Crear directorio de build
$(BUILD_DIR):
	@if not exist $(BUILD_DIR) mkdir $(BUILD_DIR)

# Limpiar
clean:
	@echo Limpiando archivos generados...
	@if exist $(BUILD_DIR) rmdir /s /q $(BUILD_DIR)
	@echo Listo.

# Verificar herramientas
check:
	@echo ═══════════════════════════════════════════════════════════════
	@echo Verificando herramientas...
	@echo ═══════════════════════════════════════════════════════════════
	@where nasm 2>nul || echo [ERROR] NASM no encontrado. Instalar desde: https://nasm.us
	@if exist "$(TOOLCHAIN_PATH)/i686-elf-gcc.exe" (echo [OK] i686-elf-gcc encontrado) else (echo [ERROR] i686-elf-gcc no encontrado en $(TOOLCHAIN_PATH))
	@where qemu-system-i386 2>nul || echo [WARN] QEMU no encontrado. Necesario para: make run
	@echo ═══════════════════════════════════════════════════════════════

# Ejecutar en QEMU
run: $(BUILD_DIR)/os-image.bin
	@echo ═══════════════════════════════════════════════════════════════
	@echo Ejecutando FastOS en QEMU...
	@echo ═══════════════════════════════════════════════════════════════
	qemu-system-i386 -drive format=raw,file=$< -m 32M

# Debug con QEMU + GDB
debug: $(BUILD_DIR)/os-image.bin
	@echo Iniciando en modo debug (puerto 1234)...
	@echo Conectar con: gdb -ex "target remote localhost:1234"
	qemu-system-i386 -drive format=raw,file=$< -m 32M -s -S

# ═══════════════════════════════════════════════════════════════════════════
# REGLAS DE COMPILACIÓN
# ═══════════════════════════════════════════════════════════════════════════

# Imagen final del sistema operativo
$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/boot_sector.bin $(BUILD_DIR)/bootloader_stage2.bin $(BUILD_DIR)/kernel.bin
	@echo ───────────────────────────────────────────────────────────────
	@echo Creando imagen del sistema operativo...
	@copy /b $(BUILD_DIR)\boot_sector.bin + $(BUILD_DIR)\bootloader_stage2.bin + $(BUILD_DIR)\kernel.bin $@ >nul
	@echo Imagen creada: $@

# Boot sector (512 bytes, binario puro)
$(BUILD_DIR)/boot_sector.bin: $(ASM_BOOT) | $(BUILD_DIR)
	@echo [ASM] Ensamblando boot sector...
	$(ASM) -f bin $< -o $@

# Stage 2 bootloader (binario puro)
$(BUILD_DIR)/bootloader_stage2.bin: $(ASM_STAGE2) | $(BUILD_DIR)
	@echo [ASM] Ensamblando stage 2 bootloader...
	$(ASM) -f bin $< -o $@

# Kernel binario (extraído del ELF)
$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel.elf
	@echo [OBJCOPY] Extrayendo binario del kernel...
	$(OBJCOPY) -O binary $< $@

# Kernel ELF (kernel_entry.o + todos los .c)
$(BUILD_DIR)/kernel.elf: $(BUILD_DIR)/kernel_entry.o $(C_OBJECTS)
	@echo [LD] Enlazando kernel...
	$(LD) $(LDFLAGS) -o $@ $^

# Kernel entry point (ASM -> ELF object)
$(BUILD_DIR)/kernel_entry.o: $(ASM_ENTRY) | $(BUILD_DIR)
	@echo [ASM] Ensamblando kernel entry...
	$(ASM) $(ASMFLAGS) $< -o $@

# Compilar archivos C
$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	@echo [CC] Compilando $<...
	$(CC) $(CFLAGS) -c $< -o $@

# ═══════════════════════════════════════════════════════════════════════════
# AYUDA
# ═══════════════════════════════════════════════════════════════════════════

help:
	@echo ═══════════════════════════════════════════════════════════════
	@echo   FastOS Makefile - Comandos disponibles
	@echo ═══════════════════════════════════════════════════════════════
	@echo.
	@echo   make          Compilar el sistema operativo
	@echo   make clean    Limpiar archivos generados
	@echo   make run      Ejecutar en QEMU
	@echo   make debug    Ejecutar con GDB (puerto 1234)
	@echo   make check    Verificar herramientas instaladas
	@echo   make help     Mostrar esta ayuda
	@echo.
	@echo ═══════════════════════════════════════════════════════════════
	@echo   Requisitos:
	@echo     - NASM (https://nasm.us)
	@echo     - i686-elf-gcc (cross-compiler)
	@echo     - QEMU (para ejecutar)
	@echo ═══════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════
# DEPENDENCIAS DE HEADERS
# ═══════════════════════════════════════════════════════════════════════════

$(BUILD_DIR)/kernel_main.o: kernel.h vga.h idt.h keyboard.h timer.h memory.h shell.h types.h
$(BUILD_DIR)/vga_driver.o: vga.h ports.h types.h string.h
$(BUILD_DIR)/keyboard_driver.o: keyboard.h ports.h idt.h vga.h types.h
$(BUILD_DIR)/idt.o: idt.h ports.h vga.h types.h
$(BUILD_DIR)/timer.o: timer.h ports.h idt.h vga.h types.h
$(BUILD_DIR)/memory.o: memory.h vga.h types.h
$(BUILD_DIR)/shell.o: shell.h vga.h keyboard.h timer.h memory.h string.h types.h
$(BUILD_DIR)/string.o: string.h types.h
