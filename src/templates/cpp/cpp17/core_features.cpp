#include <iostream>
#include <vector>
#include <string>
#include <tuple>
#include <map>
#include <unordered_map>
#include <optional>
#include <variant>
#include <any>
#include <string_view>

// C++17 Core Features Demo
// - structured bindings
// - if constexpr
// - std::optional
// - std::variant
// - std::any
// - std::string_view
// - inline variables

// Inline variable (C++17)
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
    
    // Structured bindings with map
    void demonstrate_structured_bindings() {
        std::cout << "=== Structured Bindings Demo ===" << std::endl;
        
        for (const auto& [name, score] : scores) {
            std::cout << name << ": " << score << std::endl;
        }
        
        // Structured bindings with tuple
        auto person = get_person_info();
        const auto& [name, age, is_student] = person;
        std::cout << "\nPerson: " << name << ", Age: " << age 
                  << ", Student: " << (is_student ? "Yes" : "No") << std::endl;
        
        // Structured bindings with array
        int coordinates[3] = {10, 20, 30};
        auto& [x, y, z] = coordinates;
        std::cout << "Coordinates: " << x << ", " << y << ", " << z << std::endl;
    }
    
    // std::optional
    std::optional<std::string> find_student(int score_threshold) {
        for (const auto& [name, score] : scores) {
            if (score >= score_threshold) {
                return name;
            }
        }
        return std::nullopt;
    }
    
    void demonstrate_optional() {
        std::cout << "\n=== std::optional Demo ===" << std::endl;
        
        auto excellent_student = find_student(90);
        auto perfect_student = find_student(100);
        
        if (excellent_student) {
            std::cout << "Found excellent student: " << *excellent_student << std::endl;
        }
        
        if (perfect_student) {
            std::cout << "Found perfect student: " << *perfect_student << std::endl;
        } else {
            std::cout << "No perfect student found" << std::endl;
        }
        
        // Optional with value_or
        auto result = perfect_student.value_or("None");
        std::cout << "Result with default: " << result << std::endl;
    }
    
    // std::variant
    void demonstrate_variant() {
        std::cout << "\n=== std::variant Demo ===" << std::endl;
        
        for (const auto& [key, value] : mixed_data) {
            std::cout << key << ": ";
            
            std::visit([]const auto& arg) {
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
        
        // Variant with alternative
        std::variant<int, double, std::string> data = 42;
        std::cout << "Current variant holds index: " << data.index() << std::endl;
        
        data = 3.14;
        std::cout << "After change, index: " << data.index() << std::endl;
        
        // Get with type checking
        if (auto int_ptr = std::get_if<int>(&data)) {
            std::cout << "Int value: " << *int_ptr << std::endl;
        } else {
            std::cout << "No int value stored" << std::endl;
        }
    }
    
    // std::string_view
    void demonstrate_string_view() {
        std::cout << "\n=== std::string_view Demo ===" << std::endl;
        
        std::string text = "Hello, Modern C++17 World!";
        std::string_view view = text;
        
        std::cout << "Full text: " << view << std::endl;
        
        // Substring without allocation
        std::string_view word = view.substr(7, 6);  // "Modern"
        std::cout << "Substring: " << word << std::endl;
        
        // String view parameters
        process_text("Temporary string literal");
        process_text(text);
    }
    
    // if constexpr
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
        std::cout << "\n=== if constexpr Demo ===" << std::endl;
        
        std::cout << "Process int: " << process_data(42) << std::endl;
        std::cout << "Process double: " << process_data(3.14) << std::endl;
        std::cout << "Process string: " << process_data(std::string("test")) << std::endl;
    }
    
    // std::any
    void demonstrate_any() {
        std::cout << "\n=== std::any Demo ===" << std::endl;
        
        std::any data;
        
        data = 42;
        std::cout << "Any holds int: " << std::any_cast<int>(data) << std::endl;
        
        data = 3.14;
        std::cout << "Any holds double: " << std::any_cast<double>(data) << std::endl;
        
        data = std::string("Hello");
        std::cout << "Any holds string: " << std::any_cast<std::string>(data) << std::endl;
        
        // Type checking
        if (data.type() == typeid(std::string)) {
            std::cout << "Data contains string" << std::endl;
        }
    }
    
private:
    std::tuple<std::string, int, bool> get_person_info() const {
        return {"Alice", 25, true};
    }
    
    void process_text(std::string_view text) {
        std::cout << "Processing text view: " << text << std::endl;
        std::cout << "Text length: " << text.length() << std::endl;
    }
};

int main() {
    std::cout << "C++17 Core Features Demo" << std::endl;
    std::cout << "=========================" << std::endl;
    
    ModernCPP demo;
    
    demo.demonstrate_structured_bindings();
    demo.demonstrate_optional();
    demo.demonstrate_variant();
    demo.demonstrate_string_view();
    demo.demonstrate_if_constexpr();
    demo.demonstrate_any();
    
    std::cout << "\n=== C++17 Features Complete ===" << std::endl;
    
    return 0;
}
