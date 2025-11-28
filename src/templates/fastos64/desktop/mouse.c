/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - PS/2 MOUSE DRIVER IMPLEMENTATION
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: mouse.c
 * Descripción: Driver de mouse PS/2 para el desktop
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "mouse.h"
#include "graphics.h"
#include "../ports64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES PS/2
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define PS2_DATA_PORT       0x60
#define PS2_STATUS_PORT     0x64
#define PS2_COMMAND_PORT    0x64

#define PS2_STATUS_OUTPUT   0x01
#define PS2_STATUS_INPUT    0x02

#define PS2_CMD_ENABLE_AUX  0xA8
#define PS2_CMD_GET_COMPAQ  0x20
#define PS2_CMD_SET_COMPAQ  0x60
#define PS2_CMD_WRITE_MOUSE 0xD4

#define MOUSE_CMD_RESET     0xFF
#define MOUSE_CMD_RESEND    0xFE
#define MOUSE_CMD_DEFAULTS  0xF6
#define MOUSE_CMD_DISABLE   0xF5
#define MOUSE_CMD_ENABLE    0xF4
#define MOUSE_CMD_SET_RATE  0xF3
#define MOUSE_CMD_GET_ID    0xF2
#define MOUSE_CMD_SET_RES   0xE8

/* ═══════════════════════════════════════════════════════════════════════════
 * CURSOR BITMAP (16x16)
 * ═══════════════════════════════════════════════════════════════════════════
 */

static const uint16_t cursor_bitmap[16] = {
    0b1000000000000000,
    0b1100000000000000,
    0b1110000000000000,
    0b1111000000000000,
    0b1111100000000000,
    0b1111110000000000,
    0b1111111000000000,
    0b1111111100000000,
    0b1111111110000000,
    0b1111111111000000,
    0b1111110000000000,
    0b1101111000000000,
    0b1000111100000000,
    0b0000111100000000,
    0b0000011110000000,
    0b0000011100000000
};

static const uint16_t cursor_outline[16] = {
    0b1000000000000000,
    0b1100000000000000,
    0b1010000000000000,
    0b1001000000000000,
    0b1000100000000000,
    0b1000010000000000,
    0b1000001000000000,
    0b1000000100000000,
    0b1000000010000000,
    0b1000000001000000,
    0b1000011111000000,
    0b1001001000000000,
    0b1010001000000000,
    0b0100000100000000,
    0b0000000100000000,
    0b0000000011000000
};

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static MouseState mouse = {0};
static uint8_t mouse_packet[4];
static int packet_index = 0;
static bool mouse_has_wheel = false;

/* Buffer para restaurar píxeles bajo el cursor */
static uint32_t cursor_save[16][16];
static int32_t cursor_save_x = -1;
static int32_t cursor_save_y = -1;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PS/2 AUXILIARES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void ps2_wait_input(void) {
    int timeout = 100000;
    while (timeout--) {
        if (!(inb(PS2_STATUS_PORT) & PS2_STATUS_INPUT)) return;
    }
}

static void ps2_wait_output(void) {
    int timeout = 100000;
    while (timeout--) {
        if (inb(PS2_STATUS_PORT) & PS2_STATUS_OUTPUT) return;
    }
}

static void ps2_write_command(uint8_t cmd) {
    ps2_wait_input();
    outb(PS2_COMMAND_PORT, cmd);
}

static void ps2_write_data(uint8_t data) {
    ps2_wait_input();
    outb(PS2_DATA_PORT, data);
}

static uint8_t ps2_read_data(void) {
    ps2_wait_output();
    return inb(PS2_DATA_PORT);
}

static void mouse_write(uint8_t data) {
    ps2_write_command(PS2_CMD_WRITE_MOUSE);
    ps2_write_data(data);
}

static uint8_t mouse_read(void) {
    return ps2_read_data();
}

/* ═══════════════════════════════════════════════════════════════════════════
 * INICIALIZACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void mouse_init(uint32_t screen_width, uint32_t screen_height) {
    mouse.x = screen_width / 2;
    mouse.y = screen_height / 2;
    mouse.screen_width = screen_width;
    mouse.screen_height = screen_height;
    mouse.left_button = false;
    mouse.right_button = false;
    mouse.middle_button = false;
    mouse.scroll_delta = 0;
    mouse.visible = true;
    
    /* Habilitar puerto auxiliar (mouse) */
    ps2_write_command(PS2_CMD_ENABLE_AUX);
    
    /* Obtener byte de configuración */
    ps2_write_command(PS2_CMD_GET_COMPAQ);
    uint8_t status = ps2_read_data();
    
    /* Habilitar IRQ12 y habilitar reloj del mouse */
    status |= 0x02;  /* Enable IRQ12 */
    status &= ~0x20; /* Enable mouse clock */
    
    /* Escribir configuración */
    ps2_write_command(PS2_CMD_SET_COMPAQ);
    ps2_write_data(status);
    
    /* Reset mouse */
    mouse_write(MOUSE_CMD_RESET);
    mouse_read(); /* ACK */
    mouse_read(); /* Self-test passed (0xAA) */
    mouse_read(); /* Mouse ID (0x00) */
    
    /* Intentar habilitar scroll wheel */
    mouse_write(MOUSE_CMD_SET_RATE); mouse_read(); ps2_write_data(200); mouse_read();
    mouse_write(MOUSE_CMD_SET_RATE); mouse_read(); ps2_write_data(100); mouse_read();
    mouse_write(MOUSE_CMD_SET_RATE); mouse_read(); ps2_write_data(80);  mouse_read();
    
    mouse_write(MOUSE_CMD_GET_ID);
    mouse_read(); /* ACK */
    uint8_t id = mouse_read();
    mouse_has_wheel = (id == 0x03 || id == 0x04);
    
    /* Configurar resolución y sample rate */
    mouse_write(MOUSE_CMD_SET_RES);
    mouse_read();
    ps2_write_data(0x03); /* 8 counts/mm */
    mouse_read();
    
    mouse_write(MOUSE_CMD_SET_RATE);
    mouse_read();
    ps2_write_data(100); /* 100 samples/second */
    mouse_read();
    
    /* Habilitar mouse */
    mouse_write(MOUSE_CMD_ENABLE);
    mouse_read();
    
    packet_index = 0;
}

void mouse_shutdown(void) {
    mouse_write(MOUSE_CMD_DISABLE);
    mouse_read();
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MANEJO DE INTERRUPCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

void mouse_handle_irq(void) {
    uint8_t data = inb(PS2_DATA_PORT);
    
    /* Verificar que es un paquete válido */
    if (packet_index == 0 && !(data & 0x08)) {
        return; /* Byte 0 debe tener bit 3 en 1 */
    }
    
    mouse_packet[packet_index++] = data;
    
    int packet_size = mouse_has_wheel ? 4 : 3;
    
    if (packet_index >= packet_size) {
        /* Procesar paquete completo */
        mouse_process_packet();
        packet_index = 0;
    }
}

void mouse_process_packet(void) {
    uint8_t flags = mouse_packet[0];
    int16_t dx = mouse_packet[1];
    int16_t dy = mouse_packet[2];
    
    /* Extender signo */
    if (flags & 0x10) dx |= 0xFF00;
    if (flags & 0x20) dy |= 0xFF00;
    
    /* Invertir Y (mouse reporta Y invertido) */
    dy = -dy;
    
    /* Actualizar posición */
    mouse.x += dx;
    mouse.y += dy;
    
    /* Limitar a pantalla */
    if (mouse.x < 0) mouse.x = 0;
    if (mouse.y < 0) mouse.y = 0;
    if (mouse.x >= (int32_t)mouse.screen_width) mouse.x = mouse.screen_width - 1;
    if (mouse.y >= (int32_t)mouse.screen_height) mouse.y = mouse.screen_height - 1;
    
    /* Botones */
    bool new_left = flags & 0x01;
    bool new_right = flags & 0x02;
    bool new_middle = flags & 0x04;
    
    /* Detectar clicks */
    if (new_left && !mouse.left_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_LEFT, true);
    } else if (!new_left && mouse.left_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_LEFT, false);
    }
    
    if (new_right && !mouse.right_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_RIGHT, true);
    } else if (!new_right && mouse.right_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_RIGHT, false);
    }
    
    if (new_middle && !mouse.middle_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_MIDDLE, true);
    } else if (!new_middle && mouse.middle_button) {
        mouse_on_click(mouse.x, mouse.y, MOUSE_BUTTON_MIDDLE, false);
    }
    
    mouse.left_button = new_left;
    mouse.right_button = new_right;
    mouse.middle_button = new_middle;
    
    /* Scroll wheel */
    if (mouse_has_wheel) {
        int8_t scroll = (int8_t)mouse_packet[3];
        if (scroll != 0) {
            mouse.scroll_delta = scroll;
            mouse_on_scroll(mouse.x, mouse.y, scroll);
        }
    }
    
    /* Notificar movimiento */
    mouse_on_move(mouse.x, mouse.y);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CALLBACKS (implementar en window manager)
 * ═══════════════════════════════════════════════════════════════════════════
 */

__attribute__((weak))
void mouse_on_move(int32_t x, int32_t y) {
    /* Override en window manager */
}

__attribute__((weak))
void mouse_on_click(int32_t x, int32_t y, MouseButton button, bool pressed) {
    /* Override en window manager */
}

__attribute__((weak))
void mouse_on_scroll(int32_t x, int32_t y, int8_t delta) {
    /* Override en window manager */
}

/* ═══════════════════════════════════════════════════════════════════════════
 * RENDERIZADO DEL CURSOR
 * ═══════════════════════════════════════════════════════════════════════════
 */

void mouse_save_background(uint32_t* framebuffer, uint32_t fb_width) {
    cursor_save_x = mouse.x;
    cursor_save_y = mouse.y;
    
    for (int32_t y = 0; y < 16; y++) {
        for (int32_t x = 0; x < 16; x++) {
            int32_t px = cursor_save_x + x;
            int32_t py = cursor_save_y + y;
            if (px >= 0 && px < (int32_t)mouse.screen_width &&
                py >= 0 && py < (int32_t)mouse.screen_height) {
                cursor_save[y][x] = framebuffer[py * fb_width + px];
            }
        }
    }
}

void mouse_restore_background(uint32_t* framebuffer, uint32_t fb_width) {
    if (cursor_save_x < 0) return;
    
    for (int32_t y = 0; y < 16; y++) {
        for (int32_t x = 0; x < 16; x++) {
            int32_t px = cursor_save_x + x;
            int32_t py = cursor_save_y + y;
            if (px >= 0 && px < (int32_t)mouse.screen_width &&
                py >= 0 && py < (int32_t)mouse.screen_height) {
                framebuffer[py * fb_width + px] = cursor_save[y][x];
            }
        }
    }
}

void mouse_render_cursor(uint32_t* framebuffer, uint32_t fb_width) {
    if (!mouse.visible) return;
    
    for (int32_t y = 0; y < 16; y++) {
        for (int32_t x = 0; x < 16; x++) {
            int32_t px = mouse.x + x;
            int32_t py = mouse.y + y;
            
            if (px >= 0 && px < (int32_t)mouse.screen_width &&
                py >= 0 && py < (int32_t)mouse.screen_height) {
                
                uint16_t mask = 0x8000 >> x;
                
                if (cursor_bitmap[y] & mask) {
                    /* Cursor blanco */
                    framebuffer[py * fb_width + px] = 0xFFFFFFFF;
                } else if (cursor_outline[y] & mask) {
                    /* Borde negro */
                    framebuffer[py * fb_width + px] = 0xFF000000;
                }
            }
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILIDADES
 * ═══════════════════════════════════════════════════════════════════════════
 */

MouseState* mouse_get_state(void) {
    return &mouse;
}

void mouse_set_position(int32_t x, int32_t y) {
    mouse.x = x;
    mouse.y = y;
    
    if (mouse.x < 0) mouse.x = 0;
    if (mouse.y < 0) mouse.y = 0;
    if (mouse.x >= (int32_t)mouse.screen_width) mouse.x = mouse.screen_width - 1;
    if (mouse.y >= (int32_t)mouse.screen_height) mouse.y = mouse.screen_height - 1;
}

void mouse_show(void) {
    mouse.visible = true;
}

void mouse_hide(void) {
    mouse.visible = false;
}

