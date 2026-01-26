// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello Modern C++11
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <chrono>
#include <utility>

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
    
    // Auto keyword (C++11)
    std::cout << "   ✓ Auto keyword disponible" << std::endl;
    
    // Lambda expressions (C++11)
    auto lambda_test = [](int x) { return x * 2; };
    std::cout << "   ✓ Lambda expressions disponibles" << std::endl;
    
    // Smart pointers (C++11)
    auto smart_ptr = std::make_unique<int>(42);
    std::cout << "   ✓ Smart pointers disponibles" << std::endl;
    
    // nullptr (C++11)
    int* ptr = nullptr;
    std::cout << "   ✓ nullptr disponible" << std::endl;
    
    // constexpr básico (C++11)
    constexpr int multiplier = 2;
    std::cout << "   ✓ constexpr básico disponible" << std::endl;
    
    // Range-based for loop (C++11)
    std::vector<int> test_vec = {1, 2, 3};
    for (const auto& item : test_vec) {
        (void)item; // Evitar warning de unused variable
    }
    std::cout << "   ✓ Range-based for loop disponible" << std::endl;
    
    // Move semantics (C++11)
    std::string str1 = "Hola";
    std::string str2 = std::move(str1);
    std::cout << "   ✓ Move semantics disponibles" << std::endl;
    
    std::cout << "\n✅ Programa C++11 completado exitosamente!" << std::endl;
    
    return 0;
}
