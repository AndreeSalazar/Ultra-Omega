// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Funciones
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <string>
#include <vector>
#include <cmath>

// Función simple sin parámetros
void saludar() {
    std::cout << "🔷 ¡Hola desde Ultra-Omega!" << std::endl;
}

// Función con parámetros
int sumar(int a, int b) {
    return a + b;
}

// Función con parámetros por referencia
void duplicar(int& numero) {
    numero *= 2;
}

// Función con parámetro por referencia constante
void imprimir_vector(const std::vector<int>& numeros) {
    std::cout << "Vector: [";
    for (size_t i = 0; i < numeros.size(); i++) {
        std::cout << numeros[i];
        if (i < numeros.size() - 1) std::cout << ", ";
    }
    std::cout << "]" << std::endl;
}

// Función que devuelve múltiples valores usando struct
struct ResultadoOperacion {
    int suma;
    int producto;
    double promedio;
};

ResultadoOperacion calcular_estadisticas(const std::vector<int>& numeros) {
    ResultadoOperacion resultado = {0, 1, 0.0};
    
    if (numeros.empty()) return resultado;
    
    for (int num : numeros) {
        resultado.suma += num;
        resultado.producto *= num;
    }
    
    resultado.promedio = static_cast<double>(resultado.suma) / numeros.size();
    return resultado;
}

// Función recursiva
int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

// Función lambda (C++11)
void demostrar_lambdas() {
    std::cout << "Demostración de lambdas:" << std::endl;
    
    // Lambda simple
    auto cuadrado = [](int x) { return x * x; };
    std::cout << "5² = " << cuadrado(5) << std::endl;
    
    // Lambda con captura
    int multiplicador = 3;
    auto multiplicar = [multiplicador](int x) { return x * multiplicador; };
    std::cout << "4 × 3 = " << multiplicar(4) << std::endl;
}

int main() {
    std::cout << "🔷 Funciones en C++" << std::endl;
    std::cout << "==================" << std::endl;
    
    // Llamada a función simple
    saludar();
    
    // Función con parámetros
    int resultado = sumar(10, 5);
    std::cout << "10 + 5 = " << resultado << std::endl;
    
    // Función con referencia
    int numero = 7;
    std::cout << "Antes de duplicar: " << numero << std::endl;
    duplicar(numero);
    std::cout << "Después de duplicar: " << numero << std::endl;
    
    // Función con vector
    std::vector<int> numeros = {2, 4, 6, 8, 10};
    imprimir_vector(numeros);
    
    // Función que devuelve struct
    ResultadoOperacion stats = calcular_estadisticas(numeros);
    std::cout << "Suma: " << stats.suma << std::endl;
    std::cout << "Producto: " << stats.producto << std::endl;
    std::cout << "Promedio: " << stats.promedio << std::endl;
    
    // Función recursiva
    std::cout << "5! = " << factorial(5) << std::endl;
    
    // Demostración de lambdas
    demostrar_lambdas();
    
    return 0;
}
