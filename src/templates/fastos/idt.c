/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - IDT (Interrupt Descriptor Table)
 * Nivel: Avanzado - Manejo de interrupciones
 * ═══════════════════════════════════════════════════════════════════════════
 * Este módulo configura la IDT y maneja las interrupciones del sistema.
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "idt.h"
#include "ports.h"
#include "vga.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define IDT_ENTRIES         256

/* Puertos PIC (Programmable Interrupt Controller) */
#define PIC1_CMD            0x20
#define PIC1_DATA           0x21
#define PIC2_CMD            0xA0
#define PIC2_DATA           0xA1

/* Comandos PIC */
#define PIC_EOI             0x20    /* End of Interrupt */
#define ICW1_INIT           0x11
#define ICW4_8086           0x01

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Entrada de la IDT */
struct idt_entry {
    uint16_t base_low;      /* Bits 0-15 de la dirección del handler */
    uint16_t selector;      /* Selector de código del kernel */
    uint8_t  zero;          /* Siempre cero */
    uint8_t  flags;         /* Flags de tipo y privilegio */
    uint16_t base_high;     /* Bits 16-31 de la dirección del handler */
} __attribute__((packed));

/* Puntero a la IDT */
struct idt_ptr {
    uint16_t limit;         /* Tamaño de la IDT - 1 */
    uint32_t base;          /* Dirección de la IDT */
} __attribute__((packed));

/* Registros guardados en interrupción */
typedef struct {
    uint32_t gs, fs, es, ds;
    uint32_t edi, esi, ebp, esp, ebx, edx, ecx, eax;
    uint32_t int_no, err_code;
    uint32_t eip, cs, eflags, useresp, ss;
} registers_t;

/* ═══════════════════════════════════════════════════════════════════════════
 * VARIABLES GLOBALES
 * ═══════════════════════════════════════════════════════════════════════════
 */

static struct idt_entry idt[IDT_ENTRIES];
static struct idt_ptr idtp;

/* Handlers de interrupción */
static interrupt_handler_t interrupt_handlers[IDT_ENTRIES] = {0};

/* ═══════════════════════════════════════════════════════════════════════════
 * DECLARACIONES EXTERNAS (de kernel_entry.asm)
 * ═══════════════════════════════════════════════════════════════════════════
 */

extern void load_idt(void* idt_ptr);

/* ISRs (Interrupt Service Routines) */
extern void isr0(void);
extern void isr1(void);
extern void isr2(void);
extern void isr3(void);
extern void isr4(void);
extern void isr5(void);
extern void isr6(void);
extern void isr7(void);
extern void isr8(void);
extern void isr9(void);
extern void isr10(void);
extern void isr11(void);
extern void isr12(void);
extern void isr13(void);
extern void isr14(void);
extern void isr15(void);
extern void isr16(void);
extern void isr17(void);
extern void isr18(void);
extern void isr19(void);

/* IRQs (Interrupt Requests) */
extern void irq0(void);
extern void irq1(void);
extern void irq2(void);
extern void irq3(void);
extern void irq4(void);
extern void irq5(void);
extern void irq6(void);
extern void irq7(void);
extern void irq8(void);
extern void irq9(void);
extern void irq10(void);
extern void irq11(void);
extern void irq12(void);
extern void irq13(void);
extern void irq14(void);
extern void irq15(void);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES INTERNAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Configurar una entrada de la IDT */
static void idt_set_gate(uint8_t num, uint32_t base, uint16_t sel, uint8_t flags) {
    idt[num].base_low = base & 0xFFFF;
    idt[num].base_high = (base >> 16) & 0xFFFF;
    idt[num].selector = sel;
    idt[num].zero = 0;
    idt[num].flags = flags;
}

/* Remapear el PIC */
static void pic_remap(void) {
    uint8_t mask1, mask2;
    
    /* Guardar máscaras */
    mask1 = inb(PIC1_DATA);
    mask2 = inb(PIC2_DATA);
    
    /* Iniciar secuencia de inicialización */
    outb(PIC1_CMD, ICW1_INIT);
    io_wait();
    outb(PIC2_CMD, ICW1_INIT);
    io_wait();
    
    /* Establecer offsets de vectores */
    outb(PIC1_DATA, 0x20);      /* IRQ 0-7  -> INT 32-39 */
    io_wait();
    outb(PIC2_DATA, 0x28);      /* IRQ 8-15 -> INT 40-47 */
    io_wait();
    
    /* Configurar cascada */
    outb(PIC1_DATA, 0x04);      /* IRQ2 tiene esclavo */
    io_wait();
    outb(PIC2_DATA, 0x02);      /* Identidad de cascada */
    io_wait();
    
    /* Modo 8086 */
    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();
    
    /* Restaurar máscaras */
    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * MENSAJES DE EXCEPCIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

static const char* exception_messages[] = {
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Check",
    "Machine Check",
    "Reserved"
};

/* ═══════════════════════════════════════════════════════════════════════════
 * HANDLERS DE INTERRUPCIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Handler de ISR (excepciones del CPU) */
void isr_handler(registers_t* regs) {
    /* Si hay un handler registrado, llamarlo */
    if (interrupt_handlers[regs->int_no]) {
        interrupt_handlers[regs->int_no]();
        return;
    }
    
    /* Mostrar información de la excepción */
    vga_set_color(VGA_COLOR_WHITE, VGA_COLOR_RED);
    vga_print("\n\n*** EXCEPTION: ");
    
    if (regs->int_no < 20) {
        vga_print(exception_messages[regs->int_no]);
    } else {
        vga_print("Unknown");
    }
    
    vga_print(" ***\n");
    vga_print("Error Code: ");
    vga_print_hex(regs->err_code);
    vga_print("\nEIP: ");
    vga_print_hex(regs->eip);
    vga_print("  CS: ");
    vga_print_hex(regs->cs);
    vga_print("\nEAX: ");
    vga_print_hex(regs->eax);
    vga_print("  EBX: ");
    vga_print_hex(regs->ebx);
    vga_print("\nECX: ");
    vga_print_hex(regs->ecx);
    vga_print("  EDX: ");
    vga_print_hex(regs->edx);
    vga_print("\n");
    
    /* Halt del sistema */
    vga_print("\nSistema detenido.\n");
    while (1) {
        __asm__ volatile("cli; hlt");
    }
}

/* Handler de IRQ (interrupciones de hardware) */
void irq_handler(registers_t* regs) {
    /* Enviar EOI al PIC */
    if (regs->int_no >= 40) {
        outb(PIC2_CMD, PIC_EOI);  /* EOI al PIC esclavo */
    }
    outb(PIC1_CMD, PIC_EOI);      /* EOI al PIC maestro */
    
    /* Llamar handler si existe */
    if (interrupt_handlers[regs->int_no]) {
        interrupt_handlers[regs->int_no]();
    }
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES PÚBLICAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicializar IDT */
void idt_init(void) {
    /* Configurar puntero de IDT */
    idtp.limit = (sizeof(struct idt_entry) * IDT_ENTRIES) - 1;
    idtp.base = (uint32_t)&idt;
    
    /* Limpiar IDT */
    for (int i = 0; i < IDT_ENTRIES; i++) {
        idt_set_gate(i, 0, 0, 0);
    }
    
    /* Remapear PIC */
    pic_remap();
    
    /* Configurar ISRs (excepciones) */
    idt_set_gate(0,  (uint32_t)isr0,  0x08, 0x8E);
    idt_set_gate(1,  (uint32_t)isr1,  0x08, 0x8E);
    idt_set_gate(2,  (uint32_t)isr2,  0x08, 0x8E);
    idt_set_gate(3,  (uint32_t)isr3,  0x08, 0x8E);
    idt_set_gate(4,  (uint32_t)isr4,  0x08, 0x8E);
    idt_set_gate(5,  (uint32_t)isr5,  0x08, 0x8E);
    idt_set_gate(6,  (uint32_t)isr6,  0x08, 0x8E);
    idt_set_gate(7,  (uint32_t)isr7,  0x08, 0x8E);
    idt_set_gate(8,  (uint32_t)isr8,  0x08, 0x8E);
    idt_set_gate(9,  (uint32_t)isr9,  0x08, 0x8E);
    idt_set_gate(10, (uint32_t)isr10, 0x08, 0x8E);
    idt_set_gate(11, (uint32_t)isr11, 0x08, 0x8E);
    idt_set_gate(12, (uint32_t)isr12, 0x08, 0x8E);
    idt_set_gate(13, (uint32_t)isr13, 0x08, 0x8E);
    idt_set_gate(14, (uint32_t)isr14, 0x08, 0x8E);
    idt_set_gate(15, (uint32_t)isr15, 0x08, 0x8E);
    idt_set_gate(16, (uint32_t)isr16, 0x08, 0x8E);
    idt_set_gate(17, (uint32_t)isr17, 0x08, 0x8E);
    idt_set_gate(18, (uint32_t)isr18, 0x08, 0x8E);
    idt_set_gate(19, (uint32_t)isr19, 0x08, 0x8E);
    
    /* Configurar IRQs */
    idt_set_gate(32, (uint32_t)irq0,  0x08, 0x8E);
    idt_set_gate(33, (uint32_t)irq1,  0x08, 0x8E);
    idt_set_gate(34, (uint32_t)irq2,  0x08, 0x8E);
    idt_set_gate(35, (uint32_t)irq3,  0x08, 0x8E);
    idt_set_gate(36, (uint32_t)irq4,  0x08, 0x8E);
    idt_set_gate(37, (uint32_t)irq5,  0x08, 0x8E);
    idt_set_gate(38, (uint32_t)irq6,  0x08, 0x8E);
    idt_set_gate(39, (uint32_t)irq7,  0x08, 0x8E);
    idt_set_gate(40, (uint32_t)irq8,  0x08, 0x8E);
    idt_set_gate(41, (uint32_t)irq9,  0x08, 0x8E);
    idt_set_gate(42, (uint32_t)irq10, 0x08, 0x8E);
    idt_set_gate(43, (uint32_t)irq11, 0x08, 0x8E);
    idt_set_gate(44, (uint32_t)irq12, 0x08, 0x8E);
    idt_set_gate(45, (uint32_t)irq13, 0x08, 0x8E);
    idt_set_gate(46, (uint32_t)irq14, 0x08, 0x8E);
    idt_set_gate(47, (uint32_t)irq15, 0x08, 0x8E);
    
    /* Cargar IDT */
    load_idt(&idtp);
}

/* Registrar handler de interrupción */
void register_interrupt_handler(uint8_t n, interrupt_handler_t handler) {
    interrupt_handlers[n] = handler;
}

/* Habilitar IRQ específica */
void irq_enable(uint8_t irq) {
    uint16_t port;
    uint8_t value;
    
    if (irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
    }
    
    value = inb(port) & ~(1 << irq);
    outb(port, value);
}

/* Deshabilitar IRQ específica */
void irq_disable(uint8_t irq) {
    uint16_t port;
    uint8_t value;
    
    if (irq < 8) {
        port = PIC1_DATA;
    } else {
        port = PIC2_DATA;
        irq -= 8;
    }
    
    value = inb(port) | (1 << irq);
    outb(port, value);
}

