/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Timer Header
 * Nivel: Intermedio - Definiciones del timer del sistema
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef TIMER_H
#define TIMER_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef void (*timer_callback_t)(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void timer_init(uint32_t frequency);

/* Obtener tiempo */
uint64_t timer_get_ticks(void);
uint64_t timer_get_ms(void);
uint32_t timer_get_seconds(void);
uint32_t timer_get_frequency(void);

/* Esperas */
void timer_wait_ticks(uint32_t ticks);
void timer_wait_ms(uint32_t ms);
void timer_wait_seconds(uint32_t seconds);

/* Callbacks */
int timer_register_callback(timer_callback_t callback, uint32_t interval_ms);
void timer_unregister_callback(int id);

/* Utilidades */
void timer_format_uptime(char* buffer, int buffer_size);
void timer_beep(uint32_t frequency, uint32_t duration_ms);

#endif /* TIMER_H */

