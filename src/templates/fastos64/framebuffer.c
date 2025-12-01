/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - FRAMEBUFFER GRAPHICS IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: framebuffer.c
 * Descripción: Implementación de gráficos de framebuffer
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "framebuffer.h"
#include "font8x16.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static Framebuffer fb;

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void fb_init(BootInfo* boot_info) {
    fb.buffer = (uint32_t*)boot_info->framebuffer_addr;
    fb.width = boot_info->framebuffer_width;
    fb.height = boot_info->framebuffer_height;
    fb.pitch = boot_info->framebuffer_pitch;
    fb.bpp = boot_info->framebuffer_bpp;
    
    fb.cursor_x = 0;
    fb.cursor_y = 0;
    fb.char_width = 8;
    fb.char_height = 16;
    fb.fg_color = FB_COLOR_TERM_FG;
    fb.bg_color = FB_COLOR_TERM_BG;
    
    fb_clear(FB_COLOR_TERM_BG);
}

void fb_clear(uint32_t color) {
    uint32_t* pixel = fb.buffer;
    uint32_t total = (fb.pitch / 4) * fb.height;
    
    for (uint32_t i = 0; i < total; i++) {
        pixel[i] = color;
    }
    
    fb.cursor_x = 0;
    fb.cursor_y = 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * PRIMITIVAS DE DIBUJO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void fb_put_pixel(uint32_t x, uint32_t y, uint32_t color) {
    if (x >= fb.width || y >= fb.height) return;
    
    uint32_t offset = y * (fb.pitch / 4) + x;
    fb.buffer[offset] = color;
}

uint32_t fb_get_pixel(uint32_t x, uint32_t y) {
    if (x >= fb.width || y >= fb.height) return 0;
    
    uint32_t offset = y * (fb.pitch / 4) + x;
    return fb.buffer[offset];
}

void fb_draw_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color) {
    /* Líneas horizontales */
    for (uint32_t i = x; i < x + w; i++) {
        fb_put_pixel(i, y, color);
        fb_put_pixel(i, y + h - 1, color);
    }
    /* Líneas verticales */
    for (uint32_t i = y; i < y + h; i++) {
        fb_put_pixel(x, i, color);
        fb_put_pixel(x + w - 1, i, color);
    }
}

void fb_fill_rect(uint32_t x, uint32_t y, uint32_t w, uint32_t h, uint32_t color) {
    for (uint32_t py = y; py < y + h && py < fb.height; py++) {
        for (uint32_t px = x; px < x + w && px < fb.width; px++) {
            fb_put_pixel(px, py, color);
        }
    }
}

void fb_draw_line(uint32_t x0, uint32_t y0, uint32_t x1, uint32_t y1, uint32_t color) {
    int dx = (x1 > x0) ? (x1 - x0) : (x0 - x1);
    int dy = (y1 > y0) ? (y1 - y0) : (y0 - y1);
    int sx = (x0 < x1) ? 1 : -1;
    int sy = (y0 < y1) ? 1 : -1;
    int err = dx - dy;
    
    while (1) {
        fb_put_pixel(x0, y0, color);
        
        if (x0 == x1 && y0 == y1) break;
        
        int e2 = 2 * err;
        if (e2 > -dy) {
            err -= dy;
            x0 += sx;
        }
        if (e2 < dx) {
            err += dx;
            y0 += sy;
        }
    }
}

void fb_draw_circle(uint32_t cx, uint32_t cy, uint32_t r, uint32_t color) {
    int x = r;
    int y = 0;
    int err = 0;
    
    while (x >= y) {
        fb_put_pixel(cx + x, cy + y, color);
        fb_put_pixel(cx + y, cy + x, color);
        fb_put_pixel(cx - y, cy + x, color);
        fb_put_pixel(cx - x, cy + y, color);
        fb_put_pixel(cx - x, cy - y, color);
        fb_put_pixel(cx - y, cy - x, color);
        fb_put_pixel(cx + y, cy - x, color);
        fb_put_pixel(cx + x, cy - y, color);
        
        y++;
        err += 1 + 2 * y;
        if (2 * (err - x) + 1 > 0) {
            x--;
            err += 1 - 2 * x;
        }
    }
}

void fb_fill_circle(uint32_t cx, uint32_t cy, uint32_t r, uint32_t color) {
    for (int y = -r; y <= (int)r; y++) {
        for (int x = -r; x <= (int)r; x++) {
            if (x * x + y * y <= (int)(r * r)) {
                fb_put_pixel(cx + x, cy + y, color);
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * TEXTO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void fb_set_colors(uint32_t fg, uint32_t bg) {
    fb.fg_color = fg;
    fb.bg_color = bg;
}

void fb_putchar(char c) {
    if (c == '\n') {
        fb.cursor_x = 0;
        fb.cursor_y += fb.char_height;
        if (fb.cursor_y + fb.char_height > fb.height) {
            fb_scroll(1);
            fb.cursor_y -= fb.char_height;
        }
        return;
    }
    
    if (c == '\r') {
        fb.cursor_x = 0;
        return;
    }
    
    if (c == '\t') {
        fb.cursor_x = (fb.cursor_x + 32) & ~31;
        return;
    }
    
    if (c < 32 || c > 126) {
        c = '?';
    }
    
    /* Dibujar carácter usando la fuente 8x16 */
    const uint8_t* glyph = font8x16[(uint8_t)c];
    
    for (uint32_t row = 0; row < fb.char_height; row++) {
        uint8_t bits = glyph[row];
        for (uint32_t col = 0; col < fb.char_width; col++) {
            uint32_t color = (bits & (0x80 >> col)) ? fb.fg_color : fb.bg_color;
            fb_put_pixel(fb.cursor_x + col, fb.cursor_y + row, color);
        }
    }
    
    fb.cursor_x += fb.char_width;
    
    /* Wrap al final de línea */
    if (fb.cursor_x + fb.char_width > fb.width) {
        fb.cursor_x = 0;
        fb.cursor_y += fb.char_height;
        if (fb.cursor_y + fb.char_height > fb.height) {
            fb_scroll(1);
            fb.cursor_y -= fb.char_height;
        }
    }
}

void fb_print(const char* str) {
    while (*str) {
        fb_putchar(*str++);
    }
}

void fb_print_at(uint32_t x, uint32_t y, const char* str, uint32_t color) {
    uint32_t old_x = fb.cursor_x;
    uint32_t old_y = fb.cursor_y;
    uint32_t old_fg = fb.fg_color;
    
    fb.cursor_x = x;
    fb.cursor_y = y;
    fb.fg_color = color;
    
    fb_print(str);
    
    fb.cursor_x = old_x;
    fb.cursor_y = old_y;
    fb.fg_color = old_fg;
}

void fb_set_cursor(uint32_t x, uint32_t y) {
    fb.cursor_x = x * fb.char_width;
    fb.cursor_y = y * fb.char_height;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * SCROLL
 * ═══════════════════════════════════════════════════════════════════════════
 */

void fb_scroll(uint32_t lines) {
    uint32_t scroll_pixels = lines * fb.char_height;
    uint32_t pitch_pixels = fb.pitch / 4;
    
    /* Mover contenido hacia arriba */
    for (uint32_t y = 0; y < fb.height - scroll_pixels; y++) {
        for (uint32_t x = 0; x < fb.width; x++) {
            fb.buffer[y * pitch_pixels + x] = 
                fb.buffer[(y + scroll_pixels) * pitch_pixels + x];
        }
    }
    
    /* Limpiar las líneas nuevas */
    for (uint32_t y = fb.height - scroll_pixels; y < fb.height; y++) {
        for (uint32_t x = 0; x < fb.width; x++) {
            fb.buffer[y * pitch_pixels + x] = fb.bg_color;
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

uint32_t fb_rgb(uint8_t r, uint8_t g, uint8_t b) {
    return 0xFF000000 | ((uint32_t)r << 16) | ((uint32_t)g << 8) | b;
}

uint32_t fb_rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    return ((uint32_t)a << 24) | ((uint32_t)r << 16) | ((uint32_t)g << 8) | b;
}

void fb_get_size(uint32_t* width, uint32_t* height) {
    if (width) *width = fb.width;
    if (height) *height = fb.height;
}

