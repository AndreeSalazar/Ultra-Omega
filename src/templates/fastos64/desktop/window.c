/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - WINDOW MANAGER IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: window.c
 * Descripción: Implementación del sistema de ventanas estilo Windows 11
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "window.h"
#include "graphics.h"
#include "../framebuffer.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static WindowManager wm = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void wm_init(uint32_t* framebuffer, uint32_t width, uint32_t height) {
    wm.desktop_fb = framebuffer;
    wm.screen_width = width;
    wm.screen_height = height;
    wm.window_count = 0;
    wm.focused_window = NULL;
    wm.dragging_window = NULL;
    wm.mouse_x = width / 2;
    wm.mouse_y = height / 2;
    wm.mouse_left_down = false;
    wm.mouse_right_down = false;
    
    for (int i = 0; i < MAX_WINDOWS; i++) {
        wm.windows[i] = NULL;
    }
}

void wm_shutdown(void) {
    for (uint32_t i = 0; i < wm.window_count; i++) {
        if (wm.windows[i]) {
            wm_destroy_window(wm.windows[i]);
        }
    }
    wm.window_count = 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * GESTIÓN DE VENTANAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

static uint32_t next_window_id = 1;

Window* wm_create_window(const char* title, int32_t x, int32_t y, 
                         int32_t width, int32_t height, WindowStyle style) {
    if (wm.window_count >= MAX_WINDOWS) return NULL;
    
    Window* win = (Window*)kmalloc(sizeof(Window));
    if (!win) return NULL;
    
    win->id = next_window_id++;
    
    /* Copiar título */
    int i = 0;
    while (title[i] && i < MAX_TITLE_LENGTH - 1) {
        win->title[i] = title[i];
        i++;
    }
    win->title[i] = '\0';
    
    win->x = x;
    win->y = y;
    win->width = width;
    win->height = height;
    win->state = WINDOW_NORMAL;
    win->style = style;
    win->focused = false;
    win->visible = true;
    win->resizable = true;
    win->movable = true;
    
    /* Crear framebuffer de la ventana */
    win->framebuffer = (uint32_t*)kmalloc(width * height * sizeof(uint32_t));
    if (!win->framebuffer) {
        kfree(win);
        return NULL;
    }
    
    /* Limpiar framebuffer */
    for (int j = 0; j < width * height; j++) {
        win->framebuffer[j] = THEME_BG_WINDOW;
    }
    
    win->on_draw = NULL;
    win->on_click = NULL;
    win->on_key = NULL;
    win->on_close = NULL;
    win->user_data = NULL;
    
    /* Agregar a la lista */
    wm.windows[wm.window_count++] = win;
    
    /* Enfocar la nueva ventana */
    wm_focus_window(win);
    
    return win;
}

void wm_destroy_window(Window* window) {
    if (!window) return;
    
    /* Llamar callback de cierre */
    if (window->on_close) {
        window->on_close(window);
    }
    
    /* Liberar framebuffer */
    if (window->framebuffer) {
        kfree(window->framebuffer);
    }
    
    /* Remover de la lista */
    for (uint32_t i = 0; i < wm.window_count; i++) {
        if (wm.windows[i] == window) {
            for (uint32_t j = i; j < wm.window_count - 1; j++) {
                wm.windows[j] = wm.windows[j + 1];
            }
            wm.window_count--;
            break;
        }
    }
    
    /* Liberar ventana */
    kfree(window);
    
    /* Actualizar foco */
    if (wm.focused_window == window) {
        wm.focused_window = wm.window_count > 0 ? wm.windows[wm.window_count - 1] : NULL;
    }
}

void wm_show_window(Window* window) {
    if (window) window->visible = true;
}

void wm_hide_window(Window* window) {
    if (window) window->visible = false;
}

void wm_focus_window(Window* window) {
    if (!window || !window->visible) return;
    
    /* Quitar foco de la ventana anterior */
    if (wm.focused_window) {
        wm.focused_window->focused = false;
    }
    
    /* Enfocar nueva ventana */
    window->focused = true;
    wm.focused_window = window;
    
    /* Traer al frente */
    wm_bring_to_front(window);
}

void wm_minimize_window(Window* window) {
    if (window) window->state = WINDOW_MINIMIZED;
}

void wm_maximize_window(Window* window) {
    if (!window) return;
    
    if (window->state == WINDOW_MAXIMIZED) {
        wm_restore_window(window);
    } else {
        window->state = WINDOW_MAXIMIZED;
        /* TODO: Guardar posición anterior y expandir */
    }
}

void wm_restore_window(Window* window) {
    if (window) window->state = WINDOW_NORMAL;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * RENDERIZADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void wm_render(void) {
    /* Renderizar desktop */
    wm_render_desktop();
    
    /* Renderizar ventanas (de atrás hacia adelante) */
    for (uint32_t i = 0; i < wm.window_count; i++) {
        if (wm.windows[i] && wm.windows[i]->visible && 
            wm.windows[i]->state != WINDOW_MINIMIZED) {
            wm_render_window(wm.windows[i]);
        }
    }
}

void wm_render_desktop(void) {
    /* Fondo degradado */
    for (uint32_t y = 0; y < wm.screen_height - TASKBAR_HEIGHT; y++) {
        uint8_t r = 0x1A - (y * 0x0B / wm.screen_height);
        uint8_t g = 0x1A - (y * 0x0B / wm.screen_height);
        uint8_t b = 0x2E - (y * 0x14 / wm.screen_height);
        uint32_t color = 0xFF000000 | (r << 16) | (g << 8) | b;
        
        for (uint32_t x = 0; x < wm.screen_width; x++) {
            wm.desktop_fb[y * wm.screen_width + x] = color;
        }
    }
}

void wm_render_window(Window* window) {
    if (!window) return;
    
    int32_t x = window->x;
    int32_t y = window->y;
    int32_t w = window->width;
    int32_t h = window->height;
    
    /* Sombra */
    for (int32_t sy = 4; sy < h + 8; sy++) {
        for (int32_t sx = 4; sx < w + 8; sx++) {
            int32_t px = x + sx;
            int32_t py = y + sy;
            if (px >= 0 && px < (int32_t)wm.screen_width && 
                py >= 0 && py < (int32_t)wm.screen_height) {
                /* Blend sombra */
                uint32_t idx = py * wm.screen_width + px;
                uint32_t bg = wm.desktop_fb[idx];
                uint32_t shadow = ((bg & 0xFEFEFE) >> 1);
                wm.desktop_fb[idx] = 0xFF000000 | shadow;
            }
        }
    }
    
    /* Fondo de ventana */
    uint32_t bg_color = window->focused ? THEME_BG_WINDOW : 0xFF1A1A30;
    for (int32_t wy = 0; wy < h; wy++) {
        for (int32_t wx = 0; wx < w; wx++) {
            int32_t px = x + wx;
            int32_t py = y + wy;
            if (px >= 0 && px < (int32_t)wm.screen_width && 
                py >= 0 && py < (int32_t)wm.screen_height) {
                wm.desktop_fb[py * wm.screen_width + px] = bg_color;
            }
        }
    }
    
    /* Barra de título */
    wm_render_titlebar(window);
    
    /* Contenido (callback) */
    if (window->on_draw) {
        window->on_draw(window);
    }
}

void wm_render_titlebar(Window* window) {
    int32_t x = window->x;
    int32_t y = window->y;
    int32_t w = window->width;
    
    uint32_t tb_color = window->focused ? THEME_BG_TITLEBAR : 0xFF1E1E30;
    
    /* Fondo de titlebar */
    for (int32_t ty = 0; ty < TITLEBAR_HEIGHT; ty++) {
        for (int32_t tx = 0; tx < w; tx++) {
            int32_t px = x + tx;
            int32_t py = y + ty;
            if (px >= 0 && px < (int32_t)wm.screen_width && 
                py >= 0 && py < (int32_t)wm.screen_height) {
                wm.desktop_fb[py * wm.screen_width + px] = tb_color;
            }
        }
    }
    
    /* Botón cerrar */
    int32_t btn_x = x + w - 46;
    int32_t btn_y = y;
    for (int32_t by = 0; by < TITLEBAR_HEIGHT; by++) {
        for (int32_t bx = 0; bx < 46; bx++) {
            int32_t px = btn_x + bx;
            int32_t py = btn_y + by;
            if (px >= 0 && px < (int32_t)wm.screen_width && 
                py >= 0 && py < (int32_t)wm.screen_height) {
                wm.desktop_fb[py * wm.screen_width + px] = THEME_CLOSE_BTN;
            }
        }
    }
    
    /* TODO: Dibujar título con fuente */
}

/* ═══════════════════════════════════════════════════════════════════════════
 * EVENTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void wm_handle_mouse_move(int32_t x, int32_t y) {
    wm.mouse_x = x;
    wm.mouse_y = y;
    
    /* Arrastrar ventana */
    if (wm.dragging_window && wm.mouse_left_down) {
        wm.dragging_window->x = x - wm.drag_offset_x;
        wm.dragging_window->y = y - wm.drag_offset_y;
    }
}

void wm_handle_mouse_click(int32_t x, int32_t y, bool left, bool down) {
    if (left) {
        wm.mouse_left_down = down;
        
        if (down) {
            Window* clicked = wm_get_window_at(x, y);
            if (clicked) {
                wm_focus_window(clicked);
                
                /* Verificar si click en titlebar */
                if (y >= clicked->y && y < clicked->y + TITLEBAR_HEIGHT) {
                    /* Verificar botón cerrar */
                    if (x >= clicked->x + clicked->width - 46) {
                        wm_destroy_window(clicked);
                    } else {
                        /* Iniciar drag */
                        wm.dragging_window = clicked;
                        wm.drag_offset_x = x - clicked->x;
                        wm.drag_offset_y = y - clicked->y;
                    }
                } else if (clicked->on_click) {
                    clicked->on_click(clicked, x - clicked->x, y - clicked->y);
                }
            }
        } else {
            wm.dragging_window = NULL;
        }
    } else {
        wm.mouse_right_down = down;
    }
}

void wm_handle_key(uint8_t key) {
    if (wm.focused_window && wm.focused_window->on_key) {
        wm.focused_window->on_key(wm.focused_window, key);
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

Window* wm_get_window_at(int32_t x, int32_t y) {
    /* Buscar de adelante hacia atrás */
    for (int32_t i = wm.window_count - 1; i >= 0; i--) {
        Window* win = wm.windows[i];
        if (win && win->visible && win->state != WINDOW_MINIMIZED) {
            if (x >= win->x && x < win->x + win->width &&
                y >= win->y && y < win->y + win->height) {
                return win;
            }
        }
    }
    return NULL;
}

void wm_bring_to_front(Window* window) {
    if (!window) return;
    
    /* Encontrar y mover al final */
    for (uint32_t i = 0; i < wm.window_count; i++) {
        if (wm.windows[i] == window) {
            for (uint32_t j = i; j < wm.window_count - 1; j++) {
                wm.windows[j] = wm.windows[j + 1];
            }
            wm.windows[wm.window_count - 1] = window;
            break;
        }
    }
}

/* Función auxiliar para asignación de memoria */
extern void* kmalloc(size_t size);
extern void kfree(void* ptr);

