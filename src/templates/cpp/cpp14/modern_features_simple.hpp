// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++14 Template - Modern Features Simple Header
// ═══════════════════════════════════════════════════════════════════════════

#ifndef MODERN_FEATURES_SIMPLE_HPP
#define MODERN_FEATURES_SIMPLE_HPP

#include <iostream>
#include <string>
#include <memory>
#include <vector>
#include <map>
#include <functional>
#include <type_traits>

/**
 * @brief Clase de demostración para C++14 - Modern Features Simple
 * 
 * Esta clase demuestra las características más importantes de C++14:
 * - Generic lambdas
 * - Auto return type deduction
 * - Relaxed constexpr
 * - std::make_unique
 */
class ModernFeaturesSimple {
private:
    std::unique_ptr<std::string> title;
    std::map<std::string, int> data_map;
    std::vector<std::function<int(int)>> processors;
    
public:
    /**
     * @brief Constructor con make_unique (C++14)
     * @param title Título de la demostración
     */
    explicit ModernFeaturesSimple(std::string title);
    
    /**
     * @brief Destructor virtual
     */
    virtual ~ModernFeaturesSimple() = default;
    
    /**
     * @brief Demostración de generic lambdas (C++14)
     * @param numbers Vector de números a procesar
     * @return std::vector<int> números procesados
     */
    std::vector<int> process_with_generic_lambdas(const std::vector<int>& numbers) const;
    
    /**
     * @brief Demostración de auto return type (C++14)
     * @param x Valor a procesar
     * @param add Valor a agregar
     * @return auto resultado
     */
    auto square_and_add(int x, int add) const -> int;
    
    /**
     * @brief Demostración de std::make_unique (C++14)
     * @param value Valor a encapsular
     * @return std::unique_ptr<int> puntero único
     */
    auto create_unique_int(int value) const -> std::unique_ptr<int>;
    
    /**
     * @brief Demostración de relaxed constexpr (C++14)
     * @param numbers Vector de números
     * @return int suma total
     */
    int calculate_sum(const std::vector<int>& numbers) const;
    
    /**
     * @brief Muestra todas las características C++14
     */
    void demonstrate_features() const;
    
    // Regla del cinco (C++11/14)
    ModernFeaturesSimple(const ModernFeaturesSimple&) = delete;
    ModernFeaturesSimple(ModernFeaturesSimple&&) = default;
    ModernFeaturesSimple& operator=(const ModernFeaturesSimple&) = delete;
    ModernFeaturesSimple& operator=(ModernFeaturesSimple&&) = default;
};

#endif // MODERN_FEATURES_SIMPLE_HPP
