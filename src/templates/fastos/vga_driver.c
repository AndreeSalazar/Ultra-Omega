/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - VGA Text Mode Driver
 * Nivel: Intermedio - Driver de video en modo texto
 * ═══════════════════════════════════════════════════════════════════════════
 * Este driver maneja la salida de texto en modo VGA 80x25.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vga.h"
#include "ports.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES Y VARIABLES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define VGA_WIDTH       80
#define VGA_HEIGHT      25
#define VGA_MEMORY      0xB8000

/* Puertos VGA */
#define VGA_CTRL_PORT   0x3D4
#define VGA_DATA_PORT   0x3D5

/* Buffer de video */
static uint16_t* video_memory = (uint16_t*)VGA_MEMORY;

/* Estado actual */
static uint8_t cursor_x = 0;
static uint8_t cursor_y = 0;
static uint8_t current_color = 0x07;  /* Gris claro sobre negro */

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES INTERNAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Crear entrada VGA (carácter + atributo) */
static inline uint16_t vga_entry(char c, uint8_t color) {
    return (uint16_t)c | ((uint16_t)color << 8);
}

/* Crear color VGA */
static inline uint8_t vga_color(enum vga_color fg, enum vga_color bg) {
    return fg | (bg << 4);
}

/* Actualizar posición del cursor hardware */
static void update_cursor(void) {
    uint16_t pos = cursor_y * VGA_WIDTH + cursor_x;
    
    outb(VGA_CTRL_PORT, 14);          /* Registro alto del cursor */
    outb(VGA_DATA_PORT, pos >> 8);
    outb(VGA_CTRL_PORT, 15);          /* Registro bajo del cursor */
    outb(VGA_DATA_PORT, pos & 0xFF);
}

/* Hacer scroll de una línea */
static void scroll(void) {
    if (cursor_y >= VGA_HEIGHT) {
        /* Mover todo una línea hacia arriba */
        for (int i = 0; i < (VGA_HEIGHT - 1) * VGA_WIDTH; i++) {
            video_memory[i] = video_memory[i + VGA_WIDTH];
        }
        
        /* Limpiar última línea */
        uint16_t blank = vga_entry(' ', current_color);
        for (int i = (VGA_HEIGHT - 1) * VGA_WIDTH; i < VGA_HEIGHT * VGA_WIDTH; i++) {
            video_memory[i] = blank;
        }
        
        cursor_y = VGA_HEIGHT - 1;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar driver VGA */
void vga_init(void) {
    video_memory = (uint16_t*)VGA_MEMORY;
    cursor_x = 0;
    cursor_y = 0;
    current_color = vga_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    
    /* Habilitar cursor (líneas 14-15) */
    outb(VGA_CTRL_PORT, 0x0A);
    outb(VGA_DATA_PORT, (inb(VGA_DATA_PORT) & 0xC0) | 14);
    outb(VGA_CTRL_PORT, 0x0B);
    outb(VGA_DATA_PORT, (inb(VGA_DATA_PORT) & 0xE0) | 15);
}

/* Limpiar pantalla */
void vga_clear(void) {
    uint16_t blank = vga_entry(' ', current_color);
    
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
        video_memory[i] = blank;
    }
    
    cursor_x = 0;
    cursor_y = 0;
    update_cursor();
}

/* Establecer color */
void vga_set_color(enum vga_color fg, enum vga_color bg) {
    current_color = vga_color(fg, bg);
}

/* Imprimir carácter */
void vga_putchar(char c) {
    switch (c) {
        case '\n':
            cursor_x = 0;
            cursor_y++;
            break;
            
        case '\r':
            cursor_x = 0;
            break;
            
        case '\t':
            cursor_x = (cursor_x + 8) & ~7;
            if (cursor_x >= VGA_WIDTH) {
                cursor_x = 0;
                cursor_y++;
            }
            break;
            
        case '\b':
            if (cursor_x > 0) {
                cursor_x--;
                video_memory[cursor_y * VGA_WIDTH + cursor_x] = 
                    vga_entry(' ', current_color);
            }
            break;
            
        default:
            video_memory[cursor_y * VGA_WIDTH + cursor_x] = 
                vga_entry(c, current_color);
            cursor_x++;
            
            if (cursor_x >= VGA_WIDTH) {
                cursor_x = 0;
                cursor_y++;
            }
            break;
    }
    
    scroll();
    update_cursor();
}

/* Imprimir string */
void vga_print(const char* str) {
    while (*str) {
        vga_putchar(*str++);
    }
}

/* Imprimir string con color */
void vga_print_color(const char* str, enum vga_color fg, enum vga_color bg) {
    uint8_t old_color = current_color;
    vga_set_color(fg, bg);
    vga_print(str);
    current_color = old_color;
}

/* Imprimir número hexadecimal */
void vga_print_hex(uint32_t value) {
    char hex_chars[] = "0123456789ABCDEF";
    char buffer[11] = "0x00000000";
    
    for (int i = 9; i >= 2; i--) {
        buffer[i] = hex_chars[value & 0xF];
        value >>= 4;
    }
    
    vga_print(buffer);
}

/* Imprimir número decimal */
void vga_print_dec(int32_t value) {
    char buffer[12];
    int i = 0;
    int negative = 0;
    
    if (value < 0) {
        negative = 1;
        value = -value;
    }
    
    if (value == 0) {
        vga_putchar('0');
        return;
    }
    
    while (value > 0) {
        buffer[i++] = '0' + (value % 10);
        value /= 10;
    }
    
    if (negative) {
        vga_putchar('-');
    }
    
    while (i > 0) {
        vga_putchar(buffer[--i]);
    }
}

/* Mover cursor */
void vga_move_cursor(uint8_t x, uint8_t y) {
    if (x < VGA_WIDTH && y < VGA_HEIGHT) {
        cursor_x = x;
        cursor_y = y;
        update_cursor();
    }
}

/* Obtener posición del cursor */
void vga_get_cursor(uint8_t* x, uint8_t* y) {
    *x = cursor_x;
    *y = cursor_y;
}

/* Imprimir en posición específica */
void vga_print_at(const char* str, uint8_t x, uint8_t y) {
    vga_move_cursor(x, y);
    vga_print(str);
}

/* Dibujar borde */
void vga_draw_box(uint8_t x, uint8_t y, uint8_t width, uint8_t height) {
    /* Esquinas y bordes */
    vga_move_cursor(x, y);
    vga_putchar('+');
    for (int i = 1; i < width - 1; i++) vga_putchar('-');
    vga_putchar('+');
    
    for (int j = 1; j < height - 1; j++) {
        vga_move_cursor(x, y + j);
        vga_putchar('|');
        vga_move_cursor(x + width - 1, y + j);
        vga_putchar('|');
    }
    
    vga_move_cursor(x, y + height - 1);
    vga_putchar('+');
    for (int i = 1; i < width - 1; i++) vga_putchar('-');
    vga_putchar('+');
}

