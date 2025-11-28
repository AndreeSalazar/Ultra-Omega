/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT + VULKAN - NVIDIA GPU DRIVER IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: gpu_nvidia.c
 * Descripción: Driver para NVIDIA RTX 3060 de Eddi
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "gpu_nvidia.h"
#include "framebuffer.h"
#include "ports64.h"
#include "memory64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLE GLOBAL
 * ═══════════════════════════════════════════════════════════════════════════
 */

static NvidiaGPU gpu = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * NOMBRES DE CHIPS
 * ═══════════════════════════════════════════════════════════════════════════
 */

const char* nvidia_get_chip_name(uint16_t device_id) {
    switch (device_id) {
        case NVIDIA_RTX_3060_GA106:
        case NVIDIA_RTX_3060_GA106_ALT:
            return "NVIDIA GeForce RTX 3060 (GA106)";
        case NVIDIA_RTX_3060_MOBILE:
            return "NVIDIA GeForce RTX 3060 Mobile";
        case NVIDIA_RTX_3070:
            return "NVIDIA GeForce RTX 3070";
        case NVIDIA_RTX_3080:
            return "NVIDIA GeForce RTX 3080";
        case NVIDIA_RTX_3090:
            return "NVIDIA GeForce RTX 3090";
        default:
            return "NVIDIA GPU (Unknown)";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * DETECCIÓN DE RTX 3060
 * ═══════════════════════════════════════════════════════════════════════════
 */

bool nvidia_detect_rtx3060(void) {
    /* Buscar RTX 3060 en el bus PCI */
    PCIDevice* dev = pci_find_device(NVIDIA_VENDOR_ID, NVIDIA_RTX_3060_GA106);
    if (!dev) {
        dev = pci_find_device(NVIDIA_VENDOR_ID, NVIDIA_RTX_3060_GA106_ALT);
    }
    if (!dev) {
        dev = pci_find_device(NVIDIA_VENDOR_ID, NVIDIA_RTX_3060_MOBILE);
    }
    
    if (!dev) {
        /* Buscar cualquier GPU NVIDIA */
        dev = pci_find_gpu();
        if (!dev || dev->vendor_id != NVIDIA_VENDOR_ID) {
            return false;
        }
    }
    
    gpu.pci_dev = dev;
    gpu.vendor_id = dev->vendor_id;
    gpu.device_id = dev->device_id;
    gpu.chip_name = nvidia_get_chip_name(dev->device_id);
    
    return true;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

bool nvidia_init(void) {
    if (gpu.initialized) {
        return true;
    }
    
    /* Detectar GPU */
    if (!nvidia_detect_rtx3060()) {
        fb_set_colors(FB_COLOR_RED, FB_COLOR_TERM_BG);
        fb_print("[ERROR] No se encontro GPU NVIDIA\n");
        fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
        return false;
    }
    
    /* Obtener direcciones de BAR */
    gpu.mmio_base = (volatile uint32_t*)pci_get_bar_address(gpu.pci_dev, 0);
    gpu.mmio_size = pci_get_bar_size(gpu.pci_dev, 0);
    
    gpu.vram_base = (volatile uint8_t*)pci_get_bar_address(gpu.pci_dev, 1);
    gpu.vram_size = pci_get_bar_size(gpu.pci_dev, 1);
    
    if (!gpu.mmio_base) {
        fb_set_colors(FB_COLOR_RED, FB_COLOR_TERM_BG);
        fb_print("[ERROR] No se pudo mapear MMIO de GPU\n");
        fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
        return false;
    }
    
    /* Leer ID del chip */
    gpu.chip_id = nvidia_read32(NV_PMC_BOOT_0);
    gpu.chip_rev = (gpu.chip_id >> 20) & 0xFF;
    
    /* Configurar información de RTX 3060 */
    if (gpu.device_id == NVIDIA_RTX_3060_GA106 || 
        gpu.device_id == NVIDIA_RTX_3060_GA106_ALT) {
        gpu.cuda_cores = RTX3060_CUDA_CORES;
        gpu.sm_count = RTX3060_SM_COUNT;
        gpu.memory_bus_width = RTX3060_MEMORY_BUS;
        gpu.max_clock_mhz = RTX3060_BOOST_CLOCK;
        
        /* Si el BAR1 no reporta el tamaño correcto, usar el conocido */
        if (gpu.vram_size < RTX3060_VRAM_SIZE) {
            gpu.vram_size = RTX3060_VRAM_SIZE;
        }
    }
    
    gpu.initialized = true;
    return true;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ACCESO A REGISTROS
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t nvidia_read32(uint32_t offset) {
    if (!gpu.mmio_base) return 0;
    return gpu.mmio_base[offset / 4];
}

void nvidia_write32(uint32_t offset, uint32_t value) {
    if (!gpu.mmio_base) return;
    gpu.mmio_base[offset / 4] = value;
    mfence(); /* Memory barrier */
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

NvidiaGPU* nvidia_get_gpu(void) {
    return gpu.initialized ? &gpu : NULL;
}

uint32_t nvidia_get_vram_size(void) {
    return (uint32_t)(gpu.vram_size / (1024 * 1024)); /* En MB */
}

void nvidia_print_info(void) {
    if (!gpu.initialized) {
        fb_print("GPU NVIDIA no inicializada\n");
        return;
    }
    
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("\n╔═══════════════════════════════════════════════════════════════╗\n");
    fb_print("║              NVIDIA GPU - RTX 3060 DE EDDI                     ║\n");
    fb_print("╠═══════════════════════════════════════════════════════════════╣\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ Chip: ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print(gpu.chip_name);
    fb_print("\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ PCI: ");
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    /* TODO: Imprimir bus:dev.func */
    fb_print("Detectado en bus PCI\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ CUDA Cores: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("3584\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ Streaming Multiprocessors: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("28\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ VRAM: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("12 GB GDDR6 (192-bit)\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ Boost Clock: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("1777 MHz\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ TDP: ");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("170W\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("║ MMIO Base: ");
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("(mapped)\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("╠═══════════════════════════════════════════════════════════════╣\n");
    fb_print("║              VULKAN READY (Requiere driver completo)          ║\n");
    fb_print("╚═══════════════════════════════════════════════════════════════╝\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FRAMEBUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

volatile uint8_t* nvidia_get_framebuffer(void) {
    return gpu.vram_base;
}

void nvidia_clear_framebuffer(uint32_t color) {
    if (!gpu.vram_base) return;
    
    /* Limpiar primeros 16MB del framebuffer (1920x1080x4 = ~8MB) */
    volatile uint32_t* fb = (volatile uint32_t*)gpu.vram_base;
    for (uint64_t i = 0; i < (16 * 1024 * 1024) / 4; i++) {
        fb[i] = color;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INTERRUPCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

void nvidia_enable_interrupts(void) {
    if (!gpu.initialized) return;
    
    /* Habilitar interrupciones principales */
    nvidia_write32(NV_PMC_INTR_EN_0, 0xFFFFFFFF);
}

void nvidia_disable_interrupts(void) {
    if (!gpu.initialized) return;
    
    nvidia_write32(NV_PMC_INTR_EN_0, 0);
}

void nvidia_handle_interrupt(void) {
    if (!gpu.initialized) return;
    
    uint32_t status = nvidia_read32(NV_PMC_INTR_0);
    
    /* Limpiar interrupciones */
    nvidia_write32(NV_PMC_INTR_0, status);
}

