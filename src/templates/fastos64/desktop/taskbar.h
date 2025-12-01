/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - TASKBAR (Barra de Tareas estilo Windows 11)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: taskbar.h
 * Descripción: Barra de tareas centrada con iconos, reloj y área de sistema
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef TASKBAR_H
#define TASKBAR_H

#include "../types64.h"
#include "window.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define TASKBAR_HEIGHT          48
#define TASKBAR_ICON_SIZE       40
#define TASKBAR_ICON_PADDING    4
#define TASKBAR_MARGIN          8

#define MAX_PINNED_APPS         12
#define MAX_RUNNING_APPS        16

/* ═══════════════════════════════════════════════════════════════════════════
 * ICONOS DE SISTEMA (8x8 bitmap simplificado)
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef enum {
    ICON_START,
    ICON_SEARCH,
    ICON_EXPLORER,
    ICON_TERMINAL,
    ICON_SETTINGS,
    ICON_TASKMGR,
    ICON_NOTEPAD,
    ICON_CALCULATOR,
    ICON_NETWORK,
    ICON_VOLUME,
    ICON_BATTERY,
    ICON_COUNT
} SystemIcon;

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    SystemIcon      icon;
    const char*     name;
    const char*     tooltip;
    void            (*on_click)(void);
    bool            is_running;
    bool            is_pinned;
    Window*         window;         /* Ventana asociada si está corriendo */
} TaskbarApp;

typedef struct {
    /* Posición */
    int32_t         x, y;
    int32_t         width, height;
    
    /* Apps */
    TaskbarApp      pinned_apps[MAX_PINNED_APPS];
    uint32_t        pinned_count;
    
    TaskbarApp      running_apps[MAX_RUNNING_APPS];
    uint32_t        running_count;
    
    /* Sistema */
    bool            start_menu_open;
    bool            calendar_open;
    bool            quick_settings_open;
    
    /* Reloj */
    uint8_t         hour, minute, second;
    uint8_t         day, month;
    uint16_t        year;
    
    /* Hover */
    int32_t         hover_index;    /* -1 = ninguno */
    
} Taskbar;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void taskbar_init(uint32_t screen_width, uint32_t screen_height);
void taskbar_shutdown(void);

/* Renderizado */
void taskbar_render(uint32_t* framebuffer, uint32_t fb_width, uint32_t fb_height);
void taskbar_render_start_button(uint32_t* fb, int32_t x, int32_t y);
void taskbar_render_app_icon(uint32_t* fb, TaskbarApp* app, int32_t x, int32_t y, bool hover);
void taskbar_render_system_tray(uint32_t* fb, int32_t x, int32_t y);
void taskbar_render_clock(uint32_t* fb, int32_t x, int32_t y);

/* Eventos */
void taskbar_handle_click(int32_t x, int32_t y);
void taskbar_handle_hover(int32_t x, int32_t y);

/* Apps */
void taskbar_pin_app(SystemIcon icon, const char* name, void (*on_click)(void));
void taskbar_unpin_app(SystemIcon icon);
void taskbar_add_running_app(Window* window, SystemIcon icon);
void taskbar_remove_running_app(Window* window);

/* Menú Start */
void taskbar_toggle_start_menu(void);
void taskbar_close_start_menu(void);

/* Sistema */
void taskbar_update_clock(void);
Taskbar* taskbar_get(void);

#endif /* TASKBAR_H */

