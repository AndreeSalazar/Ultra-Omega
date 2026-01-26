// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++17 Template - Core Features
// ═══════════════════════════════════════════════════════════════════════════

#include "cpp_version_detection.hpp"

// ═══════════════════════════════════════════════════════════════════════════
// DEMOSTRACIÓN DE CARACTERÍSTICAS C++17
// ═══════════════════════════════════════════════════════════════════════════

// Inline variables (C++17)
inline constexpr int DEFAULT_TIMEOUT = 30;
inline const std::string DEFAULT_CONFIG = "default";

class ModernCPP {
private:
    std::map<std::string, int> scores;
    std::unordered_map<std::string, std::variant<int, double, std::string>> mixed_data;
    
public:
    ModernCPP() {
        scores = {{"Alice", 95}, {"Bob", 87}, {"Charlie", 92}};
        mixed_data = {
            {"age", 25},
            {"gpa", 3.8},
            {"name", std::string("John")}
        };
    }
    
    // Structured bindings con map (C++17)
    void demonstrate_structured_bindings() {
        std::cout << "\n🎯 Structured Bindings:" << std::endl;
        std::cout << "   Scores:" << std::endl;
        
        for (const auto& [name, score] : scores) {
            std::cout << "   " << name << ": " << score << std::endl;
        }
        
        // Structured bindings con tuple (C++17)
        auto person = get_person_info();
        const auto& [name, age, is_student] = person;
        std::cout << "\n   Person: " << name << ", Age: " << age 
                  << ", Student: " << (is_student ? "Yes" : "No") << std::endl;
        
        // Structured bindings con array (C++17)
        int coordinates[3] = {10, 20, 30};
        auto& [x, y, z] = coordinates;
        std::cout << "   Coordinates: " << x << ", " << y << ", " << z << std::endl;
    }
    
    // std::optional (C++17)
    std::optional<std::string> find_student(int score_threshold) {
        for (const auto& [name, score] : scores) {
            if (score >= score_threshold) {
                return name;
            }
        }
        return std::nullopt;
    }
    
    void demonstrate_optional() {
        std::cout << "\n💎 std::optional:" << std::endl;
        
        auto excellent_student = find_student(90);
        auto perfect_student = find_student(100);
        
        if (excellent_student) {
            std::cout << "   Found excellent student: " << *excellent_student << std::endl;
        }
        
        if (perfect_student) {
            std::cout << "   Found perfect student: " << *perfect_student << std::endl;
        } else {
            std::cout << "   No perfect student found" << std::endl;
        }
        
        // Optional con value_or (C++17)
        auto result = perfect_student.value_or("None");
        std::cout << "   Result with default: " << result << std::endl;
    }
    
    // std::variant (C++17)
    void demonstrate_variant() {
        std::cout << "\n🎲 std::variant:" << std::endl;
        
        for (const auto& [key, value] : mixed_data) {
            std::cout << "   " << key << ": ";
            
            std::visit([](const auto& arg) {
                using T = std::decay_t<decltype(arg)>;
                if constexpr (std::is_same_v<T, int>) {
                    std::cout << arg << " (int)";
                } else if constexpr (std::is_same_v<T, double>) {
                    std::cout << arg << " (double)";
                } else if constexpr (std::is_same_v<T, std::string>) {
                    std::cout << arg << " (string)";
                }
            }, value);
            
            std::cout << std::endl;
        }
        
        // Variant con alternative (C++17)
        std::variant<int, double, std::string> data = 42;
        std::cout << "   Current variant holds index: " << data.index() << std::endl;
        
        data = 3.14;
        std::cout << "   After change, index: " << data.index() << std::endl;
        
        // Get con type checking (C++17)
        if (auto int_ptr = std::get_if<int>(&data)) {
            std::cout << "   Int value: " << *int_ptr << std::endl;
        } else {
            std::cout << "   No int value stored" << std::endl;
        }
    }
    
    // std::string_view (C++17)
    void demonstrate_string_view() {
        std::cout << "\n👁️  std::string_view:" << std::endl;
        
        std::string text = "Hello, Modern C++17 World!";
        std::string_view view = text;
        
        std::cout << "   Full text: " << view << std::endl;
        
        // Substring sin allocation (C++17)
        std::string_view word = view.substr(7, 6);  // "Modern"
        std::cout << "   Substring: " << word << std::endl;
        
        // String view parameters (C++17)
        process_text("Temporary string literal");
        process_text(text);
    }
    
    // if constexpr (C++17)
    template<typename T>
    auto process_data(T&& data) {
        if constexpr (std::is_integral_v<T>) {
            return data * 2;
        } else if constexpr (std::is_floating_point_v<T>) {
            return data * 1.5;
        } else if constexpr (std::is_same_v<std::decay_t<T>, std::string>) {
            return data + " (processed)";
        } else {
            return std::forward<T>(data);
        }
    }
    
    void demonstrate_if_constexpr() {
        std::cout << "\n🔀 if constexpr:" << std::endl;
        
        std::cout << "   Process int: " << process_data(42) << std::endl;
        std::cout << "   Process double: " << process_data(3.14) << std::endl;
        std::cout << "   Process string: " << process_data(std::string("test")) << std::endl;
    }
    
    // std::any (C++17)
    void demonstrate_any() {
        std::cout << "\n🎭 std::any:" << std::endl;
        
        std::any data;
        
        data = 42;
        std::cout << "   Any holds int: " << std::any_cast<int>(data) << std::endl;
        
        data = 3.14;
        std::cout << "   Any holds double: " << std::any_cast<double>(data) << std::endl;
        
        data = std::string("Hello");
        std::cout << "   Any holds string: " << std::any_cast<std::string>(data) << std::endl;
        
        // Type checking (C++17)
        if (data.type() == typeid(std::string)) {
            std::cout << "   Data contains string" << std::endl;
        }
    }
    
private:
    std::tuple<std::string, int, bool> get_person_info() const {
        return {"Alice", 25, true};
    }
    
    void process_text(std::string_view text) {
        std::cout << "   Processing text view: " << text << std::endl;
        std::cout << "   Text length: " << text.length() << std::endl;
    }
};

// Fold expressions (C++17)
template<typename... Args>
auto sum_all(Args... args) {
    return (args + ... + 0);
}

template<typename... Args>
auto multiply_all(Args... args) {
    return (args * ... * 1);
}

template<typename... Args>
void print_all(Args&&... args) {
    ((std::cout << args << " "), ...);
    std::cout << std::endl;
}

int main() {
    // Inicialización con detección de versión
    ULTRA_OMEGA_CPP_INIT();
    
    std::cout << "\n🎯 Demostración de Características C++17:" << std::endl;
    std::cout << "======================================" << std::endl;
    
    ModernCPP demo;
    
    demo.demonstrate_structured_bindings();
    demo.demonstrate_optional();
    demo.demonstrate_variant();
    demo.demonstrate_string_view();
    demo.demonstrate_if_constexpr();
    demo.demonstrate_any();
    
    // Demostrar características específicas de C++17
    std::cout << "\n🔍 Verificación de características C++17:" << std::endl;
    
#if HAS_STRUCTURED_BINDINGS
    std::cout << "   ✓ Structured bindings disponibles" << std::endl;
#else
    std::cout << "   ✗ Structured bindings NO disponibles" << std::endl;
#endif

#if HAS_STD_OPTIONAL
    std::cout << "   ✓ std::optional disponible" << std::endl;
#else
    std::cout << "   ✗ std::optional NO disponible" << std::endl;
#endif

#if HAS_STD_VARIANT
    std::cout << "   ✓ std::variant disponible" << std::endl;
#else
    std::cout << "   ✗ std::variant NO disponible" << std::endl;
#endif

#if HAS_STD_STRING_VIEW
    std::cout << "   ✓ std::string_view disponible" << std::endl;
#else
    std::cout << "   ✗ std::string_view NO disponible" << std::endl;
#endif

#if HAS_IF_CONSTEXPR
    std::cout << "   ✓ if constexpr disponible" << std::endl;
#else
    std::cout << "   ✗ if constexpr NO disponible" << std::endl;
#endif

#if HAS_INLINE_VARIABLES
    std::cout << "   ✓ Inline variables disponibles" << std::endl;
#else
    std::cout << "   ✗ Inline variables NO disponibles" << std::endl;
#endif

#if HAS_STD_FILESYSTEM
    std::cout << "   ✓ std::filesystem disponible" << std::endl;
#else
    std::cout << "   ✗ std::filesystem NO disponible" << std::endl;
#endif

#if HAS_PARALLEL_ALGORITHMS
    std::cout << "   ✓ Parallel algorithms disponibles" << std::endl;
#else
    std::cout << "   ✗ Parallel algorithms NO disponibles" << std::endl;
#endif

#if HAS_FOLD_EXPRESSIONS
    std::cout << "   ✓ Fold expressions disponibles" << std::endl;
#else
    std::cout << "   ✗ Fold expressions NO disponibles" << std::endl;
#endif

#if HAS_CTAD
    std::cout << "   ✓ CTAD disponible" << std::endl;
#else
    std::cout << "   ✗ CTAD NO disponible" << std::endl;
#endif

#if HAS_CONSTEXPR_LAMBDA
    std::cout << "   ✓ constexpr lambda disponible" << std::endl;
#else
    std::cout << "   ✗ constexpr lambda NO disponible" << std::endl;
#endif
    
    // Demostrar fold expressions si están disponibles
#if HAS_FOLD_EXPRESSIONS
    std::cout << "\n🔄 Fold Expressions:" << std::endl;
    auto sum = sum_all(1, 2, 3, 4, 5);
    auto product = multiply_all(2, 3, 4);
    std::cout << "   Sum of 1,2,3,4,5: " << sum << std::endl;
    std::cout << "   Product of 2,3,4: " << product << std::endl;
    
    std::cout << "   Print all: ";
    print_all("Hello", "C++17", "Fold", "Expressions!");
#endif
    
    // Demostrar CTAD si está disponible
#if HAS_CTAD
    std::cout << "\n🎯 Class Template Argument Deduction (CTAD):" << std::endl;
    std::pair p1{1, 2};  // deducido como pair<int, int>
    std::pair p2{"hello", "world"};  // deducido como pair<const char*, const char*>
    std::vector v{1, 2, 3, 4, 5};  // deducido como vector<int>
    std::cout << "   pair p1: " << p1.first << ", " << p1.second << std::endl;
    std::cout << "   pair p2: " << p2.first << ", " << p2.second << std::endl;
    std::cout << "   vector size: " << v.size() << std::endl;
#endif
    
    std::cout << "\n✅ Programa C++17 completado exitosamente!" << std::endl;
    
    return 0;
}
