// ═══════════════════════════════════════
// Estructuras en C
// ═══════════════════════════════════════

#include <stdio.h>
#include <string.h>

// ─────────────────────────────────────
// Definición de estructura
// ─────────────────────────────────────
typedef struct {
    char nombre[50];
    int edad;
    float altura;
} Persona;

// ─────────────────────────────────────
// Estructura con punteros
// ─────────────────────────────────────
typedef struct Nodo {
    int valor;
    struct Nodo *siguiente;
} Nodo;

// ─────────────────────────────────────
// Funciones para estructuras
// ─────────────────────────────────────
void imprimir_persona(const Persona *p) {
    printf("Nombre: %s\n", p->nombre);
    printf("Edad: %d años\n", p->edad);
    printf("Altura: %.2f m\n", p->altura);
}

Persona crear_persona(const char *nombre, int edad, float altura) {
    Persona p;
    strncpy(p.nombre, nombre, sizeof(p.nombre) - 1);
    p.nombre[sizeof(p.nombre) - 1] = '\0';
    p.edad = edad;
    p.altura = altura;
    return p;
}

int main() {
    // Crear e inicializar estructura
    Persona persona1 = {"Juan García", 25, 1.75f};
    
    printf("=== Persona 1 ===\n");
    imprimir_persona(&persona1);
    
    // Usar función constructora
    Persona persona2 = crear_persona("María López", 30, 1.65f);
    
    printf("\n=== Persona 2 ===\n");
    imprimir_persona(&persona2);
    
    // Lista enlazada simple
    Nodo n1 = {10, NULL};
    Nodo n2 = {20, NULL};
    Nodo n3 = {30, NULL};
    
    n1.siguiente = &n2;
    n2.siguiente = &n3;
    
    printf("\n=== Lista Enlazada ===\n");
    Nodo *actual = &n1;
    while (actual != NULL) {
        printf("Valor: %d\n", actual->valor);
        actual = actual->siguiente;
    }
    
    return 0;
}

