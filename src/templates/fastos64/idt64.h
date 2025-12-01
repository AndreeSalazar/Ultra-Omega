/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - INTERRUPT DESCRIPTOR TABLE
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: idt64.h
 * Descripción: IDT para modo largo (64-bit)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef IDT64_H
#define IDT64_H

#include "types64.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define IDT_ENTRIES         256

/* Tipos de gate */
#define IDT_TYPE_INTERRUPT  0x8E    /* Interrupt Gate (IF=0) */
#define IDT_TYPE_TRAP       0x8F    /* Trap Gate (IF unchanged) */

/* IRQs del PIC */
#define IRQ_TIMER           0
#define IRQ_KEYBOARD        1
#define IRQ_CASCADE         2
#define IRQ_COM2            3
#define IRQ_COM1            4
#define IRQ_LPT2            5
#define IRQ_FLOPPY          6
#define IRQ_LPT1            7
#define IRQ_RTC             8
#define IRQ_FREE1           9
#define IRQ_FREE2           10
#define IRQ_FREE3           11
#define IRQ_MOUSE           12
#define IRQ_COPROCESSOR     13
#define IRQ_ATA_PRIMARY     14
#define IRQ_ATA_SECONDARY   15

/* Vector base para IRQs (después de excepciones) */
#define IRQ_BASE            32

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Entrada de IDT de 64 bits */
typedef struct PACKED {
    uint16_t    offset_low;     /* Bits 0-15 del handler */
    uint16_t    selector;       /* Selector de código */
    uint8_t     ist;            /* Interrupt Stack Table (0 = disabled) */
    uint8_t     type_attr;      /* Tipo y atributos */
    uint16_t    offset_mid;     /* Bits 16-31 del handler */
    uint32_t    offset_high;    /* Bits 32-63 del handler */
    uint32_t    reserved;       /* Reservado, debe ser 0 */
} IDTEntry64;

/* Puntero a IDT */
typedef struct PACKED {
    uint16_t    limit;
    uint64_t    base;
} IDTPointer64;

/* Contexto de interrupción (stack frame) */
typedef struct PACKED {
    /* Registros guardados por el handler */
    uint64_t    r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t    rdi, rsi, rbp, rbx, rdx, rcx, rax;
    
    /* Información de la interrupción */
    uint64_t    int_no;
    uint64_t    error_code;
    
    /* Guardados automáticamente por la CPU */
    uint64_t    rip;
    uint64_t    cs;
    uint64_t    rflags;
    uint64_t    rsp;
    uint64_t    ss;
} InterruptFrame64;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void idt64_init(void);

/* Configurar handler */
void idt64_set_gate(uint8_t num, uint64_t handler, uint8_t type);

/* Handlers de interrupción */
typedef void (*InterruptHandler)(InterruptFrame64* frame);
void idt64_register_handler(uint8_t num, InterruptHandler handler);

/* Habilitar/deshabilitar interrupciones */
void enable_interrupts(void);
void disable_interrupts(void);

/* PIC */
void pic_init(void);
void pic_send_eoi(uint8_t irq);
void pic_set_mask(uint8_t irq);
void pic_clear_mask(uint8_t irq);

#endif /* IDT64_H */

