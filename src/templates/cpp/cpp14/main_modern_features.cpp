// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++14 Template - Modern Features Main
// ═══════════════════════════════════════════════════════════════════════════

#include "modern_features.hpp"
#include <chrono>

int main() {
    std::cout << "\n🔷 C++14 - Ultra-Omega Modern Features" << std::endl;
    std::cout << "==========================================" << std::endl;
    
    auto start_time = std::chrono::high_resolution_clock::now();
    
    // Crear objeto usando move semantics
    auto features = ModernFeatures("C++14 Modern Features Demo");
    
    // Demostrar todas las características
    features.demonstrate_features();
    
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end_time - start_time);
    
    std::cout << "\n⏱️  Tiempo de ejecución: " << duration.count() << " microsegundos" << std::endl;
    
    // Resumen de características C++14
    std::cout << "\n🔍 Características C++14 demostradas:" << std::endl;
    std::cout << "   ✓ Generic lambdas (auto parameters)" << std::endl;
    std::cout << "   ✓ Auto return type deduction" << std::endl;
    std::cout << "   ✓ Relaxed constexpr functions" << std::endl;
    std::cout << "   ✓ Variable templates" << std::endl;
    std::cout << "   ✓ std::make_unique" << std::endl;
    std::cout << "   ✓ std::shared_timed_mutex" << std::endl;
    std::cout << "   ✓ std::exchange" << std::endl;
    std::cout << "   ✓ std::get (by type)" << std::endl;
    std::cout << "   ✓ std::quoted" << std::endl;
    std::cout << "   ✓ Heterogeneous lookup in associative containers" << std::endl;
    std::cout << "   ✓ Standard user-defined literals" << std::endl;
    
    std::cout << "\n✅ Template C++14 completado exitosamente!" << std::endl;
    
    return 0;
}
