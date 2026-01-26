# ═══════════════════════════════════════════════════════════════════════════
# CMakeLists.txt - Ultra-Omega C++17 Template
# ═══════════════════════════════════════════════════════════════════════════

cmake_minimum_required(VERSION 3.12)

# Nombre del proyecto
project(UltraOmega_CPP17 
    VERSION 1.0.0
    DESCRIPTION "Ultra-Omega C++17 Template"
    LANGUAGES CXX
)

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN DE ESTÁNDAR C++17
# ═══════════════════════════════════════════════════════════════════════════

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN DE COMPILADOR
# ═══════════════════════════════════════════════════════════════════════════

# Opciones de compilación
option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(ENABLE_WARNINGS "Enable all warnings" ON)
option(ENABLE_DEBUG "Enable debug symbols" ON)
option(ENABLE_OPTIMIZATIONS "Enable aggressive optimizations" ON)
option(ENABLE_PARALLEL "Enable parallel algorithms support" ON)
option(ENABLE_FILESYSTEM "Enable filesystem support" ON)

# Configuración de flags según el compilador
if(MSVC)
    # Microsoft Visual C++
    add_compile_options(/W4 /std:c++17)  # C++17 explícito
    if(ENABLE_DEBUG)
        add_compile_options(/Zi /Od)
    else()
        add_compile_options(/O2)
    endif()
    add_compile_definitions(_CRT_SECURE_NO_WARNINGS)
else()
    # GCC / Clang
    add_compile_options(-Wall -Wextra -Wpedantic)
    if(ENABLE_DEBUG)
        add_compile_options(-g -O0)
    else()
        add_compile_options(-O2)
    endif()
    
    # Optimizaciones específicas de C++17
    if(ENABLE_OPTIMIZATIONS)
        if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
            add_compile_options(-flto -march=native)
        elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            add_compile_options(-flto)
        endif()
    endif()
    
    # Específicos de GCC
    if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
        add_compile_options(-fdiagnostics-color=always)
    endif()
    
    # Específicos de Clang
    if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
        add_compile_options(-fcolor-diagnostics)
    endif()
endif()

# ═══════════════════════════════════════════════════════════════════════════
# BÚSQUEDA DE PAQUETES
# ═══════════════════════════════════════════════════════════════════════════

find_package(Threads REQUIRED)

# Soporte para filesystem (C++17)
if(ENABLE_FILESYSTEM)
    if(MSVC)
        # MSVC tiene soporte nativo para filesystem
        add_compile_definitions(HAS_STD_FILESYSTEM=1)
    else()
        # GCC/Clang pueden requerir -lstdc++fs
        if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
            target_link_libraries(${PROJECT_NAME} PRIVATE -lstdc++fs)
        elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            target_link_libraries(${PROJECT_NAME} PRIVATE -lc++fs)
        endif()
        add_compile_definitions(HAS_STD_FILESYSTEM=1)
    endif()
endif()

# ═══════════════════════════════════════════════════════════════════════════
# FUENTES DEL PROYECTO
# ═══════════════════════════════════════════════════════════════════════════

# Archivos fuente (automático)
file(GLOB_RECURSE SOURCES 
    "src/*.cpp"
    "src/*.cc"
    "*.cpp"
    "*.cc"
)

# Headers (para IDEs)
file(GLOB_RECURSE HEADERS
    "include/*.hpp"
    "include/*.h"
    "*.hpp"
    "*.h"
)

# ═══════════════════════════════════════════════════════════════════════════
# EJECUTABLE PRINCIPAL
# ═══════════════════════════════════════════════════════════════════════════

# Crear ejecutable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# ═══════════════════════════════════════════════════════════════════════════
# LIBRERÍAS Y ENLACE
# ═══════════════════════════════════════════════════════════════════════════

# Enlazar con librerías
target_link_libraries(${PROJECT_NAME} 
    Threads::Threads
)

# ═══════════════════════════════════════════════════════════════════════════
# DIRECTORIOS DE INCLUSIÓN
# ═══════════════════════════════════════════════════════════════════════════

target_include_directories(${PROJECT_NAME} 
    PUBLIC 
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}
)

# ═══════════════════════════════════════════════════════════════════════════
# DEFINICIONES DE PREPROCESADOR
# ═══════════════════════════════════════════════════════════════════════════

target_compile_definitions(${PROJECT_NAME} 
    PUBLIC
        CPP_VERSION=17
        ULTRA_OMEGA_VERSION="1.0.0"
        ENABLE_PARALLEL=${ENABLE_PARALLEL}
        ENABLE_FILESYSTEM=${ENABLE_FILESYSTEM}
)

# ═══════════════════════════════════════════════════════════════════════════
# CARACTERÍSTICAS ESPECÍFICAS DE C++17
# ═══════════════════════════════════════════════════════════════════════════

# Habilitar características específicas de C++17
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
    target_compile_features(${PROJECT_NAME} PUBLIC cxx_std_17)
elseif(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    target_compile_features(${PROJECT_NAME} PUBLIC cxx_std_17)
elseif(MSVC)
    # MSVC soporta C++17 con /std:c++17
endif()

# ═══════════════════════════════════════════════════════════════════════════
# INSTALACIÓN
# ═══════════════════════════════════════════════════════════════════════════

install(TARGETS ${PROJECT_NAME}
    RUNTIME DESTINATION bin
)

# ═══════════════════════════════════════════════════════════════════════════
# INFORMACIÓN DE CONFIGURACIÓN
# ═══════════════════════════════════════════════════════════════════════════

message(STATUS "")
message(STATUS "╔══════════════════════════════════════════════════════════════╗")
message(STATUS "║     Ultra-Omega C++17 Template - Configuración CMake       ║")
message(STATUS "╚══════════════════════════════════════════════════════════════╝")
message(STATUS "")
message(STATUS "📋 Configuración:")
message(STATUS "   Proyecto: ${PROJECT_NAME}")
message(STATUS "   Versión: ${PROJECT_VERSION}")
message(STATUS "   Estándar C++: ${CMAKE_CXX_STANDARD}")
message(STATUS "   Compilador: ${CMAKE_CXX_COMPILER_ID}")
message(STATUS "   Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "")
message(STATUS "🔧 Características C++17:")
message(STATUS "   ✓ Structured bindings")
message(STATUS "   ✓ std::optional")
message(STATUS "   ✓ std::variant")
message(STATUS "   ✓ std::any")
message(STATUS "   ✓ std::string_view")
message(STATUS "   ✓ std::filesystem")
message(STATUS "   ✓ Parallel algorithms")
message(STATUS "   ✓ Fold expressions")
message(STATUS "   ✓ Class template argument deduction")
message(STATUS "   ✓ constexpr lambda")
message(STATUS "   ✓ Inline variables")
message(STATUS "")
message(STATUS "🔧 Opciones avanzadas:")
message(STATUS "   Parallel algorithms: ${ENABLE_PARALLEL}")
message(STATUS "   Filesystem support: ${ENABLE_FILESYSTEM}")
message(STATUS "")
message(STATUS "📁 Directorios:")
message(STATUS "   Source: ${CMAKE_CURRENT_SOURCE_DIR}")
message(STATUS "   Binary: ${CMAKE_CURRENT_BINARY_DIR}")
message(STATUS "")

# ═══════════════════════════════════════════════════════════════════════════
# TARGETS PERSONALIZADOS
# ═══════════════════════════════════════════════════════════════════════════

# Target para ejecutar el programa
add_custom_target(run
    COMMAND ${PROJECT_NAME}
    DEPENDS ${PROJECT_NAME}
    WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
    COMMENT "🚀 Ejecutando ${PROJECT_NAME}"
)

# Target para limpiar y recompilar
add_custom_target(rebuild
    COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR} --target clean
    COMMAND ${CMAKE_COMMAND} --build ${CMAKE_BINARY_DIR}
    COMMENT "🔄 Limpiando y recompilando"
)

# Target para ver información del compilador
add_custom_target(compiler_info
    COMMAND ${CMAKE_CXX_COMPILER} --version
    COMMENT "🔍 Mostrando información del compilador"
)

# Target para verificar características de C++17
add_custom_target(check_cpp17
    COMMAND ${CMAKE_CXX_COMPILER} -std=c++17 -dM -E - < /dev/null | grep __cplusplus
    COMMENT "🔍 Verificando soporte C++17"
)

# Target para probar filesystem
if(ENABLE_FILESYSTEM)
    add_custom_target(test_filesystem
        COMMAND ${PROJECT_NAME} --test-filesystem
        DEPENDS ${PROJECT_NAME}
        COMMENT "📁 Probando std::filesystem"
    )
endif()

# Target para benchmark con parallel algorithms
if(ENABLE_PARALLEL)
    add_custom_target(benchmark_parallel
        COMMAND ${PROJECT_NAME} --benchmark
        DEPENDS ${PROJECT_NAME}
        COMMENT "⚡ Ejecutando benchmark de algoritmos paralelos"
    )
endif()
