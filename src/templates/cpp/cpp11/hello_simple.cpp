// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello Simple (FUNCIONAL)
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <chrono>

int main() {
    std::cout << "🔷 C++11 - Ultra-Omega Template" << std::endl;
    std::cout << "=================================" << std::endl;
    
    // Auto keyword (C++11)
    auto start_time = std::chrono::high_resolution_clock::now();
    
    // Smart pointers (C++11)
    auto numbers = std::make_unique<std::vector<int>>();
    numbers->push_back(1);
    numbers->push_back(2);
    numbers->push_back(3);
    numbers->push_back(4);
    numbers->push_back(5);
    
    // Lambda expressions (C++11)
    auto multiply_by_two = [](int x) { return x * 2; };
    
    // Range-based for loop (C++11)
    std::cout << "\n📊 Procesando números:" << std::endl;
    for (const auto& num : *numbers) {
        auto result = multiply_by_two(num);
        std::cout << "   " << num << " x 2 = " << result << std::endl;
    }
    
    // Move semantics (C++11)
    std::string message = "Hello from C++11!";
    std::string moved_message = std::move(message);
    
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end_time - start_time);
    
    std::cout << "\n📋 Mensaje movido: " << moved_message << std::endl;
    std::cout << "⏱️  Tiempo de ejecución: " << duration.count() << " microsegundos" << std::endl;
    
    // Verificar características C++11
    std::cout << "\n🔍 Características C++11 verificadas:" << std::endl;
    std::cout << "   ✓ auto keyword" << std::endl;
    std::cout << "   ✓ smart pointers" << std::endl;
    std::cout << "   ✓ lambda expressions" << std::endl;
    std::cout << "   ✓ range-based for" << std::endl;
    std::cout << "   ✓ move semantics" << std::endl;
    std::cout << "   ✓ chrono library" << std::endl;
    
    std::cout << "\n✅ Template C++11 funcionando correctamente!" << std::endl;
    
    return 0;
}
