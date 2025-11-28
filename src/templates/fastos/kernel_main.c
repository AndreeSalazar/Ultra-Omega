/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Kernel Main
 * Nivel: Avanzado - Núcleo principal del sistema operativo
 * ═══════════════════════════════════════════════════════════════════════════
 * Este es el kernel principal de FastOS. Inicializa todos los subsistemas
 * y entra en el bucle principal del sistema.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "kernel.h"
#include "vga.h"
#include "idt.h"
#include "keyboard.h"
#include "timer.h"
#include "memory.h"
#include "shell.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * INFORMACIÓN DEL SISTEMA
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define FASTOS_VERSION      "1.0.0"
#define FASTOS_NAME         "FastOS"
#define FASTOS_AUTHOR       "Tu Nombre"
#define FASTOS_YEAR         "2024"

/* ═══════════════════════════════════════════════════════════════════════════
 * PROTOTIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_banner(void);
static void init_subsystems(void);
static void kernel_panic(const char* message);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIÓN PRINCIPAL DEL KERNEL
 * ═══════════════════════════════════════════════════════════════════════════
 */

void kernel_main(void) {
    /* Inicializar pantalla VGA */
    vga_init();
    vga_clear();
    
    /* Mostrar banner de bienvenida */
    print_banner();
    
    /* Inicializar subsistemas */
    init_subsystems();
    
    /* Mensaje de éxito */
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("\n[OK] FastOS iniciado correctamente!\n\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Iniciar shell */
    vga_print("Escribe 'help' para ver los comandos disponibles.\n\n");
    shell_init();
    
    /* Bucle principal del kernel */
    while (1) {
        shell_update();
        
        /* Esperar interrupción */
        __asm__ volatile("hlt");
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * BANNER DE BIENVENIDA
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void print_banner(void) {
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    
    vga_print("╔═══════════════════════════════════════════════════════════════╗\n");
    vga_print("║                                                               ║\n");
    vga_print("║   ███████╗ █████╗ ███████╗████████╗ ██████╗ ███████╗         ║\n");
    vga_print("║   ██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔════╝         ║\n");
    vga_print("║   █████╗  ███████║███████╗   ██║   ██║   ██║███████╗         ║\n");
    vga_print("║   ██╔══╝  ██╔══██║╚════██║   ██║   ██║   ██║╚════██║         ║\n");
    vga_print("║   ██║     ██║  ██║███████║   ██║   ╚██████╔╝███████║         ║\n");
    vga_print("║   ╚═╝     ╚═╝  ╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚══════╝         ║\n");
    vga_print("║                                                               ║\n");
    vga_print("║   Version: ");
    vga_print(FASTOS_VERSION);
    vga_print("                                              ║\n");
    vga_print("║   (c) ");
    vga_print(FASTOS_YEAR);
    vga_print(" ");
    vga_print(FASTOS_AUTHOR);
    vga_print("                                       ║\n");
    vga_print("║                                                               ║\n");
    vga_print("╚═══════════════════════════════════════════════════════════════╝\n\n");
    
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN DE SUBSISTEMAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void init_subsystems(void) {
    vga_print("Inicializando subsistemas...\n\n");
    
    /* IDT - Tabla de Descriptores de Interrupción */
    vga_print("  [*] Configurando IDT... ");
    idt_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Timer - Reloj del sistema */
    vga_print("  [*] Inicializando timer... ");
    timer_init(100);  /* 100 Hz */
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Teclado */
    vga_print("  [*] Inicializando teclado... ");
    keyboard_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Memoria */
    vga_print("  [*] Inicializando memoria... ");
    memory_init();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Habilitar interrupciones */
    vga_print("  [*] Habilitando interrupciones... ");
    enable_interrupts();
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print("[OK]\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * KERNEL PANIC
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void kernel_panic(const char* message) {
    disable_interrupts();
    
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_RED);
    vga_clear();
    
    vga_print("\n\n");
    vga_print("╔═══════════════════════════════════════════════════════════════╗\n");
    vga_print("║                     KERNEL PANIC                              ║\n");
    vga_print("╠═══════════════════════════════════════════════════════════════╣\n");
    vga_print("║                                                               ║\n");
    vga_print("║  Error: ");
    vga_print(message);
    vga_print("\n");
    vga_print("║                                                               ║\n");
    vga_print("║  El sistema se ha detenido.                                   ║\n");
    vga_print("║  Por favor, reinicie el equipo.                               ║\n");
    vga_print("║                                                               ║\n");
    vga_print("╚═══════════════════════════════════════════════════════════════╝\n");
    
    /* Halt infinito */
    while (1) {
        __asm__ volatile("cli; hlt");
    }
}

