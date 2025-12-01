# ═══════════════════════════════════════════════════════════════════════════════
# FASTOS 64-BIT + VULKAN - MAKEFILE
# ═══════════════════════════════════════════════════════════════════════════════
# Sistema: FastOS 64-bit con soporte Vulkan-like
# Hardware: AMD Ryzen 5 5600X + NVIDIA RTX 3060 12GB
# Autor: Eddi Andreé Salazar Matos
# ═══════════════════════════════════════════════════════════════════════════════

# Cross-compiler para x86_64
CC = x86_64-elf-gcc
LD = x86_64-elf-ld
AS = nasm
OBJCOPY = x86_64-elf-objcopy

# Flags
CFLAGS = -ffreestanding -fno-stack-protector -fno-pic -mno-red-zone \
         -mno-mmx -mno-sse -mno-sse2 -mcmodel=kernel \
         -Wall -Wextra -O2 -I.

LDFLAGS = -nostdlib -z max-page-size=0x1000

ASFLAGS = -f elf64

# Directorios
BUILD_DIR = build
ISO_DIR = $(BUILD_DIR)/iso

# Archivos fuente
ASM_SOURCES = kernel_entry64.asm
C_SOURCES = kernel_main64.c framebuffer.c pci.c gpu_nvidia.c vulkan_fastos.c \
            string64.c memory64.c keyboard64.c shell64.c idt64.c

# Archivos objeto
ASM_OBJECTS = $(patsubst %.asm,$(BUILD_DIR)/%.o,$(ASM_SOURCES))
C_OBJECTS = $(patsubst %.c,$(BUILD_DIR)/%.o,$(C_SOURCES))
OBJECTS = $(ASM_OBJECTS) $(C_OBJECTS)

# Imagen final
KERNEL = $(BUILD_DIR)/kernel64.elf
ISO = $(BUILD_DIR)/fastos64.iso

# ═══════════════════════════════════════════════════════════════════════════════
# REGLAS PRINCIPALES
# ═══════════════════════════════════════════════════════════════════════════════

.PHONY: all clean run debug iso

all: $(BUILD_DIR) $(KERNEL)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Compilar archivos ASM
$(BUILD_DIR)/%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@

# Compilar archivos C
$(BUILD_DIR)/%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Enlazar kernel
$(KERNEL): $(OBJECTS) linker64.ld
	$(LD) $(LDFLAGS) -T linker64.ld -o $@ $(OBJECTS)
	@echo ""
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  FastOS 64-bit + Vulkan compilado exitosamente!"
	@echo "  Kernel: $(KERNEL)"
	@echo "═══════════════════════════════════════════════════════════════"

# Crear ISO con GRUB
iso: $(KERNEL)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/kernel64.elf
	cp grub.cfg $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) $(ISO_DIR)
	@echo "ISO creada: $(ISO)"

# Ejecutar en QEMU
run: $(KERNEL)
	qemu-system-x86_64 \
		-kernel $(KERNEL) \
		-m 4G \
		-cpu host \
		-enable-kvm \
		-vga std \
		-serial stdio

# Ejecutar sin KVM (Windows/Mac)
run-nokvm: $(KERNEL)
	qemu-system-x86_64 \
		-kernel $(KERNEL) \
		-m 4G \
		-cpu qemu64 \
		-vga std \
		-serial stdio

# Debug con GDB
debug: $(KERNEL)
	qemu-system-x86_64 \
		-kernel $(KERNEL) \
		-m 4G \
		-cpu qemu64 \
		-vga std \
		-s -S \
		-serial stdio

clean:
	rm -rf $(BUILD_DIR)
	@echo "Limpieza completada"

# ═══════════════════════════════════════════════════════════════════════════════
# INFORMACIÓN
# ═══════════════════════════════════════════════════════════════════════════════

info:
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  FASTOS 64-BIT + VULKAN"
	@echo "═══════════════════════════════════════════════════════════════"
	@echo "  Compilador: $(CC)"
	@echo "  Linker: $(LD)"
	@echo "  Assembler: $(AS)"
	@echo ""
	@echo "  Comandos:"
	@echo "    make          - Compilar kernel"
	@echo "    make run      - Ejecutar en QEMU (Linux con KVM)"
	@echo "    make run-nokvm- Ejecutar en QEMU (Windows/Mac)"
	@echo "    make iso      - Crear ISO booteable"
	@echo "    make debug    - Debug con GDB"
	@echo "    make clean    - Limpiar"
	@echo "═══════════════════════════════════════════════════════════════"

