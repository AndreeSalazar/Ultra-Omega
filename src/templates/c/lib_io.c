// ═══════════════════════════════════════════════════════════════
// LIBRERÍA: Entrada/Salida (C)
// Nivel: Básico - Componente independiente
// ═══════════════════════════════════════════════════════════════
// Funciones para entrada y salida de datos con validación.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ═══════════════════════════════════════════════════════════════
// Macros de formato
// ═══════════════════════════════════════════════════════════════
#define PRINT_LINE() printf("────────────────────────────────────\n")
#define PRINT_TITLE(t) do { PRINT_LINE(); printf("  %s\n", t); PRINT_LINE(); } while(0)
#define NEWLINE() printf("\n")

// ═══════════════════════════════════════════════════════════════
// Funciones de entrada segura
// ═══════════════════════════════════════════════════════════════

// Leer entero con validación
int read_int(const char* prompt) {
    int value;
    printf("%s", prompt);
    while (scanf("%d", &value) != 1) {
        while (getchar() != '\n'); // Limpiar buffer
        printf("Error. Ingrese un número: ");
    }
    while (getchar() != '\n'); // Limpiar buffer
    return value;
}

// Leer float con validación
float read_float(const char* prompt) {
    float value;
    printf("%s", prompt);
    while (scanf("%f", &value) != 1) {
        while (getchar() != '\n');
        printf("Error. Ingrese un número: ");
    }
    while (getchar() != '\n');
    return value;
}

// Leer string seguro
void read_string(const char* prompt, char* buffer, size_t size) {
    printf("%s", prompt);
    if (fgets(buffer, size, stdin)) {
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len-1] == '\n') {
            buffer[len-1] = '\0';
        }
    }
}

// Leer confirmación (s/n)
int read_confirm(const char* prompt) {
    char response;
    printf("%s (s/n): ", prompt);
    scanf(" %c", &response);
    while (getchar() != '\n');
    return (response == 's' || response == 'S');
}

// ═══════════════════════════════════════════════════════════════
// Ejemplo de uso
// ═══════════════════════════════════════════════════════════════
int main() {
    PRINT_TITLE("Librería de I/O");
    
    char nombre[50];
    read_string("Tu nombre: ", nombre, sizeof(nombre));
    
    int edad = read_int("Tu edad: ");
    float altura = read_float("Tu altura (m): ");
    
    NEWLINE();
    PRINT_LINE();
    printf("Nombre: %s\n", nombre);
    printf("Edad: %d años\n", edad);
    printf("Altura: %.2f m\n", altura);
    PRINT_LINE();
    
    if (read_confirm("¿Datos correctos?")) {
        printf("¡Guardado!\n");
    } else {
        printf("Cancelado.\n");
    }
    
    return 0;
}

