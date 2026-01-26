#pragma once

#include <iostream>

// ═══════════════════════════════════════════════════════════════════════════
// DETECCIÓN DE VERSIÓN C++ - Ultra-Omega
// ═══════════════════════════════════════════════════════════════════════════

// Detectar versión de C++ en tiempo de compilación
#if __cplusplus >= 201703L
    #define CPP_VERSION 17
    #define CPP_VERSION_STR "C++17"
    #define CPP_STANDARD_AVAILABLE 1
    #define HAS_STRUCTURED_BINDINGS 1
    #define HAS_STD_OPTIONAL 1
    #define HAS_STD_VARIANT 1
    #define HAS_STD_STRING_VIEW 1
    #define HAS_STD_ANY 1
    #define HAS_IF_CONSTEXPR 1
    #define HAS_INLINE_VARIABLES 1
    #define HAS_STD_FILESYSTEM 1
    #define HAS_PARALLEL_ALGORITHMS 1
    #define HAS_FOLD_EXPRESSIONS 1
    #define HAS_CTAD 1
    #define HAS_CONSTEXPR_LAMBDA 1
#elif __cplusplus >= 201402L
    #define CPP_VERSION 14
    #define CPP_VERSION_STR "C++14"
    #define CPP_STANDARD_AVAILABLE 1
    #define HAS_GENERIC_LAMBDAS 1
    #define HAS_AUTO_RETURN_TYPE 1
    #define HAS_VARIABLE_TEMPLATES 1
    #define HAS_STD_MAKE_UNIQUE 1
    #define HAS_RELAXED_CONSTEXPR 1
    #define HAS_STD_SHARED_TIMED_MUTEX 1
    #define HAS_STD_INVOKE 1
    #define HAS_CHRONO_LITERALS 1
    #define HAS_COMPLEX_LITERALS 1
#elif __cplusplus >= 201103L
    #define CPP_VERSION 11
    #define CPP_VERSION_STR "C++11"
    #define CPP_STANDARD_AVAILABLE 1
    #define HAS_AUTO_KEYWORD 1
    #define HAS_RANGE_BASED_FOR 1
    #define HAS_LAMBDA_EXPRESSIONS 1
    #define HAS_SMART_POINTERS 1
    #define HAS_MOVE_SEMANTICS 1
    #define HAS_CONSTEXPR_BASIC 1
    #define HAS_NOEXCEPT 1
    #define HAS_ENUM_CLASS 1
    #define HAS_INITIALIZER_LISTS 1
    #define HAS_STD_THREAD 1
    #define HAS_STD_MUTEX 1
    #define HAS_STD_ATOMIC 1
    #define HAS_STD_CHRONO 1
    #define HAS_STD_TUPLE 1
    #define HAS_STD_ARRAY 1
    #define HAS_STD_UNORDERED_MAP 1
    #define HAS_STD_FORWARD_LIST 1
#else
    #define CPP_VERSION 0
    #define CPP_VERSION_STR "C++98/03 o anterior"
    #define CPP_STANDARD_AVAILABLE 0
    #error "Este código requiere C++11 o superior. Por favor usa -std=c++11 o superior."
#endif

// ═══════════════════════════════════════════════════════════════════════════
// MACROS DE DETECCIÓN DE CARACTERÍSTICAS
// ═══════════════════════════════════════════════════════════════════════════

// Mensaje de bienvenida con versión detectada
#define PRINT_CPP_VERSION() \
    std::cout << "╔══════════════════════════════════════════════════════════════╗" << std::endl; \
    std::cout << "║ Ultra-Omega C++ Template - Versión Detectada: " << CPP_VERSION_STR << "           ║" << std::endl; \
    std::cout << "║ Compilado con estándar: " << __cplusplus << "                              ║" << std::endl; \
    std::cout << "╚══════════════════════════════════════════════════════════════╝" << std::endl

// Macros para características específicas
#if HAS_AUTO_KEYWORD
    #define FEATURE_AUTO "✓ Auto keyword disponible"
#else
    #define FEATURE_AUTO "✗ Auto keyword NO disponible"
#endif

#if HAS_LAMBDA_EXPRESSIONS
    #define FEATURE_LAMBDA "✓ Lambda expressions disponibles"
#else
    #define FEATURE_LAMBDA "✗ Lambda expressions NO disponibles"
#endif

#if HAS_SMART_POINTERS
    #define FEATURE_SMART_PTR "✓ Smart pointers disponibles"
#else
    #define FEATURE_SMART_PTR "✗ Smart pointers NO disponibles"
#endif

#if HAS_GENERIC_LAMBDAS
    #define FEATURE_GENERIC_LAMBDA "✓ Generic lambdas disponibles"
#else
    #define FEATURE_GENERIC_LAMBDA "✗ Generic lambdas NO disponibles"
#endif

#if HAS_STRUCTURED_BINDINGS
    #define FEATURE_STRUCTURED "✓ Structured bindings disponibles"
#else
    #define FEATURE_STRUCTURED "✗ Structured bindings NO disponibles"
#endif

#if HAS_STD_OPTIONAL
    #define FEATURE_OPTIONAL "✓ std::optional disponible"
#else
    #define FEATURE_OPTIONAL "✗ std::optional NO disponible"
#endif

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIÓN DE REPORT DE CARACTERÍSTICAS
// ═══════════════════════════════════════════════════════════════════════════

inline void print_cpp_features() {
    std::cout << "\n🔍 Características C++ Disponibles:" << std::endl;
    std::cout << "   " << FEATURE_AUTO << std::endl;
    std::cout << "   " << FEATURE_LAMBDA << std::endl;
    std::cout << "   " << FEATURE_SMART_PTR << std::endl;
    
#if CPP_VERSION >= 14
    std::cout << "   " << FEATURE_GENERIC_LAMBDA << std::endl;
#endif

#if CPP_VERSION >= 17
    std::cout << "   " << FEATURE_STRUCTURED << std::endl;
    std::cout << "   " << FEATURE_OPTIONAL << std::endl;
#endif

    std::cout << "\n📋 Información del Compilador:" << std::endl;
    
#if defined(_MSC_VER)
    std::cout << "   Compilador: MSVC " << _MSC_VER << std::endl;
#elif defined(__GNUC__)
    std::cout << "   Compilador: GCC " << __GNUC__ << "." << __GNUC_MINOR__ << "." << __GNUC_PATCHLEVEL__ << std::endl;
#elif defined(__clang__)
    std::cout << "   Compilador: Clang " << __clang_major__ << "." << __clang_minor__ << "." << __clang_patchlevel__ << std::endl;
#else
    std::cout << "   Compilador: Desconocido" << std::endl;
#endif

    std::cout << "   Estándar C++: " << CPP_VERSION_STR << std::endl;
    std::cout << "   __cplusplus: " << __cplusplus << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPATIBILIDAD DE HEADERS
// ═══════════════════════════════════════════════════════════════════════════

// Incluir headers según la versión disponible
#if CPP_VERSION >= 11
    #include <memory>
    #include <vector>
    #include <string>
    #include <algorithm>
    #include <chrono>
    #include <thread>
    #include <mutex>
    #include <atomic>
    #include <tuple>
    #include <array>
    #include <unordered_map>
    #include <unordered_set>
    #include <forward_list>
#endif

#if CPP_VERSION >= 14
    #include <utility>
    #include <complex>
#endif

#if CPP_VERSION >= 17
    #include <optional>
    #include <variant>
    #include <any>
    #include <string_view>
    #include <filesystem>
    #include <execution>
#endif

// ═══════════════════════════════════════════════════════════════════════════
// NAMESPACE PARA UTILIDADES DE ULTRA-OMEGA
// ═══════════════════════════════════════════════════════════════════════════

namespace ultra_omega {
    inline void show_version_info() {
        PRINT_CPP_VERSION();
        print_cpp_features();
    }
    
    inline bool is_cpp11_or_higher() { return CPP_VERSION >= 11; }
    inline bool is_cpp14_or_higher() { return CPP_VERSION >= 14; }
    inline bool is_cpp17_or_higher() { return CPP_VERSION >= 17; }
    
    inline const char* get_version_string() { return CPP_VERSION_STR; }
    inline int get_version_number() { return CPP_VERSION; }
}

// ═══════════════════════════════════════════════════════════════════════════
// MACRO FINAL DE INICIALIZACIÓN
// ═══════════════════════════════════════════════════════════════════════════

#define ULTRA_OMEGA_CPP_INIT() \
    ultra_omega::show_version_info(); \
    std::cout << "\n🚀 Iniciando programa Ultra-Omega C++..." << std::endl
