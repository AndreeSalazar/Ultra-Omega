// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++14 Template - Generic Features
// ═══════════════════════════════════════════════════════════════════════════

#include "cpp_version_detection.hpp"

// ═══════════════════════════════════════════════════════════════════════════
// DEMOSTRACIÓN DE CARACTERÍSTICAS C++14
// ═══════════════════════════════════════════════════════════════════════════

// Variable templates (C++14)
template<typename T>
constexpr T pi = T(3.1415926535897932385);

template<typename T>
constexpr T e = T(2.7182818284590452353);

class Calculator {
private:
    std::string name;
    
public:
    Calculator(std::string calc_name) : name(std::move(calc_name)) {}
    
    // Auto return type deduction (C++14)
    auto add(auto a, auto b) {
        return a + b;
    }
    
    auto multiply(auto a, auto b) {
        return a * b;
    }
    
    // Generic lambda (C++14)
    auto calculate_area(auto radius) {
        // Generic lambda inside function
        auto area_calc = [](auto r) {
            return pi<decltype(r)> * r * r;
        };
        return area_calc(radius);
    }
    
    // Multiple return types with auto (C++14)
    auto get_info() const {
        return std::make_pair(name, pi<double>);
    }
    
    // SFINAE con auto (C++14)
    template<typename T>
    auto process_number(T value) -> decltype(value * 2) {
        static_assert(std::is_arithmetic<T>::value, "T must be arithmetic");
        return value * 2;
    }
    
    [[deprecated("Use get_info() instead")]]
    std::string get_old_name() const {
        return name;
    }
};

// Generic lambda at global scope (C++14)
auto generic_transform = [](auto container, auto func) {
    using Container = std::decay_t<decltype(container)>;
    Container result;
    result.reserve(container.size());
    
    for (const auto& item : container) {
        result.push_back(func(item));
    }
    
    return result;
};

// Variable template for type checking (C++14)
template<typename T>
constexpr bool is_numeric = std::is_arithmetic<T>::value;

int main() {
    // Inicialización con detección de versión
    ULTRA_OMEGA_CPP_INIT();
    
    std::cout << "\n🎯 Demostración de Características C++14:" << std::endl;
    std::cout << "======================================" << std::endl;
    
    // Variable templates (C++14)
    std::cout << "\n📐 Variable Templates:" << std::endl;
    std::cout << "   pi (float): " << pi<float> << std::endl;
    std::cout << "   pi (double): " << pi<double> << std::endl;
    std::cout << "   e (double): " << e<double> << std::endl;
    
    Calculator calc("C++14 Calculator");
    
    // Auto return type con diferentes tipos (C++14)
    auto sum_int = calc.add(5, 3);
    auto sum_double = calc.add(3.14, 2.86);
    auto sum_string = calc.add(std::string("Hello, "), std::string("C++14!"));
    
    std::cout << "\n➕ Auto Return Type:" << std::endl;
    std::cout << "   Sum int: " << sum_int << std::endl;
    std::cout << "   Sum double: " << sum_double << std::endl;
    std::cout << "   Sum string: " << sum_string << std::endl;
    
    // Area calculation con variable templates (C++14)
    auto area = calc.calculate_area(5.0);
    std::cout << "\n📊 Cálculo de área (r=5): " << area << std::endl;
    
    // Generic lambda usage (C++14)
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    auto doubled = generic_transform(numbers, [](auto x) { return x * 2; });
    
    std::cout << "\n🔄 Generic Lambda:" << std::endl;
    std::cout << "   Original: ";
    for (auto num : numbers) std::cout << num << " ";
    std::cout << std::endl;
    
    std::cout << "   Duplicado: ";
    for (auto num : doubled) std::cout << num << " ";
    std::cout << std::endl;
    
    // Type checking con variable templates (C++14)
    std::cout << "\n🔍 Variable Templates para Type Checking:" << std::endl;
    std::cout << "   is_numeric<int>: " << is_numeric<int> << std::endl;
    std::cout << "   is_numeric<std::string>: " << is_numeric<std::string> << std::endl;
    
    // std::make_unique (C++14)
    auto unique_calc = std::make_unique<Calculator>("Unique Calculator");
    auto unique_info = unique_calc->get_info();
    std::cout << "\n🎯 std::make_unique:" << std::endl;
    std::cout << "   Unique calculator: " << unique_info.first << std::endl;
    
    // Process diferentes numeric types (C++14)
    auto processed_int = calc.process_number(42);
    auto processed_double = calc.process_number(3.14159);
    
    std::cout << "\n⚙️  SFINAE con decltype:" << std::endl;
    std::cout << "   Processed int: " << processed_int << std::endl;
    std::cout << "   Processed double: " << processed_double << std::endl;
    
    // Demostrar características específicas de C++14
    std::cout << "\n🔍 Verificación de características C++14:" << std::endl;
    
#if HAS_GENERIC_LAMBDAS
    std::cout << "   ✓ Generic lambdas disponibles" << std::endl;
#else
    std::cout << "   ✗ Generic lambdas NO disponibles" << std::endl;
#endif

#if HAS_AUTO_RETURN_TYPE
    std::cout << "   ✓ Auto return type deduction disponible" << std::endl;
#else
    std::cout << "   ✗ Auto return type deduction NO disponible" << std::endl;
#endif

#if HAS_VARIABLE_TEMPLATES
    std::cout << "   ✓ Variable templates disponibles" << std::endl;
#else
    std::cout << "   ✗ Variable templates NO disponibles" << std::endl;
#endif

#if HAS_STD_MAKE_UNIQUE
    std::cout << "   ✓ std::make_unique disponible" << std::endl;
#else
    std::cout << "   ✗ std::make_unique NO disponible" << std::endl;
#endif

#if HAS_RELAXED_CONSTEXPR
    std::cout << "   ✓ Relaxed constexpr disponible" << std::endl;
#else
    std::cout << "   ✗ Relaxed constexpr NO disponible" << std::endl;
#endif

#if HAS_CHRONO_LITERALS
    std::cout << "   ✓ Chrono literals disponibles" << std::endl;
#else
    std::cout << "   ✗ Chrono literals NO disponibles" << std::endl;
#endif
    
    // Demostrar chrono literals si están disponibles
#if HAS_CHRONO_LITERALS
    using namespace std::chrono_literals;
    auto duration = 2h + 30min + 45s;
    std::cout << "\n⏰ Chrono Literals: " << duration.count() << " seconds" << std::endl;
#endif
    
    std::cout << "\n✅ Programa C++14 completado exitosamente!" << std::endl;
    
    return 0;
}
