// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++17 Template - Advanced Features Implementation
// ═══════════════════════════════════════════════════════════════════════════

#include "advanced_features.hpp"
#include <algorithm>
#include <numeric>

AdvancedFeatures::AdvancedFeatures(std::string title) 
    : title(std::make_unique<std::string>(std::move(title))),
      data_variants({42, 3.14159, std::string("Hello C++17")}),
      optional_data({{"clave1", 100}, {"clave2", std::nullopt}, {"clave3", 300}}) {
    
    // Almacenar diferentes tipos en std::any
    any_data.push_back(42);
    any_data.push_back(3.14);
    any_data.push_back(std::string("C++17"));
    any_data.push_back(std::vector<int>{1, 2, 3, 4, 5});
}

auto AdvancedFeatures::process_string_view(std::string_view text) const -> std::string_view {
    // std::string_view - vista sin copia (C++17)
    auto first_space = text.find(' ');
    if (first_space != std::string_view::npos) {
        return text.substr(0, first_space);
    }
    return text;
}

auto AdvancedFeatures::find_optional(const std::string& key) const -> std::optional<int> {
    // std::optional - valor que puede o no existir (C++17)
    auto it = optional_data.find(key);
    if (it != optional_data.end()) {
        return it->second;
    }
    return std::nullopt;
}

auto AdvancedFeatures::process_variant(size_t index) const -> std::string {
    // std::variant - unión tipo segura (C++17)
    if (index >= data_variants.size()) {
        return "Index out of range";
    }
    
    return std::visit([](auto&& arg) -> std::string {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, int>) {
            return "int: " + std::to_string(arg);
        } else if constexpr (std::is_same_v<T, double>) {
            return "double: " + std::to_string(arg);
        } else if constexpr (std::is_same_v<T, std::string>) {
            return "string: " + arg;
        }
        return "unknown";
    }, data_variants[index]);
}

template<typename T>
void AdvancedFeatures::store_any(T value) {
    // std::any - tipo dinámico (C++17)
    any_data.push_back(value);
}

auto AdvancedFeatures::create_structured_binding() const -> std::tuple<int, double, std::string> {
    // Structured bindings preparation (C++17)
    return {42, 3.14159, "Structured Binding Demo"};
}

auto AdvancedFeatures::process_filesystem_path(const std::string& path) const -> std::filesystem::path {
    // std::filesystem - sistema de archivos (C++17)
    std::filesystem::path fs_path(path);
    
    std::cout << "   Path: " << fs_path << std::endl;
    std::cout << "   Filename: " << fs_path.filename() << std::endl;
    std::cout << "   Extension: " << fs_path.extension() << std::endl;
    std::cout << "   Parent: " << fs_path.parent_path() << std::endl;
    
    return fs_path;
}

template<typename T>
auto AdvancedFeatures::constexpr_if_demo(T value) const -> auto {
    // constexpr if - compilación condicional (C++17)
    if constexpr (std::is_integral_v<T>) {
        return value * 2;
    } else if constexpr (std::is_floating_point_v<T>) {
        return value + 1.0;
    } else {
        return value; // Para strings, devolver el valor directamente
    }
}

void AdvancedFeatures::demonstrate_features() const {
    std::cout << "🎯 " << *title << std::endl;
    std::cout << "==================================" << std::endl;
    
    // Demostrar string_view
    std::string_view text = "Hello C++17 World";
    auto first_word = process_string_view(text);
    std::cout << "\n📝 String view demo: '" << first_word << "'" << std::endl;
    
    // Demostrar optional
    auto opt_value = find_optional("clave1");
    if (opt_value) {
        std::cout << "🔍 Optional demo: clave1 = " << *opt_value << std::endl;
    } else {
        std::cout << "🔍 Optional demo: clave1 = nullopt" << std::endl;
    }
    
    // Demostrar variant
    std::cout << "\n🎭 Variant demo:" << std::endl;
    for (size_t i = 0; i < data_variants.size(); ++i) {
        std::cout << "   [" << i << "] " << process_variant(i) << std::endl;
    }
    
    // Demostrar any
    std::cout << "\n🎯 Any demo:" << std::endl;
    for (size_t i = 0; i < std::min(any_data.size(), size_t(3)); ++i) {
        std::cout << "   [" << i << "] ";
        if (any_data[i].type() == typeid(int)) {
            std::cout << "int: " << std::any_cast<int>(any_data[i]);
        } else if (any_data[i].type() == typeid(double)) {
            std::cout << "double: " << std::any_cast<double>(any_data[i]);
        } else if (any_data[i].type() == typeid(std::string)) {
            std::cout << "string: " << std::any_cast<std::string>(any_data[i]);
        } else {
            std::cout << "complex type";
        }
        std::cout << std::endl;
    }
    
    // Demostrar structured bindings
    auto [int_val, double_val, str_val] = create_structured_binding();
    std::cout << "\n🔗 Structured bindings demo:" << std::endl;
    std::cout << "   int: " << int_val << ", double: " << double_val << ", string: " << str_val << std::endl;
    
    // Demostrar filesystem
    std::cout << "\n📁 Filesystem demo:" << std::endl;
    process_filesystem_path("C:/Users/usuario/documentos/archivo.txt");
    
    // Demostrar constexpr if
    std::cout << "\n⚡ constexpr if demo:" << std::endl;
    std::cout << "   int(5) -> " << constexpr_if_demo(5) << std::endl;
    std::cout << "   double(3.14) -> " << constexpr_if_demo(3.14) << std::endl;
    std::cout << "   string('test') -> " << constexpr_if_demo(std::string("test")) << std::endl;
    
    // Demostrar inline variable
    std::cout << "\n🔧 Inline variable demo: " << APP_VERSION << std::endl;
}
