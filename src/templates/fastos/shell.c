/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Shell (Intérprete de Comandos)
 * Nivel: Avanzado - Interfaz de línea de comandos
 * ═══════════════════════════════════════════════════════════════════════════
 * Shell interactivo con comandos básicos para interactuar con el sistema.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "shell.h"
#include "vga.h"
#include "keyboard.h"
#include "timer.h"
#include "memory.h"
#include "string.h"
#include "ports.h"
#include "idt.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define SHELL_BUFFER_SIZE       256
#define SHELL_HISTORY_SIZE      10
#define SHELL_MAX_ARGS          16
#define SHELL_PROMPT            "FastOS> "

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Estructura de comando */
typedef struct {
    const char* name;
    const char* description;
    void (*handler)(int argc, char** argv);
} shell_command_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static char input_buffer[SHELL_BUFFER_SIZE];
static int input_pos = 0;

static char history[SHELL_HISTORY_SIZE][SHELL_BUFFER_SIZE];
static int history_count = 0;
static int history_index = 0;

/* ═══════════════════════════════════════════════════════════════════════════
 * PROTOTIPOS DE COMANDOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void cmd_help(int argc, char** argv);
static void cmd_clear(int argc, char** argv);
static void cmd_echo(int argc, char** argv);
static void cmd_uptime(int argc, char** argv);
static void cmd_memory(int argc, char** argv);
static void cmd_reboot(int argc, char** argv);
static void cmd_shutdown(int argc, char** argv);
static void cmd_version(int argc, char** argv);
static void cmd_color(int argc, char** argv);
static void cmd_beep(int argc, char** argv);
static void cmd_calc(int argc, char** argv);
static void cmd_date(int argc, char** argv);

/* ═══════════════════════════════════════════════════════════════════════════
 * TABLA DE COMANDOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static const shell_command_t commands[] = {
    {"help",     "Muestra esta ayuda",                    cmd_help},
    {"clear",    "Limpia la pantalla",                    cmd_clear},
    {"cls",      "Limpia la pantalla (alias)",            cmd_clear},
    {"echo",     "Muestra texto en pantalla",             cmd_echo},
    {"uptime",   "Muestra tiempo de ejecucion",           cmd_uptime},
    {"memory",   "Muestra estadisticas de memoria",       cmd_memory},
    {"mem",      "Muestra estadisticas de memoria",       cmd_memory},
    {"reboot",   "Reinicia el sistema",                   cmd_reboot},
    {"shutdown", "Apaga el sistema",                      cmd_shutdown},
    {"version",  "Muestra version del sistema",           cmd_version},
    {"ver",      "Muestra version del sistema (alias)",   cmd_version},
    {"color",    "Cambia el color del texto",             cmd_color},
    {"beep",     "Emite un sonido",                       cmd_beep},
    {"calc",     "Calculadora simple",                    cmd_calc},
    {"date",     "Muestra fecha y hora",                  cmd_date},
    {NULL, NULL, NULL}
};

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES INTERNAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Parsear línea de comando en argumentos */
static int parse_command(char* line, char** argv) {
    int argc = 0;
    char* ptr = line;
    int in_quotes = 0;
    
    while (*ptr && argc < SHELL_MAX_ARGS) {
        /* Saltar espacios */
        while (*ptr == ' ') ptr++;
        
        if (*ptr == '\0') break;
        
        /* Manejar comillas */
        if (*ptr == '"') {
            in_quotes = 1;
            ptr++;
            argv[argc++] = ptr;
            while (*ptr && *ptr != '"') ptr++;
        } else {
            argv[argc++] = ptr;
            while (*ptr && *ptr != ' ') ptr++;
        }
        
        if (*ptr) {
            *ptr = '\0';
            ptr++;
        }
    }
    
    return argc;
}

/* Ejecutar comando */
static void execute_command(char* line) {
    char* argv[SHELL_MAX_ARGS];
    int argc = parse_command(line, argv);
    
    if (argc == 0) return;
    
    /* Buscar comando */
    for (int i = 0; commands[i].name != NULL; i++) {
        if (strcmp(argv[0], commands[i].name) == 0) {
            commands[i].handler(argc, argv);
            return;
        }
    }
    
    /* Comando no encontrado */
    vga_set_color(VGA_COLOR_LIGHT_RED, VGA_COLOR_BLACK);
    vga_print("Comando no reconocido: ");
    vga_print(argv[0]);
    vga_print("\nEscribe 'help' para ver los comandos disponibles.\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
}

/* Añadir línea al historial */
static void add_to_history(const char* line) {
    if (strlen(line) == 0) return;
    
    /* Copiar al historial */
    strcpy(history[history_count % SHELL_HISTORY_SIZE], line);
    history_count++;
    history_index = history_count;
}

/* Mostrar prompt */
static void show_prompt(void) {
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print(SHELL_PROMPT);
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * IMPLEMENTACIÓN DE COMANDOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void cmd_help(int argc, char** argv) {
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("\n=== Comandos Disponibles ===\n\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    for (int i = 0; commands[i].name != NULL; i++) {
        /* Evitar mostrar alias */
        if (i > 0 && commands[i].handler == commands[i-1].handler) {
            continue;
        }
        
        vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
        vga_print("  ");
        vga_print(commands[i].name);
        
        /* Padding */
        int len = strlen(commands[i].name);
        for (int j = len; j < 12; j++) vga_print(" ");
        
        vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
        vga_print("- ");
        vga_print(commands[i].description);
        vga_print("\n");
    }
    vga_print("\n");
}

static void cmd_clear(int argc, char** argv) {
    vga_clear();
}

static void cmd_echo(int argc, char** argv) {
    for (int i = 1; i < argc; i++) {
        vga_print(argv[i]);
        if (i < argc - 1) vga_print(" ");
    }
    vga_print("\n");
}

static void cmd_uptime(int argc, char** argv) {
    char buffer[16];
    timer_format_uptime(buffer, sizeof(buffer));
    
    vga_print("Tiempo de ejecucion: ");
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print(buffer);
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print("\n");
}

static void cmd_memory(int argc, char** argv) {
    memory_print_stats();
}

static void cmd_reboot(int argc, char** argv) {
    vga_print("Reiniciando sistema...\n");
    timer_wait_ms(1000);
    
    /* Reiniciar usando el controlador de teclado */
    outb(0x64, 0xFE);
    
    /* Si eso falla, triple fault */
    __asm__ volatile("int $0");
}

static void cmd_shutdown(int argc, char** argv) {
    vga_set_color(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
    vga_print("\n");
    vga_print("╔══════════════════════════════════════╗\n");
    vga_print("║  El sistema se ha detenido.          ║\n");
    vga_print("║  Es seguro apagar el equipo.         ║\n");
    vga_print("╚══════════════════════════════════════╝\n");
    
    disable_interrupts();
    while (1) {
        __asm__ volatile("hlt");
    }
}

static void cmd_version(int argc, char** argv) {
    vga_set_color(VGA_COLOR_LIGHT_CYAN, VGA_COLOR_BLACK);
    vga_print("\nFastOS Version 1.0.0\n");
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print("(c) 2024 Tu Nombre\n");
    vga_print("Kernel de 32 bits para x86\n\n");
}

static void cmd_color(int argc, char** argv) {
    if (argc < 2) {
        vga_print("Uso: color <0-15>\n");
        vga_print("Colores: 0=negro, 1=azul, 2=verde, ..., 15=blanco\n");
        return;
    }
    
    int color = argv[1][0] - '0';
    if (argv[1][1] >= '0' && argv[1][1] <= '9') {
        color = color * 10 + (argv[1][1] - '0');
    }
    
    if (color >= 0 && color <= 15) {
        vga_set_color(color, VGA_COLOR_BLACK);
        vga_print("Color cambiado.\n");
    } else {
        vga_print("Color invalido. Use 0-15.\n");
    }
}

static void cmd_beep(int argc, char** argv) {
    uint32_t freq = 1000;
    uint32_t duration = 200;
    
    if (argc >= 2) {
        freq = 0;
        for (int i = 0; argv[1][i]; i++) {
            freq = freq * 10 + (argv[1][i] - '0');
        }
    }
    
    if (argc >= 3) {
        duration = 0;
        for (int i = 0; argv[2][i]; i++) {
            duration = duration * 10 + (argv[2][i] - '0');
        }
    }
    
    vga_print("Beep: ");
    vga_print_dec(freq);
    vga_print(" Hz, ");
    vga_print_dec(duration);
    vga_print(" ms\n");
    
    timer_beep(freq, duration);
}

static void cmd_calc(int argc, char** argv) {
    if (argc != 4) {
        vga_print("Uso: calc <num1> <op> <num2>\n");
        vga_print("Operadores: + - * /\n");
        return;
    }
    
    int a = 0, b = 0;
    int sign_a = 1, sign_b = 1;
    int i = 0;
    
    /* Parsear primer número */
    if (argv[1][0] == '-') { sign_a = -1; i = 1; }
    for (; argv[1][i]; i++) {
        a = a * 10 + (argv[1][i] - '0');
    }
    a *= sign_a;
    
    /* Parsear segundo número */
    i = 0;
    if (argv[3][0] == '-') { sign_b = -1; i = 1; }
    for (; argv[3][i]; i++) {
        b = b * 10 + (argv[3][i] - '0');
    }
    b *= sign_b;
    
    int result = 0;
    char op = argv[2][0];
    
    switch (op) {
        case '+': result = a + b; break;
        case '-': result = a - b; break;
        case '*': result = a * b; break;
        case '/':
            if (b == 0) {
                vga_print("Error: Division por cero\n");
                return;
            }
            result = a / b;
            break;
        default:
            vga_print("Operador desconocido\n");
            return;
    }
    
    vga_print_dec(a);
    vga_print(" ");
    vga_putchar(op);
    vga_print(" ");
    vga_print_dec(b);
    vga_print(" = ");
    vga_set_color(VGA_COLOR_LIGHT_GREEN, VGA_COLOR_BLACK);
    vga_print_dec(result);
    vga_set_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_print("\n");
}

static void cmd_date(int argc, char** argv) {
    /* Leer RTC (Real Time Clock) */
    uint8_t second, minute, hour, day, month, year;
    
    outb(0x70, 0x00);
    second = inb(0x71);
    outb(0x70, 0x02);
    minute = inb(0x71);
    outb(0x70, 0x04);
    hour = inb(0x71);
    outb(0x70, 0x07);
    day = inb(0x71);
    outb(0x70, 0x08);
    month = inb(0x71);
    outb(0x70, 0x09);
    year = inb(0x71);
    
    /* Convertir BCD a binario */
    second = (second & 0x0F) + ((second >> 4) * 10);
    minute = (minute & 0x0F) + ((minute >> 4) * 10);
    hour = (hour & 0x0F) + ((hour >> 4) * 10);
    day = (day & 0x0F) + ((day >> 4) * 10);
    month = (month & 0x0F) + ((month >> 4) * 10);
    year = (year & 0x0F) + ((year >> 4) * 10);
    
    vga_print("Fecha: ");
    vga_print_dec(day);
    vga_print("/");
    vga_print_dec(month);
    vga_print("/20");
    vga_print_dec(year);
    vga_print("  Hora: ");
    vga_print_dec(hour);
    vga_print(":");
    if (minute < 10) vga_print("0");
    vga_print_dec(minute);
    vga_print(":");
    if (second < 10) vga_print("0");
    vga_print_dec(second);
    vga_print("\n");
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar shell */
void shell_init(void) {
    input_pos = 0;
    input_buffer[0] = '\0';
    history_count = 0;
    history_index = 0;
    
    show_prompt();
}

/* Actualizar shell (llamar en el bucle principal) */
void shell_update(void) {
    int c = keyboard_getchar_nonblock();
    
    if (c < 0) return;
    
    char ch = (char)c;
    
    switch (ch) {
        case '\n':
            /* Enter - ejecutar comando */
            vga_print("\n");
            
            if (input_pos > 0) {
                input_buffer[input_pos] = '\0';
                add_to_history(input_buffer);
                execute_command(input_buffer);
            }
            
            input_pos = 0;
            input_buffer[0] = '\0';
            show_prompt();
            break;
            
        case '\b':
            /* Backspace */
            if (input_pos > 0) {
                input_pos--;
                vga_putchar('\b');
            }
            break;
            
        default:
            /* Carácter normal */
            if (input_pos < SHELL_BUFFER_SIZE - 1 && ch >= 32 && ch < 127) {
                input_buffer[input_pos++] = ch;
                vga_putchar(ch);
            }
            break;
    }
}

