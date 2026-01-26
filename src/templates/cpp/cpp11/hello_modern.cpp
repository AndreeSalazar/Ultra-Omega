// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello Modern C++11
// ═══════════════════════════════════════════════════════════════════════════

#include "cpp_version_detection.hpp"

// ═══════════════════════════════════════════════════════════════════════════
// DEMOSTRACIÓN DE CARACTERÍSTICAS C++11
// ═══════════════════════════════════════════════════════════════════════════

class DataProcessor {
private:
    std::vector<int> data;
    std::unique_ptr<std::string> name;
    
public:
    // Constructor con move semantics (C++11)
    DataProcessor(std::vector<int>&& input_data, std::string processor_name) 
        : data(std::move(input_data)), 
          name(std::make_unique<std::string>(std::move(processor_name))) {}
    
    // Lambda expression con auto (C++11)
    auto process_data() -> std::vector<int> {
        std::vector<int> result;
        result.reserve(data.size());
        
        // Lambda con capture (C++11)
        auto transform = [](int x) { return x * 2; };
        
        // Range-based for loop (C++11)
        for (const auto& item : data) {
            result.push_back(transform(item));
        }
        
        return result;
    }
    
    // constexpr function (C++11 básico)
    constexpr int get_multiplier() const { return 2; }
    
    // Auto return type (C++11)
    auto get_info() const -> std::pair<size_t, std::string> {
        return {data.size(), *name};
    }
};

int main() {
    // Inicialización con detección de versión
    ULTRA_OMEGA_CPP_INIT();
    
    std::cout << "\n🎯 Demostración de Características C++11:" << std::endl;
    std::cout << "======================================" << std::endl;
    
    // Auto type deduction (C++11)
    auto start_time = std::chrono::high_resolution_clock::now();
    
    // Inicializar con move semantics (C++11)
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    DataProcessor processor(std::move(numbers), "C++11 Processor");
    
    // Procesar datos
    auto results = processor.process_data();
    
    // Range-based for loop con auto (C++11)
    std::cout << "\n📊 Resultados del procesamiento:" << std::endl;
    for (const auto& result : results) {
        std::cout << "   Procesado: " << result << std::endl;
    }
    
    // Structured binding simulado con tuple (C++11)
    auto info = processor.get_info();
    std::cout << "\n📋 Información del procesador:" << std::endl;
    std::cout << "   Procesador '" << info.second << "' procesó " << info.first << " items" << std::endl;
    
    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(end_time - start_time);
    
    std::cout << "\n⏱️  Tiempo de ejecución: " << duration.count() << " microsegundos" << std::endl;
    
    // Demostrar características específicas de C++11
    std::cout << "\n🔍 Verificación de características C++11:" << std::endl;
    
#if HAS_AUTO_KEYWORD
    std::cout << "   ✓ Auto keyword disponible" << std::endl;
#else
    std::cout << "   ✗ Auto keyword NO disponible" << std::endl;
#endif

#if HAS_LAMBDA_EXPRESSIONS
    std::cout << "   ✓ Lambda expressions disponibles" << std::endl;
#else
    std::cout << "   ✗ Lambda expressions NO disponibles" << std::endl;
#endif

#if HAS_SMART_POINTERS
    std::cout << "   ✓ Smart pointers disponibles" << std::endl;
#else
    std::cout << "   ✗ Smart pointers NO disponibles" << std::endl;
#endif

#if HAS_STD_THREAD
    std::cout << "   ✓ std::thread disponible" << std::endl;
#else
    std::cout << "   ✗ std::thread NO disponible" << std::endl;
#endif

#if HAS_CONSTEXPR_BASIC
    std::cout << "   ✓ constexpr básico disponible" << std::endl;
#else
    std::cout << "   ✗ constexpr básico NO disponible" << std::endl;
#endif
    
    std::cout << "\n✅ Programa C++11 completado exitosamente!" << std::endl;
    
    return 0;
}
