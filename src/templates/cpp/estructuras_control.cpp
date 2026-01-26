// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Estructuras de Control
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <string>

int main() {
    std::cout << "🔷 Estructuras de Control en C++" << std::endl;
    std::cout << "=================================" << std::endl;
    
    // IF-ELSE
    int edad = 18;
    if (edad >= 18) {
        std::cout << "Eres mayor de edad" << std::endl;
    } else {
        std::cout << "Eres menor de edad" << std::endl;
    }
    
    // SWITCH-CASE
    int dia = 3;
    switch (dia) {
        case 1:
            std::cout << "Lunes" << std::endl;
            break;
        case 2:
            std::cout << "Martes" << std::endl;
            break;
        case 3:
            std::cout << "Miércoles" << std::endl;
            break;
        default:
            std::cout << "Otro día" << std::endl;
            break;
    }
    
    // BUCLE FOR tradicional
    std::cout << "Números del 1 al 5 (for): ";
    for (int i = 1; i <= 5; i++) {
        std::cout << i << " ";
    }
    std::cout << std::endl;
    
    // BUCLE FOR basado en rango (C++11)
    std::vector<std::string> frutas = {"Manzana", "Naranja", "Plátano"};
    std::cout << "Frutas (range-based for): ";
    for (const auto& fruta : frutas) {
        std::cout << fruta << " ";
    }
    std::cout << std::endl;
    
    // BUCLE WHILE
    int contador = 0;
    std::cout << "Contador (while): ";
    while (contador < 3) {
        std::cout << contador << " ";
        contador++;
    }
    std::cout << std::endl;
    
    // BUCLE DO-WHILE
    int numero = 1;
    std::cout << "Do-While: ";
    do {
        std::cout << numero << " ";
        numero++;
    } while (numero <= 3);
    std::cout << std::endl;
    
    return 0;
}
