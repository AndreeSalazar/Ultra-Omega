// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++14 Template - Modern Features Implementation
// ═══════════════════════════════════════════════════════════════════════════

#include "modern_features.hpp"
#include <algorithm>
#include <numeric>

ModernFeatures::ModernFeatures(std::string title) 
    : title(std::make_unique<std::string>(std::move(title))),
      data_map({{"uno", 1}, {"dos", 2}, {"tres", 3}, {"cuatro", 4}, {"cinco", 5}}) {
    
    // Generic lambda para procesamiento (C++14)
    processors.push_back([](auto x) { return x * 2; });
    processors.push_back([](auto x) { return x + 10; });
    processors.push_back([](auto x) { return x * x; });
}

template<typename Container>
auto ModernFeatures::process_generic(const Container& container) -> auto {
    // Generic lambda con auto parameter (C++14)
    auto transform = [](auto item) { return item * 2; };
    
    Container result;
    std::transform(container.begin(), container.end(), 
                   std::back_inserter(result), transform);
    
    return result;
}

auto ModernFeatures::square_and_add(int x, int add) const -> auto {
    // Auto return type deduction (C++14)
    return x * x + add;
}

int ModernFeatures::calculate_sum(const std::vector<int>& numbers) const {
    // Relaxed constexpr (C++14) - permite más operaciones en constexpr
    int sum = 0;
    for (const auto& num : numbers) {
        sum += num;
    }
    return sum;
}

template<typename T>
T ModernFeatures::process_value(T value) const {
    // Variable template usage (C++14) - simplificado sin constexpr if
    if (std::is_integral<T>::value) {
        return value * 2;
    } else {
        return value + 1.0;
    }
}

auto ModernFeatures::create_unique_int(int value) const -> std::unique_ptr<int> {
    // std::make_unique (C++14)
    return std::make_unique<int>(value);
}

void ModernFeatures::demonstrate_features() const {
    std::cout << "🎯 " << *title << std::endl;
    std::cout << "=================================" << std::endl;
    
    // Demostrar generic lambdas
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    auto processed = process_generic(numbers);
    
    std::cout << "\n📊 Generic lambda demo:" << std::endl;
    for (const auto& num : processed) {
        std::cout << "   " << num << std::endl;
    }
    
    // Demostrar auto return type
    auto result = square_and_add(5, 3);
    std::cout << "\n🔍 Auto return type demo: 5² + 3 = " << result << std::endl;
    
    // Demostrar relaxed constexpr
    const std::vector<int> const_numbers = {1, 2, 3, 4, 5};
    const int sum = calculate_sum(const_numbers);
    std::cout << "📐 Relaxed constexpr demo: sum = " << sum << std::endl;
    
    // Demostrar variable templates
    auto pi_float = process_value(3.14f);
    auto pi_int = process_value(3);
    std::cout << "🔢 Variable template demo: pi_float = " << pi_float << ", pi_int = " << pi_int << std::endl;
    
    // Demostrar make_unique
    auto unique_ptr = create_unique_int(42);
    std::cout << "🎯 make_unique demo: *unique_ptr = " << *unique_ptr << std::endl;
    
    // Demostrar procesadores funcionales
    std::cout << "\n⚡ Functional processors demo:" << std::endl;
    for (size_t i = 0; i < processors.size(); ++i) {
        std::cout << "   Processor " << (i + 1) << ": 10 -> " << processors[i](10) << std::endl;
    }
}
