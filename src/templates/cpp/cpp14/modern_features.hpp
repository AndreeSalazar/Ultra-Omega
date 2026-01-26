// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++14 Template - Modern Features Header
// ═══════════════════════════════════════════════════════════════════════════

#ifndef MODERN_FEATURES_HPP
#define MODERN_FEATURES_HPP

#include <iostream>
#include <string>
#include <memory>
#include <vector>
#include <map>
#include <functional>

/**
 * @brief Clase de demostración para C++14 - Modern Features
 * 
 * Esta clase demuestra las características avanzadas de C++14:
 * - Generic lambdas
 * - Auto return type deduction
 * - Relaxed constexpr
 * - Variable templates
 * - std::make_unique
 */
class ModernFeatures {
private:
    std::unique_ptr<std::string> title;
    std::map<std::string, int> data_map;
    std::vector<std::function<int(int)>> processors;
    
public:
    /**
     * @brief Constructor con make_unique (C++14)
     * @param title Título de la demostración
     */
    explicit ModernFeatures(std::string title);
    
    /**
     * @brief Destructor virtual
     */
    virtual ~ModernFeatures() = default;
    
    /**
     * @brief Demostración de generic lambdas (C++14)
     * @param container Contenedor a procesar
     * @return auto resultado procesado
     */
    template<typename Container>
    auto process_generic(const Container& container) const -> auto;
    
    /**
     * @brief Auto return type deduction (C++14)
     * @param x Valor a procesar
     * @return auto resultado
     */
    auto square_and_add(int x, int add) const -> auto;
    
    /**
     * @brief Relaxed constexpr (C++14)
     * @param numbers Vector de números
     * @return int suma total
     */
    int calculate_sum(const std::vector<int>& numbers) const;
    
    /**
     * @brief Variable templates demo (C++14)
     * @tparam T Tipo de dato
     * @param value Valor a procesar
     * @return T valor procesado
     */
    template<typename T>
    T process_value(T value) const;
    
    /**
     * @brief Demostración de std::make_unique (C++14)
     * @param value Valor a encapsular
     * @return std::unique_ptr<int> puntero único
     */
    auto create_unique_int(int value) const -> std::unique_ptr<int>;
    
    /**
     * @brief Muestra todas las características C++14
     */
    void demonstrate_features() const;
    
    // Regla del cinco (C++11/14)
    ModernFeatures(const ModernFeatures&) = delete;
    ModernFeatures(ModernFeatures&&) = default;
    ModernFeatures& operator=(const ModernFeatures&) = delete;
    ModernFeatures& operator=(ModernFeatures&&) = default;
};

// Variable template (C++14)
template<typename T>
constexpr T pi = T(3.14159265358979323846);

#endif // MODERN_FEATURES_HPP
