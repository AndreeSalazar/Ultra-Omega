/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - TASKBAR IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: taskbar.c
 * Descripción: Barra de tareas estilo Windows 11
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "taskbar.h"
#include "graphics.h"
#include "start_menu.h"
#include "../framebuffer.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLE GLOBAL
 * ═══════════════════════════════════════════════════════════════════════════
 */

static Taskbar taskbar = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * ICONOS 8x8 (bitmap)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static const uint8_t icon_start[8] = {
    0b11001100,
    0b11001100,
    0b00000000,
    0b00000000,
    0b11001100,
    0b11001100,
    0b00000000,
    0b00000000
};

static const uint8_t icon_explorer[8] = {
    0b11111110,
    0b10000010,
    0b10111010,
    0b10111010,
    0b10111010,
    0b10000010,
    0b10000010,
    0b11111110
};

static const uint8_t icon_terminal[8] = {
    0b11111111,
    0b10000001,
    0b10100001,
    0b10010001,
    0b10001001,
    0b10000001,
    0b10111101,
    0b11111111
};

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_init(uint32_t screen_width, uint32_t screen_height) {
    taskbar.x = 0;
    taskbar.y = screen_height - TASKBAR_HEIGHT;
    taskbar.width = screen_width;
    taskbar.height = TASKBAR_HEIGHT;
    
    taskbar.pinned_count = 0;
    taskbar.running_count = 0;
    taskbar.start_menu_open = false;
    taskbar.calendar_open = false;
    taskbar.quick_settings_open = false;
    taskbar.hover_index = -1;
    
    /* Apps por defecto */
    taskbar_pin_app(ICON_EXPLORER, "Explorer", NULL);
    taskbar_pin_app(ICON_TERMINAL, "Terminal", NULL);
    taskbar_pin_app(ICON_SETTINGS, "Settings", NULL);
    taskbar_pin_app(ICON_TASKMGR, "Task Manager", NULL);
}

void taskbar_shutdown(void) {
    taskbar.pinned_count = 0;
    taskbar.running_count = 0;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * RENDERIZADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_render(uint32_t* framebuffer, uint32_t fb_width, uint32_t fb_height) {
    /* Fondo semi-transparente */
    for (int32_t y = taskbar.y; y < taskbar.y + taskbar.height; y++) {
        for (int32_t x = 0; x < (int32_t)fb_width; x++) {
            if (y >= 0 && y < (int32_t)fb_height) {
                /* Blend con fondo */
                uint32_t bg = framebuffer[y * fb_width + x];
                uint32_t r = ((bg >> 16) & 0xFF) / 2 + 0x10;
                uint32_t g = ((bg >> 8) & 0xFF) / 2 + 0x10;
                uint32_t b = (bg & 0xFF) / 2 + 0x18;
                framebuffer[y * fb_width + x] = 0xE0000000 | (r << 16) | (g << 8) | b;
            }
        }
    }
    
    /* Línea superior */
    for (int32_t x = 0; x < (int32_t)fb_width; x++) {
        framebuffer[taskbar.y * fb_width + x] = 0xFF404050;
    }
    
    /* Botón Start (centro) */
    int32_t start_x = fb_width / 2 - 24;
    int32_t start_y = taskbar.y + 4;
    taskbar_render_start_button(framebuffer, start_x, start_y);
    
    /* Apps pinneadas */
    int32_t app_x = fb_width / 2 + 30;
    for (uint32_t i = 0; i < taskbar.pinned_count; i++) {
        bool hover = (taskbar.hover_index == (int32_t)i);
        taskbar_render_app_icon(framebuffer, &taskbar.pinned_apps[i], 
                                app_x, taskbar.y + 4, hover);
        app_x += TASKBAR_ICON_SIZE + 4;
    }
    
    /* Sistema (derecha) */
    taskbar_render_system_tray(framebuffer, fb_width - 150, taskbar.y + 8);
    taskbar_render_clock(framebuffer, fb_width - 80, taskbar.y + 8);
}

void taskbar_render_start_button(uint32_t* fb, int32_t x, int32_t y) {
    uint32_t color = taskbar.start_menu_open ? 0xFF0088E8 : 0xFF0078D4;
    
    /* Fondo del botón */
    for (int32_t dy = 0; dy < 40; dy++) {
        for (int32_t dx = 0; dx < 48; dx++) {
            fb[(y + dy) * 1920 + (x + dx)] = color; /* Asumiendo 1920 de ancho */
        }
    }
    
    /* Logo Windows (4 cuadrados) */
    uint32_t white = 0xFFFFFFFF;
    int32_t logo_x = x + 12;
    int32_t logo_y = y + 8;
    
    /* Superior izquierdo */
    for (int32_t dy = 0; dy < 10; dy++) {
        for (int32_t dx = 0; dx < 10; dx++) {
            fb[(logo_y + dy) * 1920 + (logo_x + dx)] = white;
        }
    }
    
    /* Superior derecho */
    for (int32_t dy = 0; dy < 10; dy++) {
        for (int32_t dx = 0; dx < 10; dx++) {
            fb[(logo_y + dy) * 1920 + (logo_x + 14 + dx)] = white;
        }
    }
    
    /* Inferior izquierdo */
    for (int32_t dy = 0; dy < 10; dy++) {
        for (int32_t dx = 0; dx < 10; dx++) {
            fb[(logo_y + 14 + dy) * 1920 + (logo_x + dx)] = white;
        }
    }
    
    /* Inferior derecho */
    for (int32_t dy = 0; dy < 10; dy++) {
        for (int32_t dx = 0; dx < 10; dx++) {
            fb[(logo_y + 14 + dy) * 1920 + (logo_x + 14 + dx)] = white;
        }
    }
}

void taskbar_render_app_icon(uint32_t* fb, TaskbarApp* app, int32_t x, int32_t y, bool hover) {
    uint32_t bg = hover ? 0xFF404060 : 0xFF303050;
    
    /* Fondo */
    for (int32_t dy = 0; dy < TASKBAR_ICON_SIZE; dy++) {
        for (int32_t dx = 0; dx < TASKBAR_ICON_SIZE; dx++) {
            fb[(y + dy) * 1920 + (x + dx)] = bg;
        }
    }
    
    /* Indicador de running */
    if (app->is_running) {
        for (int32_t dx = 12; dx < 28; dx++) {
            fb[(y + TASKBAR_ICON_SIZE - 2) * 1920 + (x + dx)] = 0xFF0078D4;
        }
    }
}

void taskbar_render_system_tray(uint32_t* fb, int32_t x, int32_t y) {
    /* Iconos de sistema simplificados */
    uint32_t gray = 0xFFB0B0B0;
    
    /* WiFi icon (simplificado) */
    for (int32_t i = 0; i < 3; i++) {
        fb[(y + 20 - i*4) * 1920 + (x + 8 + i*2)] = gray;
        fb[(y + 20 - i*4) * 1920 + (x + 16 - i*2)] = gray;
    }
    
    /* Volume icon */
    for (int32_t dy = 8; dy < 24; dy++) {
        fb[(y + dy) * 1920 + (x + 35)] = gray;
    }
    
    /* Battery icon */
    for (int32_t dx = 50; dx < 70; dx++) {
        fb[(y + 10) * 1920 + (x + dx)] = gray;
        fb[(y + 22) * 1920 + (x + dx)] = gray;
    }
    for (int32_t dy = 10; dy < 22; dy++) {
        fb[(y + dy) * 1920 + (x + 50)] = gray;
        fb[(y + dy) * 1920 + (x + 70)] = gray;
    }
}

void taskbar_render_clock(uint32_t* fb, int32_t x, int32_t y) {
    /* Por ahora solo un placeholder */
    /* TODO: Dibujar texto con fuente */
}

/* ═══════════════════════════════════════════════════════════════════════════
 * EVENTOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_handle_click(int32_t x, int32_t y) {
    /* Verificar click en Start */
    int32_t start_x = 1920 / 2 - 24; /* Asumiendo 1920 */
    if (x >= start_x && x < start_x + 48 && 
        y >= taskbar.y + 4 && y < taskbar.y + 44) {
        taskbar_toggle_start_menu();
        return;
    }
    
    /* Verificar click en apps */
    int32_t app_x = 1920 / 2 + 30;
    for (uint32_t i = 0; i < taskbar.pinned_count; i++) {
        if (x >= app_x && x < app_x + TASKBAR_ICON_SIZE &&
            y >= taskbar.y + 4 && y < taskbar.y + 4 + TASKBAR_ICON_SIZE) {
            if (taskbar.pinned_apps[i].on_click) {
                taskbar.pinned_apps[i].on_click();
            }
            return;
        }
        app_x += TASKBAR_ICON_SIZE + 4;
    }
}

void taskbar_handle_hover(int32_t x, int32_t y) {
    taskbar.hover_index = -1;
    
    if (y < taskbar.y || y >= taskbar.y + taskbar.height) return;
    
    int32_t app_x = 1920 / 2 + 30;
    for (uint32_t i = 0; i < taskbar.pinned_count; i++) {
        if (x >= app_x && x < app_x + TASKBAR_ICON_SIZE) {
            taskbar.hover_index = i;
            return;
        }
        app_x += TASKBAR_ICON_SIZE + 4;
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * GESTIÓN DE APPS
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_pin_app(SystemIcon icon, const char* name, void (*on_click)(void)) {
    if (taskbar.pinned_count >= MAX_PINNED_APPS) return;
    
    TaskbarApp* app = &taskbar.pinned_apps[taskbar.pinned_count++];
    app->icon = icon;
    app->name = name;
    app->tooltip = name;
    app->on_click = on_click;
    app->is_running = false;
    app->is_pinned = true;
    app->window = NULL;
}

void taskbar_unpin_app(SystemIcon icon) {
    for (uint32_t i = 0; i < taskbar.pinned_count; i++) {
        if (taskbar.pinned_apps[i].icon == icon) {
            for (uint32_t j = i; j < taskbar.pinned_count - 1; j++) {
                taskbar.pinned_apps[j] = taskbar.pinned_apps[j + 1];
            }
            taskbar.pinned_count--;
            return;
        }
    }
}

void taskbar_add_running_app(Window* window, SystemIcon icon) {
    if (taskbar.running_count >= MAX_RUNNING_APPS) return;
    
    TaskbarApp* app = &taskbar.running_apps[taskbar.running_count++];
    app->icon = icon;
    app->name = window->title;
    app->is_running = true;
    app->window = window;
}

void taskbar_remove_running_app(Window* window) {
    for (uint32_t i = 0; i < taskbar.running_count; i++) {
        if (taskbar.running_apps[i].window == window) {
            for (uint32_t j = i; j < taskbar.running_count - 1; j++) {
                taskbar.running_apps[j] = taskbar.running_apps[j + 1];
            }
            taskbar.running_count--;
            return;
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MENÚ START
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_toggle_start_menu(void) {
    taskbar.start_menu_open = !taskbar.start_menu_open;
    if (taskbar.start_menu_open) {
        start_menu_show();
    } else {
        start_menu_hide();
    }
}

void taskbar_close_start_menu(void) {
    taskbar.start_menu_open = false;
    start_menu_hide();
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

void taskbar_update_clock(void) {
    /* TODO: Leer RTC */
    taskbar.hour = 23;
    taskbar.minute = 45;
    taskbar.second = 0;
    taskbar.day = 28;
    taskbar.month = 11;
    taskbar.year = 2025;
}

Taskbar* taskbar_get(void) {
    return &taskbar;
}

