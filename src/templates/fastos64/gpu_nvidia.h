/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT + VULKAN - NVIDIA GPU DRIVER HEADER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: gpu_nvidia.h
 * Descripción: Driver básico para NVIDIA RTX 3060
 * Hardware: NVIDIA GeForce RTX 3060 12GB (GA106)
 * 
 * NOTA: Este es un driver de nivel bajo para acceso directo a la GPU.
 *       Para Vulkan completo, se necesita el runtime de NVIDIA.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef GPU_NVIDIA_H
#define GPU_NVIDIA_H

#include "types64.h"
#include "pci.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * NVIDIA DEVICE IDs (RTX 30 Series - Ampere)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define NVIDIA_VENDOR_ID            0x10DE

/* RTX 3060 variantes (GA106) */
#define NVIDIA_RTX_3060_GA106       0x2503
#define NVIDIA_RTX_3060_GA106_ALT   0x2504
#define NVIDIA_RTX_3060_MOBILE      0x2520

/* Otras RTX 30 series */
#define NVIDIA_RTX_3070             0x2484
#define NVIDIA_RTX_3080             0x2206
#define NVIDIA_RTX_3090             0x2204

/* ═══════════════════════════════════════════════════════════════════════════
 * REGISTROS DE GPU NVIDIA (Offsets conocidos)
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* BAR0 - MMIO Registers */
#define NV_PMC_BOOT_0               0x00000000  /* Boot/ID register */
#define NV_PMC_ENABLE               0x00000200  /* Engine enable */
#define NV_PMC_INTR_0               0x00000100  /* Interrupt status */
#define NV_PMC_INTR_EN_0            0x00000140  /* Interrupt enable */

/* PBUS */
#define NV_PBUS_PCI_NV_0            0x00001800
#define NV_PBUS_PCI_NV_1            0x00001804

/* PFIFO */
#define NV_PFIFO_INTR_0             0x00002100
#define NV_PFIFO_INTR_EN_0          0x00002140

/* PGRAPH */
#define NV_PGRAPH_INTR              0x00400100
#define NV_PGRAPH_INTR_EN           0x00400140

/* Display */
#define NV_PDISP_FE_RM_INTR_STAT    0x006100A0
#define NV_PDISP_FE_RM_INTR_EN      0x006100A4

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURA DE INFORMACIÓN DE GPU
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    /* PCI Info */
    PCIDevice*      pci_dev;
    uint16_t        vendor_id;
    uint16_t        device_id;
    
    /* Memory Mapped I/O */
    volatile uint32_t*  mmio_base;      /* BAR0 */
    uint64_t            mmio_size;
    
    /* VRAM */
    volatile uint8_t*   vram_base;      /* BAR1 */
    uint64_t            vram_size;
    
    /* GPU Info */
    uint32_t        chip_id;
    uint32_t        chip_rev;
    const char*     chip_name;
    
    /* Capabilities */
    uint32_t        cuda_cores;
    uint32_t        sm_count;           /* Streaming Multiprocessors */
    uint32_t        memory_bus_width;   /* bits */
    uint32_t        max_clock_mhz;
    
    /* Estado */
    bool            initialized;
    bool            display_enabled;
    
} NvidiaGPU;

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN DE RTX 3060 (EDDI'S GPU)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define RTX3060_CUDA_CORES          3584
#define RTX3060_SM_COUNT            28
#define RTX3060_VRAM_SIZE_GB        12
#define RTX3060_VRAM_SIZE           (12ULL * 1024 * 1024 * 1024)
#define RTX3060_MEMORY_BUS          192     /* bits */
#define RTX3060_BASE_CLOCK          1320    /* MHz */
#define RTX3060_BOOST_CLOCK         1777    /* MHz */
#define RTX3060_TDP                 170     /* Watts */

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
bool nvidia_init(void);
bool nvidia_detect_rtx3060(void);
NvidiaGPU* nvidia_get_gpu(void);

/* Acceso a registros */
uint32_t nvidia_read32(uint32_t offset);
void nvidia_write32(uint32_t offset, uint32_t value);

/* Información */
void nvidia_print_info(void);
const char* nvidia_get_chip_name(uint16_t device_id);
uint32_t nvidia_get_vram_size(void);

/* Display */
bool nvidia_enable_display(void);
void nvidia_set_display_mode(uint32_t width, uint32_t height, uint32_t bpp);

/* Framebuffer */
volatile uint8_t* nvidia_get_framebuffer(void);
void nvidia_clear_framebuffer(uint32_t color);

/* Interrupciones */
void nvidia_enable_interrupts(void);
void nvidia_disable_interrupts(void);
void nvidia_handle_interrupt(void);

/* Power Management */
void nvidia_set_power_state(uint8_t state);
uint32_t nvidia_get_temperature(void);
uint32_t nvidia_get_power_usage(void);

#endif /* GPU_NVIDIA_H */

