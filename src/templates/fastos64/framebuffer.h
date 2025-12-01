/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - FRAMEBUFFER GRAPHICS
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: framebuffer.h
 * Descripción: Gráficos de framebuffer lineal (GOP/VESA)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef FRAMEBUFFER_H
#define FRAMEBUFFER_H

#include "types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * COLORES (32-bit ARGB)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define FB_COLOR_BLACK          0xFF000000
#define FB_COLOR_WHITE          0xFFFFFFFF
#define FB_COLOR_RED            0xFFFF0000
#define FB_COLOR_GREEN          0xFF00FF00
#define FB_COLOR_BLUE           0xFF0000FF
#define FB_COLOR_YELLOW         0xFFFFFF00
#define FB_COLOR_CYAN           0xFF00FFFF
#define FB_COLOR_MAGENTA        0xFFFF00FF
#define FB_COLOR_ORANGE         0xFFFF8800
#define FB_COLOR_GRAY           0xFF808080
#define FB_COLOR_DARK_GRAY      0xFF404040
#define FB_COLOR_LIGHT_GRAY     0xFFC0C0C0

/* Colores de Perú 🇵🇪 */
#define FB_COLOR_PERU_RED       0xFFD91023
#define FB_COLOR_PERU_WHITE     0xFFFFFFFF

/* Colores de terminal */
#define FB_COLOR_TERM_BG        0xFF1A1A2E
#define FB_COLOR_TERM_FG        0xFF00FF88
#define FB_COLOR_TERM_PROMPT    0xFFFFD700

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURA DEL FRAMEBUFFER
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    uint32_t*   buffer;         /* Puntero al framebuffer */
    uint32_t    width;          /* Ancho en píxeles */
    uint32_t    height;         /* Alto en píxeles */
    uint32_t    pitch;          /* Bytes por línea */
    uint32_t    bpp;            /* Bits por píxel */
    
    /* Cursor de texto */
    uint32_t    cursor_x;
    uint32_t    cursor_y;
    uint32_t    char_width;
    uint32_t    char_height;
    uint32_t    fg_color;
    uint32_t    bg_color;
} Framebuffer;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void fb_init(BootInfo* boot_info);
void fb_clear(uint32_t color);

/* Primitivas de dibujo */
void fb_put_pixel(uint32_t x, uint32_t y, uint32_t color);
uint32_t fb_get_pixel(uint32_t x, uint32_t y);
void fb_draw_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color);
void fb_fill_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color);
void fb_draw_line(uint32_t x0, uint32_t y0, uint32_t x1, uint32_t y1, uint32_t color);
void fb_draw_circle(uint32_t cx, uint32_t cy, uint32_t r, uint32_t color);
void fb_fill_circle(uint32_t cx, uint32_t cy, uint32_t r, uint32_t color);

/* Texto */
void fb_set_colors(uint32_t fg, uint32_t bg);
void fb_putchar(char c);
void fb_print(const char* str);
void fb_printf(const char* fmt, ...);
void fb_print_at(uint32_t x, uint32_t y, const char* str, uint32_t color);
void fb_set_cursor(uint32_t x, uint32_t y);

/* Scroll */
void fb_scroll(uint32_t lines);

/* Utilidades */
uint32_t fb_rgb(uint8_t r, uint8_t g, uint8_t b);
uint32_t fb_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);
void fb_get_size(uint32_t* width, uint32_t* height);

/* Double buffering (opcional) */
void fb_swap_buffers(void);
void fb_set_double_buffer(bool enabled);

#endif /* FRAMEBUFFER_H */

