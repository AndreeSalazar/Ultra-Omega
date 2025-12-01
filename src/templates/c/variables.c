// ═══════════════════════════════════════
// Variables y Tipos de Datos en C
// ═══════════════════════════════════════

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

int main() {
    // Tipos enteros
    char c = 'A';
    short s = 1000;
    int i = 100000;
    long l = 1000000L;
    long long ll = 10000000000LL;
    
    // Tipos sin signo
    unsigned int ui = 4294967295U;
    
    // Tipos de tamaño fijo
    int8_t i8 = -128;
    uint8_t u8 = 255;
    int32_t i32 = -2147483648;
    uint64_t u64 = 18446744073709551615ULL;
    
    // Flotantes
    float f = 3.14159f;
    double d = 3.141592653589793;
    
    // Booleanos
    bool verdadero = true;
    bool falso = false;
    
    // Imprimir valores
    printf("char: %c\n", c);
    printf("int: %d\n", i);
    printf("long long: %lld\n", ll);
    printf("float: %.5f\n", f);
    printf("double: %.15f\n", d);
    printf("bool: %s\n", verdadero ? "true" : "false");
    
    return 0;
}

