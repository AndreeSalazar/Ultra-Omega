/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - KERNEL MAIN
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: kernel_main64.c
 * Descripción: Punto de entrada principal del kernel de 64 bits
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "types64.h"
#include "framebuffer.h"
#include "idt64.h"
#include "memory64.h"
#include "keyboard64.h"
#include "pci.h"
#include "shell64.h"

#define FASTOS_VERSION      "2.0.0"
#define FASTOS_NAME         "FastOS 64-bit"
#define FASTOS_AUTHOR       "Eddi Andree Salazar Matos"
#define FASTOS_YEAR         "2024"

/* ═══════════════════════════════════════════════════════════════════════════
 * PROTOTIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_banner(void);
static void print_peru_flag(uint32_t x, uint32_t y, uint32_t scale);
static void init_subsystems(BootInfo* boot_info);
static void print_system_info(BootInfo* boot_info);

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
    
    /* Mostrar información del sistema */
    print_system_info(boot_info);
    
    /* Mensaje de éxito */
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("\n[OK] FastOS 64-bit iniciado correctamente!\n\n");
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
 * BANNER
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
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("                      ");
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("[ 64-BIT EDITION ]\n\n");
    
    /* Línea decorativa */
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("   =====");
    fb_set_colors(FB_COLOR_PERU_WHITE, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_PERU_RED, FB_COLOR_TERM_BG);
    fb_print("=====");
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print(" Sistema Operativo de 64 bits ");
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
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
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
    fb_print("==========================");
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
    
    /* PCI */
    fb_print("  ");
    fb_set_colors(FB_COLOR_CYAN, FB_COLOR_TERM_BG);
    fb_print("[*]");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    fb_print(" Escaneando bus PCI........... ");
    pci_init();
    fb_set_colors(FB_COLOR_GREEN, FB_COLOR_TERM_BG);
    fb_print("[OK]\n");
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
 * INFORMACIÓN DEL SISTEMA
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_system_info(BootInfo* boot_info) {
    fb_set_colors(FB_COLOR_YELLOW, FB_COLOR_TERM_BG);
    fb_print("Sistema:\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Resolución */
    fb_print("  Resolucion: ");
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    /* TODO: Implementar fb_printf */
    fb_print("(ver boot_info)\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Memoria */
    fb_print("  Memoria total: ");
    fb_set_colors(FB_COLOR_WHITE, FB_COLOR_TERM_BG);
    uint64_t mem_mb = boot_info->total_memory / (1024 * 1024);
    /* TODO: Implementar conversión a string */
    fb_print("(ver boot_info) MB\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    /* Arquitectura */
    fb_print("  Arquitectura: ");
    fb_set_colors(FB_COLOR_MAGENTA, FB_COLOR_TERM_BG);
    fb_print("x86_64 (Long Mode)\n");
    fb_set_colors(FB_COLOR_TERM_FG, FB_COLOR_TERM_BG);
    
    fb_print("\n");
    
    UNUSED(mem_mb);
}

