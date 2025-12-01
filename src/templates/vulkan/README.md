# 🎮 Vulkan API Templates - Ultra-Omega

## Descripción

Templates completos para desarrollo con **Vulkan API** en C++. Diseñados para integrarse con el sistema de nodos de Ultra-Omega.

## 📁 Estructura de Archivos

```
vulkan/
├── vulkan_types.h      # Tipos y estructuras base
├── instance.cpp        # Crear instancia Vulkan
├── device.cpp          # Selección de GPU y dispositivo lógico
├── swapchain.cpp       # Cadena de intercambio
├── pipeline.cpp        # Graphics pipeline
├── buffers.cpp         # Vertex, Index y Uniform buffers
├── commands.cpp        # Command pool y command buffers
├── sync.cpp            # Semáforos y fences
├── render_loop.cpp     # Loop principal de renderizado
├── texture.cpp         # Carga y manejo de texturas
├── shader.vert         # Vertex shader (GLSL)
├── shader.frag         # Fragment shader (GLSL)
├── main.cpp            # Punto de entrada
├── CMakeLists.txt      # Configuración CMake
└── README.md           # Este archivo
```

## 🔗 Mapa de Dependencias (Nodos)

```
                    ┌─────────────────────┐
                    │  vulkan_types.h     │ ← Tipos base
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │   instance.cpp      │ ← Crear instancia
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │    device.cpp       │ ← Seleccionar GPU
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │  swapchain.cpp      │ ← Crear swapchain
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │   pipeline.cpp      │ ← Graphics pipeline
                    └──────────┬──────────┘
                ┌──────────────┼──────────────┐
                │              │              │
     ┌──────────▼────┐  ┌──────▼──────┐  ┌────▼──────────┐
     │ shader.vert   │  │ shader.frag │  │ buffers.cpp   │
     └───────────────┘  └─────────────┘  └───────┬───────┘
                                                 │
                    ┌────────────────────────────▼────┐
                    │       commands.cpp              │
                    └────────────────┬────────────────┘
                                     │
                    ┌────────────────▼────────────────┐
                    │         sync.cpp                │
                    └────────────────┬────────────────┘
                                     │
                    ┌────────────────▼────────────────┐
                    │      render_loop.cpp            │
                    └────────────────┬────────────────┘
                                     │
                    ┌────────────────▼────────────────┐
                    │         main.cpp                │
                    │    [NODO FINAL - COMBINA TODO]  │
                    └─────────────────────────────────┘
```

## 🛠️ Requisitos

### Windows
- Visual Studio 2019+ con soporte C++17
- [Vulkan SDK](https://vulkan.lunarg.com/sdk/home)
- [GLFW](https://www.glfw.org/)

### Linux
```bash
# Ubuntu/Debian
sudo apt install vulkan-tools libvulkan-dev libglfw3-dev

# Arch Linux
sudo pacman -S vulkan-tools vulkan-devel glfw-x11
```

### macOS
```bash
brew install glfw vulkan-loader molten-vk
```

## 🔨 Compilación

### Con CMake
```bash
mkdir build && cd build
cmake ..
cmake --build .
./VulkanApp
```

### Manualmente (Linux)
```bash
# Compilar shaders
glslc shader.vert -o shaders/vert.spv
glslc shader.frag -o shaders/frag.spv

# Compilar aplicación
g++ -std=c++17 -O2 -o vulkan_app \
    main.cpp instance.cpp device.cpp swapchain.cpp \
    pipeline.cpp buffers.cpp commands.cpp sync.cpp \
    render_loop.cpp texture.cpp \
    -lglfw -lvulkan -ldl -lpthread -lX11 -lXxf86vm -lXrandr -lXi
```

### Manualmente (Windows - MSVC)
```batch
:: Compilar shaders
glslc shader.vert -o shaders/vert.spv
glslc shader.frag -o shaders/frag.spv

:: Compilar aplicación
cl /EHsc /std:c++17 /O2 main.cpp instance.cpp device.cpp swapchain.cpp ^
   pipeline.cpp buffers.cpp commands.cpp sync.cpp render_loop.cpp texture.cpp ^
   /link glfw3.lib vulkan-1.lib user32.lib gdi32.lib shell32.lib
```

## 🎯 Uso en Ultra-Omega

1. **Crear proyecto Vulkan** desde el menú Templates → Vulkan
2. Los nodos se conectan automáticamente siguiendo el flujo de dependencias
3. Modifica cada nodo según tus necesidades
4. Exporta y compila con CMake

## 📝 Conceptos Clave

### Inicialización
1. **Instance**: Conexión con el driver Vulkan
2. **Physical Device**: Selección de GPU
3. **Logical Device**: Interfaz con la GPU
4. **Surface**: Conexión con la ventana (GLFW)

### Renderizado
5. **Swapchain**: Imágenes para presentación
6. **Render Pass**: Define attachments y subpasses
7. **Pipeline**: Configuración completa del renderizado
8. **Framebuffers**: Destino del renderizado

### Datos
9. **Buffers**: Vertex, Index, Uniform
10. **Descriptors**: Bindings para shaders
11. **Textures**: Imágenes y samplers

### Ejecución
12. **Command Pool/Buffers**: Comandos de GPU
13. **Sync Objects**: Semáforos y fences
14. **Render Loop**: Frame por frame

## 🎨 Personalización

### Cambiar color de fondo
En `commands.cpp`:
```cpp
VkClearValue clearColor = {{{0.02f, 0.02f, 0.05f, 1.0f}}};
```

### Cambiar geometría
En `buffers.cpp`:
```cpp
const std::vector<Vertex> vertices = {
    // Modifica posiciones, colores, UVs
};
```

### Agregar textura
1. Descomentar código de textura en `main.cpp`
2. Modificar `shader.frag` para usar sampler

## 👨‍💻 Autor

**Eddi Andreé Salazar Matos**  
Desarrollador Peruano 🇵🇪  
Ultra-Omega Project

## 📄 Licencia

MIT License - Libre para uso personal y comercial.

