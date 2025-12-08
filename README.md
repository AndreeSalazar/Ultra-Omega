# 🔥 Ultra-Omega

<div align="center">

![Ultra-Omega Logo](https://img.shields.io/badge/Ultra--Omega-Node%20Lab-orange?style=for-the-badge&logo=rust)

**Sistema de Desarrollo Visual Basado en Nodos**  
*Inspirado en Houdini + Unreal Engine 5 + Visual Studio Code*

Desarrollado por **Eddi Andreé Salazar Matos** 🇵🇪

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Java](https://img.shields.io/badge/Java-25-red.svg)](https://www.java.com/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)]()

</div>

---

## 📋 Tabla de Contenidos

- [Descripción](#-descripción)
- [Características Principales](#-características-principales)
- [Instalación](#-instalación)
- [Uso Básico](#-uso-básico)
- [Arquitectura](#-arquitectura)
- [Sistema de Templates](#-sistema-de-templates)
- [Sistema de Expresiones](#-sistema-de-expresiones)
- [Atajos de Teclado](#-atajos-de-teclado)
- [Estructura del Proyecto](#-estructura-del-proyecto)
- [Desarrollo](#-desarrollo)
- [Créditos](#-créditos)

---

## 🎯 Descripción

**Ultra-Omega** es un entorno de desarrollo visual basado en nodos que combina la potencia de sistemas como Houdini (para efectos visuales y programación procedural), la flexibilidad de Unreal Engine 5 (para desarrollo de juegos y aplicaciones), y la experiencia de usuario de Visual Studio Code.

El sistema permite crear proyectos complejos conectando nodos visuales, donde cada nodo puede contener código en diferentes lenguajes (Assembly, C, C++, Rust, Zig, **Java 25**, Mojo) y heredar código de nodos padre mediante un sistema de herencia visual.

### Filosofía de Diseño

- **Visual First**: Todo se puede hacer visualmente, sin necesidad de escribir código manualmente
- **Herencia de Código**: Los nodos pueden heredar código de sus nodos padre, permitiendo composición y reutilización
- **Multi-lenguaje**: Soporte nativo para múltiples lenguajes de programación
- **Templates Pre-construidos**: Proyectos completos listos para usar (FastOS, Vulkan, etc.)
- **Expresiones Houdini-style**: Sistema de expresiones `ch()` para referenciar valores de otros nodos

---

## ✨ Características Principales

### 🎨 Interfaz Visual

- **Editor de Nodos Visual**: Arrastra, conecta y organiza nodos en un viewport 2D
- **Tema Visual Studio Code Dark+**: Interfaz familiar con colores y estilos de VS Code
- **Zoom y Pan**: Navegación fluida por el viewport con scroll y arrastre
- **Selección Múltiple**: Selecciona múltiples nodos con box selection (arrastrar con Shift)
- **Herencia Visual**: Visualiza la cadena de herencia de código con `Ctrl+I`

### 💻 Editor de Código Integrado

- **Editor Multi-lenguaje**: Soporte para Assembly (NASM), C, C++, Rust, Mojo, y texto plano
- **Resaltado de Sintaxis**: Colores personalizados por lenguaje
- **Historial de Edición**: Deshacer/Rehacer con `Ctrl+Z` / `Ctrl+Y`
- **Búsqueda y Reemplazo**: `Ctrl+F` para buscar, `Ctrl+H` para reemplazar
- **Exportación Rápida**: `Ctrl+R` para exportar selección a parámetros

### 🔗 Sistema de Nodos

- **Tipos de Nodos**:
  - `Auto`: Detecta automáticamente el lenguaje
  - `Asm`: Assembly NASM (x86_64)
  - `C`: C estándar
  - `Cpp`: C++ moderno
  - `Rust`: Rust
  - `Text`: Texto/Documentación (no se compila)
  - `Mojo`: Lenguaje Mojo para IA/ML
  - `MojoAI`: Nodos especializados con capacidades de IA

- **Pines de Conexión**: Entradas y salidas para conectar nodos
- **Herencia de Código**: Los nodos heredan código de sus nodos padre automáticamente
- **Código Propio vs Heredado**: Visualiza y edita solo el código propio de cada nodo

### 📦 Sistema de Templates

Proyectos completos pre-construidos listos para usar:

- **FastOS ASM+Rust+Zig**: Sistema operativo multi-lenguaje completo
- **Vulkan**: Aplicación Vulkan completa con shaders
- **DirectX12**: Aplicación DirectX 12 completa para Windows
- **Lenguajes**: Templates para Assembly, C, C++, Rust, Zig, **Java 25**
- **Java 25**: Soporte completo con 27 templates (básico, intermedio, avanzado, librerías)

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

- **Workspace**: Abre carpetas como workspaces (similar a VS Code)
- **Auto-guardado**: Guarda automáticamente el estado del grafo
- **Persistencia**: Guarda posición de ventana, tamaño, y configuración
- **Exportación**: Exporta el grafo completo a JSON

### 🖥️ Terminal Integrado

- **Terminal Multi-tab**: Múltiples terminales en pestañas
- **Integración con Workspace**: Ejecuta comandos en el directorio del workspace
- **Historial**: Historial de comandos ejecutados

---

## 🚀 Instalación

### Requisitos

- **Rust**: Versión 1.70 o superior
- **Cargo**: Gestor de paquetes de Rust (incluido con Rust)
- **Compiladores** (opcionales, según los lenguajes que uses):
  - **NASM** (para Assembly)
  - **GCC/Clang** (para C/C++)
  - **Rustc** (para Rust)
  - **Zig** (para Zig)
  - **JDK 25** (para Java 25) - Ver [Instalación de Java 25](#instalación-de-java-25)
- **Dependencias del Sistema**:
  - Linux: `libxcb`, `libx11`, `libxrandr`, `libasound2`
  - Windows: No requiere dependencias adicionales
  - macOS: No requiere dependencias adicionales

### Instalación desde Código Fuente

```bash
# Clonar el repositorio
git clone <repository-url>
cd Ultra-Omega

# Compilar en modo release
cargo build --release

# Ejecutar
cargo run --release
```

### Instalación de Java 25

Para usar templates y compilar código Java 25:

#### Windows

```powershell
# Opción 1: Chocolatey
choco install temurin25-jdk

# Opción 2: Descarga manual
# Visita: https://adoptium.net/
# Descarga JDK 25 y agrega a PATH
```

#### Linux

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install openjdk-25-jdk

# O desde Adoptium
wget https://github.com/adoptium/temurin25-binaries/releases/download/...
```

#### macOS

```bash
# Homebrew
brew install --cask temurin25

# O descarga desde Adoptium
```

**Verificar instalación:**
```bash
java -version
javac -version
```

Deberías ver algo como:
```
openjdk version "25" 2025-XX-XX
OpenJDK Runtime Environment (build 25+XX)
OpenJDK 64-Bit Server VM (build 25+XX, mixed mode, sharing)
```

### Instalación con Soporte Mojo (Opcional)

```bash
# Compilar con feature mojo
cargo build --release --features mojo
```

---

## 📖 Uso Básico

### Crear un Nuevo Proyecto

1. **Abrir Workspace**: `File > Open Workspace` o `Ctrl+O`
2. **Crear Nodo**: Presiona `Tab` o haz clic derecho en el viewport
3. **Escribir Código**: Doble clic en un nodo para editar su código
4. **Conectar Nodos**: Arrastra desde un pin de salida a un pin de entrada
5. **Guardar**: `Ctrl+S` para guardar el grafo

### Crear un Proyecto desde Template

1. **Menú de Templates**: `File > New from Template`
2. **Seleccionar Template**: Elige entre FastOS, Vulkan, etc.
3. **Personalizar**: Edita los nodos generados según tus necesidades

### Herencia de Código

1. **Conectar Nodos**: Conecta el pin de salida de un nodo al pin de entrada de otro
2. **Heredar Código**: El nodo hijo hereda automáticamente el código del padre
3. **Ver Herencia**: Presiona `Ctrl+I` en un nodo para ver la cadena de herencia
4. **Editar Código Propio**: El editor muestra solo el código propio del nodo

### Usar Expresiones

1. **Registrar Nodo**: Los nodos se registran automáticamente en el sistema de canales
2. **Referenciar**: Usa `ch("nombre_nodo")` en expresiones
3. **Evaluar**: El sistema evalúa las expresiones automáticamente

---

## 🏗️ Arquitectura

### Componentes Principales

```
Ultra-Omega/
├── src/
│   ├── main.rs              # Punto de entrada, inicialización de ventana
│   ├── app.rs               # Lógica principal de la aplicación
│   ├── node_graph.rs        # Estructura de datos del grafo de nodos
│   ├── workspace.rs         # Gestión de workspace y persistencia
│   ├── config.rs            # Configuración de la aplicación
│   ├── terminal.rs          # Terminal integrado
│   ├── editor_history.rs    # Historial de edición (undo/redo)
│   ├── ui/                  # Interfaz de usuario
│   │   ├── viewport.rs      # Viewport 2D para nodos
│   │   ├── nodes.rs         # Renderizado de nodos
│   │   ├── connectors.rs   # Renderizado de conexiones
│   │   ├── sidebar.rs       # Barra lateral
│   │   ├── menu/            # Menús (File, Edit, View, etc.)
│   │   └── layout.rs        # Sistema de layout automático
│   ├── expressions/         # Sistema de expresiones Houdini-style
│   │   ├── channels.rs      # Gestión de canales
│   │   ├── parser.rs        # Parser de expresiones
│   │   └── evaluator.rs     # Evaluador de expresiones
│   ├── templates/           # Templates pre-construidos
│   │   ├── fastos/          # Sistema operativo FastOS
│   │   ├── fastos64/        # FastOS 64-bit
│   │   ├── fastos64_rust/   # FastOS con Rust
│   │   ├── vulkan/          # Templates Vulkan
│   │   ├── binary/          # Templates binarios
│   │   └── [lenguajes]/     # Templates por lenguaje
│   └── mojo/                # Soporte Mojo (opcional)
│       ├── ai.rs            # Nodos de IA
│       ├── math.rs          # Funciones matemáticas
│       └── evaluator.rs     # Evaluador Mojo
```

### Flujo de Datos

```
Usuario → UI → App → NodeGraph → Expressions → Channels
                ↓
            Workspace → Persistencia (JSON)
```

### Sistema de Herencia

```
Nodo A (código base)
    ↓ (conexión)
Nodo B (hereda A + código propio)
    ↓ (conexión)
Nodo C (hereda A + B + código propio)
```

---

## 📦 Sistema de Templates

### FastOS

Sistema operativo educativo completo con dos ramas:

- **Rama ASM (Izquierda)**: Bootloader, stage2, kernel_entry
- **Rama C (Derecha)**: Headers, drivers, kernel
- **Combinador**: Une ambas ramas en un sistema operativo completo

**Uso**:
```rust
let graph = NodeGraph::create_fastos_project();
```

### Vulkan

Aplicación Vulkan completa con pipeline de renderizado:

- **Inicialización**: Instance, Device, Swapchain
- **Pipeline**: Graphics pipeline con shaders
- **Recursos**: Buffers, Textures
- **Renderizado**: Command buffers, sync, render loop

**Uso**:
```rust
let graph = NodeGraph::create_vulkan_project();
```

### Binary Templates

Templates binarios ejecutables para CPU y GPU:

- **CPU**: Hello World, syscalls, operaciones matemáticas
- **GPU**: Shaders SPIR-V, vertex/fragment shaders

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

// Variables
$variable
$PI * 2
```

### Integración

El sistema de expresiones se integra automáticamente con el grafo de nodos:

1. **Registro Automático**: Cada nodo se registra en el `ChannelManager`
2. **Evaluación en Tiempo Real**: Las expresiones se evalúan cuando se necesitan
3. **Cache**: Los resultados se cachean para mejor rendimiento

### Ejemplo de Uso

```rust
// En el código de un nodo
let code = format!(
    r#"
    // Código base
    {}
    
    // Código adicional
    int main() {{
        return 0;
    }}
    "#,
    ch("nodo_base")
);
```

---

## ⌨️ Atajos de Teclado

### Navegación

| Atajo | Acción |
|-------|--------|
| `Tab` | Mostrar/Ocultar menú de creación de nodos |
| `F3` | Búsqueda rápida estilo Blender |
| `F` | Enfocar viewport en los nodos |
| `Ctrl + Scroll` | Zoom in/out |
| `Middle Click + Drag` | Pan del viewport |

### Edición

| Atajo | Acción |
|-------|--------|
| `Double Click` | Editar nodo |
| `Ctrl + Z` | Deshacer |
| `Ctrl + Y` | Rehacer |
| `Ctrl + F` | Buscar en editor |
| `Ctrl + H` | Reemplazar en editor |
| `Ctrl + R` | Exportar selección a parámetros |
| `Ctrl + Shift + P` | Modo múltiples parámetros |
| `Ctrl + I` | Ver cadena de herencia |

### Archivos

| Atajo | Acción |
|-------|--------|
| `Ctrl + O` | Abrir workspace |
| `Ctrl + S` | Guardar |
| `Ctrl + Shift + S` | Guardar como |
| `Ctrl + N` | Nuevo proyecto |

### Selección

| Atajo | Acción |
|-------|--------|
| `Shift + Drag` | Selección múltiple (box selection) |
| `Delete` | Eliminar nodos seleccionados |
| `Ctrl + A` | Seleccionar todos los nodos |

### Terminal

| Atajo | Acción |
|-------|--------|
| `Ctrl + ~` | Mostrar/Ocultar terminal |
| `Ctrl + T` | Nueva pestaña de terminal |

---

## 📁 Estructura del Proyecto

```
Ultra-Omega/
├── Cargo.toml              # Configuración del proyecto Rust
├── Cargo.lock              # Lock file de dependencias
├── README.md               # Este archivo
├── src/                    # Código fuente
│   ├── main.rs            # Punto de entrada
│   ├── app.rs             # Lógica principal
│   ├── node_graph.rs      # Grafo de nodos
│   ├── workspace.rs       # Workspace
│   ├── config.rs          # Configuración
│   ├── terminal.rs        # Terminal
│   ├── editor_history.rs  # Historial
│   ├── ui/                # Interfaz de usuario
│   ├── expressions/       # Sistema de expresiones
│   ├── templates/         # Templates
│   └── mojo/              # Soporte Mojo
├── target/                # Archivos de compilación
└── program.exe            # Ejecutable (si existe)
```

---

## 🛠️ Desarrollo

### Compilar

```bash
# Modo debug (desarrollo)
cargo build

# Modo release (producción)
cargo build --release
```

### Ejecutar Tests

```bash
cargo test
```

### Ejecutar con Features

```bash
# Con soporte Mojo
cargo run --features mojo
```

### Estructura de Código

- **Rust 2021 Edition**: Usa la edición 2021 de Rust
- **egui**: Framework de UI inmediata
- **serde**: Serialización/Deserialización JSON
- **rfd**: Diálogos de archivos nativos

### Contribuir

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

---

## 🎨 Personalización

### Temas

El tema actual está basado en Visual Studio Code Dark+. Para personalizar:

1. Edita `src/main.rs` función `apply_vscode_theme()`
2. Modifica los colores en `Visuals`
3. Recompila

### Templates Personalizados

Crea tus propios templates:

1. Agrega archivos en `src/templates/`
2. Expórtalos en `src/templates/mod.rs`
3. Crea una función `create_*_project()` en `node_graph.rs`

---

## 📚 Documentación Adicional

- [Sistema de Expresiones](src/expressions/README.md)
- [Templates FastOS](src/templates/fastos/README.md)
- [Templates Vulkan](src/templates/vulkan/README.md)
- [Templates Binary](src/templates/binary/README.md)

---

## 🐛 Problemas Conocidos

- El sistema de expresiones puede tener problemas con referencias circulares (en desarrollo)
- Algunos templates pueden requerir herramientas externas (NASM, GCC, etc.)
- El rendimiento puede degradarse con grafos muy grandes (>1000 nodos)

---

## 🔮 Roadmap

### Próximas Características

- [ ] Validación de referencias circulares en expresiones
- [ ] Cache de resultados de expresiones
- [ ] Soporte para más lenguajes (Python, JavaScript, etc.)
- [ ] Sistema de plugins
- [ ] Exportación a código fuente compilable
- [ ] Integración con compiladores externos
- [ ] Sistema de debugging visual
- [ ] Colaboración en tiempo real

---

## 📄 Licencia

Este proyecto está bajo la licencia MIT. Ver `LICENSE` para más detalles.

---

## 👤 Créditos

**Desarrollado por:** Eddi Andreé Salazar Matos 🇵🇪

### Inspiraciones

- **Houdini**: Sistema de expresiones y programación procedural
- **Unreal Engine 5**: Blueprint system y desarrollo visual
- **Visual Studio Code**: Interfaz de usuario y experiencia

### Tecnologías Utilizadas

- **Rust**: Lenguaje de programación
- **egui**: Framework de UI inmediata
- **serde**: Serialización
- **rfd**: Diálogos de archivos

---

## 📞 Contacto

Para preguntas, sugerencias o colaboraciones, por favor abre un issue en el repositorio.

---

<div align="center">

**Hecho con ❤️ en Perú 🇵🇪**

![Peru Flag](https://img.shields.io/badge/Peru-🇵🇪-red?style=flat-square)

</div>

