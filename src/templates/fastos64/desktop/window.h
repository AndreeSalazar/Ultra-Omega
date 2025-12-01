/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - WINDOW MANAGER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: window.h
 * Descripción: Sistema de ventanas estilo Windows 11
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef WINDOW_H
#define WINDOW_H

#include "../types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * COLORES DEL TEMA (Windows 11 Dark Mode)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define THEME_BG_DARK           0xFF1A1A2E  /* Fondo oscuro principal */
#define THEME_BG_DARKER         0xFF0F0F1A  /* Fondo más oscuro */
#define THEME_BG_WINDOW         0xFF202040  /* Fondo de ventana */
#define THEME_BG_TITLEBAR       0xFF252545  /* Barra de título */
#define THEME_BG_TASKBAR        0xE0202030  /* Barra de tareas (semi-transparente) */
#define THEME_BG_MENU           0xF0252540  /* Menú Start */

#define THEME_ACCENT            0xFF0078D4  /* Azul Windows */
#define THEME_ACCENT_HOVER      0xFF1084D8  /* Azul hover */
#define THEME_ACCENT_LIGHT      0xFF60CDFF  /* Azul claro */

#define THEME_TEXT_PRIMARY      0xFFFFFFFF  /* Texto blanco */
#define THEME_TEXT_SECONDARY    0xFFB0B0B0  /* Texto gris */
#define THEME_TEXT_DISABLED     0xFF606060  /* Texto deshabilitado */

#define THEME_BORDER            0xFF404060  /* Bordes */
#define THEME_BORDER_ACTIVE     0xFF0078D4  /* Borde activo */

#define THEME_CLOSE_BTN         0xFFE81123  /* Botón cerrar rojo */
#define THEME_MINIMIZE_BTN      0xFF3A3A5A  /* Botón minimizar */
#define THEME_MAXIMIZE_BTN      0xFF3A3A5A  /* Botón maximizar */

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define MAX_WINDOWS             32
#define MAX_TITLE_LENGTH        64
#define TITLEBAR_HEIGHT         32
#define BORDER_RADIUS           8
#define TASKBAR_HEIGHT          48
#define TASKBAR_ICON_SIZE       40

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef enum {
    WINDOW_NORMAL,
    WINDOW_MINIMIZED,
    WINDOW_MAXIMIZED,
    WINDOW_HIDDEN
} WindowState;

typedef enum {
    WINDOW_STYLE_NORMAL,
    WINDOW_STYLE_DIALOG,
    WINDOW_STYLE_POPUP,
    WINDOW_STYLE_BORDERLESS
} WindowStyle;

typedef struct Window {
    uint32_t        id;
    char            title[MAX_TITLE_LENGTH];
    
    /* Posición y tamaño */
    int32_t         x, y;
    int32_t         width, height;
    
    /* Estado */
    WindowState     state;
    WindowStyle     style;
    bool            focused;
    bool            visible;
    bool            resizable;
    bool            movable;
    
    /* Contenido */
    uint32_t*       framebuffer;    /* Buffer de la ventana */
    
    /* Callbacks */
    void            (*on_draw)(struct Window* win);
    void            (*on_click)(struct Window* win, int32_t x, int32_t y);
    void            (*on_key)(struct Window* win, uint8_t key);
    void            (*on_close)(struct Window* win);
    
    /* Datos de usuario */
    void*           user_data;
    
} Window;

typedef struct {
    Window*         windows[MAX_WINDOWS];
    uint32_t        window_count;
    Window*         focused_window;
    Window*         dragging_window;
    
    /* Framebuffer del escritorio */
    uint32_t*       desktop_fb;
    uint32_t        screen_width;
    uint32_t        screen_height;
    
    /* Estado del mouse */
    int32_t         mouse_x, mouse_y;
    bool            mouse_left_down;
    bool            mouse_right_down;
    
    /* Drag */
    int32_t         drag_offset_x;
    int32_t         drag_offset_y;
    
} WindowManager;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DEL WINDOW MANAGER
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void wm_init(uint32_t* framebuffer, uint32_t width, uint32_t height);
void wm_shutdown(void);

/* Gestión de ventanas */
Window* wm_create_window(const char* title, int32_t x, int32_t y, 
                         int32_t width, int32_t height, WindowStyle style);
void wm_destroy_window(Window* window);
void wm_show_window(Window* window);
void wm_hide_window(Window* window);
void wm_focus_window(Window* window);
void wm_minimize_window(Window* window);
void wm_maximize_window(Window* window);
void wm_restore_window(Window* window);

/* Renderizado */
void wm_render(void);
void wm_render_window(Window* window);
void wm_render_titlebar(Window* window);
void wm_render_desktop(void);

/* Eventos */
void wm_handle_mouse_move(int32_t x, int32_t y);
void wm_handle_mouse_click(int32_t x, int32_t y, bool left, bool down);
void wm_handle_key(uint8_t key);

/* Utilidades */
Window* wm_get_window_at(int32_t x, int32_t y);
void wm_bring_to_front(Window* window);

#endif /* WINDOW_H */

