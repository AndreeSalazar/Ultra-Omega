# ═══════════════════════════════════════════════════════════════════════════
# DIRECTX12 CMakeLists.txt - Configuración de build
# ═══════════════════════════════════════════════════════════════════════════

cmake_minimum_required(VERSION 3.20)
project(DirectX12App)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Archivos fuente
set(SOURCES
    main.cpp
    adapter.cpp
    device.cpp
    swapchain.cpp
    command_allocator.cpp
    root_signature.cpp
    pipeline_state.cpp
    command_list.cpp
    buffers.cpp
    render_loop.cpp
)

# Headers
set(HEADERS
    directx12_types.h
)

# Compilar shaders HLSL
find_program(FXC fxc)
if(NOT FXC)
    message(WARNING "fxc no encontrado. Compila los shaders manualmente.")
else()
    add_custom_command(
        OUTPUT shader_vs.cso shader_ps.cso
        COMMAND ${FXC} /T vs_5_1 /E VSMain shader.hlsl /Fo shader_vs.cso
        COMMAND ${FXC} /T ps_5_1 /E PSMain shader.hlsl /Fo shader_ps.cso
        DEPENDS shader.hlsl
    )
    add_custom_target(shaders DEPENDS shader_vs.cso shader_ps.cso)
endif()

# Ejecutable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# Enlazar librerías de DirectX 12
target_link_libraries(${PROJECT_NAME}
    d3d12.lib
    dxgi.lib
    dxguid.lib
)

# Configuraciones de build
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    target_compile_definitions(${PROJECT_NAME} PRIVATE _DEBUG)
    target_compile_options(${PROJECT_NAME} PRIVATE /Od /Zi)
else()
    target_compile_options(${PROJECT_NAME} PRIVATE /O2)
endif()

# Windows específico
if(WIN32)
    set_target_properties(${PROJECT_NAME} PROPERTIES
        WIN32_EXECUTABLE TRUE
    )
endif()

