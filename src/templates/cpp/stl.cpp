// ═══════════════════════════════════════
// STL - Standard Template Library
// ═══════════════════════════════════════

#include <iostream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <numeric>

int main() {
    // ─────────────────────────────────────
    // Vector
    // ─────────────────────────────────────
    std::vector<int> vec = {5, 2, 8, 1, 9, 3};
    
    std::cout << "=== Vector ===" << std::endl;
    std::cout << "Original: ";
    for (int n : vec) std::cout << n << " ";
    std::cout << std::endl;
    
    // Ordenar
    std::sort(vec.begin(), vec.end());
    std::cout << "Ordenado: ";
    for (int n : vec) std::cout << n << " ";
    std::cout << std::endl;
    
    // Sumar todos
    int suma = std::accumulate(vec.begin(), vec.end(), 0);
    std::cout << "Suma: " << suma << std::endl;
    
    // ─────────────────────────────────────
    // Map (diccionario)
    // ─────────────────────────────────────
    std::map<std::string, int> edades;
    edades["Juan"] = 25;
    edades["María"] = 30;
    edades["Pedro"] = 22;
    
    std::cout << "\n=== Map ===" << std::endl;
    for (const auto& [nombre, edad] : edades) {
        std::cout << nombre << ": " << edad << " años" << std::endl;
    }
    
    // ─────────────────────────────────────
    // Set (conjunto único)
    // ─────────────────────────────────────
    std::set<int> conjunto = {3, 1, 4, 1, 5, 9, 2, 6, 5};
    
    std::cout << "\n=== Set (únicos, ordenados) ===" << std::endl;
    for (int n : conjunto) std::cout << n << " ";
    std::cout << std::endl;
    
    // ─────────────────────────────────────
    // Algoritmos
    // ─────────────────────────────────────
    std::vector<int> nums = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    
    std::cout << "\n=== Algoritmos ===" << std::endl;
    
    // Filtrar pares
    std::vector<int> pares;
    std::copy_if(nums.begin(), nums.end(), std::back_inserter(pares),
                 [](int n) { return n % 2 == 0; });
    
    std::cout << "Pares: ";
    for (int n : pares) std::cout << n << " ";
    std::cout << std::endl;
    
    // Transformar (cuadrados)
    std::vector<int> cuadrados;
    std::transform(nums.begin(), nums.end(), std::back_inserter(cuadrados),
                   [](int n) { return n * n; });
    
    std::cout << "Cuadrados: ";
    for (int n : cuadrados) std::cout << n << " ";
    std::cout << std::endl;
    
    return 0;
}

