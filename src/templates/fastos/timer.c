/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Timer Driver (PIT 8254)
 * Nivel: Intermedio - Driver del timer del sistema
 * ═══════════════════════════════════════════════════════════════════════════
 * Este driver configura el Programmable Interval Timer (PIT) para generar
 * interrupciones periódicas que permiten el scheduling y medición de tiempo.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "timer.h"
#include "ports.h"
#include "idt.h"
#include "vga.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Puertos del PIT */
#define PIT_CHANNEL0        0x40
#define PIT_CHANNEL1        0x41
#define PIT_CHANNEL2        0x42
#define PIT_CMD             0x43

/* Frecuencia base del PIT (1.193182 MHz) */
#define PIT_FREQUENCY       1193182

/* Modos del PIT */
#define PIT_MODE_SQUARE     0x36    /* Canal 0, modo 3 (square wave) */

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static volatile uint64_t tick_count = 0;
static uint32_t timer_frequency = 0;

/* Callbacks del timer */
#define MAX_TIMER_CALLBACKS     8

static struct {
    timer_callback_t callback;
    uint32_t interval;          /* Intervalo en ticks */
    uint32_t next_tick;         /* Próximo tick para ejecutar */
} timer_callbacks[MAX_TIMER_CALLBACKS] = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * HANDLER DE INTERRUPCIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

static void timer_handler(void) {
    tick_count++;
    
    /* Ejecutar callbacks programados */
    for (int i = 0; i < MAX_TIMER_CALLBACKS; i++) {
        if (timer_callbacks[i].callback && 
            tick_count >= timer_callbacks[i].next_tick) {
            
            timer_callbacks[i].callback();
            timer_callbacks[i].next_tick = tick_count + timer_callbacks[i].interval;
        }
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar timer con frecuencia específica (Hz) */
void timer_init(uint32_t frequency) {
    timer_frequency = frequency;
    tick_count = 0;
    
    /* Calcular divisor */
    uint32_t divisor = PIT_FREQUENCY / frequency;
    
    /* Limitar divisor a 16 bits */
    if (divisor > 65535) {
        divisor = 65535;
        timer_frequency = PIT_FREQUENCY / divisor;
    }
    
    /* Configurar PIT */
    outb(PIT_CMD, PIT_MODE_SQUARE);
    outb(PIT_CHANNEL0, divisor & 0xFF);         /* Byte bajo */
    outb(PIT_CHANNEL0, (divisor >> 8) & 0xFF);  /* Byte alto */
    
    /* Registrar handler de interrupción (IRQ0 = INT 32) */
    register_interrupt_handler(32, timer_handler);
}

/* Obtener ticks transcurridos */
uint64_t timer_get_ticks(void) {
    return tick_count;
}

/* Obtener tiempo en milisegundos */
uint64_t timer_get_ms(void) {
    return (tick_count * 1000) / timer_frequency;
}

/* Obtener tiempo en segundos */
uint32_t timer_get_seconds(void) {
    return tick_count / timer_frequency;
}

/* Esperar un número de ticks */
void timer_wait_ticks(uint32_t ticks) {
    uint64_t target = tick_count + ticks;
    while (tick_count < target) {
        __asm__ volatile("hlt");
    }
}

/* Esperar milisegundos */
void timer_wait_ms(uint32_t ms) {
    uint32_t ticks = (ms * timer_frequency) / 1000;
    if (ticks == 0) ticks = 1;
    timer_wait_ticks(ticks);
}

/* Esperar segundos */
void timer_wait_seconds(uint32_t seconds) {
    timer_wait_ticks(seconds * timer_frequency);
}

/* Registrar callback periódico */
int timer_register_callback(timer_callback_t callback, uint32_t interval_ms) {
    for (int i = 0; i < MAX_TIMER_CALLBACKS; i++) {
        if (timer_callbacks[i].callback == NULL) {
            timer_callbacks[i].callback = callback;
            timer_callbacks[i].interval = (interval_ms * timer_frequency) / 1000;
            if (timer_callbacks[i].interval == 0) {
                timer_callbacks[i].interval = 1;
            }
            timer_callbacks[i].next_tick = tick_count + timer_callbacks[i].interval;
            return i;
        }
    }
    return -1;  /* No hay espacio */
}

/* Eliminar callback */
void timer_unregister_callback(int id) {
    if (id >= 0 && id < MAX_TIMER_CALLBACKS) {
        timer_callbacks[id].callback = NULL;
    }
}

/* Obtener frecuencia actual */
uint32_t timer_get_frequency(void) {
    return timer_frequency;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE UTILIDAD
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Formatear tiempo para mostrar */
void timer_format_uptime(char* buffer, int buffer_size) {
    uint32_t total_seconds = timer_get_seconds();
    uint32_t hours = total_seconds / 3600;
    uint32_t minutes = (total_seconds % 3600) / 60;
    uint32_t seconds = total_seconds % 60;
    
    /* Formato: HH:MM:SS */
    int i = 0;
    
    /* Horas */
    buffer[i++] = '0' + (hours / 10);
    buffer[i++] = '0' + (hours % 10);
    buffer[i++] = ':';
    
    /* Minutos */
    buffer[i++] = '0' + (minutes / 10);
    buffer[i++] = '0' + (minutes % 10);
    buffer[i++] = ':';
    
    /* Segundos */
    buffer[i++] = '0' + (seconds / 10);
    buffer[i++] = '0' + (seconds % 10);
    buffer[i++] = '\0';
}

/* Beep usando el speaker del PC */
void timer_beep(uint32_t frequency, uint32_t duration_ms) {
    if (frequency == 0) return;
    
    uint32_t divisor = PIT_FREQUENCY / frequency;
    
    /* Configurar canal 2 del PIT para el speaker */
    outb(PIT_CMD, 0xB6);    /* Canal 2, modo 3 */
    outb(PIT_CHANNEL2, divisor & 0xFF);
    outb(PIT_CHANNEL2, (divisor >> 8) & 0xFF);
    
    /* Habilitar speaker */
    uint8_t tmp = inb(0x61);
    outb(0x61, tmp | 0x03);
    
    /* Esperar */
    timer_wait_ms(duration_ms);
    
    /* Deshabilitar speaker */
    outb(0x61, tmp & 0xFC);
}

