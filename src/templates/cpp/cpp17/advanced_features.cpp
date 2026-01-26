#include <iostream>
#include <vector>
#include <string>
#include <tuple>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include <string_view>

// C++17 Advanced Features Demo
// - fold expressions
// - class template argument deduction (CTAD)
// - constexpr lambda
// - guaranteed copy elision
// - inline variables
// - std::invoke

// Inline variables (C++17)
inline constexpr double PI = 3.14159265358979323846;
inline const std::string APP_NAME = "C++17 Advanced Demo";

// Fold expressions helper
template<typename... Args>
auto sum_all(Args... args) {
    return (args + ... + 0);
}

template<typename... Args>
auto multiply_all(Args... args) {
    return (args * ... * 1);
}

template<typename... Args>
void print_all(Args&&... args)    // Fold expression over comma operator
{
    ((std::cout << args << " "), ...);
    std::cout << std::endl;
}

// Class Template Argument Deduction (CTAD) demo
class Pair {
private:
    std::variant<int, double, std::string> first;
    std::variant<int, double, std::string> second;
    
public:
    template<typename T, typename U>
    Pair(T&& t, U&& u) : first(std::forward<T>(t)), second(std::forward<U>(u)) {}
    
    auto get_first() const { return first; }
    auto get_second() const { return second; }
};

// Deduction guide (C++17)
Pair(const char*, const char*) -> Pair<std::string, std::string>;

class AdvancedFeatures {
public:
    void demonstrate_fold_expressions() {
        std::cout << "=== Fold Expressions Demo ===" << std::endl;
        
        auto sum = sum_all(1, 2, 3, 4, 5);
        auto product = multiply_all(2, 3, 4);
        
        std::cout << "Sum of 1,2,3,4,5: " << sum << std::endl;
        std::cout << "Product of 2,3,4: " << product << std::endl;
        
        print_all("Hello", "C++17", "Fold", "Expressions!");
        print_all(1, 2.5, "mixed", 4);
    }
    
    void demonstrate_ctad() {
        std::cout << "\n=== Class Template Argument Deduction Demo ===" << std::endl;
        
        // CTAD - no need to specify template arguments
        std::pair p1{1, 2};                    // deduced as pair<int, int>
        std::pair p2{"hello", "world"};       // deduced as pair<const char*, const char*>
        std::vector v{1, 2, 3, 4, 5};         // deduced as vector<int>
        
        std::cout << "pair p1: " << p1.first << ", " << p1.second << std::endl;
        std::cout << "pair p2: " << p2.first << ", " << p2.second << std::endl;
        std::cout << "vector size: " << v.size() << std::endl;
        
        // Custom class with CTAD
        Pair custom_pair{42, 3.14};
        Pair string_pair{"C++", "17"};
        
        std::cout << "Custom pair with int and double" << std::endl;
        std::cout << "Custom pair with strings" << std::endl;
    }
    
    void demonstrate_constexpr_lambda() {
        std::cout << "\n=== constexpr Lambda Demo ===" << std::endl;
        
        // constexpr lambda (C++17)
        constexpr auto square = [](int x) { return x * x; };
        constexpr auto add = [](auto a, auto b) { return a + b; };
        
        // Compile-time evaluation
        constexpr int squared_5 = square(5);
        constexpr int sum_3_4 = add(3, 4);
        
        std::cout << "square(5) at compile time: " << squared_5 << std::endl;
        std::cout << "add(3, 4) at compile time: " << sum_3_4 << std::endl;
        
        // Runtime use
        auto runtime_square = square;
        std::cout << "square(10) at runtime: " << runtime_square(10) << std::endl;
    }
    
    void demonstrate_guaranteed_copy_elision() {
        std::cout << "\n=== Guaranteed Copy Elision Demo ===" << std::endl;
        
        // Guaranteed copy elision - no copies or moves
        auto get_large_object = []() -> std::vector<int> {
            std::vector<int> large(1000);
            std::iota(large.begin(), large.end(), 1);
            return large;  // No copy/move - direct initialization
        };
        
        auto vec = get_large_object();  // Direct initialization, no copy
        std::cout << "Vector size (no copy): " << vec.size() << std::endl;
        
        // Temporary materialization conversion
        std::vector<int> vec2 = std::vector<int>(500);  // No copy
        std::cout << "Vector2 size (no copy): " << vec2.size() << std::endl;
    }
    
    void demonstrate_std_invoke() {
        std::cout << "\n=== std::invoke Demo ===" << std::endl;
        
        auto add = [](int a, int b) { return a + b; };
        auto multiply = [](int a, int b) { return a * b; };
        
        // std::invoke with function objects
        auto result1 = std::invoke(add, 3, 4);
        auto result2 = std::invoke(multiply, 5, 6);
        
        std::cout << "invoke(add, 3, 4): " << result1 << std::endl;
        std::cout << "invoke(multiply, 5, 6): " << result2 << std::endl;
        
        // std::invoke with member functions
        class Calculator {
        public:
            int calculate(int x, int y) { return x + y; }
            int value = 42;
        };
        
        Calculator calc;
        
        // Invoke member function
        auto result3 = std::invoke(&Calculator::calculate, calc, 10, 20);
        std::cout << "invoke(member function): " << result3 << std::endl;
        
        // Invoke member data
        auto result4 = std::invoke(&Calculator::value, calc);
        std::cout << "invoke(member data): " << result4 << std::endl;
    }
    
    void demonstrate_inline_variables() {
        std::cout << "\n=== Inline Variables Demo ===" << std::endl;
        
        std::cout << "PI constant: " << PI << std::endl;
        std::cout << "App name: " << APP_NAME << std::endl;
        
        // Inline variables can be defined in header files
        // without violating ODR (One Definition Rule)
    }
    
    void demonstrate_lambda_capture_this() {
        std::cout << "\n=== Lambda Capture *this Demo ===" << std::endl;
        
        class Widget {
        private:
            int value = 100;
            
        public:
            void demonstrate() {
                // Capture *this by value (C++17)
                auto lambda = [*this]() {
                    std::cout << "Captured widget value: " << value << std::endl;
                };
                
                lambda();
                
                // Modify original
                value = 200;
                std::cout << "Original widget value: " << value << std::endl;
                
                // Lambda still has old value (captured by copy)
                lambda();
            }
        };
        
        Widget widget;
        widget.demonstrate();
    }
    
    void demonstrate_attributes() {
        std::cout << "\n=== New Attributes Demo ===" << std::endl;
        
        // [[maybe_unused]] attribute
        [[maybe_unused]] int unused_var = 42;
        
        // [[nodiscard]] attribute
        [[nodiscard]] int important_function() {
            return 100;
        }
        
        auto result = important_function();  // Warning if result is ignored
        std::cout << "Important function result: " << result << std::endl;
        
        // [[fallthrough]] attribute in switch
        int value = 2;
        switch (value) {
            case 1:
                std::cout << "Case 1" << std::endl;
                [[fallthrough]];
            case 2:
                std::cout << "Case 2 (fallthrough from case 1)" << std::endl;
                break;
            default:
                std::cout << "Default case" << std::endl;
        }
    }
};

int main() {
    std::cout << "C++17 Advanced Features Demo" << std::endl;
    std::cout << "=============================" << std::endl;
    
    AdvancedFeatures demo;
    
    demo.demonstrate_fold_expressions();
    demo.demonstrate_ctad();
    demo.demonstrate_constexpr_lambda();
    demo.demonstrate_guaranteed_copy_elision();
    demo.demonstrate_std_invoke();
    demo.demonstrate_inline_variables();
    demo.demonstrate_lambda_capture_this();
    demo.demonstrate_attributes();
    
    std::cout << "\n=== C++17 Advanced Features Complete ===" << std::endl;
    
    return 0;
}
