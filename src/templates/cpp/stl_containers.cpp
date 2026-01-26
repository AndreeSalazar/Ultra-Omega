// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - STL (Standard Template Library)
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <list>
#include <map>
#include <set>
#include <queue>
#include <stack>
#include <algorithm>
#include <string>

void demostrar_vector() {
    std::cout << "📦 Vector:" << std::endl;
    std::vector<int> numeros = {5, 2, 8, 1, 9};
    
    // Agregar elementos
    numeros.push_back(3);
    numeros.insert(numeros.begin() + 2, 7);
    
    // Ordenar
    std::sort(numeros.begin(), numeros.end());
    
    // Mostrar
    for (int num : numeros) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    // Buscar
    auto it = std::find(numeros.begin(), numeros.end(), 7);
    if (it != numeros.end()) {
        std::cout << "Encontrado 7 en posición " << std::distance(numeros.begin(), it) << std::endl;
    }
    std::cout << std::endl;
}

void demostrar_map() {
    std::cout << "🗺️ Map:" << std::endl;
    std::map<std::string, int> edades;
    
    // Insertar elementos
    edades["Ana"] = 25;
    edades["Carlos"] = 30;
    edades["Luis"] = 22;
    
    // Mostrar
    for (const auto& [nombre, edad] : edades) {
        std::cout << nombre << ": " << edad << " años" << std::endl;
    }
    
    // Buscar
    if (edades.count("Ana")) {
        std::cout << "Ana tiene " << edades["Ana"] << " años" << std::endl;
    }
    std::cout << std::endl;
}

void demostrar_set() {
    std::cout << "🔢 Set:" << std::endl;
    std::set<int> numeros_unicos;
    
    // Insertar (automáticamente elimina duplicados)
    numeros_unicos.insert(5);
    numeros_unicos.insert(2);
    numeros_unicos.insert(8);
    numeros_unicos.insert(2); // Duplicado, no se agregará
    numeros_unicos.insert(5); // Duplicado, no se agregará
    
    // Mostrar (ordenado automáticamente)
    for (int num : numeros_unicos) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    std::cout << "Tamaño: " << numeros_unicos.size() << std::endl;
    std::cout << std::endl;
}

void demostrar_queue_stack() {
    std::cout << "📚 Queue (FIFO):" << std::endl;
    std::queue<std::string> cola;
    cola.push("Primero");
    cola.push("Segundo");
    cola.push("Tercero");
    
    while (!cola.empty()) {
        std::cout << cola.front() << " ";
        cola.pop();
    }
    std::cout << std::endl;
    
    std::cout << "📚 Stack (LIFO):" << std::endl;
    std::stack<int> pila;
    pila.push(1);
    pila.push(2);
    pila.push(3);
    
    while (!pila.empty()) {
        std::cout << pila.top() << " ";
        pila.pop();
    }
    std::cout << std::endl << std::endl;
}

void demostrar_algoritmos() {
    std::cout << "🔧 Algoritmos STL:" << std::endl;
    std::vector<int> datos = {3, 1, 4, 1, 5, 9, 2, 6};
    
    std::cout << "Original: ";
    for (int x : datos) std::cout << x << " ";
    std::cout << std::endl;
    
    // Encontrar mínimo y máximo
    auto min_it = std::min_element(datos.begin(), datos.end());
    auto max_it = std::max_element(datos.begin(), datos.end());
    std::cout << "Mínimo: " << *min_it << ", Máximo: " << *max_it << std::endl;
    
    // Contar
    int count_unos = std::count(datos.begin(), datos.end(), 1);
    std::cout << "Cantidad de unos: " << count_unos << std::endl;
    
    // Reemplazar
    std::replace(datos.begin(), datos.end(), 1, 99);
    std::cout << "Después de reemplazar 1 por 99: ";
    for (int x : datos) std::cout << x << " ";
    std::cout << std::endl;
    
    // Eliminar duplicados
    std::sort(datos.begin(), datos.end());
    auto last = std::unique(datos.begin(), datos.end());
    datos.erase(last, datos.end());
    std::cout << "Sin duplicados: ";
    for (int x : datos) std::cout << x << " ";
    std::cout << std::endl << std::endl;
}

int main() {
    std::cout << "🔷 Standard Template Library (STL)" << std::endl;
    std::cout << "====================================" << std::endl;
    
    demostrar_vector();
    demostrar_map();
    demostrar_set();
    demostrar_queue_stack();
    demostrar_algoritmos();
    
    return 0;
}
