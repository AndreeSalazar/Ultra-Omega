// ═══════════════════════════════════════════════════════════════
// LIBRERÍA: Manipulación de Strings (C)
// Nivel: Intermedio - Componente independiente
// ═══════════════════════════════════════════════════════════════
// Funciones para trabajar con cadenas de texto.

#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

// ═══════════════════════════════════════════════════════════════
// Funciones de manipulación de strings
// ═══════════════════════════════════════════════════════════════

// Convertir a mayúsculas
void str_to_upper(char* str) {
    while (*str) {
        *str = toupper(*str);
        str++;
    }
}

// Convertir a minúsculas
void str_to_lower(char* str) {
    while (*str) {
        *str = tolower(*str);
        str++;
    }
}

// Invertir string
void str_reverse(char* str) {
    int len = strlen(str);
    for (int i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
}

// Contar palabras
int str_word_count(const char* str) {
    int count = 0;
    int in_word = 0;
    
    while (*str) {
        if (isspace(*str)) {
            in_word = 0;
        } else if (!in_word) {
            in_word = 1;
            count++;
        }
        str++;
    }
    return count;
}

// Eliminar espacios al inicio y final
char* str_trim(char* str) {
    // Inicio
    while (isspace(*str)) str++;
    
    // Final
    char* end = str + strlen(str) - 1;
    while (end > str && isspace(*end)) end--;
    *(end + 1) = '\0';
    
    return str;
}

// ═══════════════════════════════════════════════════════════════
// Ejemplo de uso
// ═══════════════════════════════════════════════════════════════
int main() {
    char texto[] = "  Hola Mundo  ";
    
    printf("Original: '%s'\n", texto);
    printf("Palabras: %d\n", str_word_count(texto));
    
    char* trimmed = str_trim(texto);
    printf("Trim: '%s'\n", trimmed);
    
    str_to_upper(trimmed);
    printf("Mayúsculas: '%s'\n", trimmed);
    
    str_reverse(trimmed);
    printf("Invertido: '%s'\n", trimmed);
    
    return 0;
}

