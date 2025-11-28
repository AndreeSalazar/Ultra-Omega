/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Keyboard Driver
 * Nivel: Intermedio - Driver de teclado PS/2
 * ═══════════════════════════════════════════════════════════════════════════
 * Este driver maneja el teclado PS/2 a través del controlador 8042.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "keyboard.h"
#include "ports.h"
#include "idt.h"
#include "vga.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define KEYBOARD_DATA_PORT      0x60
#define KEYBOARD_STATUS_PORT    0x64
#define KEYBOARD_CMD_PORT       0x64

/* Códigos especiales */
#define KEY_RELEASE             0x80
#define KEY_EXTENDED            0xE0

/* Teclas especiales */
#define KEY_ESCAPE              0x01
#define KEY_BACKSPACE           0x0E
#define KEY_TAB                 0x0F
#define KEY_ENTER               0x1C
#define KEY_LCTRL               0x1D
#define KEY_LSHIFT              0x2A
#define KEY_RSHIFT              0x36
#define KEY_LALT                0x38
#define KEY_CAPSLOCK            0x3A
#define KEY_F1                  0x3B
#define KEY_F2                  0x3C
#define KEY_F3                  0x3D
#define KEY_F4                  0x3E
#define KEY_F5                  0x3F
#define KEY_F6                  0x40
#define KEY_F7                  0x41
#define KEY_F8                  0x42
#define KEY_F9                  0x43
#define KEY_F10                 0x44
#define KEY_NUMLOCK             0x45
#define KEY_SCROLLLOCK          0x46
#define KEY_F11                 0x57
#define KEY_F12                 0x58

/* ═══════════════════════════════════════════════════════════════════════════
 * MAPAS DE TECLADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Mapa de teclas normales (US layout) */
static const char scancode_to_char[128] = {
    0,    27,  '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\b',
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
    0,    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0,    '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0,
    '*',  0,   ' ', 0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,    0,   0,   0,   0,   0,   '-', 0,   0,   0,   '+', 0,   0,
    0,    0,   0,   0,   0,   0,   0,   0,   0
};

/* Mapa de teclas con Shift */
static const char scancode_to_char_shift[128] = {
    0,    27,  '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\b',
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
    0,    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0,    '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0,
    '*',  0,   ' ', 0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,    0,   0,   0,   0,   0,   '-', 0,   0,   0,   '+', 0,   0,
    0,    0,   0,   0,   0,   0,   0,   0,   0
};

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTADO DEL TECLADO
 * ═══════════════════════════════════════════════════════════════════════════
 */

static struct {
    uint8_t shift_pressed;
    uint8_t ctrl_pressed;
    uint8_t alt_pressed;
    uint8_t capslock_on;
    uint8_t numlock_on;
    uint8_t extended_key;
} keyboard_state = {0};

/* Buffer circular de teclas */
#define KEYBOARD_BUFFER_SIZE    256

static struct {
    char buffer[KEYBOARD_BUFFER_SIZE];
    uint16_t read_pos;
    uint16_t write_pos;
    uint16_t count;
} key_buffer = {0};

/* Callback para teclas */
static keyboard_callback_t key_callback = NULL;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES INTERNAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Añadir tecla al buffer */
static void buffer_push(char c) {
    if (key_buffer.count < KEYBOARD_BUFFER_SIZE) {
        key_buffer.buffer[key_buffer.write_pos] = c;
        key_buffer.write_pos = (key_buffer.write_pos + 1) % KEYBOARD_BUFFER_SIZE;
        key_buffer.count++;
    }
}

/* Procesar scancode */
static void process_scancode(uint8_t scancode) {
    /* Manejar código extendido */
    if (scancode == KEY_EXTENDED) {
        keyboard_state.extended_key = 1;
        return;
    }
    
    /* ¿Es una liberación de tecla? */
    uint8_t released = scancode & KEY_RELEASE;
    scancode &= ~KEY_RELEASE;
    
    /* Manejar teclas modificadoras */
    switch (scancode) {
        case KEY_LSHIFT:
        case KEY_RSHIFT:
            keyboard_state.shift_pressed = !released;
            return;
            
        case KEY_LCTRL:
            keyboard_state.ctrl_pressed = !released;
            return;
            
        case KEY_LALT:
            keyboard_state.alt_pressed = !released;
            return;
            
        case KEY_CAPSLOCK:
            if (!released) {
                keyboard_state.capslock_on = !keyboard_state.capslock_on;
            }
            return;
            
        case KEY_NUMLOCK:
            if (!released) {
                keyboard_state.numlock_on = !keyboard_state.numlock_on;
            }
            return;
    }
    
    /* Solo procesar tecla presionada, no liberada */
    if (released) {
        keyboard_state.extended_key = 0;
        return;
    }
    
    /* Obtener carácter */
    char c = 0;
    
    if (keyboard_state.shift_pressed) {
        c = scancode_to_char_shift[scancode];
    } else {
        c = scancode_to_char[scancode];
    }
    
    /* Aplicar Caps Lock a letras */
    if (keyboard_state.capslock_on && c >= 'a' && c <= 'z') {
        c -= 32;  /* Convertir a mayúscula */
    } else if (keyboard_state.capslock_on && c >= 'A' && c <= 'Z') {
        c += 32;  /* Convertir a minúscula (ya tiene shift) */
    }
    
    /* Añadir al buffer si es un carácter válido */
    if (c != 0) {
        buffer_push(c);
        
        /* Llamar callback si existe */
        if (key_callback) {
            key_callback(c, scancode, keyboard_state.ctrl_pressed, 
                        keyboard_state.alt_pressed, keyboard_state.shift_pressed);
        }
    }
    
    keyboard_state.extended_key = 0;
}

/* Handler de interrupción del teclado (IRQ1) */
void keyboard_handler(void) {
    uint8_t scancode = inb(KEYBOARD_DATA_PORT);
    process_scancode(scancode);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar driver de teclado */
void keyboard_init(void) {
    /* Registrar handler de interrupción */
    register_interrupt_handler(33, keyboard_handler);
    
    /* Limpiar buffer del teclado */
    while (inb(KEYBOARD_STATUS_PORT) & 1) {
        inb(KEYBOARD_DATA_PORT);
    }
    
    /* Inicializar estado */
    keyboard_state.shift_pressed = 0;
    keyboard_state.ctrl_pressed = 0;
    keyboard_state.alt_pressed = 0;
    keyboard_state.capslock_on = 0;
    keyboard_state.numlock_on = 0;
    keyboard_state.extended_key = 0;
    
    key_buffer.read_pos = 0;
    key_buffer.write_pos = 0;
    key_buffer.count = 0;
}

/* Leer carácter del buffer (bloqueante) */
char keyboard_getchar(void) {
    while (key_buffer.count == 0) {
        __asm__ volatile("hlt");
    }
    
    char c = key_buffer.buffer[key_buffer.read_pos];
    key_buffer.read_pos = (key_buffer.read_pos + 1) % KEYBOARD_BUFFER_SIZE;
    key_buffer.count--;
    
    return c;
}

/* Leer carácter sin bloquear (-1 si no hay) */
int keyboard_getchar_nonblock(void) {
    if (key_buffer.count == 0) {
        return -1;
    }
    
    char c = key_buffer.buffer[key_buffer.read_pos];
    key_buffer.read_pos = (key_buffer.read_pos + 1) % KEYBOARD_BUFFER_SIZE;
    key_buffer.count--;
    
    return c;
}

/* ¿Hay teclas disponibles? */
int keyboard_available(void) {
    return key_buffer.count > 0;
}

/* Leer línea de texto */
int keyboard_readline(char* buffer, int max_length) {
    int i = 0;
    
    while (i < max_length - 1) {
        char c = keyboard_getchar();
        
        if (c == '\n') {
            buffer[i] = '\0';
            vga_putchar('\n');
            return i;
        } else if (c == '\b') {
            if (i > 0) {
                i--;
                vga_putchar('\b');
            }
        } else {
            buffer[i++] = c;
            vga_putchar(c);
        }
    }
    
    buffer[i] = '\0';
    return i;
}

/* Registrar callback */
void keyboard_set_callback(keyboard_callback_t callback) {
    key_callback = callback;
}

/* Obtener estado de modificadores */
int keyboard_shift_pressed(void) {
    return keyboard_state.shift_pressed;
}

int keyboard_ctrl_pressed(void) {
    return keyboard_state.ctrl_pressed;
}

int keyboard_alt_pressed(void) {
    return keyboard_state.alt_pressed;
}

