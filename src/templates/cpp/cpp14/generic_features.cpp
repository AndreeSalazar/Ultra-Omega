#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <utility>
#include <type_traits>

// C++14 Features Demo
// - generic lambdas
// - auto return type deduction
// - variable templates
// - std::make_unique
// - deprecated attribute
// - [[deprecated]] attribute

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
    
    // Generic lambda capture and usage
    auto calculate_area(auto radius) {
        // Generic lambda inside function
        auto area_calc = [](auto r) {
            return pi<decltype(r)> * r * r;
        };
        return area_calc(radius);
    }
    
    // Multiple return types with auto
    auto get_info() const {
        return std::make_pair(name, pi<double>);
    }
    
    // SFINAE with auto
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

// Generic lambda at global scope
auto generic_transform = [](auto container, auto func) {
    using Container = decltype(container);
    Container result;
    result.reserve(container.size());
    
    for (const auto& item : container) {
        result.push_back(func(item));
    }
    
    return result;
};

// Variable template for type checking
template<typename T>
constexpr bool is_numeric = std::is_arithmetic<T>::value;

int main() {
    std::cout << "C++14 Features Demo" << std::endl;
    std::cout << "===================" << std::endl;
    
    // Variable templates
    std::cout << "pi (float): " << pi<float> << std::endl;
    std::cout << "pi (double): " << pi<double> << std::endl;
    std::cout << "e (double): " << e<double> << std::endl;
    
    Calculator calc("C++14 Calculator");
    
    // Auto return type with different types
    auto sum_int = calc.add(5, 3);
    auto sum_double = calc.add(3.14, 2.86);
    auto sum_string = calc.add(std::string("Hello, "), std::string("C++14!"));
    
    std::cout << "Sum int: " << sum_int << std::endl;
    std::cout << "Sum double: " << sum_double << std::endl;
    std::cout << "Sum string: " << sum_string << std::endl;
    
    // Area calculation with variable templates
    auto area = calc.calculate_area(5.0);
    std::cout << "Circle area (r=5): " << area << std::endl;
    
    // Generic lambda usage
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    auto doubled = generic_transform(numbers, [](auto x) { return x * 2; });
    
    std::cout << "Original: ";
    for (auto num : numbers) std::cout << num << " ";
    std::cout << std::endl;
    
    std::cout << "Doubled: ";
    for (auto num : doubled) std::cout << num << " ";
    std::cout << std::endl;
    
    // Type checking with variable templates
    std::cout << "is_numeric<int>: " << is_numeric<int> << std::endl;
    std::cout << "is_numeric<std::string>: " << is_numeric<std::string> << std::endl;
    
    // std::make_unique (C++14)
    auto unique_calc = std::make_unique<Calculator>("Unique Calculator");
    auto unique_info = unique_calc->get_info();
    std::cout << "Unique calculator: " << unique_info.first << std::endl;
    
    // Process different numeric types
    auto processed_int = calc.process_number(42);
    auto processed_double = calc.process_number(3.14159);
    
    std::cout << "Processed int: " << processed_int << std::endl;
    std::cout << "Processed double: " << processed_double << std::endl;
    
    return 0;
}
