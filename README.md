# 🔥 Ultra-Omega v2.0

<div align="center">

![Ultra-Omega Logo](https://img.shields.io/badge/Ultra--Omega-v2.0%20Rust%20%2B%20Vulkan-orange?style=for-the-badge&logo=rust)

**Entorno de Desarrollo Visual Basado en Nodos (100% Rust)**  
*Motor de Renderizado Vulkan (ash) para Control Total*

Desarrollado por **Eddi Andreé Salazar Matos** 🇵🇪

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Vulkan](https://img.shields.io/badge/Vulkan-1.3%20(ash)-red.svg)](https://www.vulkan.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-lightgrey)]()

</div>

---

## 📋 Tabla de Contenidos

- [Descripción](#-descripción)
- [Cambios en v2.0](#-cambios-en-v20)
- [Características Principales](#-características-principales)
- [Arquitectura Vulkan](#-arquitectura-vulkan)
- [Instalación](#-instalación)
- [Uso Básico](#-uso-básico)
- [Sistema de Expresiones](#-sistema-de-expresiones)
- [Atajos de Teclado](#-atajos-de-teclado)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [Roadmap](#-roadmap)

---

## 🎯 Descripción

**Ultra-Omega** es un entorno de desarrollo visual basado en nodos de **alto rendimiento**, enfocado **100% en el lenguaje Rust**. A diferencia de la versión anterior, v2.0 elimina el soporte multi-lenguaje para especializarse en ofrecer la mejor experiencia posible para desarrolladores de sistemas, motores y aplicaciones de alto rendimiento en Rust.

El motor de renderizado ha sido reescrito desde cero utilizando **Vulkan a través de `ash`**, otorgando control total sobre la GPU, optimizando el dibujo de miles de nodos y conexiones con latencia mínima.

### Filosofía de Diseño v2.0

- **100% Rust**: El editor y los nodos están escritos exclusivamente en Rust.
- **Control Total con Vulkan**: Uso de `ash` para gestionar la GPU directamente, sin capas de abstracción pesadas.
- **Visual First**: Todo se puede hacer visualmente, sin necesidad de escribir código manualmente.
- **Herencia de Código**: Los nodos pueden heredar código de sus nodos padre, permitiendo composición y reutilización.
- **Expresiones Houdini-style**: Sistema de expresiones `ch()` para referenciar valores de otros nodos.

---

## 🔄 Cambios en v2.0

Para optimizar el rendimiento y el enfoque del proyecto, se han realizado cambios drásticos:

### ❌ Eliminado (Movido a `trash/`)
- **Soporte Multi-lenguaje**: Eliminados templates y compiladores para Assembly, Java, Python y C++.
- **`wgpu` y `eframe`**: Eliminadas las capas de abstracción de renderizado y UI.
- **Puentes FFI**: Eliminados los bridges entre lenguajes (JNI, PyO3, etc.).

### ✅ Añadido / Modificado
- **Motor Vulkan (`ash`)**: Renderizado nativo de la UI y conexiones de nodos directamente en la GPU.
- **Enfoque 100% Rust**: El editor ahora es un IDE visual exclusivo para proyectos Rust.
- **Templates Rust Optimizados**: 22+ templates de Rust de alto rendimiento (async, concurrency, unsafe, macros).
- **Compilador Integrado**: Detección y ejecución directa de `rustc` y `cargo`.

---

## ✨ Características Principales

### 🎨 Interfaz Visual (Vulkan Powered)

- **Renderizado GPU Directo**: Miles de nodos y conexiones dibujados con Vulkan, manteniendo 60+ FPS.
- **Tema Visual Studio Code Dark+**: Interfaz familiar con colores y estilos de VS Code.
- **Zoom y Pan Fluido**: Navegación optimizada en el viewport 2D.
- **Selección Múltiple**: Box selection para manipular grupos de nodos.
- **Herencia Visual**: Visualiza la cadena de herencia de código con `Ctrl+I`.

### 💻 Editor de Código Rust

- **Resaltado de Sintaxis Rust**: Colores personalizados para keywords, lifetimes, macros y traits.
- **Integración con Cargo**: Compilación, testing y ejecución directa desde el editor.
- **Historial de Edición**: Deshacer/Rehacer con `Ctrl+Z` / `Ctrl+Y`.
- **Búsqueda y Reemplazo**: `Ctrl+F` para buscar, `Ctrl+H` para reemplazar.

### 🔗 Sistema de Nodos (Solo Rust)

- **Tipos de Nodos**:
  - `Binary`: Ejecutables Rust (`fn main()`).
  - `Library`: Librerías y módulos (`pub fn`, `pub struct`).
  - `Macro`: Definición de macros (`macro_rules!`, proc-macros).
  - `Unsafe`: Bloques de código `unsafe` para control de bajo nivel.
  - `Async`: Nodos con funciones `async`/`await`.
  - `Text`: Documentación y comentarios.

- **Herencia de Código**: Los nodos heredan `impl` blocks, `structs` y `traits` de sus nodos padre.
- **Conexiones Tipadas**: Las conexiones validan tipos de Rust en tiempo real (gracias al LSP interno).

### 📦 Sistema de Templates (Rust)

**Templates pre-construidos listos para usar:**

- **Básicos**: `hello_world`, `cli_app`, `variables`, `functions`.
- **Sistemas**: `ownership`, `lifetimes`, `traits`, `generics`.
- **Avanzados**: `async`, `concurrency`, `unsafe`, `macros`.
- **Librerías**: `lib_io`, `lib_math`, `lib_utils`, `lib_error`.

### 🧮 Sistema de Expresiones

Sistema inspirado en Houdini para referenciar valores de otros nodos:

```rust
// Referenciar código de otro nodo
ch("nodo1")

// Operaciones aritméticas
ch("nodo1") + ch("nodo2")

// Comparaciones
ch("valor") > 10

// Variables
$variable
```

### 🗂️ Workspace y Persistencia

- **Workspace**: Abre carpetas como workspaces (similar a VS Code).
- **Auto-guardado**: Guarda automáticamente el estado del grafo.
- **Persistencia**: Guarda posición de ventana, tamaño, y configuración.
- **File Watcher**: Detección en tiempo real de cambios en archivos `.rs`.

---

## 🖥️ Arquitectura Vulkan

Ultra-Omega v2.0 utiliza **Vulkan** a través de la librería `ash` para el renderizado. Esto nos permite:

1. **Control Total de la GPU**: Gestión manual de memory barriers, command buffers y synchronization.
2. **Batch Rendering**: Dibujar miles de líneas (conexiones) y rectángulos (nodos) en un solo draw call.
3. **Sin Overhead**: Eliminamos las capas de abstracción de `wgpu` y `eframe` para máxima eficiencia.
4. **Compute Shaders**: Preparado para futuros cálculos de auto-layout en la GPU.

### Dependencias de Renderizado

```toml
[dependencies]
ash = "0.37"
ash-window = "0.12"
raw-window-handle = "0.5"
winit = "0.28"
```

---

## 🚀 Instalación

### Requisitos

- **Rust**: Versión 1.70 o superior (Rustc + Cargo).
- **Vulkan SDK**: Instalado y configurado en tu sistema (para desarrollo).
- **Sistema Operativo**: Linux o Windows (con drivers Vulkan actualizados).

### Instalación desde Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/AndreeSalazar/Ultra-Omega.git
cd Ultra-Omega

# Compilar en modo release
cargo build --release

# Ejecutar
cargo run --release
```

---

## 📖 Uso Básico

### Crear un Nuevo Proyecto Rust

1. **Abrir Workspace**: `File > Open Workspace` o `Ctrl+O`
2. **Crear Nodo**: Presiona `Tab` o haz clic derecho en el viewport
3. **Seleccionar Template**: Elige entre los templates de Rust disponibles
4. **Escribir Código**: Doble clic en un nodo para editar su código Rust
5. **Conectar Nodos**: Arrastra desde un pin de salida a un pin de entrada
6. **Compilar y Ejecutar**: `Ctrl+R` para ejecutar el grafo como un proyecto Cargo

### Herencia de Código

1. **Conectar Nodos**: Conecta el pin de salida de un nodo al pin de entrada de otro.
2. **Heredar Código**: El nodo hijo hereda automáticamente `structs`, `traits` y `impl` del padre.
3. **Ver Herencia**: Presiona `Ctrl+I` en un nodo para ver la cadena de herencia.

### Ejemplo: Hello World en Rust

```rust
// Nodo: main_rust
fn main() {
    println!("Hello from Ultra-Omega v2.0!");
}
```

---

## 🧮 Sistema de Expresiones

### Sintaxis

```rust
// Referencia básica
ch("nodo")

// Referencia con parámetro
ch("nodo/param")

// Operaciones aritméticas
ch("nodo1") + ch("nodo2")
ch("valor") * 2

// Comparaciones
ch("valor") > 10
ch("texto") == "hola"
```

---

## ⌨️ Atajos de Teclado

### Navegación

| Atajo | Acción |
|-------|--------|
| `Tab` | Mostrar/Ocultar menú de creación de nodos |
| `F` | Enfocar viewport en los nodos |
| `Ctrl + Scroll` | Zoom in/out |
| `Middle Click + Drag` | Pan del viewport |
| `Ctrl + T` | Abrir árbol de navegación de nodos |

### Edición

| Atajo | Acción |
|-------|--------|
| `Double Click` | Editar nodo |
| `Ctrl + Z` | Deshacer |
| `Ctrl + Y` | Rehacer |
| `Ctrl + F` | Buscar en editor |
| `Ctrl + H` | Reemplazar en editor |
| `Ctrl + I` | Ver cadena de herencia |

### Archivos y Ejecución

| Atajo | Acción |
|-------|--------|
| `Ctrl + O` | Abrir workspace |
| `Ctrl + S` | Guardar |
| `Ctrl + R` | Compilar y Ejecutar (Cargo run) |
| `Ctrl + Shift + B` | Solo Compilar (Cargo build) |

---

## 📁 Estructura del Proyecto

```
Ultra-Omega/
├── Cargo.toml              # Configuración del proyecto Rust (ash + winit)
├── README.md               # Este archivo
├── trash/                  # Archivos obsoletos (multi-lenguaje, wgpu)
├── src/                    # Código fuente
│   ├── main.rs             # Punto de entrada, inicialización de Vulkan
│   ├── vulkan/             # Motor de renderizado Vulkan (ash)
│   │   ├── instance.rs     # Vulkan Instance & Device
│   │   ├── swapchain.rs    # Swapchain management
│   │   ├── pipeline.rs     # Graphics pipeline
│   │   └── renderer.rs     # Node & connection rendering
│   ├── core/               # Lógica principal
│   │   ├── app.rs          # Estado de la aplicación
│   │   ├── node_graph.rs   # Grafo de nodos
│   │   └── folder_node.rs  # Gestión de carpetas
│   ├── expressions/        # Sistema de expresiones Houdini-style
│   ├── templates/          # Templates 100% Rust
│   ├── storage/            # Sistema de almacenamiento y file watcher
│   ├── compilation/        # Integración con rustc/cargo
│   ├── inheritance/        # Sistema de herencia de código
│   ├── ui/                 # Interfaz de usuario (renderizada en Vulkan)
│   └── utils/              # Utilidades varias
└── target/                 # Archivos de compilación
```

---

## 🔮 Roadmap

### ✅ v2.0 - Refactorización Total (Actual)

- [x] **Eliminación de Multi-lenguaje**: Movido a `trash/`.
- [x] **Motor Vulkan (`ash`)**: Reemplazo de `wgpu`/`eframe` por Vulkan nativo.
- [x] **Enfoque 100% Rust**: Templates y compilación exclusivos para Rust.
- [x] **Optimización de UI**: Renderizado directo en GPU.

### Próximas Características (v2.1 - v2.5)

- [ ] **LSP Integrado**: Autocompletado y validación de tipos en tiempo real.
- [ ] **Compute Shaders**: Auto-layout de grafos masivos en la GPU.
- [ ] **Subnetworks**: Nodos que contienen grafos completos (macros visuales).
- [ ] **Debugger Visual**: Integración con `gdb`/`lldb` para debuggear nodos.
- [ ] **Cargo Integration**: Gestión de dependencias `Cargo.toml` visual.

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver `LICENSE` para más detalles.

---

## 👤 Créditos

**Desarrollado por:** Eddi Andreé Salazar Matos 🇵🇪

### Inspiraciones

- **Houdini**: Sistema de expresiones y programación procedural.
- **Unreal Engine 5**: Blueprint system y desarrollo visual.
- **Visual Studio Code**: Interfaz de usuario y experiencia.

### Tecnologías Utilizadas

- **Rust**: Lenguaje de programación principal (100%).
- **Vulkan (ash)**: API gráfica para control total de la GPU.
- **winit**: Manejo de ventanas y eventos.
- **serde**: Serialización JSON.
- **rfd**: Diálogos de archivos nativos.

---

<div align="center">

**Hecho con ❤️ en Perú 🇵🇪**

![Peru Flag](https://img.shields.io/badge/Peru-🇵🇪-red?style=flat-square)

</div>
