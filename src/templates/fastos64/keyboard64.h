/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - KEYBOARD DRIVER
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef KEYBOARD64_H
#define KEYBOARD64_H

#include "types64.h"

void keyboard64_init(void);
char keyboard_getchar(void);
bool keyboard_has_key(void);

#endif

