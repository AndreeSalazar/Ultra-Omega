# ═══════════════════════════════════════════════════════════════════════════
# CMakeLists.txt - Ultra-Omega C++11 Template
# ═══════════════════════════════════════════════════════════════════════════

cmake_minimum_required(VERSION 3.10)

# Nombre del proyecto
project(UltraOmega_CPP11 
    VERSION 1.0.0
    DESCRIPTION "Ultra-Omega C++11 Template"
    LANGUAGES CXX
)

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN DE ESTÁNDAR C++11
# ═══════════════════════════════════════════════════════════════════════════

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN DE COMPILADOR
# ═══════════════════════════════════════════════════════════════════════════

# Opciones de compilación
option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(ENABLE_WARNINGS "Enable all warnings" ON)
option(ENABLE_DEBUG "Enable debug symbols" ON)

# Configuración de flags según el compilador
if(MSVC)
    # Microsoft Visual C++
    add_compile_options(/W4)  # Nivel alto de warnings
    if(ENABLE_DEBUG)
        add_compile_options(/Zi /Od)  # Debug info y optimización desactivada
    else()
        add_compile_options(/O2)     # Optimización máxima
    endif()
    add_compile_definitions(_CRT_SECURE_NO_WARNINGS)  # Para compatibilidad
else()
    # GCC / Clang
    add_compile_options(-Wall -Wextra -Wpedantic)
    if(ENABLE_DEBUG)
        add_compile_options(-g -O0)
    else()
        add_compile_options(-O2)
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
        CPP_VERSION=11
        ULTRA_OMEGA_VERSION="1.0.0"
)

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
message(STATUS "║     Ultra-Omega C++11 Template - Configuración CMake       ║")
message(STATUS "╚══════════════════════════════════════════════════════════════╝")
message(STATUS "")
message(STATUS "📋 Configuración:")
message(STATUS "   Proyecto: ${PROJECT_NAME}")
message(STATUS "   Versión: ${PROJECT_VERSION}")
message(STATUS "   Estándar C++: ${CMAKE_CXX_STANDARD}")
message(STATUS "   Compilador: ${CMAKE_CXX_COMPILER_ID}")
message(STATUS "   Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "")
message(STATUS "📁 Directorios:")
message(STATUS "   Source: ${CMAKE_CURRENT_SOURCE_DIR}")
message(STATUS "   Binary: ${CMAKE_CURRENT_BINARY_DIR}")
message(STATUS "")
message(STATUS "🔧 Opciones:")
message(STATUS "   Warnings: ${ENABLE_WARNINGS}")
message(STATUS "   Debug: ${ENABLE_DEBUG}")
message(STATUS "   Shared Libs: ${BUILD_SHARED_LIBS}")
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
