/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - KERNEL MAIN
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: kernel_main64.c
 * Descripción: Kernel personalizado para el sistema de Eddi Andreé
 * 
 * HARDWARE DETECTADO:
 *   - CPU: AMD Ryzen 5 5600X (6 cores / 12 threads)
 *   - GPU: NVIDIA GeForce RTX 3060 12GB
 *   - RAM: 16 GB DDR4
 *   - Storage: 1 TB NVMe
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "types64.h"
#include "framebuffer.h"
#include "idt64.h"
#include "memory64.h"
#include "keyboard64.h"
#include "pci.h"
#include "shell64.h"
#include "ports64.h"

#define FASTOS_VERSION      "2.0.0"
#define FASTOS_NAME         "FastOS 64-bit"
#define FASTOS_AUTHOR       "Eddi Andree Salazar Matos"
#define FASTOS_YEAR         "2024"

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN DEL HARDWARE DE EDDI
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define EDDI_CPU_NAME       "AMD Ryzen 5 5600X"
#define EDDI_CPU_CORES      6
#define EDDI_CPU_THREADS    12
#define EDDI_CPU_BASE_MHZ   3700
#define EDDI_CPU_BOOST_MHZ  4600

#define EDDI_GPU_NAME       "NVIDIA GeForce RTX 3060"
#define EDDI_GPU_VRAM_GB    12
#define EDDI_GPU_CUDA_CORES 3584

#define EDDI_RAM_GB         16
#define EDDI_STORAGE_GB     1000

/* Vendor IDs */
#define PCI_VENDOR_NVIDIA   0x10DE
#define PCI_VENDOR_AMD_GPU  0x1002

/* Device IDs conocidos */
#define NVIDIA_RTX_3060     0x2503  /* GA106 */
#define NVIDIA_RTX_3060_ALT 0x2504

/* ═══════════════════════════════════════════════════════════════════════════
 * PROTOTIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_banner(void);
static void print_peru_flag(uint32_t x, uint32_t y, uint32_t scale);
static void init_subsystems(BootInfo* boot_info);
static void print_eddi_hardware(void);
static void detect_cpu_info(void);
static void detect_gpu_info(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * KERNEL MAIN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void kernel_main(BootInfo* boot_info) {
    /* Inicializar framebuffer primero */
    fb_init(boot_info);
    fb_clear(FB_COLOR_TERM_BG);
    
    /* Mostrar banner */
    print_banner();
    
    /* Inicializar subsistemas */
    init_subsystems(boot_info);
    
    /* Mostrar hardware de Eddi */
    print_eddi_hardware();
    
    /* Mensaje de éxito */
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("\n[OK] FastOS 64-bit iniciado en el sistema de Eddi!\n\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("Escribe 'help' para ver los comandos disponibles.\n\n");
    
    /* Iniciar shell */
    shell_init();
    
    /* Loop principal */
    while (1) {
        shell_update();
        __asm__ volatile("hlt");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * BANNER PERSONALIZADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_banner(void) {
    fb_print("\n");
    
    /* Logo ASCII de FastOS */
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("   ########    ###     ######  ########  #######   ######  \n");
    fb_print("   ##         ## ##   ##    ##    ##    ##     ## ##    ## \n");
    fb_print("   ##        ##   ##  ##          ##    ##     ## ##       \n");
    fb_print("   ######   ##     ##  ######     ##    ##     ##  ######  \n");
    fb_print("   ##       #########       ##    ##    ##     ##       ## \n");
    fb_print("   ##       ##     ## ##    ##    ##    ##     ## ##    ## \n");
    fb_print("   ##       ##     ##  ######     ##     #######   ######  \n");
    fb_print("\n");
    
    /* Subtítulo */
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("                      [ 64-BIT EDITION ]\n");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("                   Sistema Personal de Eddi\n\n");
    
    /* Línea decorativa con colores de Perú */
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("   =====");
    fb_set_colors(FB_COLOR_PERU_WHITE, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print(" Ultra-Omega Project ");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_WHITE, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====\n\n");
    
    /* Versión */
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    fb_print("                        Version ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print(FASTOS_VERSION);
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    fb_print(" | ");
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("x86_64 Long Mode\n\n");
    
    /* Dibujar bandera de Perú gráficamente */
    print_peru_flag(24, 180, 4);
    
    /* Info del desarrollador (al lado de la bandera) */
    fb_print_at(120, 180, "Desarrollado con <3 por:", FB_COLOR_CYAN);
    fb_print_at(120, 200, FASTOS_AUTHOR, FB_COLOR_YELLOW);
    fb_print_at(120, 220, "Desarrollador Peruano", FB_COLOR_GREEN);
    fb_print_at(120, 240, "Ultra-Omega Project | " FASTOS_YEAR, FB_COLOR_GRAY);
    
    /* Mover cursor debajo de la bandera */
    fb_set_cursor(0, 18);
    
    /* Línea inferior */
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("   =====");
    fb_set_colors(FB_COLOR_PERU_WHITE, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("=====================");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_WHITE, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====\n\n");
    
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * BANDERA DE PERÚ (Gráfica)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_peru_flag(uint32_t x, uint32_t y, uint32_t scale) {
    uint32_t stripe_width = 12 * scale;
    uint32_t flag_height = 24 * scale;
    
    /* Franja roja izquierda */
    fb_fill_rect(x, y, stripe_width, flag_height, FB_COLOR_PERU_RED);
    
    /* Franja blanca central */
    fb_fill_rect(x + stripe_width, y, stripe_width, flag_height, FB_COLOR_PERU_WHITE);
    
    /* Franja roja derecha */
    fb_fill_rect(x + stripe_width * 2, y, stripe_width, flag_height, FB_COLOR_PERU_RED);
    
    /* Borde negro */
    fb_draw_rect(x, y, stripe_width * 3, flag_height, FB_COLOR_BLACK);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN DE SUBSISTEMAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void init_subsystems(BootInfo* boot_info) {
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    fb_print("Inicializando subsistemas de 64 bits...\n\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* IDT */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Configurando IDT (64-bit).... ");
    idt64_init();
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Memoria */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Inicializando memoria........ ");
    memory64_init(boot_info);
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Teclado */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Inicializando teclado........ ");
    keyboard64_init();
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* PCI - Detectar GPU */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Escaneando bus PCI........... ");
    pci_init();
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Detectar RTX 3060 */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Detectando RTX 3060.......... ");
    PCIDevice* gpu = pci_find_device(PCI_VENDOR_NVIDIA, NVIDIA_RTX_3060);
    if (!gpu) {
        gpu = pci_find_device(PCI_VENDOR_NVIDIA, NVIDIA_RTX_3060_ALT);
    }
    if (!gpu) {
        gpu = pci_find_gpu(); /* Buscar cualquier GPU */
    }
    if (gpu) {
        fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
        fb_print("[ENCONTRADA]\n");
    } else {
        fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
        fb_print("[QEMU/VirtIO]\n");
    }
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Interrupciones */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Habilitando interrupciones... ");
    __asm__ volatile("sti");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * DETECTAR CPU (CPUID)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void detect_cpu_info(void) {
    uint32_t eax, ebx, ecx, edx;
    char cpu_brand[49];
    
    /* Obtener marca del CPU usando CPUID */
    cpuid(0x80000002, &eax, &ebx, &ecx, &edx);
    *((uint32_t*)&cpu_brand[0]) = eax;
    *((uint32_t*)&cpu_brand[4]) = ebx;
    *((uint32_t*)&cpu_brand[8]) = ecx;
    *((uint32_t*)&cpu_brand[12]) = edx;
    
    cpuid(0x80000003, &eax, &ebx, &ecx, &edx);
    *((uint32_t*)&cpu_brand[16]) = eax;
    *((uint32_t*)&cpu_brand[20]) = ebx;
    *((uint32_t*)&cpu_brand[24]) = ecx;
    *((uint32_t*)&cpu_brand[28]) = edx;
    
    cpuid(0x80000004, &eax, &ebx, &ecx, &edx);
    *((uint32_t*)&cpu_brand[32]) = eax;
    *((uint32_t*)&cpu_brand[36]) = ebx;
    *((uint32_t*)&cpu_brand[40]) = ecx;
    *((uint32_t*)&cpu_brand[44]) = edx;
    
    cpu_brand[48] = '\0';
    
    fb_print("  CPU: ");
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    fb_print(cpu_brand);
    fb_print("\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MOSTRAR HARDWARE DE EDDI
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_eddi_hardware(void) {
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("╔═══════════════════════════════════════════════════════════════╗\n");
    fb_print("║              SISTEMA PERSONAL DE EDDI                         ║\n");
    fb_print("╠═══════════════════════════════════════════════════════════════╣\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* CPU - Detectar con CPUID */
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    detect_cpu_info();
    
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print("       Cores: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("6");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" | Threads: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("12");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" | Base: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("3.7 GHz");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" | Boost: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("4.6 GHz\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* GPU */
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print("  GPU: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print(EDDI_GPU_NAME);
    fb_print("\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print("       VRAM: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("12 GB GDDR6");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" | CUDA Cores: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("3584\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* RAM */
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print("  RAM: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("16 GB DDR4\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Storage */
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("║ ");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print("  SSD: ");
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("1 TB NVMe\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("╚═══════════════════════════════════════════════════════════════╝\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
}
