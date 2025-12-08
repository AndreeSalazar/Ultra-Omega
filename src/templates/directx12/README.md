# 💎 DirectX 12 Templates - Ultra-Omega

## Descripción

Templates completos para desarrollo con **DirectX 12** en C++ para Windows. Diseñados para integrarse con el sistema de nodos de Ultra-Omega.

## 📁 Estructura de Archivos

```
directx12/
├── directx12_types.h       # Tipos y estructuras base
├── adapter.cpp             # Enumeración de adaptadores (GPUs)
├── device.cpp              # Creación del dispositivo DirectX 12
├── swapchain.cpp           # Cadena de intercambio
├── command_allocator.cpp   # Allocators de comandos
├── root_signature.cpp      # Root signature para shaders
├── pipeline_state.cpp      # Pipeline State Object (PSO)
├── command_list.cpp        # Command lists
├── buffers.cpp             # Vertex, Index y Constant buffers
├── shader.hlsl             # Shaders HLSL (Vertex y Pixel)
├── render_loop.cpp         # Loop principal de renderizado
├── main.cpp                # Punto de entrada
│
├── 📚 LIBRERÍAS MODULARES (Heredables) 📚
├── helpers.cpp             # Utilidades matemáticas y debug
├── resource_manager.cpp    # Wrappers para gestión de recursos
├── window_manager.cpp      # Gestión de ventanas
├── sync_manager.cpp        # Gestión de sincronización (fences)
│
├── CMakeLists.txt          # Configuración CMake
└── README.md               # Este archivo
```

## 🔗 Mapa de Dependencias (Nodos)

```
                    ┌─────────────────────┐
                    │ directx12_types.h   │ ← Tipos base
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │   adapter.cpp       │ ← Enumerar GPUs
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │    device.cpp       │ ← Crear dispositivo
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │  swapchain.cpp      │ ← Crear swapchain
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │command_allocator.cpp│ ← Allocators
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ root_signature.cpp  │ ← Root signature
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │ pipeline_state.cpp  │ ← PSO
                    └──────────┬──────────┘
                    ┌──────────┼──────────┐
                    │          │          │
        ┌───────────▼────┐  ┌─▼─────┐  ┌─▼───────────┐
        │  shader.hlsl   │  │buffers│  │command_list │
        └───────────────┘  └───────┘  └──────┬───────┘
                                             │
        ┌────────────────────────────────────┼──────────────────────┐
        │                                    │                      │
        │                    ┌───────────────▼──────────┐           │
        │                    │   render_loop.cpp        │           │
        │                    └───────────────┬──────────┘           │
        │                                    │                      │
        │                    ┌───────────────▼──────────┐           │
        │                    │      main.cpp            │           │
        │                    │  [NODO FINAL - COMBINA]  │           │
        │                    └──────────────────────────┘           │
        │                                                           │
        └─────────────────── LIBRERÍAS (Heredables) ────────────────┘
                            Conectar para heredar código
        ┌──────────────────────────────────────────────────────────┐
        │                                                          │
        │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
        │  │  helpers.cpp │  │resource_     │  │window_       │  │
        │  │  (Utilidades)│  │manager.cpp   │  │manager.cpp   │  │
        │  └──────────────┘  └──────────────┘  └──────────────┘  │
        │                                                          │
        │  ┌──────────────┐                                       │
        │  │ sync_        │                                       │
        │  │manager.cpp   │                                       │
        │  └──────────────┘                                       │
        │                                                          │
        │  💡 Conecta cualquier nodo a estas librerías para      │
        │     heredar funciones automáticamente                   │
        └──────────────────────────────────────────────────────────┘
```

## 🛠️ Requisitos

### Windows
- **Windows 10/11** (DirectX 12 solo está disponible en Windows 10+)
- **Visual Studio 2019+** con soporte C++17
- **Windows SDK 10.0.19041.0 o superior**
- **GPU compatible con DirectX 12**

## 🔨 Compilación

### Con CMake

```bash
mkdir build && cd build
cmake .. -G "Visual Studio 17 2022"
cmake --build . --config Release
```

### Manualmente (Visual Studio)

1. Crea un proyecto de C++ vacío
2. Agrega todos los archivos `.cpp` y `.h`
3. Configura las librerías:
   - `d3d12.lib`
   - `dxgi.lib`
   - `dxguid.lib`
4. Compila los shaders:
   ```batch
   fxc /T vs_5_1 /E VSMain shader.hlsl /Fo shader_vs.cso
   fxc /T ps_5_1 /E PSMain shader.hlsl /Fo shader_ps.cso
   ```

## 🎯 Uso en Ultra-Omega

1. **Crear proyecto DirectX 12** desde el menú Templates → DirectX12
2. Los nodos se conectan automáticamente siguiendo el flujo de dependencias
3. Modifica cada nodo según tus necesidades
4. Exporta y compila con CMake o Visual Studio

## 📝 Conceptos Clave

### Inicialización
1. **Adapter**: Enumerar y seleccionar GPU
2. **Device**: Crear dispositivo DirectX 12
3. **Command Queue**: Cola de comandos para ejecutar
4. **Swapchain**: Cadena de intercambio para presentación

### Renderizado
5. **Root Signature**: Define bindings de recursos
6. **Pipeline State**: Configuración completa del pipeline
7. **Command Allocator/List**: Comandos de renderizado
8. **Fences**: Sincronización CPU-GPU

### Recursos
9. **Buffers**: Vertex, Index, Constant
10. **Descriptors**: Views de recursos
11. **Render Targets**: Destinos de renderizado

## 🔍 Características

- ✅ Enumeración automática de adaptadores
- ✅ Soporte para múltiples back buffers
- ✅ Sincronización con fences
- ✅ Pipeline completo configurable
- ✅ Shaders HLSL
- ✅ Constant buffers
- ✅ Vertex e Index buffers

## 📚 Librerías Modulares (Sistema de Herencia)

Los templates de DirectX12 están organizados en **librerías modulares independientes** que pueden heredarse fácilmente entre nodos:

### **Cómo Usar Librerías:**

1. **Crear nodo librería**: Por ejemplo, crear nodo "Helpers (Utilidades)"
2. **Conectar desde otro nodo**: Conecta tu nodo (ej: "Render Loop") al nodo "Helpers"
3. **Heredar código**: El nodo hijo hereda automáticamente el código del padre
4. **Usar funciones heredadas**: Accede con `ch("Helpers (Utilidades)")` en tu código

### **Ejemplo Práctico:**

```
Nodo A: "Helpers (Utilidades)" 
  └── Contiene: CreateProjectionMatrix(), CreateViewMatrix()

Nodo B: "Render Loop" (conectado a Nodo A)
  └── Hereda código de Helpers
  └── Puede usar: ch("Helpers (Utilidades)") para acceder a funciones
  
Nodo C: "Main" (conectado a Nodo B)
  └── Hereda código de Render Loop (que a su vez hereda de Helpers)
  └── Tiene acceso a todo el código heredado
```

### **Librerías Disponibles:**

- **📚 Helpers (Utilidades)**: Funciones matemáticas (matrices, proyección, vista)
- **📚 Resource Manager**: Wrappers para crear y gestionar buffers
- **📚 Window Manager**: Gestión de ventanas y mensajes
- **📚 Sync Manager**: Gestión de sincronización CPU-GPU

### **Ventajas:**

- ✅ **Reutilización**: Una vez creada una librería, se puede usar en múltiples nodos
- ✅ **Modularidad**: Cada librería es independiente y autocontenida
- ✅ **Sin dependencias circulares**: Las librerías no dependen entre sí
- ✅ **Fácil mantenimiento**: Cambiar una librería actualiza todos los nodos que la heredan

## 📚 Recursos

- [DirectX 12 Documentation](https://docs.microsoft.com/en-us/windows/win32/direct3d12/direct3d-12-graphics)
- [DirectX 12 Programming Guide](https://docs.microsoft.com/en-us/windows/win32/direct3d12/directx-12-programming-guide)
- [HLSL Shader Compiler](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-part1)

## 🐛 Debug

Para habilitar la capa de debug de DirectX 12:

```cpp
// En device.cpp, ya está habilitado con _DEBUG
#ifdef _DEBUG
    // Debug layer está activa
#endif
```

## ⚠️ Notas

- DirectX 12 solo funciona en Windows 10/11
- Requiere GPU compatible con DirectX 12
- Los shaders deben compilarse con `fxc` o `dxc` antes de ejecutar
- La gestión de memoria es más explícita que en DirectX 11

## 💡 Consejos de Uso con Ultra-Omega

### **Trabajar con Librerías Modulares:**

1. **Crear librerías primero**: Empieza creando nodos de librerías (Helpers, Resource Manager, etc.)
2. **Conectar para heredar**: Conecta tus nodos principales a las librerías
3. **Usar funciones heredadas**: Accede al código heredado con `ch("nombre_librería")`
4. **Sin preocuparse por dependencias**: Las librerías son independientes y autocontenidas

### **Ejemplo de Flujo de Trabajo:**

```
1. Crear nodo "Helpers (Utilidades)" → Contiene funciones matemáticas
2. Crear nodo "Render Loop" → Conectar a "Helpers"
3. En "Render Loop", usar: ch("Helpers (Utilidades)") para acceder a funciones
4. Crear nodo "Main" → Conectar a "Render Loop" → Hereda todo automáticamente
```

### **Ventajas del Sistema Modular:**

- ✅ **Reutilización**: Una librería puede usarse en múltiples proyectos
- ✅ **Mantenimiento fácil**: Cambiar una librería actualiza todos los nodos que la heredan
- ✅ **Sin dependencias circulares**: Las librerías son independientes
- ✅ **Organización clara**: Código separado por funcionalidad

