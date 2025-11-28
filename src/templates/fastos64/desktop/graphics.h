/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - GRAPHICS PRIMITIVES
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: graphics.h
 * Descripción: Primitivas gráficas para el escritorio
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef GRAPHICS_H
#define GRAPHICS_H

#include "../types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    int32_t x, y;
} Point;

typedef struct {
    int32_t x, y, width, height;
} Rect;

typedef struct {
    uint32_t*   buffer;
    uint32_t    width;
    uint32_t    height;
    uint32_t    pitch;      /* bytes por línea */
} Surface;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES BÁSICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Pixel */
void gfx_put_pixel(Surface* surf, int32_t x, int32_t y, uint32_t color);
uint32_t gfx_get_pixel(Surface* surf, int32_t x, int32_t y);

/* Líneas */
void gfx_draw_line(Surface* surf, int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color);
void gfx_draw_hline(Surface* surf, int32_t x, int32_t y, int32_t width, uint32_t color);
void gfx_draw_vline(Surface* surf, int32_t x, int32_t y, int32_t height, uint32_t color);

/* Rectángulos */
void gfx_draw_rect(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);
void gfx_fill_rect(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color);

/* Rectángulos redondeados (estilo Windows 11) */
void gfx_draw_rounded_rect(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, 
                           int32_t radius, uint32_t color);
void gfx_fill_rounded_rect(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, 
                           int32_t radius, uint32_t color);

/* Círculos */
void gfx_draw_circle(Surface* surf, int32_t cx, int32_t cy, int32_t radius, uint32_t color);
void gfx_fill_circle(Surface* surf, int32_t cx, int32_t cy, int32_t radius, uint32_t color);

/* Gradientes */
void gfx_fill_gradient_v(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h,
                         uint32_t color_top, uint32_t color_bottom);
void gfx_fill_gradient_h(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h,
                         uint32_t color_left, uint32_t color_right);

/* ═══════════════════════════════════════════════════════════════════════════
 * TEXTO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_char(Surface* surf, int32_t x, int32_t y, char c, uint32_t color, int32_t scale);
void gfx_draw_string(Surface* surf, int32_t x, int32_t y, const char* str, uint32_t color, int32_t scale);
int32_t gfx_string_width(const char* str, int32_t scale);

/* ═══════════════════════════════════════════════════════════════════════════
 * ICONOS (8x8 o 16x16)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_icon_8x8(Surface* surf, int32_t x, int32_t y, const uint8_t* icon, 
                       uint32_t color, int32_t scale);
void gfx_draw_icon_16x16(Surface* surf, int32_t x, int32_t y, const uint16_t* icon, 
                         uint32_t color, int32_t scale);

/* ═══════════════════════════════════════════════════════════════════════════
 * EFECTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Blur (simplificado) */
void gfx_blur_rect(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, int32_t radius);

/* Sombra */
void gfx_draw_shadow(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, 
                     int32_t radius, int32_t offset);

/* Alpha blend */
uint32_t gfx_blend(uint32_t bg, uint32_t fg, uint8_t alpha);
void gfx_fill_rect_alpha(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h, 
                         uint32_t color, uint8_t alpha);

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t gfx_rgb(uint8_t r, uint8_t g, uint8_t b);
uint32_t gfx_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a);
void gfx_clear(Surface* surf, uint32_t color);
void gfx_blit(Surface* dest, Surface* src, int32_t x, int32_t y);
void gfx_blit_scaled(Surface* dest, Surface* src, int32_t x, int32_t y, int32_t scale);

/* Clipping */
bool gfx_clip_rect(Rect* rect, int32_t max_w, int32_t max_h);
void gfx_set_clip(Surface* surf, int32_t x, int32_t y, int32_t w, int32_t h);
void gfx_reset_clip(Surface* surf);

#endif /* GRAPHICS_H */

