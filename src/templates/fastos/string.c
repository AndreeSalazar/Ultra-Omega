/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - String Library
 * Nivel: Básico - Funciones de manipulación de strings
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "string.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE LONGITUD
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Longitud de string */
size_t strlen(const char* str) {
    size_t len = 0;
    while (str[len]) len++;
    return len;
}

/* Longitud con límite */
size_t strnlen(const char* str, size_t maxlen) {
    size_t len = 0;
    while (len < maxlen && str[len]) len++;
    return len;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE COPIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Copiar string */
char* strcpy(char* dest, const char* src) {
    char* d = dest;
    while ((*d++ = *src++));
    return dest;
}

/* Copiar string con límite */
char* strncpy(char* dest, const char* src, size_t n) {
    size_t i;
    for (i = 0; i < n && src[i]; i++) {
        dest[i] = src[i];
    }
    for (; i < n; i++) {
        dest[i] = '\0';
    }
    return dest;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE CONCATENACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Concatenar strings */
char* strcat(char* dest, const char* src) {
    char* d = dest;
    while (*d) d++;
    while ((*d++ = *src++));
    return dest;
}

/* Concatenar con límite */
char* strncat(char* dest, const char* src, size_t n) {
    char* d = dest;
    while (*d) d++;
    while (n-- && (*d++ = *src++));
    *d = '\0';
    return dest;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE COMPARACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Comparar strings */
int strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

/* Comparar con límite */
int strncmp(const char* s1, const char* s2, size_t n) {
    while (n && *s1 && (*s1 == *s2)) {
        s1++;
        s2++;
        n--;
    }
    if (n == 0) return 0;
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

/* Comparar ignorando mayúsculas/minúsculas */
int strcasecmp(const char* s1, const char* s2) {
    while (*s1 && *s2) {
        char c1 = *s1, c2 = *s2;
        if (c1 >= 'A' && c1 <= 'Z') c1 += 32;
        if (c2 >= 'A' && c2 <= 'Z') c2 += 32;
        if (c1 != c2) return c1 - c2;
        s1++;
        s2++;
    }
    return *s1 - *s2;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE BÚSQUEDA
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Buscar carácter */
char* strchr(const char* str, int c) {
    while (*str) {
        if (*str == (char)c) return (char*)str;
        str++;
    }
    return (c == '\0') ? (char*)str : NULL;
}

/* Buscar carácter desde el final */
char* strrchr(const char* str, int c) {
    const char* last = NULL;
    while (*str) {
        if (*str == (char)c) last = str;
        str++;
    }
    return (c == '\0') ? (char*)str : (char*)last;
}

/* Buscar substring */
char* strstr(const char* haystack, const char* needle) {
    if (!*needle) return (char*)haystack;
    
    while (*haystack) {
        const char* h = haystack;
        const char* n = needle;
        
        while (*h && *n && (*h == *n)) {
            h++;
            n++;
        }
        
        if (!*n) return (char*)haystack;
        haystack++;
    }
    
    return NULL;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE CONVERSIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* String a entero */
int atoi(const char* str) {
    int result = 0;
    int sign = 1;
    
    /* Saltar espacios */
    while (*str == ' ') str++;
    
    /* Manejar signo */
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    /* Convertir dígitos */
    while (*str >= '0' && *str <= '9') {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return sign * result;
}

/* String a long */
long atol(const char* str) {
    long result = 0;
    int sign = 1;
    
    while (*str == ' ') str++;
    
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    while (*str >= '0' && *str <= '9') {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return sign * result;
}

/* Entero a string */
char* itoa(int value, char* str, int base) {
    char* ptr = str;
    char* ptr1 = str;
    char tmp_char;
    int tmp_value;
    
    /* Manejar números negativos para base 10 */
    if (value < 0 && base == 10) {
        *ptr++ = '-';
        ptr1++;
        value = -value;
    }
    
    /* Convertir a string (orden inverso) */
    do {
        tmp_value = value;
        value /= base;
        *ptr++ = "0123456789ABCDEF"[tmp_value - value * base];
    } while (value);
    
    *ptr-- = '\0';
    
    /* Invertir string */
    while (ptr1 < ptr) {
        tmp_char = *ptr;
        *ptr-- = *ptr1;
        *ptr1++ = tmp_char;
    }
    
    return str;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE TRANSFORMACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Convertir a mayúsculas */
char* strupr(char* str) {
    char* s = str;
    while (*s) {
        if (*s >= 'a' && *s <= 'z') {
            *s -= 32;
        }
        s++;
    }
    return str;
}

/* Convertir a minúsculas */
char* strlwr(char* str) {
    char* s = str;
    while (*s) {
        if (*s >= 'A' && *s <= 'Z') {
            *s += 32;
        }
        s++;
    }
    return str;
}

/* Invertir string */
char* strrev(char* str) {
    char* start = str;
    char* end = str + strlen(str) - 1;
    char tmp;
    
    while (start < end) {
        tmp = *start;
        *start++ = *end;
        *end-- = tmp;
    }
    
    return str;
}

/* Eliminar espacios al inicio y final */
char* strtrim(char* str) {
    char* end;
    
    /* Eliminar espacios al inicio */
    while (*str == ' ') str++;
    
    if (*str == '\0') return str;
    
    /* Eliminar espacios al final */
    end = str + strlen(str) - 1;
    while (end > str && *end == ' ') end--;
    
    *(end + 1) = '\0';
    
    return str;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE VERIFICACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

int isdigit(int c) {
    return c >= '0' && c <= '9';
}

int isalpha(int c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

int isalnum(int c) {
    return isdigit(c) || isalpha(c);
}

int isspace(int c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\f' || c == '\v';
}

int isupper(int c) {
    return c >= 'A' && c <= 'Z';
}

int islower(int c) {
    return c >= 'a' && c <= 'z';
}

int toupper(int c) {
    if (c >= 'a' && c <= 'z') return c - 32;
    return c;
}

int tolower(int c) {
    if (c >= 'A' && c <= 'Z') return c + 32;
    return c;
}

