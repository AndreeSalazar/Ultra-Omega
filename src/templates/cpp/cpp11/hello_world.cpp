// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello World Implementation
// ═══════════════════════════════════════════════════════════════════════════

#include "hello_world.hpp"
#include <algorithm>

// Constructor con inicialización uniforme (C++11)
HelloWorld::HelloWorld(std::string msg) 
    : message(std::make_unique<std::string>(std::move(msg))),
      numbers({1, 2, 3, 4, 5}) {
    // Inicialización usando lista de inicialización (C++11)
}

void HelloWorld::show_message() const {
    std::cout << "🎯 " << *message << std::endl;
    std::cout << "=================================" << std::endl;
}

std::vector<int> HelloWorld::process_numbers() const {
    std::vector<int> results;
    results.reserve(numbers.size());
    
    // Lambda expression con capture (C++11)
    auto multiply_by_two = [](int x) { return x * 2; };
    
    // Range-based for loop (C++11)
    for (const auto& num : numbers) {
        results.push_back(multiply_by_two(num));
    }
    
    return results;
}

auto HelloWorld::lambda_demo(int value) const -> int {
    // Lambda compleja con múltiples capturas (C++11)
    auto processor = [this, value](int multiplier) -> int {
        return value * multiplier * numbers.size();
    };
    
    return processor(2);
}

auto HelloWorld::get_info() const -> std::pair<size_t, std::string> {
    return {numbers.size(), *message};
}
