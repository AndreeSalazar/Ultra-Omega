/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - IDT Header
 * Nivel: Avanzado - Definiciones de IDT e interrupciones
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef IDT_H
#define IDT_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * TIPOS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Tipo de handler de interrupción */
typedef void (*interrupt_handler_t)(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void idt_init(void);

/* Registrar handler */
void register_interrupt_handler(uint8_t n, interrupt_handler_t handler);

/* Control de IRQs */
void irq_enable(uint8_t irq);
void irq_disable(uint8_t irq);

/* Control de interrupciones (definidas en ASM) */
extern void enable_interrupts(void);
extern void disable_interrupts(void);

#endif /* IDT_H */

