#include <iostream>
#include <vector>
#include <memory>
#include <algorithm>
#include <string>
#include <tuple>

// C++14 Template Metaprogramming Demo
// - relaxed constexpr
// - variable templates
// - generic lambdas with templates
// - decltype(auto) perfect forwarding

template<typename T>
constexpr T factorial(T n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

// Variable template for type traits
template<typename T>
constexpr bool is_pointer = std::is_pointer<T>::value;

template<typename T>
constexpr bool is_integral = std::is_integral<T>::value;

class TemplateMeta {
public:
    // Relaxed constexpr (C++14) - more complex than C++11
    constexpr int fibonacci(int n) const {
        if (n <= 1) return n;
        int a = 0, b = 1;
        for (int i = 2; i <= n; ++i) {
            int temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
    
    // Perfect forwarding with decltype(auto)
    template<typename T>
    decltype(auto) forward_and_process(T&& arg) {
        return process_impl(std::forward<T>(arg));
    }
    
private:
    template<typename T>
    auto process_impl(T&& value) -> decltype(auto) {
        if constexpr (is_pointer<T>) {
            return *value;  // Dereference pointer
        } else if constexpr (is_integral<T>) {
            return value * 2;  // Double integral values
        } else {
            return std::forward<T>(value);  // Forward as-is
        }
    }
    
public:
    // Generic lambda with template-like behavior
    auto create_processor() {
        return [](auto container, auto predicate) {
            using Container = std::decay_t<decltype(container)>;
            Container result;
            
            std::copy_if(container.begin(), container.end(), 
                        std::back_inserter(result), predicate);
            
            return result;
        };
    }
    
    // Variable templates in action
    template<typename T>
    void analyze_type(T&& value) {
        std::cout << "Type analysis:" << std::endl;
        std::cout << "  Is pointer: " << is_pointer<T> << std::endl;
        std::cout << "  Is integral: " << is_integral<T> << std::endl;
        
        auto processed = forward_and_process(std::forward<T>(value));
        std::cout << "  Processed value: " << processed << std::endl;
    }
    
    // Compile-time calculations
    void demonstrate_constexpr() {
        constexpr int fact_5 = factorial(5);
        constexpr int fib_10 = fibonacci(10);
        
        std::cout << "Factorial(5): " << fact_5 << std::endl;
        std::cout << "Fibonacci(10): " << fib_10 << std::endl;
        
        // Constexpr with complex logic
        constexpr bool is_power_of_two(int n) {
            return n > 0 && (n & (n - 1)) == 0;
        }
        
        std::cout << "Is 16 power of two: " << is_power_of_two(16) << std::endl;
        std::cout << "Is 15 power of two: " << is_power_of_two(15) << std::endl;
    }
};

// Variable template for constants
template<typename T>
constexpr T gravity = T(9.81);

template<typename T>
constexpr T speed_of_light = T(299792458);

int main() {
    std::cout << "C++14 Template Metaprogramming Demo" << std::endl;
    std::cout << "====================================" << std::endl;
    
    TemplateMeta meta;
    
    // Demonstrate constexpr improvements
    meta.demonstrate_constexpr();
    
    // Variable templates for physics constants
    std::cout << "\nPhysics constants:" << std::endl;
    std::cout << "Gravity (float): " << gravity<float> << " m/s²" << std::endl;
    std::cout << "Gravity (double): " << gravity<double> << " m/s²" << std::endl;
    std::cout << "Speed of light: " << speed_of_light<long> << " m/s" << std::endl;
    
    // Type analysis with variable templates
    std::cout << "\nType analysis:" << std::endl;
    
    int x = 42;
    meta.analyze_type(x);
    
    double d = 3.14159;
    meta.analyze_type(d);
    
    std::string s = "C++14";
    meta.analyze_type(s);
    
    int* ptr = &x;
    meta.analyze_type(ptr);
    
    // Generic lambda processor
    auto processor = meta.create_processor();
    
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    auto even_numbers = processor(numbers, [](auto n) { return n % 2 == 0; });
    
    std::cout << "\nEven numbers: ";
    for (int n : even_numbers) {
        std::cout << n << " ";
    }
    std::cout << std::endl;
    
    std::vector<std::string> words = {"hello", "world", "C++14", "programming"};
    auto long_words = processor(words, [](const auto& s) { return s.length() > 4; });
    
    std::cout << "Long words: ";
    for (const auto& word : long_words) {
        std::cout << word << " ";
    }
    std::cout << std::endl;
    
    // Perfect forwarding demonstration
    std::cout << "\nPerfect forwarding with decltype(auto):" << std::endl;
    
    auto&& ref1 = meta.forward_and_process(x);      // lvalue
    auto&& ref2 = meta.forward_and_process(42);     // rvalue
    auto&& ref3 = meta.forward_and_process(ptr);    // pointer
    
    std::cout << "Forwarded int: " << ref1 << std::endl;
    std::cout << "Forwarded rvalue: " << ref2 << std::endl;
    std::cout << "Forwarded pointer: " << ref3 << std::endl;
    
    return 0;
}
