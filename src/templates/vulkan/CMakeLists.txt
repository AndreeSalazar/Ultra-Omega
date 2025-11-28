# ═══════════════════════════════════════════════════════════════════════════
# VULKAN PROJECT - CMake Configuration
# ═══════════════════════════════════════════════════════════════════════════
# Archivo: CMakeLists.txt
# Descripción: Configuración de CMake para compilar el proyecto Vulkan
# 
# Uso:
#   mkdir build && cd build
#   cmake ..
#   cmake --build .
# ═══════════════════════════════════════════════════════════════════════════

cmake_minimum_required(VERSION 3.16)
project(VulkanApp VERSION 1.0.0 LANGUAGES CXX)

# C++17 requerido
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ═══════════════════════════════════════════════════════════════════════════
# ENCONTRAR DEPENDENCIAS
# ═══════════════════════════════════════════════════════════════════════════

# Vulkan
find_package(Vulkan REQUIRED)

# GLFW
find_package(glfw3 3.3 REQUIRED)

# ═══════════════════════════════════════════════════════════════════════════
# ARCHIVOS FUENTE
# ═══════════════════════════════════════════════════════════════════════════

set(SOURCES
    main.cpp
    instance.cpp
    device.cpp
    swapchain.cpp
    pipeline.cpp
    buffers.cpp
    commands.cpp
    sync.cpp
    render_loop.cpp
    texture.cpp
)

set(HEADERS
    vulkan_types.h
)

# ═══════════════════════════════════════════════════════════════════════════
# COMPILAR SHADERS
# ═══════════════════════════════════════════════════════════════════════════

find_program(GLSLC glslc HINTS 
    $ENV{VULKAN_SDK}/bin 
    /usr/bin 
    /usr/local/bin
)

if(NOT GLSLC)
    message(FATAL_ERROR "glslc not found! Install Vulkan SDK.")
endif()

set(SHADER_DIR ${CMAKE_SOURCE_DIR})
set(SHADER_OUTPUT_DIR ${CMAKE_BINARY_DIR}/shaders)

file(MAKE_DIRECTORY ${SHADER_OUTPUT_DIR})

# Vertex shader
add_custom_command(
    OUTPUT ${SHADER_OUTPUT_DIR}/vert.spv
    COMMAND ${GLSLC} ${SHADER_DIR}/shader.vert -o ${SHADER_OUTPUT_DIR}/vert.spv
    DEPENDS ${SHADER_DIR}/shader.vert
    COMMENT "Compilando vertex shader..."
)

# Fragment shader
add_custom_command(
    OUTPUT ${SHADER_OUTPUT_DIR}/frag.spv
    COMMAND ${GLSLC} ${SHADER_DIR}/shader.frag -o ${SHADER_OUTPUT_DIR}/frag.spv
    DEPENDS ${SHADER_DIR}/shader.frag
    COMMENT "Compilando fragment shader..."
)

add_custom_target(shaders DEPENDS 
    ${SHADER_OUTPUT_DIR}/vert.spv 
    ${SHADER_OUTPUT_DIR}/frag.spv
)

# ═══════════════════════════════════════════════════════════════════════════
# EJECUTABLE
# ═══════════════════════════════════════════════════════════════════════════

add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

add_dependencies(${PROJECT_NAME} shaders)

target_include_directories(${PROJECT_NAME} PRIVATE
    ${Vulkan_INCLUDE_DIRS}
    ${CMAKE_SOURCE_DIR}
)

target_link_libraries(${PROJECT_NAME} PRIVATE
    Vulkan::Vulkan
    glfw
)

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURACIÓN ESPECÍFICA POR PLATAFORMA
# ═══════════════════════════════════════════════════════════════════════════

if(WIN32)
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        VK_USE_PLATFORM_WIN32_KHR
        NOMINMAX
        WIN32_LEAN_AND_MEAN
    )
elseif(UNIX AND NOT APPLE)
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        VK_USE_PLATFORM_XLIB_KHR
    )
    target_link_libraries(${PROJECT_NAME} PRIVATE
        dl
        pthread
        X11
        Xxf86vm
        Xrandr
        Xi
    )
elseif(APPLE)
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        VK_USE_PLATFORM_MACOS_MVK
    )
    find_library(COCOA_LIBRARY Cocoa)
    find_library(IOKIT_LIBRARY IOKit)
    find_library(COREVIDEO_LIBRARY CoreVideo)
    target_link_libraries(${PROJECT_NAME} PRIVATE
        ${COCOA_LIBRARY}
        ${IOKIT_LIBRARY}
        ${COREVIDEO_LIBRARY}
    )
endif()

# ═══════════════════════════════════════════════════════════════════════════
# COPIAR SHADERS AL DIRECTORIO DE SALIDA
# ═══════════════════════════════════════════════════════════════════════════

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${SHADER_OUTPUT_DIR}
        $<TARGET_FILE_DIR:${PROJECT_NAME}>/shaders
    COMMENT "Copiando shaders..."
)

# ═══════════════════════════════════════════════════════════════════════════
# INFORMACIÓN
# ═══════════════════════════════════════════════════════════════════════════

message(STATUS "═══════════════════════════════════════════════════════════")
message(STATUS "  Vulkan App - Ultra-Omega Template")
message(STATUS "═══════════════════════════════════════════════════════════")
message(STATUS "  Vulkan SDK: ${Vulkan_INCLUDE_DIRS}")
message(STATUS "  GLSLC: ${GLSLC}")
message(STATUS "  Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "═══════════════════════════════════════════════════════════")

