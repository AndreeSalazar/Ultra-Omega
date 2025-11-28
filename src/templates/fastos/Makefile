# ═══════════════════════════════════════════════════════════════════════════
# FastOS - Makefile
# Nivel: Avanzado - Sistema de compilación
# ═══════════════════════════════════════════════════════════════════════════
# Este Makefile compila todo el sistema operativo FastOS.
# ═══════════════════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN
# ═══════════════════════════════════════════════════════════════════════════

# Nombre del proyecto
PROJECT = FastOS

# Directorios
SRC_DIR = src
BUILD_DIR = build
ISO_DIR = iso

# Herramientas
ASM = nasm
CC = i686-elf-gcc
LD = i686-elf-ld
OBJCOPY = i686-elf-objcopy

# Flags de ensamblador
ASMFLAGS = -f elf32 -g

# Flags de compilador C
CFLAGS = -m32 -ffreestanding -fno-builtin -fno-stack-protector \
         -nostdlib -nodefaultlibs -Wall -Wextra -O2 -g \
         -I$(SRC_DIR)

# Flags de enlazador
LDFLAGS = -T $(SRC_DIR)/linker.ld -nostdlib

# ═══════════════════════════════════════════════════════════════════════════
# ARCHIVOS FUENTE
# ═══════════════════════════════════════════════════════════════════════════

# Archivos ASM
ASM_SOURCES = $(SRC_DIR)/boot_sector.asm \
              $(SRC_DIR)/bootloader_stage2.asm \
              $(SRC_DIR)/kernel_entry.asm

# Archivos C
C_SOURCES = $(SRC_DIR)/kernel_main.c \
            $(SRC_DIR)/vga_driver.c \
            $(SRC_DIR)/keyboard_driver.c \
            $(SRC_DIR)/idt.c \
            $(SRC_DIR)/timer.c \
            $(SRC_DIR)/memory.c \
            $(SRC_DIR)/shell.c \
            $(SRC_DIR)/string.c

# Objetos
ASM_OBJECTS = $(patsubst $(SRC_DIR)/%.asm,$(BUILD_DIR)/%.o,$(ASM_SOURCES))
C_OBJECTS = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(C_SOURCES))
OBJECTS = $(BUILD_DIR)/kernel_entry.o $(filter-out $(BUILD_DIR)/kernel_entry.o,$(C_OBJECTS))

# ═══════════════════════════════════════════════════════════════════════════
# REGLAS PRINCIPALES
# ═══════════════════════════════════════════════════════════════════════════

.PHONY: all clean run debug iso

# Compilar todo
all: $(BUILD_DIR)/os-image.bin

# Limpiar
clean:
	@echo "Limpiando..."
	@rm -rf $(BUILD_DIR)
	@rm -rf $(ISO_DIR)
	@echo "Listo."

# Ejecutar en QEMU
run: $(BUILD_DIR)/os-image.bin
	@echo "Ejecutando FastOS en QEMU..."
	qemu-system-i386 -drive format=raw,file=$< -m 32M

# Debug con GDB
debug: $(BUILD_DIR)/os-image.bin
	@echo "Iniciando en modo debug..."
	qemu-system-i386 -drive format=raw,file=$< -m 32M -s -S &
	gdb -ex "target remote localhost:1234" -ex "symbol-file $(BUILD_DIR)/kernel.elf"

# Crear ISO booteable
iso: $(BUILD_DIR)/os-image.bin
	@echo "Creando ISO..."
	@mkdir -p $(ISO_DIR)/boot/grub
	@cp $< $(ISO_DIR)/boot/
	@echo 'set timeout=0' > $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'set default=0' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo 'menuentry "FastOS" {' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    multiboot /boot/os-image.bin' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '    boot' >> $(ISO_DIR)/boot/grub/grub.cfg
	@echo '}' >> $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(BUILD_DIR)/$(PROJECT).iso $(ISO_DIR)
	@echo "ISO creado: $(BUILD_DIR)/$(PROJECT).iso"

# ═══════════════════════════════════════════════════════════════════════════
# REGLAS DE COMPILACIÓN
# ═══════════════════════════════════════════════════════════════════════════

# Crear directorio de build
$(BUILD_DIR):
	@mkdir -p $@

# Imagen del sistema operativo
$(BUILD_DIR)/os-image.bin: $(BUILD_DIR)/boot_sector.bin $(BUILD_DIR)/bootloader_stage2.bin $(BUILD_DIR)/kernel.bin
	@echo "Creando imagen del sistema..."
	@cat $^ > $@
	@# Rellenar hasta múltiplo de 512 bytes
	@truncate -s %512 $@
	@echo "Imagen creada: $@ ($(shell stat -f%z $@ 2>/dev/null || stat -c%s $@) bytes)"

# Boot sector
$(BUILD_DIR)/boot_sector.bin: $(SRC_DIR)/boot_sector.asm | $(BUILD_DIR)
	@echo "Ensamblando boot sector..."
	$(ASM) -f bin $< -o $@

# Stage 2 bootloader
$(BUILD_DIR)/bootloader_stage2.bin: $(SRC_DIR)/bootloader_stage2.asm | $(BUILD_DIR)
	@echo "Ensamblando stage 2..."
	$(ASM) -f bin $< -o $@

# Kernel binario
$(BUILD_DIR)/kernel.bin: $(BUILD_DIR)/kernel.elf
	@echo "Extrayendo binario del kernel..."
	$(OBJCOPY) -O binary $< $@

# Kernel ELF
$(BUILD_DIR)/kernel.elf: $(OBJECTS) | $(BUILD_DIR)
	@echo "Enlazando kernel..."
	$(LD) $(LDFLAGS) -o $@ $^

# Compilar archivos ASM del kernel
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.asm | $(BUILD_DIR)
	@echo "Ensamblando $<..."
	$(ASM) $(ASMFLAGS) $< -o $@

# Compilar archivos C
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	@echo "Compilando $<..."
	$(CC) $(CFLAGS) -c $< -o $@

# ═══════════════════════════════════════════════════════════════════════════
# REGLAS DE AYUDA
# ═══════════════════════════════════════════════════════════════════════════

help:
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "FastOS Makefile - Comandos disponibles:"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  make          - Compilar el sistema operativo"
	@echo "  make clean    - Limpiar archivos generados"
	@echo "  make run      - Ejecutar en QEMU"
	@echo "  make debug    - Ejecutar con GDB"
	@echo "  make iso      - Crear imagen ISO booteable"
	@echo "  make help     - Mostrar esta ayuda"
	@echo "═══════════════════════════════════════════════════════════════"

# ═══════════════════════════════════════════════════════════════════════════
# DEPENDENCIAS
# ═══════════════════════════════════════════════════════════════════════════

# Dependencias de headers
$(BUILD_DIR)/kernel_main.o: $(SRC_DIR)/kernel.h $(SRC_DIR)/vga.h $(SRC_DIR)/idt.h \
                            $(SRC_DIR)/keyboard.h $(SRC_DIR)/timer.h $(SRC_DIR)/memory.h \
                            $(SRC_DIR)/shell.h

$(BUILD_DIR)/vga_driver.o: $(SRC_DIR)/vga.h $(SRC_DIR)/ports.h $(SRC_DIR)/types.h

$(BUILD_DIR)/keyboard_driver.o: $(SRC_DIR)/keyboard.h $(SRC_DIR)/ports.h $(SRC_DIR)/idt.h \
                                 $(SRC_DIR)/vga.h $(SRC_DIR)/types.h

$(BUILD_DIR)/idt.o: $(SRC_DIR)/idt.h $(SRC_DIR)/ports.h $(SRC_DIR)/vga.h $(SRC_DIR)/types.h

$(BUILD_DIR)/timer.o: $(SRC_DIR)/timer.h $(SRC_DIR)/ports.h $(SRC_DIR)/idt.h \
                      $(SRC_DIR)/vga.h $(SRC_DIR)/types.h

$(BUILD_DIR)/memory.o: $(SRC_DIR)/memory.h $(SRC_DIR)/vga.h $(SRC_DIR)/types.h

$(BUILD_DIR)/shell.o: $(SRC_DIR)/shell.h $(SRC_DIR)/vga.h $(SRC_DIR)/keyboard.h \
                      $(SRC_DIR)/timer.h $(SRC_DIR)/memory.h $(SRC_DIR)/string.h \
                      $(SRC_DIR)/types.h

$(BUILD_DIR)/string.o: $(SRC_DIR)/string.h $(SRC_DIR)/types.h

