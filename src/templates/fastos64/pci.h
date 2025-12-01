/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - PCI BUS DRIVER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: pci.h
 * Descripción: Driver para enumerar y acceder al bus PCI/PCIe
 * Nota: Necesario para detectar GPUs y otros dispositivos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef PCI_H
#define PCI_H

#include "types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * PUERTOS PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PCI_CONFIG_ADDRESS      0xCF8
#define PCI_CONFIG_DATA         0xCFC

/* ═══════════════════════════════════════════════════════════════════════════
 * REGISTROS PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PCI_VENDOR_ID           0x00
#define PCI_DEVICE_ID           0x02
#define PCI_COMMAND             0x04
#define PCI_STATUS              0x06
#define PCI_REVISION_ID         0x08
#define PCI_PROG_IF             0x09
#define PCI_SUBCLASS            0x0A
#define PCI_CLASS               0x0B
#define PCI_CACHE_LINE_SIZE     0x0C
#define PCI_LATENCY_TIMER       0x0D
#define PCI_HEADER_TYPE         0x0E
#define PCI_BIST                0x0F
#define PCI_BAR0                0x10
#define PCI_BAR1                0x14
#define PCI_BAR2                0x18
#define PCI_BAR3                0x1C
#define PCI_BAR4                0x20
#define PCI_BAR5                0x24
#define PCI_INTERRUPT_LINE      0x3C
#define PCI_INTERRUPT_PIN       0x3D

/* ═══════════════════════════════════════════════════════════════════════════
 * CLASES PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PCI_CLASS_UNCLASSIFIED      0x00
#define PCI_CLASS_STORAGE           0x01
#define PCI_CLASS_NETWORK           0x02
#define PCI_CLASS_DISPLAY           0x03    /* GPUs están aquí! */
#define PCI_CLASS_MULTIMEDIA        0x04
#define PCI_CLASS_MEMORY            0x05
#define PCI_CLASS_BRIDGE            0x06
#define PCI_CLASS_COMMUNICATION     0x07
#define PCI_CLASS_SYSTEM            0x08
#define PCI_CLASS_INPUT             0x09
#define PCI_CLASS_DOCKING           0x0A
#define PCI_CLASS_PROCESSOR         0x0B
#define PCI_CLASS_SERIAL            0x0C
#define PCI_CLASS_WIRELESS          0x0D
#define PCI_CLASS_INTELLIGENT       0x0E
#define PCI_CLASS_SATELLITE         0x0F
#define PCI_CLASS_ENCRYPTION        0x10
#define PCI_CLASS_SIGNAL            0x11

/* Subclases de Display (0x03) */
#define PCI_SUBCLASS_VGA            0x00
#define PCI_SUBCLASS_XGA            0x01
#define PCI_SUBCLASS_3D             0x02
#define PCI_SUBCLASS_OTHER_DISPLAY  0x80

/* ═══════════════════════════════════════════════════════════════════════════
 * VENDOR IDs CONOCIDOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PCI_VENDOR_INTEL        0x8086
#define PCI_VENDOR_AMD          0x1022
#define PCI_VENDOR_NVIDIA       0x10DE
#define PCI_VENDOR_QEMU         0x1234
#define PCI_VENDOR_VIRTIO       0x1AF4
#define PCI_VENDOR_VMWARE       0x15AD
#define PCI_VENDOR_REDHAT       0x1B36

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURA DE DISPOSITIVO PCI
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    uint8_t     bus;
    uint8_t     device;
    uint8_t     function;
    
    uint16_t    vendor_id;
    uint16_t    device_id;
    uint8_t     class_code;
    uint8_t     subclass;
    uint8_t     prog_if;
    uint8_t     revision;
    uint8_t     header_type;
    uint8_t     interrupt_line;
    
    uint64_t    bar[6];
    uint64_t    bar_size[6];
    bool        bar_is_mmio[6];
    bool        bar_is_64bit[6];
    
    const char* class_name;
    const char* vendor_name;
} PCIDevice;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void pci_init(void);

/* Acceso a configuración PCI */
uint32_t pci_config_read(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset);
void pci_config_write(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset, uint32_t value);

uint8_t pci_read8(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset);
uint16_t pci_read16(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset);
uint32_t pci_read32(uint8_t bus, uint8_t device, uint8_t func, uint8_t offset);

/* Enumeración */
void pci_enumerate(void);
PCIDevice* pci_find_device(uint16_t vendor_id, uint16_t device_id);
PCIDevice* pci_find_class(uint8_t class_code, uint8_t subclass);
PCIDevice* pci_find_gpu(void);

/* Información */
uint32_t pci_get_device_count(void);
PCIDevice* pci_get_device(uint32_t index);
const char* pci_get_class_name(uint8_t class_code);
const char* pci_get_vendor_name(uint16_t vendor_id);

/* BAR (Base Address Register) */
uint64_t pci_get_bar_address(PCIDevice* dev, uint8_t bar_index);
uint64_t pci_get_bar_size(PCIDevice* dev, uint8_t bar_index);
bool pci_bar_is_mmio(PCIDevice* dev, uint8_t bar_index);

/* Debug */
void pci_print_devices(void);

#endif /* PCI_H */

