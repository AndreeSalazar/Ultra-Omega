// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++17 Template - Advanced Features Header
// ═══════════════════════════════════════════════════════════════════════════

#ifndef ADVANCED_FEATURES_HPP
#define ADVANCED_FEATURES_HPP

#include <iostream>
#include <string>
#include <memory>
#include <vector>
#include <map>
#include <optional>
#include <variant>
#include <any>
#include <string_view>
#include <tuple>
#include <algorithm>
#include <filesystem>

/**
 * @brief Clase de demostración para C++17 - Advanced Features
 * 
 * Esta clase demuestra las características más avanzadas de C++17:
 * - std::string_view
 * - std::optional
 * - std::variant
 * - std::any
 * - Structured bindings
 * - std::filesystem
 * - Inline variables
 * - constexpr if
 * - Class template argument deduction
 */
class AdvancedFeatures {
private:
    std::unique_ptr<std::string> title;
    std::vector<std::variant<int, double, std::string>> data_variants;
    std::map<std::string, std::optional<int>> optional_data;
    std::vector<std::any> any_data;
    
public:
    /**
     * @brief Constructor con make_unique
     * @param title Título de la demostración
     */
    explicit AdvancedFeatures(std::string title);
    
    /**
     * @brief Destructor virtual
     */
    virtual ~AdvancedFeatures() = default;
    
    /**
     * @brief Demostración de std::string_view (C++17)
     * @param text Vista de string a procesar
     * @return std::string_view resultado
     */
    auto process_string_view(std::string_view text) const -> std::string_view;
    
    /**
     * @brief Demostración de std::optional (C++17)
     * @param key Clave a buscar
     * @return std::optional<int> valor encontrado o nullopt
     */
    auto find_optional(const std::string& key) const -> std::optional<int>;
    
    /**
     * @brief Demostración de std::variant (C++17)
     * @param index Índice del variant a procesar
     * @return std::string resultado del procesamiento
     */
    auto process_variant(size_t index) const -> std::string;
    
    /**
     * @brief Demostración de std::any (C++17)
     * @param value Valor a almacenar
     */
    template<typename T>
    void store_any(T value);
    
    /**
     * @brief Demostración de structured bindings (C++17)
     * @return std::tuple<int, double, std::string> tupla de valores
     */
    auto create_structured_binding() const -> std::tuple<int, double, std::string>;
    
    /**
     * @brief Demostración de std::filesystem (C++17)
     * @param path Ruta a procesar
     * @return std::filesystem::path resultado
     */
    auto process_filesystem_path(const std::string& path) const -> std::filesystem::path;
    
    /**
     * @brief Demostración de constexpr if (C++17)
     * @tparam T Tipo a procesar
     * @param value Valor a procesar
     * @return auto resultado
     */
    template<typename T>
    auto constexpr_if_demo(T value) const -> auto;
    
    /**
     * @brief Muestra todas las características C++17
     */
    void demonstrate_features() const;
    
    // Regla del cinco (C++17)
    AdvancedFeatures(const AdvancedFeatures&) = delete;
    AdvancedFeatures(AdvancedFeatures&&) = default;
    AdvancedFeatures& operator=(const AdvancedFeatures&) = delete;
    AdvancedFeatures& operator=(AdvancedFeatures&&) = default;
};

// Inline variable (C++17)
inline const std::string APP_VERSION = "Ultra-Omega C++17 v1.0";

#endif // ADVANCED_FEATURES_HPP
