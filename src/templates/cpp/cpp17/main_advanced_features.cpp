// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++17 Template - Advanced Features Main
// ═══════════════════════════════════════════════════════════════════════════

#include "advanced_features.hpp"
#include <chrono>

int main() {
    std::cout << "\n🔷 C++17 - Ultra-Omega Advanced Features" << std::endl;
    std::cout << "=============================================" << std::endl;
    
    auto start_time = std::chrono::high_resolution_clock::now();
    
    // Crear objeto usando move semantics
    auto features = AdvancedFeatures("C++17 Advanced Features Demo");
    
    // Demostrar todas las características
    features.demonstrate_features();
    
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end_time - start_time);
    
    std::cout << "\n⏱️  Tiempo de ejecución: " << duration.count() << " microsegundos" << std::endl;
    
    // Resumen de características C++17
    std::cout << "\n🔍 Características C++17 demostradas:" << std::endl;
    std::cout << "   ✓ std::string_view" << std::endl;
    std::cout << "   ✓ std::optional" << std::endl;
    std::cout << "   ✓ std::variant" << std::endl;
    std::cout << "   ✓ std::any" << std::endl;
    std::cout << "   ✓ Structured bindings" << std::endl;
    std::cout << "   ✓ std::filesystem" << std::endl;
    std::cout << "   ✓ Inline variables" << std::endl;
    std::cout << "   ✓ constexpr if" << std::endl;
    std::cout << "   ✓ Class template argument deduction" << std::endl;
    std::cout << "   ✓ std::invoke" << std::endl;
    std::cout << "   ✓ std::apply" << std::endl;
    std::cout << "   ✓ std::make_from_tuple" << std::endl;
    std::cout << "   ✓ std::byte" << std::endl;
    std::cout << "   ✓ [[nodiscard]] attribute" << std::endl;
    std::cout << "   ✓ [[maybe_unused]] attribute" << std::endl;
    std::cout << "   ✓ Nested namespace definitions" << std::endl;
    std::cout << "   ✓ Hexadecimal floating literals" << std::endl;
    std::cout << "   ✓ UTF-8 character literals" << std::endl;
    std::cout << "   ✓ Direct list initialization of enums" << std::endl;
    
    std::cout << "\n✅ Template C++17 completado exitosamente!" << std::endl;
    
    return 0;
}
