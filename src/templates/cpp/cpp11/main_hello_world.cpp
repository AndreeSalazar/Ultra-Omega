// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello World Main
// ═══════════════════════════════════════════════════════════════════════════

#include "hello_world.hpp"
#include <chrono>

int main() {
    std::cout << "\n🔷 C++11 - Ultra-Omega Hello World" << std::endl;
    std::cout << "====================================" << std::endl;
    
    // Auto keyword y smart pointers (C++11)
    auto start_time = std::chrono::high_resolution_clock::now();
    
    // Crear objeto usando move semantics (C++11)
    auto hello = HelloWorld("Hello from C++11 World!");
    
    // Mostrar mensaje
    hello.show_message();
    
    // Procesar números
    auto results = hello.process_numbers();
    
    // Range-based for loop (C++11)
    std::cout << "\n📊 Resultados del procesamiento:" << std::endl;
    for (const auto& result : results) {
        std::cout << "   Procesado: " << result << std::endl;
    }
    
    // Demostrar lambda expressions (C++11)
    auto lambda_result = hello.lambda_demo(10);
    std::cout << "\n🔍 Lambda demo result: " << lambda_result << std::endl;
    
    // Obtener información (C++11 auto return)
    auto info = hello.get_info();
    std::cout << "\n📋 Información del objeto:" << std::endl;
    std::cout << "   Mensaje: '" << info.second << "'" << std::endl;
    std::cout << "   Elementos: " << info.first << std::endl;
    
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end_time - start_time);
    
    std::cout << "\n⏱️  Tiempo de ejecución: " << duration.count() << " microsegundos" << std::endl;
    
    // Demostrar características C++11 específicas
    std::cout << "\n🔍 Características C++11 demostradas:" << std::endl;
    std::cout << "   ✓ auto keyword" << std::endl;
    std::cout << "   ✓ smart pointers (unique_ptr)" << std::endl;
    std::cout << "   ✓ lambda expressions" << std::endl;
    std::cout << "   ✓ range-based for loops" << std::endl;
    std::cout << "   ✓ move semantics" << std::endl;
    std::cout << "   ✓ uniform initialization" << std::endl;
    std::cout << "   ✓ nullptr (implícito)" << std::endl;
    std::cout << "   ✓ override keyword" << std::endl;
    std::cout << "   ✓ deleted functions" << std::endl;
    std::cout << "   ✓ defaulted functions" << std::endl;
    
    std::cout << "\n✅ Template C++11 completado exitosamente!" << std::endl;
    
    return 0;
}
