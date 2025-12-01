// ═══════════════════════════════════════
// Funciones en C
// ═══════════════════════════════════════

#include <stdio.h>

// ─────────────────────────────────────
// Función simple
// ─────────────────────────────────────
int sumar(int a, int b) {
    return a + b;
}

// ─────────────────────────────────────
// Función con punteros
// ─────────────────────────────────────
void intercambiar(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

// ─────────────────────────────────────
// Función recursiva
// ─────────────────────────────────────
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// ─────────────────────────────────────
// Función con array
// ─────────────────────────────────────
int suma_array(int arr[], int len) {
    int suma = 0;
    for (int i = 0; i < len; i++) {
        suma += arr[i];
    }
    return suma;
}

// ─────────────────────────────────────
// Función inline
// ─────────────────────────────────────
static inline int max(int a, int b) {
    return (a > b) ? a : b;
}

int main() {
    // Probar funciones
    printf("5 + 3 = %d\n", sumar(5, 3));
    printf("5! = %d\n", factorial(5));
    
    int x = 10, y = 20;
    printf("Antes: x=%d, y=%d\n", x, y);
    intercambiar(&x, &y);
    printf("Después: x=%d, y=%d\n", x, y);
    
    int arr[] = {1, 2, 3, 4, 5};
    printf("Suma array: %d\n", suma_array(arr, 5));
    printf("Max(7, 3): %d\n", max(7, 3));
    
    return 0;
}

