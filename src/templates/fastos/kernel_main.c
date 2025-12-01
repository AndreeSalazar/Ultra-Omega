/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Kernel Main
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "kernel.h"
#include "vga.h"
#include "idt.h"
#include "keyboard.h"
#include "timer.h"
#include "memory.h"
#include "shell.h"

#define FASTOS_VERSION      "1.0.0"
#define FASTOS_NAME         "FastOS"
#define FASTOS_AUTHOR       "Eddi Andree Salazar Matos"
#define FASTOS_YEAR         "2024"

static void print_banner(void);
static void print_peru_flag(int row);
static void init_subsystems(void);

void kernel_main(void) {
    vga_init();
    vga_clear();
    
    print_banner();
    init_subsystems();
    
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK] FastOS iniciado correctamente!\n\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("Escribe 'help' para ver los comandos disponibles.\n\n");
    shell_init();
    
    while (1) {
        shell_update();
        __asm__ volatile("hlt");
    }
}

/* Imprimir una fila de la bandera de Peru (3 filas en total) */
static void print_peru_flag(int row) {
    int i;
    (void)row; /* Todas las filas son iguales */
    
    /* Franja roja izquierda */
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_RED);
    for (i = 0; i < 4; i++) vga_putchar(' ');
    
    /* Franja blanca central */
    vga_set_color(VGA_COLOR_RED, VGA_COLOR_WHITE);
    for (i = 0; i < 4; i++) vga_putchar(' ');
    
    /* Franja roja derecha */
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_RED);
    for (i = 0; i < 4; i++) vga_putchar(' ');
    
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

static void print_banner(void) {
    vga_print("\n");
    
    /* ========== LOGO FASTOS ========== */
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("   ########    ###     ######  ########  #######   ######  \n");
    vga_print("   ##         ## ##   ##    ##    ##    ##     ## ##    ## \n");
    vga_print("   ##        ##   ##  ##          ##    ##     ## ##       \n");
    vga_print("   ######   ##     ##  ######     ##    ##     ##  ######  \n");
    vga_print("   ##       #########       ##    ##    ##     ##       ## \n");
    vga_print("   ##       ##     ## ##    ##    ##    ##     ## ##    ## \n");
    vga_print("   ##       ##     ##  ######     ##     #######   ######  \n");
    vga_print("\n");
    
    /* Linea decorativa superior */
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print("   ");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print(" Sistema Operativo Educativo ");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====\n\n");
    
    /* Version centrada */
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("                        Version ");
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print(FASTOS_VERSION);
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print(" | ");
    vga_set_color(VGA_COLOR_LIGHT_MAGENTA, VGA_COLOR_BLACK);
    vga_print("x86 32-bit");
    vga_print("\n\n");
    
    /* ========== SECCION DESARROLLADOR CON BANDERA ========== */
    
    /* Fila 1 de bandera + info */
    vga_print("   ");
    print_peru_flag(0);
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("Desarrollado con ");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("<3");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print(" por:\n");
    
    /* Fila 2 de bandera + nombre */
    vga_print("   ");
    print_peru_flag(1);
    vga_print("  ");
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print("Eddi Andree Salazar Matos\n");
    
    /* Fila 3 de bandera + titulo */
    vga_print("   ");
    print_peru_flag(2);
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("Desarrollador Peruano\n");
    
    /* Fila 4 - info adicional */
    vga_print("   ");
    print_peru_flag(3);
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print("Ultra-Omega Project | ");
    vga_set_color(VGA_COLOR_CYAN, VGA_COLOR_BLACK);
    vga_print(FASTOS_YEAR);
    vga_print("\n\n");
    
    /* Linea decorativa inferior */
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("   =====");
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print("==========================");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("=====");
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("=====\n\n");
    
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

static void init_subsystems(void) {
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    vga_print("Inicializando subsistemas...\n\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("[*]");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print(" Configurando IDT............. ");
    idt_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("[*]");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print(" Inicializando timer.......... ");
    timer_init(100);
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("[*]");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print(" Inicializando teclado........ ");
    keyboard_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("[*]");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print(" Inicializando memoria........ ");
    memory_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    vga_print("  ");
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("[*]");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print(" Habilitando interrupciones... ");
    enable_interrupts();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}
