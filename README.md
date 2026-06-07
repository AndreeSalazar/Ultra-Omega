# Ultra-Omega v2.0 - Editor Visual de Nodos 100% Rust + Vulkan

<div align="center">

![Rust](https://img.shields.io/badge/Rust-100%25-orange?style=for-the-badge&logo=rust)
![Vulkan](https://img.shields.io/badge/Vulkan-ash-red?style=for-the-badge&logo=vulkan)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

**Control Total. Rendimiento Extremo. Sin Abstracciones.**

</div>

---

## 🚀 Visión

Ultra-Omega v2.0 es un **Entorno de Desarrollo Visual basado en Nodos** construido desde cero con un enfoque radical:

- **100% Rust**: Todo el código del editor y los nodos es Rust nativo.
- **Vulkan Directo**: Usamos `ash` para tener control total sobre la GPU, sin capas de abstracción como `wgpu` o `eframe`.
- **Rendimiento Extremo**: Diseñado para manejar miles de nodos con conexiones complejas sin pérdida de FPS.
- **Arquitectura Modular**: Cada componente es independiente y puede ser reemplazado o extendido.

---

## ✨ Características Principales

### 🎨 Motor de Renderizado Vulkan
- **Control Total**: Acceso directo a la API de Vulkan mediante `ash`.
- **Swapchain Optimizado**: Gestión manual de la swapchain para mínimo latency.
- **Compute Shaders**: Preparado para usar compute shaders para auto-layout y cálculos intensivos.
- **Sin Dependencias de UI**: No usamos `egui`, `imgui`, ni ningún framework de UI. Todo se renderiza directamente.

### 🦀 Enfoque 100% Rust
- **Nodos Rust**: Todos los nodos contienen código Rust nativo.
- **Compilación Integrada**: Soporte para compilar y ejecutar nodos directamente desde el editor.
- **LSP Ready**: Preparado para integrar `rust-analyzer` para autocompletado y validación en tiempo real.
- **Cargo Visual**: Gestión visual de dependencias y crates.

### 🔗 Sistema de Nodos Avanzado
- **Herencia de Código**: Los nodos pueden heredar código de sus nodos padre.
- **Subnetworks**: Nodos que contienen grafos completos (estilo Houdini).
- **Expresiones `ch()`**: Sistema de expresiones para referenciar valores de otros nodos.
- **Templates**: Biblioteca de templates para arrancar proyectos rápidamente.

### 💾 Gestión de Workspace
- **Archivos Separados**: Cada nodo se guarda en su propio archivo `.rs` para mejor compatibilidad con Git.
- **File Watcher**: Detección en tiempo real de cambios en archivos externos.
- **Auto-Save**: Guardado automático configurable.
- **Migración**: Sistema de migración automática de proyectos antiguos.

---

## 🛠️ Stack Tecnológico

| Componente | Tecnología | Propósito |
|------------|------------|-----------|
| **Lenguaje** | Rust 2021 | Núcleo del sistema |
| **API Gráfica** | Vulkan 1.2 vía `ash` | Renderizado directo |
| **Ventanas** | `winit` | Gestión de ventanas y eventos |
| **Serialización** | `serde` + `serde_json` | Guardado/carga de proyectos |
| **Matemáticas** | `glam` + `bytemuck` | Cálculos GPU-friendly |
| **Logging** | `log` + `env_logger` | Debug y diagnóstico |

---

## 📦 Instalación y Compilación

### Requisitos Previos

1. **Rust**: Instala desde [rustup.rs](https://rustup.rs/)
2. **Vulkan SDK**: Descarga desde [LunarG](https://www.lunarg.com/vulkan-sdk/)
3. **GPU Compatible**: Tarjeta gráfica con soporte Vulkan 1.2+

### Compilar el Proyecto

```bash
# Clonar el repositorio
git clone <repo-url>
cd Ultra-Omega

# Compilar en modo debug
cargo build

# Ejecutar
cargo run

# Compilar en modo release (optimizado)
cargo build --release
```

---

## 🏗️ Arquitectura

```
src/
├── main.rs              # Punto de entrada, inicialización de ventana y Vulkan
├── config.rs            # Configuración de la aplicación
├── core/                # Lógica principal del grafo de nodos
│   ├── app.rs           # Estructura principal de la aplicación
│   ├── node_graph.rs    # Grafo de nodos y conexiones
│   └── folder_node.rs   # Nodos carpeta y herencia
├── vulkan/              # Motor de renderizado Vulkan
│   ├── mod.rs           # Exportaciones del módulo
│   └── context.rs       # Contexto Vulkan (Instance, Device, Surface)
├── ui/                  # Componentes de UI (a ser reescritos para Vulkan)
│   ├── mod.rs           # Exportaciones
│   └── theme.rs         # Sistema de temas y colores
├── storage/             # Gestión de workspace y archivos
├── expressions/         # Parser y evaluador de expresiones ch()
├── inheritance/         # Sistema de herencia de código
├── templates/           # Templates de código Rust
└── utils/               # Utilidades generales
```

---

## 🚧 Estado Actual y Roadmap

### ✅ Completado (v2.0)
- [x] Migración de `wgpu`/`eframe` a `ash`/Vulkan
- [x] Inicialización básica de Vulkan (Instance, Device, Surface)
- [x] Ventana con `winit`
- [x] Limpieza de código multi-lenguaje (solo Rust)
- [x] Estructura modular para el motor gráfico

### 🚧 En Progreso
- [ ] Swapchain y render loop
- [ ] Renderizado de nodos con Vulkan
- [ ] Renderizado de conexiones (Bézier curves)
- [ ] Input handling (mouse, keyboard)
- [ ] Text rendering con Vulkan

### 📅 Próximas Fases
- [ ] Integración de `rust-analyzer` (LSP)
- [ ] Compute shaders para auto-layout
- [ ] Subnetworks completos
- [ ] Debugger visual integrado
- [ ] Profiler de rendimiento

---

## 🎯 Diferenciadores Clave

| Característica | Ultra-Omega v2.0 | Otros Editores |
|----------------|------------------|----------------|
| **Motor Gráfico** | Vulkan directo (ash) | OpenGL/WebGPU/Abstracto |
| **Lenguaje** | 100% Rust | Multi-lenguaje |
| **Control** | Total sobre GPU | Limitado por abstracciones |
| **Rendimiento** | Optimizado para miles de nodos | Degradación con escala |
| **Extensibilidad** | Arquitectura modular | Plugins limitados |

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

---

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor, abre un issue primero para discutir cambios mayores.

---

<div align="center">

**Construido con 🦀 Rust y 🔥 Vulkan**

*Ultra-Omega v2.0 - Control Total. Rendimiento Extremo.*

</div>
