// ═══════════════════════════════════════
// C++ Moderno (C++17/20)
// ═══════════════════════════════════════

#include <iostream>
#include <optional>
#include <variant>
#include <string_view>
#include <tuple>
#include <array>

// ─────────────────────────────────────
// std::optional - Valores opcionales
// ─────────────────────────────────────
std::optional<int> dividir(int a, int b) {
    if (b == 0) return std::nullopt;
    return a / b;
}

// ─────────────────────────────────────
// std::variant - Uniones tipadas
// ─────────────────────────────────────
using Resultado = std::variant<int, double, std::string>;

void imprimir_resultado(const Resultado& r) {
    std::visit([](const auto& valor) {
        std::cout << "Valor: " << valor << std::endl;
    }, r);
}

// ─────────────────────────────────────
// Structured bindings
// ─────────────────────────────────────
std::tuple<std::string, int, double> obtener_persona() {
    return {"Juan", 25, 1.75};
}

// ─────────────────────────────────────
// constexpr - Evaluación en compilación
// ─────────────────────────────────────
constexpr int factorial(int n) {
    return (n <= 1) ? 1 : n * factorial(n - 1);
}

// ─────────────────────────────────────
// Lambda expressions avanzadas
// ─────────────────────────────────────
auto crear_multiplicador(int factor) {
    return [factor](int x) { return x * factor; };
}

int main() {
    // Optional
    std::cout << "=== std::optional ===" << std::endl;
    if (auto res = dividir(10, 2)) {
        std::cout << "10 / 2 = " << *res << std::endl;
    }
    if (auto res = dividir(10, 0)) {
        std::cout << "10 / 0 = " << *res << std::endl;
    } else {
        std::cout << "División por cero!" << std::endl;
    }
    
    // Variant
    std::cout << "\n=== std::variant ===" << std::endl;
    Resultado r1 = 42;
    Resultado r2 = 3.14;
    Resultado r3 = std::string("Hola");
    
    imprimir_resultado(r1);
    imprimir_resultado(r2);
    imprimir_resultado(r3);
    
    // Structured bindings
    std::cout << "\n=== Structured Bindings ===" << std::endl;
    auto [nombre, edad, altura] = obtener_persona();
    std::cout << nombre << ", " << edad << " años, " << altura << "m" << std::endl;
    
    // constexpr
    std::cout << "\n=== constexpr ===" << std::endl;
    constexpr int fact5 = factorial(5);
    std::cout << "5! = " << fact5 << " (calculado en compilación)" << std::endl;
    
    // Lambdas
    std::cout << "\n=== Lambdas ===" << std::endl;
    auto doble = crear_multiplicador(2);
    auto triple = crear_multiplicador(3);
    std::cout << "Doble de 5: " << doble(5) << std::endl;
    std::cout << "Triple de 5: " << triple(5) << std::endl;
    
    return 0;
}

