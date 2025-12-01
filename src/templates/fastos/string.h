/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - String Library Header
 * Nivel: Básico - Definiciones de funciones de string
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef STRING_H
#define STRING_H

#include "types.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE LONGITUD
 * ═══════════════════════════════════════════════════════════════════════════
 */

size_t strlen(const char* str);
size_t strnlen(const char* str, size_t maxlen);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE COPIA
 * ═══════════════════════════════════════════════════════════════════════════
 */

char* strcpy(char* dest, const char* src);
char* strncpy(char* dest, const char* src, size_t n);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE CONCATENACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

char* strcat(char* dest, const char* src);
char* strncat(char* dest, const char* src, size_t n);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE COMPARACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

int strcmp(const char* s1, const char* s2);
int strncmp(const char* s1, const char* s2, size_t n);
int strcasecmp(const char* s1, const char* s2);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE BÚSQUEDA
 * ═══════════════════════════════════════════════════════════════════════════
 */

char* strchr(const char* str, int c);
char* strrchr(const char* str, int c);
char* strstr(const char* haystack, const char* needle);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE CONVERSIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

int atoi(const char* str);
long atol(const char* str);
char* itoa(int value, char* str, int base);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE TRANSFORMACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

char* strupr(char* str);
char* strlwr(char* str);
char* strrev(char* str);
char* strtrim(char* str);

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES DE VERIFICACIÓN
 * ═══════════════════════════════════════════════════════════════════════════
 */

int isdigit(int c);
int isalpha(int c);
int isalnum(int c);
int isspace(int c);
int isupper(int c);
int islower(int c);
int toupper(int c);
int tolower(int c);

#endif /* STRING_H */

