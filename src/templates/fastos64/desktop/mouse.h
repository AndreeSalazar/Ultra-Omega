/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - MOUSE DRIVER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: mouse.h
 * Descripción: Driver de mouse PS/2 para el escritorio
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef MOUSE_H
#define MOUSE_H

#include "../types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define MOUSE_PORT_DATA         0x60
#define MOUSE_PORT_STATUS       0x64
#define MOUSE_PORT_COMMAND      0x64

#define MOUSE_CMD_ENABLE        0xA8
#define MOUSE_CMD_GET_COMPAQ    0x20
#define MOUSE_CMD_SET_COMPAQ    0x60
#define MOUSE_CMD_WRITE         0xD4

#define MOUSE_WRITE_SET_DEFAULTS    0xF6
#define MOUSE_WRITE_ENABLE          0xF4
#define MOUSE_WRITE_SET_SAMPLE      0xF3
#define MOUSE_WRITE_GET_ID          0xF2

/* Cursores */
typedef enum {
    CURSOR_ARROW,
    CURSOR_HAND,
    CURSOR_TEXT,
    CURSOR_RESIZE_H,
    CURSOR_RESIZE_V,
    CURSOR_RESIZE_DIAG1,
    CURSOR_RESIZE_DIAG2,
    CURSOR_MOVE,
    CURSOR_WAIT,
    CURSOR_CROSS,
    CURSOR_COUNT
} CursorType;

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    /* Posición */
    int32_t         x, y;
    
    /* Límites */
    int32_t         min_x, min_y;
    int32_t         max_x, max_y;
    
    /* Botones */
    bool            left_button;
    bool            right_button;
    bool            middle_button;
    
    /* Estado anterior (para detectar clicks) */
    bool            prev_left;
    bool            prev_right;
    bool            prev_middle;
    
    /* Scroll */
    int8_t          scroll_delta;
    
    /* Cursor */
    CursorType      cursor;
    bool            visible;
    
    /* Buffer de paquetes */
    uint8_t         packet[4];
    uint8_t         packet_index;
    bool            has_scroll_wheel;
    
} MouseState;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void mouse_init(void);
void mouse_set_bounds(int32_t min_x, int32_t min_y, int32_t max_x, int32_t max_y);

/* Interrupción */
void mouse_handle_interrupt(void);

/* Estado */
MouseState* mouse_get_state(void);
void mouse_get_position(int32_t* x, int32_t* y);
bool mouse_left_clicked(void);
bool mouse_right_clicked(void);
bool mouse_left_down(void);
bool mouse_right_down(void);
int8_t mouse_get_scroll(void);

/* Cursor */
void mouse_set_cursor(CursorType cursor);
void mouse_show(void);
void mouse_hide(void);

/* Renderizado */
void mouse_render(uint32_t* framebuffer, uint32_t fb_width, uint32_t fb_height);

#endif /* MOUSE_H */

