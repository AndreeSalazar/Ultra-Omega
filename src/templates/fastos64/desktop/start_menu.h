/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - START MENU (Menú Inicio estilo Windows 11)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: start_menu.h
 * Descripción: Menú de inicio centrado con apps fijadas y recomendados
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef START_MENU_H
#define START_MENU_H

#include "../types64.h"
#include "taskbar.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define START_MENU_WIDTH        600
#define START_MENU_HEIGHT       700
#define START_MENU_RADIUS       12

#define START_SEARCH_HEIGHT     40
#define START_APP_SIZE          72
#define START_APP_COLS          6
#define START_APP_ROWS          3

#define MAX_PINNED_START        18
#define MAX_RECOMMENDED         6

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    SystemIcon      icon;
    const char*     name;
    void            (*on_click)(void);
} StartMenuItem;

typedef struct {
    /* Posición (calculada desde el centro de la pantalla) */
    int32_t         x, y;
    int32_t         width, height;
    
    /* Estado */
    bool            visible;
    bool            search_focused;
    char            search_text[64];
    
    /* Apps fijadas */
    StartMenuItem   pinned[MAX_PINNED_START];
    uint32_t        pinned_count;
    
    /* Recomendados */
    StartMenuItem   recommended[MAX_RECOMMENDED];
    uint32_t        recommended_count;
    
    /* Hover */
    int32_t         hover_index;
    bool            hover_all_apps;
    bool            hover_power;
    bool            hover_user;
    
    /* Animación */
    float           animation_progress;  /* 0.0 - 1.0 */
    bool            animating;
    
} StartMenu;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void start_menu_init(uint32_t screen_width, uint32_t screen_height);
void start_menu_shutdown(void);

/* Renderizado */
void start_menu_render(uint32_t* framebuffer, uint32_t fb_width, uint32_t fb_height);
void start_menu_render_search(uint32_t* fb, int32_t x, int32_t y);
void start_menu_render_pinned(uint32_t* fb, int32_t x, int32_t y);
void start_menu_render_recommended(uint32_t* fb, int32_t x, int32_t y);
void start_menu_render_footer(uint32_t* fb, int32_t x, int32_t y);

/* Eventos */
void start_menu_handle_click(int32_t x, int32_t y);
void start_menu_handle_hover(int32_t x, int32_t y);
void start_menu_handle_key(uint8_t key);

/* Control */
void start_menu_show(void);
void start_menu_hide(void);
void start_menu_toggle(void);
bool start_menu_is_visible(void);

/* Apps */
void start_menu_pin_app(SystemIcon icon, const char* name, void (*on_click)(void));
void start_menu_add_recommended(SystemIcon icon, const char* name, void (*on_click)(void));

/* Búsqueda */
void start_menu_search(const char* query);
void start_menu_clear_search(void);

StartMenu* start_menu_get(void);

#endif /* START_MENU_H */

