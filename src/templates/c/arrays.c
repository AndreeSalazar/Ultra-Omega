// ═══════════════════════════════════════
// Arrays y Punteros en C
// ═══════════════════════════════════════

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // ─────────────────────────────────────
    // Arrays estáticos
    // ─────────────────────────────────────
    int numeros[5] = {10, 20, 30, 40, 50};
    
    printf("Array estático:\n");
    for (int i = 0; i < 5; i++) {
        printf("  numeros[%d] = %d\n", i, numeros[i]);
    }
    
    // ─────────────────────────────────────
    // Arrays multidimensionales
    // ─────────────────────────────────────
    int matriz[3][3] = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };
    
    printf("\nMatriz 3x3:\n");
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            printf("%d ", matriz[i][j]);
        }
        printf("\n");
    }
    
    // ─────────────────────────────────────
    // Arrays dinámicos
    // ─────────────────────────────────────
    int n = 10;
    int *dinamico = (int*)malloc(n * sizeof(int));
    
    for (int i = 0; i < n; i++) {
        dinamico[i] = i * i;
    }
    
    printf("\nArray dinámico (cuadrados):\n");
    for (int i = 0; i < n; i++) {
        printf("  %d^2 = %d\n", i, dinamico[i]);
    }
    
    free(dinamico);
    
    // ─────────────────────────────────────
    // Strings (arrays de char)
    // ─────────────────────────────────────
    char saludo[] = "Hola Mundo";
    printf("\nString: %s (longitud: %zu)\n", saludo, strlen(saludo));
    
    return 0;
}

