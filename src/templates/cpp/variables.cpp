// ═══════════════════════════════════════
// Variables y Tipos de Datos en C++
// ═══════════════════════════════════════

#include <iostream>
#include <string>
#include <vector>
#include <cstdint>

int main() {
    // Tipos básicos
    int entero = 42;
    double decimal = 3.14159;
    char caracter = 'A';
    bool booleano = true;
    
    // Tipos modernos C++
    auto automatico = 100;  // int
    auto flotante = 3.14f;  // float
    
    // Strings de C++
    std::string texto = "Hola C++";
    
    // Constantes
    const int CONSTANTE = 100;
    constexpr int COMPILE_TIME = 200;
    
    // Tipos de tamaño fijo
    int32_t i32 = -2147483648;
    uint64_t u64 = 18446744073709551615ULL;
    
    // Vectores (arrays dinámicos)
    std::vector<int> numeros = {1, 2, 3, 4, 5};
    
    // Imprimir valores
    std::cout << "Entero: " << entero << std::endl;
    std::cout << "Decimal: " << decimal << std::endl;
    std::cout << "Texto: " << texto << std::endl;
    std::cout << "Bool: " << std::boolalpha << booleano << std::endl;
    
    std::cout << "Vector: ";
    for (const auto& n : numeros) {
        std::cout << n << " ";
    }
    std::cout << std::endl;
    
    return 0;
}

