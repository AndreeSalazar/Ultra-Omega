/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Keyboard Header
 * Nivel: Intermedio - Definiciones del driver de teclado
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef KEYBOARD_H
#define KEYBOARD_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Callback para teclas presionadas */
typedef void (*keyboard_callback_t)(char c, uint8_t scancode, 
                                    int ctrl, int alt, int shift);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void keyboard_init(void);

/* Lectura de teclas */
char keyboard_getchar(void);
int keyboard_getchar_nonblock(void);
int keyboard_available(void);
int keyboard_readline(char* buffer, int max_length);

/* Callback */
void keyboard_set_callback(keyboard_callback_t callback);

/* Estado de modificadores */
int keyboard_shift_pressed(void);
int keyboard_ctrl_pressed(void);
int keyboard_alt_pressed(void);

#endif /* KEYBOARD_H */

