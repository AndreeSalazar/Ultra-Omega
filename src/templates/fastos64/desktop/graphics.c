/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - GRAPHICS PRIMITIVES IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: graphics.c
 * Descripción: Primitivas gráficas para el desktop
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "graphics.h"
#include "../font8x16.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static GraphicsContext gfx = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_init(uint32_t* framebuffer, uint32_t width, uint32_t height, uint32_t pitch) {
    gfx.framebuffer = framebuffer;
    gfx.width = width;
    gfx.height = height;
    gfx.pitch = pitch;
    
    /* Crear back buffer */
    gfx.back_buffer = framebuffer; /* Por ahora usar el mismo */
}

void gfx_shutdown(void) {
    /* Nada que liberar por ahora */
}

void gfx_swap_buffers(void) {
    /* TODO: Implementar double buffering real */
}

/* ═══════════════════════════════════════════════════════════════════════════
 * PRIMITIVAS BÁSICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_put_pixel(int32_t x, int32_t y, uint32_t color) {
    if (x < 0 || x >= (int32_t)gfx.width || y < 0 || y >= (int32_t)gfx.height) return;
    gfx.framebuffer[y * gfx.width + x] = color;
}

uint32_t gfx_get_pixel(int32_t x, int32_t y) {
    if (x < 0 || x >= (int32_t)gfx.width || y < 0 || y >= (int32_t)gfx.height) return 0;
    return gfx.framebuffer[y * gfx.width + x];
}

void gfx_clear(uint32_t color) {
    uint32_t pixels = gfx.width * gfx.height;
    for (uint32_t i = 0; i < pixels; i++) {
        gfx.framebuffer[i] = color;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * LÍNEAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_line(int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t color) {
    int32_t dx = x1 - x0;
    int32_t dy = y1 - y0;
    int32_t sx = dx > 0 ? 1 : -1;
    int32_t sy = dy > 0 ? 1 : -1;
    
    dx = dx > 0 ? dx : -dx;
    dy = dy > 0 ? dy : -dy;
    
    int32_t err = (dx > dy ? dx : -dy) / 2;
    int32_t e2;
    
    while (1) {
        gfx_put_pixel(x0, y0, color);
        
        if (x0 == x1 && y0 == y1) break;
        
        e2 = err;
        if (e2 > -dx) { err -= dy; x0 += sx; }
        if (e2 < dy) { err += dx; y0 += sy; }
    }
}

void gfx_draw_hline(int32_t x, int32_t y, int32_t width, uint32_t color) {
    for (int32_t i = 0; i < width; i++) {
        gfx_put_pixel(x + i, y, color);
    }
}

void gfx_draw_vline(int32_t x, int32_t y, int32_t height, uint32_t color) {
    for (int32_t i = 0; i < height; i++) {
        gfx_put_pixel(x, y + i, color);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * RECTÁNGULOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color) {
    gfx_draw_hline(x, y, w, color);
    gfx_draw_hline(x, y + h - 1, w, color);
    gfx_draw_vline(x, y, h, color);
    gfx_draw_vline(x + w - 1, y, h, color);
}

void gfx_fill_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t color) {
    for (int32_t j = 0; j < h; j++) {
        for (int32_t i = 0; i < w; i++) {
            gfx_put_pixel(x + i, y + j, color);
        }
    }
}

void gfx_draw_rounded_rect(int32_t x, int32_t y, int32_t w, int32_t h, 
                           int32_t radius, uint32_t color) {
    /* Bordes horizontales */
    gfx_draw_hline(x + radius, y, w - 2*radius, color);
    gfx_draw_hline(x + radius, y + h - 1, w - 2*radius, color);
    
    /* Bordes verticales */
    gfx_draw_vline(x, y + radius, h - 2*radius, color);
    gfx_draw_vline(x + w - 1, y + radius, h - 2*radius, color);
    
    /* Esquinas */
    gfx_draw_corner(x + radius, y + radius, radius, 0, color);
    gfx_draw_corner(x + w - radius - 1, y + radius, radius, 1, color);
    gfx_draw_corner(x + radius, y + h - radius - 1, radius, 2, color);
    gfx_draw_corner(x + w - radius - 1, y + h - radius - 1, radius, 3, color);
}

void gfx_fill_rounded_rect(int32_t x, int32_t y, int32_t w, int32_t h, 
                           int32_t radius, uint32_t color) {
    /* Centro */
    gfx_fill_rect(x + radius, y, w - 2*radius, h, color);
    gfx_fill_rect(x, y + radius, radius, h - 2*radius, color);
    gfx_fill_rect(x + w - radius, y + radius, radius, h - 2*radius, color);
    
    /* Esquinas */
    gfx_fill_corner(x + radius, y + radius, radius, 0, color);
    gfx_fill_corner(x + w - radius - 1, y + radius, radius, 1, color);
    gfx_fill_corner(x + radius, y + h - radius - 1, radius, 2, color);
    gfx_fill_corner(x + w - radius - 1, y + h - radius - 1, radius, 3, color);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CÍRCULOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_circle(int32_t cx, int32_t cy, int32_t radius, uint32_t color) {
    int32_t x = radius;
    int32_t y = 0;
    int32_t err = 0;
    
    while (x >= y) {
        gfx_put_pixel(cx + x, cy + y, color);
        gfx_put_pixel(cx + y, cy + x, color);
        gfx_put_pixel(cx - y, cy + x, color);
        gfx_put_pixel(cx - x, cy + y, color);
        gfx_put_pixel(cx - x, cy - y, color);
        gfx_put_pixel(cx - y, cy - x, color);
        gfx_put_pixel(cx + y, cy - x, color);
        gfx_put_pixel(cx + x, cy - y, color);
        
        if (err <= 0) {
            y++;
            err += 2*y + 1;
        }
        if (err > 0) {
            x--;
            err -= 2*x + 1;
        }
    }
}

void gfx_fill_circle(int32_t cx, int32_t cy, int32_t radius, uint32_t color) {
    for (int32_t y = -radius; y <= radius; y++) {
        for (int32_t x = -radius; x <= radius; x++) {
            if (x*x + y*y <= radius*radius) {
                gfx_put_pixel(cx + x, cy + y, color);
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * ESQUINAS REDONDEADAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_corner(int32_t cx, int32_t cy, int32_t radius, int32_t corner, uint32_t color) {
    int32_t x = radius;
    int32_t y = 0;
    int32_t err = 0;
    
    while (x >= y) {
        switch (corner) {
            case 0: /* Top-left */
                gfx_put_pixel(cx - x, cy - y, color);
                gfx_put_pixel(cx - y, cy - x, color);
                break;
            case 1: /* Top-right */
                gfx_put_pixel(cx + x, cy - y, color);
                gfx_put_pixel(cx + y, cy - x, color);
                break;
            case 2: /* Bottom-left */
                gfx_put_pixel(cx - x, cy + y, color);
                gfx_put_pixel(cx - y, cy + x, color);
                break;
            case 3: /* Bottom-right */
                gfx_put_pixel(cx + x, cy + y, color);
                gfx_put_pixel(cx + y, cy + x, color);
                break;
        }
        
        if (err <= 0) {
            y++;
            err += 2*y + 1;
        }
        if (err > 0) {
            x--;
            err -= 2*x + 1;
        }
    }
}

void gfx_fill_corner(int32_t cx, int32_t cy, int32_t radius, int32_t corner, uint32_t color) {
    for (int32_t y = 0; y <= radius; y++) {
        for (int32_t x = 0; x <= radius; x++) {
            if (x*x + y*y <= radius*radius) {
                switch (corner) {
                    case 0: gfx_put_pixel(cx - x, cy - y, color); break;
                    case 1: gfx_put_pixel(cx + x, cy - y, color); break;
                    case 2: gfx_put_pixel(cx - x, cy + y, color); break;
                    case 3: gfx_put_pixel(cx + x, cy + y, color); break;
                }
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * GRADIENTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_fill_gradient_v(int32_t x, int32_t y, int32_t w, int32_t h, 
                         uint32_t color1, uint32_t color2) {
    uint8_t r1 = (color1 >> 16) & 0xFF;
    uint8_t g1 = (color1 >> 8) & 0xFF;
    uint8_t b1 = color1 & 0xFF;
    
    uint8_t r2 = (color2 >> 16) & 0xFF;
    uint8_t g2 = (color2 >> 8) & 0xFF;
    uint8_t b2 = color2 & 0xFF;
    
    for (int32_t j = 0; j < h; j++) {
        uint8_t r = r1 + (r2 - r1) * j / h;
        uint8_t g = g1 + (g2 - g1) * j / h;
        uint8_t b = b1 + (b2 - b1) * j / h;
        uint32_t color = 0xFF000000 | (r << 16) | (g << 8) | b;
        
        for (int32_t i = 0; i < w; i++) {
            gfx_put_pixel(x + i, y + j, color);
        }
    }
}

void gfx_fill_gradient_h(int32_t x, int32_t y, int32_t w, int32_t h, 
                         uint32_t color1, uint32_t color2) {
    uint8_t r1 = (color1 >> 16) & 0xFF;
    uint8_t g1 = (color1 >> 8) & 0xFF;
    uint8_t b1 = color1 & 0xFF;
    
    uint8_t r2 = (color2 >> 16) & 0xFF;
    uint8_t g2 = (color2 >> 8) & 0xFF;
    uint8_t b2 = color2 & 0xFF;
    
    for (int32_t i = 0; i < w; i++) {
        uint8_t r = r1 + (r2 - r1) * i / w;
        uint8_t g = g1 + (g2 - g1) * i / w;
        uint8_t b = b1 + (b2 - b1) * i / w;
        uint32_t color = 0xFF000000 | (r << 16) | (g << 8) | b;
        
        for (int32_t j = 0; j < h; j++) {
            gfx_put_pixel(x + i, y + j, color);
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * TEXTO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_char(int32_t x, int32_t y, char c, uint32_t fg, uint32_t bg) {
    if (c < 32 || c > 126) c = '?';
    
    const uint8_t* glyph = font8x16[(uint8_t)c];
    
    for (int32_t row = 0; row < 16; row++) {
        uint8_t bits = glyph[row];
        for (int32_t col = 0; col < 8; col++) {
            uint32_t color = (bits & (0x80 >> col)) ? fg : bg;
            if (color != 0) { /* Skip if transparent */
                gfx_put_pixel(x + col, y + row, color);
            }
        }
    }
}

void gfx_draw_string(int32_t x, int32_t y, const char* str, uint32_t fg, uint32_t bg) {
    int32_t cx = x;
    while (*str) {
        if (*str == '\n') {
            cx = x;
            y += 16;
        } else {
            gfx_draw_char(cx, y, *str, fg, bg);
            cx += 8;
        }
        str++;
    }
}

void gfx_draw_string_scaled(int32_t x, int32_t y, const char* str, 
                            uint32_t fg, uint32_t bg, int32_t scale) {
    int32_t cx = x;
    while (*str) {
        if (*str == '\n') {
            cx = x;
            y += 16 * scale;
        } else {
            /* Dibujar carácter escalado */
            if (*str >= 32 && *str <= 126) {
                const uint8_t* glyph = font8x16[(uint8_t)*str];
                for (int32_t row = 0; row < 16; row++) {
                    uint8_t bits = glyph[row];
                    for (int32_t col = 0; col < 8; col++) {
                        uint32_t color = (bits & (0x80 >> col)) ? fg : bg;
                        if (color != 0) {
                            gfx_fill_rect(cx + col*scale, y + row*scale, scale, scale, color);
                        }
                    }
                }
            }
            cx += 8 * scale;
        }
        str++;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * BLENDING
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t gfx_blend(uint32_t fg, uint32_t bg, uint8_t alpha) {
    uint8_t fg_r = (fg >> 16) & 0xFF;
    uint8_t fg_g = (fg >> 8) & 0xFF;
    uint8_t fg_b = fg & 0xFF;
    
    uint8_t bg_r = (bg >> 16) & 0xFF;
    uint8_t bg_g = (bg >> 8) & 0xFF;
    uint8_t bg_b = bg & 0xFF;
    
    uint8_t r = (fg_r * alpha + bg_r * (255 - alpha)) / 255;
    uint8_t g = (fg_g * alpha + bg_g * (255 - alpha)) / 255;
    uint8_t b = (fg_b * alpha + bg_b * (255 - alpha)) / 255;
    
    return 0xFF000000 | (r << 16) | (g << 8) | b;
}

void gfx_fill_rect_alpha(int32_t x, int32_t y, int32_t w, int32_t h, 
                         uint32_t color, uint8_t alpha) {
    for (int32_t j = 0; j < h; j++) {
        for (int32_t i = 0; i < w; i++) {
            uint32_t bg = gfx_get_pixel(x + i, y + j);
            uint32_t blended = gfx_blend(color, bg, alpha);
            gfx_put_pixel(x + i, y + j, blended);
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * IMÁGENES / BITMAPS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void gfx_draw_bitmap(int32_t x, int32_t y, const uint32_t* bitmap, 
                     int32_t w, int32_t h) {
    for (int32_t j = 0; j < h; j++) {
        for (int32_t i = 0; i < w; i++) {
            uint32_t color = bitmap[j * w + i];
            if ((color >> 24) > 0) { /* Si tiene alpha */
                gfx_put_pixel(x + i, y + j, color);
            }
        }
    }
}

void gfx_draw_icon_8x8(int32_t x, int32_t y, const uint8_t* icon, uint32_t color) {
    for (int32_t row = 0; row < 8; row++) {
        uint8_t bits = icon[row];
        for (int32_t col = 0; col < 8; col++) {
            if (bits & (0x80 >> col)) {
                gfx_put_pixel(x + col, y + row, color);
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

GraphicsContext* gfx_get_context(void) {
    return &gfx;
}

