/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - DESKTOP ENVIRONMENT
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: desktop.h
 * Descripción: Entorno de escritorio completo estilo Windows 11
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef DESKTOP_H
#define DESKTOP_H

#include "../types64.h"
#include "window.h"
#include "taskbar.h"
#include "start_menu.h"
#include "taskmgr.h"
#include "graphics.h"
#include "mouse.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define DESKTOP_FPS             60
#define WALLPAPER_GRADIENT      1   /* 1 = gradient, 0 = solid */

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURA DEL DESKTOP
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    /* Framebuffer */
    uint32_t*       framebuffer;
    uint32_t        width;
    uint32_t        height;
    uint32_t        pitch;
    
    /* Componentes */
    WindowManager*  wm;
    Taskbar*        taskbar;
    StartMenu*      start_menu;
    TaskManager*    task_manager;
    
    /* Estado */
    bool            running;
    uint64_t        frame_count;
    uint64_t        last_tick;
    
    /* Fondo de pantalla */
    uint32_t        wallpaper_color1;
    uint32_t        wallpaper_color2;
    
} Desktop;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PRINCIPALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void desktop_init(uint32_t* framebuffer, uint32_t width, uint32_t height);
void desktop_shutdown(void);

/* Loop principal */
void desktop_run(void);
void desktop_update(void);
void desktop_render(void);

/* Eventos */
void desktop_handle_key(uint8_t scancode);
void desktop_handle_mouse(void);

/* Aplicaciones integradas */
Window* desktop_open_terminal(void);
Window* desktop_open_explorer(void);
Window* desktop_open_settings(void);
Window* desktop_open_notepad(void);
Window* desktop_open_calculator(void);

/* Utilidades */
Desktop* desktop_get(void);
void desktop_set_wallpaper(uint32_t color1, uint32_t color2);

#endif /* DESKTOP_H */

