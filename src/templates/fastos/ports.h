/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Ports Header
 * Nivel: Básico - Funciones de E/S de puertos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef PORTS_H
#define PORTS_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE E/S (definidas en ASM)
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Escribir byte a puerto */
extern void outb(uint16_t port, uint8_t value);

/* Leer byte de puerto */
extern uint8_t inb(uint16_t port);

/* Escribir word a puerto */
extern void outw(uint16_t port, uint16_t value);

/* Leer word de puerto */
extern uint16_t inw(uint16_t port);

/* Espera de I/O */
extern void io_wait(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * MACROS DE PUERTOS COMUNES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* PIC (Programmable Interrupt Controller) */
#define PIC1_CMD            0x20
#define PIC1_DATA           0x21
#define PIC2_CMD            0xA0
#define PIC2_DATA           0xA1

/* PIT (Programmable Interval Timer) */
#define PIT_CHANNEL0        0x40
#define PIT_CHANNEL1        0x41
#define PIT_CHANNEL2        0x42
#define PIT_CMD             0x43

/* Teclado */
#define KEYBOARD_DATA       0x60
#define KEYBOARD_STATUS     0x64
#define KEYBOARD_CMD        0x64

/* RTC (Real Time Clock) */
#define RTC_INDEX           0x70
#define RTC_DATA            0x71

/* VGA */
#define VGA_CTRL            0x3D4
#define VGA_DATA            0x3D5

/* Serial (COM1) */
#define COM1_DATA           0x3F8
#define COM1_INT            0x3F9
#define COM1_FIFO           0x3FA
#define COM1_LINE           0x3FB
#define COM1_MODEM          0x3FC
#define COM1_STATUS         0x3FD

#endif /* PORTS_H */

