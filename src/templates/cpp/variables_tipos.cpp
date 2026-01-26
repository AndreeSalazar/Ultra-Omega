// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Variables y Tipos de Datos
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <string>
#include <vector>
#include <iomanip>

int main() {
    std::cout << "🔷 Variables y Tipos de Datos en C++" << std::endl;
    std::cout << "====================================" << std::endl;
    
    // Tipos fundamentales
    int entero = 42;
    float flotante = 3.14f;
    double doble_precision = 3.14159265359;
    char caracter = 'A';
    bool booleano = true;
    
    // Tipos con signed/unsigned
    unsigned int sin_signo = 100;
    long long entero_largo = 1234567890LL;
    
    // Cadena de texto
    std::string texto = "Ultra-Omega C++";
    
    // Contenedores básicos
    std::vector<int> numeros = {1, 2, 3, 4, 5};
    
    // Mostrar valores
    std::cout << std::fixed << std::setprecision(2);
    std::cout << "Entero: " << entero << std::endl;
    std::cout << "Flotante: " << flotante << std::endl;
    std::cout << "Double: " << doble_precision << std::endl;
    std::cout << "Caracter: " << caracter << std::endl;
    std::cout << "Booleano: " << std::boolalpha << booleano << std::endl;
    std::cout << "Sin signo: " << sin_signo << std::endl;
    std::cout << "Entero largo: " << entero_largo << std::endl;
    std::cout << "Texto: " << texto << std::endl;
    
    std::cout << "Vector: ";
    for (int num : numeros) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    return 0;
}
