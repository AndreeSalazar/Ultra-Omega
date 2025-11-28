// ═══════════════════════════════════════════════════════════════
// LIBRERÍA: Gestión de Memoria (C)
// Nivel: Avanzado - Componente independiente
// ═══════════════════════════════════════════════════════════════
// Funciones para manejo seguro de memoria dinámica.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ═══════════════════════════════════════════════════════════════
// Macros de seguridad
// ═══════════════════════════════════════════════════════════════
#define SAFE_MALLOC(ptr, size) do { \
    ptr = malloc(size); \
    if (!ptr) { \
        fprintf(stderr, "Error: malloc falló\n"); \
        exit(1); \
    } \
} while(0)

#define SAFE_FREE(ptr) do { \
    if (ptr) { \
        free(ptr); \
        ptr = NULL; \
    } \
} while(0)

#define ARRAY_NEW(type, count) ((type*)calloc(count, sizeof(type)))
#define ARRAY_RESIZE(ptr, type, count) ((type*)realloc(ptr, (count) * sizeof(type)))

// ═══════════════════════════════════════════════════════════════
// Estructura de array dinámico
// ═══════════════════════════════════════════════════════════════
typedef struct {
    int* data;
    size_t size;
    size_t capacity;
} DynamicArray;

DynamicArray* array_create(size_t initial_capacity) {
    DynamicArray* arr = malloc(sizeof(DynamicArray));
    arr->data = calloc(initial_capacity, sizeof(int));
    arr->size = 0;
    arr->capacity = initial_capacity;
    return arr;
}

void array_push(DynamicArray* arr, int value) {
    if (arr->size >= arr->capacity) {
        arr->capacity *= 2;
        arr->data = realloc(arr->data, arr->capacity * sizeof(int));
    }
    arr->data[arr->size++] = value;
}

int array_get(DynamicArray* arr, size_t index) {
    if (index >= arr->size) return -1;
    return arr->data[index];
}

void array_free(DynamicArray* arr) {
    SAFE_FREE(arr->data);
    SAFE_FREE(arr);
}

// ═══════════════════════════════════════════════════════════════
// Ejemplo de uso
// ═══════════════════════════════════════════════════════════════
int main() {
    printf("=== Gestión de Memoria ===\n\n");
    
    // Crear array dinámico
    DynamicArray* arr = array_create(4);
    
    // Añadir elementos
    for (int i = 0; i < 10; i++) {
        array_push(arr, i * 10);
    }
    
    // Mostrar elementos
    printf("Array dinámico (%zu elementos):\n", arr->size);
    for (size_t i = 0; i < arr->size; i++) {
        printf("  [%zu] = %d\n", i, array_get(arr, i));
    }
    
    // Liberar memoria
    array_free(arr);
    printf("\nMemoria liberada correctamente.\n");
    
    return 0;
}

