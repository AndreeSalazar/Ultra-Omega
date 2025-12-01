/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - VGA Header
 * Nivel: Intermedio - Definiciones del driver VGA
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef VGA_H
#define VGA_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * COLORES VGA
 * ═══════════════════════════════════════════════════════════════════════════
 */

enum vga_color {
    VGA_COLOR_BLACK         = 0,
    VGA_COLOR_BLUE          = 1,
    VGA_COLOR_GREEN         = 2,
    VGA_COLOR_CYAN          = 3,
    VGA_COLOR_RED           = 4,
    VGA_COLOR_MAGENTA       = 5,
    VGA_COLOR_BROWN         = 6,
    VGA_COLOR_LIGHT_GREY    = 7,
    VGA_COLOR_DARK_GREY     = 8,
    VGA_COLOR_LIGHT_BLUE    = 9,
    VGA_COLOR_LIGHT_GREEN   = 10,
    VGA_COLOR_LIGHT_CYAN    = 11,
    VGA_COLOR_LIGHT_RED     = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_YELLOW        = 14,
    VGA_COLOR_WHITE         = 15
};

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void vga_init(void);
void vga_clear(void);

/* Colores */
void vga_set_color(enum vga_color fg, enum vga_color bg);

/* Impresión */
void vga_putchar(char c);
void vga_print(const char* str);
void vga_print_color(const char* str, enum vga_color fg, enum vga_color bg);
void vga_print_hex(uint32_t value);
void vga_print_dec(int32_t value);

/* Cursor */
void vga_move_cursor(uint8_t x, uint8_t y);
void vga_get_cursor(uint8_t* x, uint8_t* y);

/* Utilidades */
void vga_print_at(const char* str, uint8_t x, uint8_t y);
void vga_draw_box(uint8_t x, uint8_t y, uint8_t width, uint8_t height);

#endif /* VGA_H */

