// ═══════════════════════════════════════════════════════════════
// LIBRERÍA: Utilidades Básicas (C)
// Nivel: Básico - Componente independiente
// ═══════════════════════════════════════════════════════════════
// Funciones utilitarias de uso común.
// Hereda a otros nodos para añadir funcionalidades básicas.

#include <stdio.h>
#include <stdlib.h>

// ═══════════════════════════════════════════════════════════════
// Macros de utilidad
// ═══════════════════════════════════════════════════════════════
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define ABS(x) ((x) < 0 ? -(x) : (x))
#define SWAP(a, b, type) { type temp = a; a = b; b = temp; }

// ═══════════════════════════════════════════════════════════════
// Funciones de impresión formateada
// ═══════════════════════════════════════════════════════════════
void print_separator(const char* title) {
    printf("\n══════════════════════════════════════\n");
    printf("  %s\n", title);
    printf("══════════════════════════════════════\n");
}

void print_int(const char* label, int value) {
    printf("%s: %d\n", label, value);
}

void print_float(const char* label, float value) {
    printf("%s: %.2f\n", label, value);
}

// ═══════════════════════════════════════════════════════════════
// Ejemplo de uso
// ═══════════════════════════════════════════════════════════════
int main() {
    print_separator("Librería de Utilidades");
    
    int a = 10, b = 5;
    print_int("MAX(10, 5)", MAX(a, b));
    print_int("MIN(10, 5)", MIN(a, b));
    print_int("ABS(-7)", ABS(-7));
    
    SWAP(a, b, int);
    print_int("Después de SWAP, a", a);
    print_int("Después de SWAP, b", b);
    
    return 0;
}

