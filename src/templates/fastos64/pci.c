/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - PCI BUS DRIVER IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: pci.c
 * Descripción: Implementación del driver PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "pci.h"
#include "ports64.h"
#include "framebuffer.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define MAX_PCI_DEVICES 256

static PCIDevice pci_devices[MAX_PCI_DEVICES];
static uint32_t pci_device_count = 0;

/* ═══════════════════════════════════════════════════════════════════════════
 * ACCESO A CONFIGURACIÓN PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t pci_config_read(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset) {
    uint32_t address = (1 << 31)              /* Enable bit */
                     | ((uint32_t)bus << 16)
                     | ((uint32_t)device << 11)
                     | ((uint32_t)func << 8)
                     | (offset & 0xFC);
    
    outl(PCI_CONFIG_ADDRESS, address);
    return inl(PCI_CONFIG_DATA);
}

void pci_config_write(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset, uint32_t value) {
    uint32_t address = (1 << 31)
                     | ((uint32_t)bus << 16)
                     | ((uint32_t)device << 11)
                     | ((uint32_t)func << 8)
                     | (offset & 0xFC);
    
    outl(PCI_CONFIG_ADDRESS, address);
    outl(PCI_CONFIG_DATA, value);
}

uint8_t pci_read8(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset) {
    uint32_t value = pci_config_read(bus, device, func, offset);
    return (value >> ((offset & 3) * 8)) & 0xFF;
}

uint16_t pci_read16(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset) {
    uint32_t value = pci_config_read(bus, device, func, offset);
    return (value >> ((offset & 2) * 8)) & 0xFFFF;
}

uint32_t pci_read32(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset) {
    return pci_config_read(bus, device, func, offset);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * NOMBRES DE CLASES
 * ═══════════════════════════════════════════════════════════════════════════
 */

const char* pci_get_class_name(uint8_t class_code) {
    switch (class_code) {
        case PCI_CLASS_UNCLASSIFIED:    return "Unclassified";
        case PCI_CLASS_STORAGE:         return "Storage";
        case PCI_CLASS_NETWORK:         return "Network";
        case PCI_CLASS_DISPLAY:         return "Display/GPU";
        case PCI_CLASS_MULTIMEDIA:      return "Multimedia";
        case PCI_CLASS_MEMORY:          return "Memory";
        case PCI_CLASS_BRIDGE:          return "Bridge";
        case PCI_CLASS_COMMUNICATION:   return "Communication";
        case PCI_CLASS_SYSTEM:          return "System";
        case PCI_CLASS_INPUT:           return "Input";
        case PCI_CLASS_DOCKING:         return "Docking";
        case PCI_CLASS_PROCESSOR:       return "Processor";
        case PCI_CLASS_SERIAL:          return "Serial Bus";
        case PCI_CLASS_WIRELESS:        return "Wireless";
        default:                        return "Unknown";
    }
}

const char* pci_get_vendor_name(uint16_t vendor_id) {
    switch (vendor_id) {
        case PCI_VENDOR_INTEL:      return "Intel";
        case PCI_VENDOR_AMD:        return "AMD";
        case PCI_VENDOR_NVIDIA:     return "NVIDIA";
        case PCI_VENDOR_QEMU:       return "QEMU";
        case PCI_VENDOR_VIRTIO:     return "VirtIO";
        case PCI_VENDOR_VMWARE:     return "VMware";
        case PCI_VENDOR_REDHAT:     return "Red Hat";
        default:                    return "Unknown";
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * LEER BARs
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void pci_read_bars(PCIDevice* dev) {
    for (int i = 0; i < 6; i++) {
        uint32_t bar = pci_read32(dev->bus, dev->device, dev->function, PCI_BAR0 + i * 4);
        
        if (bar == 0) {
            dev->bar[i] = 0;
            dev->bar_size[i] = 0;
            continue;
        }
        
        /* Determinar tipo de BAR */
        dev->bar_is_mmio[i] = !(bar & 1);
        
        if (dev->bar_is_mmio[i]) {
            /* Memory BAR */
            uint8_t type = (bar >> 1) & 3;
            dev->bar_is_64bit[i] = (type == 2);
            
            if (dev->bar_is_64bit[i] && i < 5) {
                /* BAR de 64 bits */
                uint32_t bar_high = pci_read32(dev->bus, dev->device, dev->function, 
                                               PCI_BAR0 + (i + 1) * 4);
                dev->bar[i] = ((uint64_t)bar_high << 32) | (bar & 0xFFFFFFF0);
                dev->bar[i + 1] = 0;
                dev->bar_is_mmio[i + 1] = false;
                i++; /* Saltar el siguiente BAR */
            } else {
                dev->bar[i] = bar & 0xFFFFFFF0;
            }
        } else {
            /* I/O BAR */
            dev->bar[i] = bar & 0xFFFFFFFC;
            dev->bar_is_64bit[i] = false;
        }
        
        /* Calcular tamaño del BAR */
        pci_config_write(dev->bus, dev->device, dev->function, PCI_BAR0 + i * 4, 0xFFFFFFFF);
        uint32_t size = pci_read32(dev->bus, dev->device, dev->function, PCI_BAR0 + i * 4);
        pci_config_write(dev->bus, dev->device, dev->function, PCI_BAR0 + i * 4, bar);
        
        if (dev->bar_is_mmio[i]) {
            size &= 0xFFFFFFF0;
        } else {
            size &= 0xFFFFFFFC;
        }
        
        dev->bar_size[i] = (~size) + 1;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ESCANEAR DISPOSITIVO
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void pci_scan_device(uint8_t bus, uint8_t device, uint8_t function) {
    uint16_t vendor_id = pci_read16(bus, device, function, PCI_VENDOR_ID);
    
    if (vendor_id == 0xFFFF) return;
    
    if (pci_device_count >= MAX_PCI_DEVICES) return;
    
    PCIDevice* dev = &pci_devices[pci_device_count++];
    
    dev->bus = bus;
    dev->device = device;
    dev->function = function;
    dev->vendor_id = vendor_id;
    dev->device_id = pci_read16(bus, device, function, PCI_DEVICE_ID);
    dev->class_code = pci_read8(bus, device, function, PCI_CLASS);
    dev->subclass = pci_read8(bus, device, function, PCI_SUBCLASS);
    dev->prog_if = pci_read8(bus, device, function, PCI_PROG_IF);
    dev->revision = pci_read8(bus, device, function, PCI_REVISION_ID);
    dev->header_type = pci_read8(bus, device, function, PCI_HEADER_TYPE);
    dev->interrupt_line = pci_read8(bus, device, function, PCI_INTERRUPT_LINE);
    
    dev->class_name = pci_get_class_name(dev->class_code);
    dev->vendor_name = pci_get_vendor_name(dev->vendor_id);
    
    pci_read_bars(dev);
}

static void pci_scan_bus(uint8_t bus) {
    for (uint8_t device = 0; device < 32; device++) {
        uint16_t vendor_id = pci_read16(bus, device, 0, PCI_VENDOR_ID);
        if (vendor_id == 0xFFFF) continue;
        
        uint8_t header_type = pci_read8(bus, device, 0, PCI_HEADER_TYPE);
        
        pci_scan_device(bus, device, 0);
        
        /* Multi-function device? */
        if (header_type & 0x80) {
            for (uint8_t func = 1; func < 8; func++) {
                pci_scan_device(bus, device, func);
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void pci_init(void) {
    pci_device_count = 0;
    pci_enumerate();
}

void pci_enumerate(void) {
    /* Escanear todos los buses */
    for (uint16_t bus = 0; bus < 256; bus++) {
        pci_scan_bus((uint8_t)bus);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * BÚSQUEDA
 * ═══════════════════════════════════════════════════════════════════════════
 */

PCIDevice* pci_find_device(uint16_t vendor_id, uint16_t device_id) {
    for (uint32_t i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].vendor_id == vendor_id && 
            pci_devices[i].device_id == device_id) {
            return &pci_devices[i];
        }
    }
    return NULL;
}

PCIDevice* pci_find_class(uint8_t class_code, uint8_t subclass) {
    for (uint32_t i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].class_code == class_code && 
            pci_devices[i].subclass == subclass) {
            return &pci_devices[i];
        }
    }
    return NULL;
}

PCIDevice* pci_find_gpu(void) {
    /* Buscar VGA compatible */
    PCIDevice* gpu = pci_find_class(PCI_CLASS_DISPLAY, PCI_SUBCLASS_VGA);
    if (gpu) return gpu;
    
    /* Buscar controlador 3D */
    gpu = pci_find_class(PCI_CLASS_DISPLAY, PCI_SUBCLASS_3D);
    if (gpu) return gpu;
    
    /* Buscar cualquier dispositivo de display */
    for (uint32_t i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].class_code == PCI_CLASS_DISPLAY) {
            return &pci_devices[i];
        }
    }
    
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * GETTERS
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t pci_get_device_count(void) {
    return pci_device_count;
}

PCIDevice* pci_get_device(uint32_t index) {
    if (index >= pci_device_count) return NULL;
    return &pci_devices[index];
}

uint64_t pci_get_bar_address(PCIDevice* dev, uint8_t bar_index) {
    if (bar_index >= 6) return 0;
    return dev->bar[bar_index];
}

uint64_t pci_get_bar_size(PCIDevice* dev, uint8_t bar_index) {
    if (bar_index >= 6) return 0;
    return dev->bar_size[bar_index];
}

bool pci_bar_is_mmio(PCIDevice* dev, uint8_t bar_index) {
    if (bar_index >= 6) return false;
    return dev->bar_is_mmio[bar_index];
}

/* ═══════════════════════════════════════════════════════════════════════════
 * DEBUG
 * ═══════════════════════════════════════════════════════════════════════════
 */

void pci_print_devices(void) {
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("\nDispositivos PCI detectados:\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    for (uint32_t i = 0; i < pci_device_count; i++) {
        PCIDevice* dev = &pci_devices[i];
        
        fb_print("  [");
        /* TODO: Imprimir bus:dev.func */
        fb_print("] ");
        fb_print(dev->vendor_name);
        fb_print(" - ");
        fb_print(dev->class_name);
        
        if (dev->class_code == PCI_CLASS_DISPLAY) {
            fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
            fb_print(" [GPU]");
            fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
        }
        
        fb_print("\n");
    }
}

