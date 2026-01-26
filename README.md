# 🔥 Ultra-Omega

<div align="center">

![Ultra-Omega Logo](https://img.shields.io/badge/Ultra--Omega-Node%20Lab-orange?style=for-the-badge&logo=rust)

**Sistema de Desarrollo Visual Basado en Nodos**  
*Inspirado en Houdini + Unreal Engine 5 + Visual Studio Code*

Desarrollado por **Eddi Andreé Salazar Matos** 🇵🇪

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![ASM](https://img.shields.io/badge/ASM-NASM%20x64-red.svg)](https://www.nasm.us/)
[![Java](https://img.shields.io/badge/Java-25-orange.svg)](https://www.java.com/)
[![Python](https://img.shields.io/badge/Python-3.12-blue.svg)](https://www.python.org/)
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

El sistema permite crear proyectos complejos conectando nodos visuales, donde cada nodo puede contener código en 5 lenguajes principales (Assembly, Rust, Java, Python, C++) y heredar código de nodos padre mediante un sistema de herencia visual.

### Filosofía de Diseño

- **Visual First**: Todo se puede hacer visualmente, sin necesidad de escribir código manualmente
- **Herencia de Código**: Los nodos pueden heredar código de sus nodos padre, permitiendo composición y reutilización
- **5 Lenguajes Principales**: Rust (core), Assembly (bajo nivel), Java (enterprise), Python (scripting), C++ (legacy/moderno)
- **Templates Pre-construidos**: 91+ templates listos para usar (22 Rust, 26 ASM, 28 Java, 6 Python, 9 C++)
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

- **Editor Multi-lenguaje**: Soporte para Assembly (NASM), Rust, Java 25, Python 3.12, C++ (11/14/17)
- **Resaltado de Sintaxis**: Colores personalizados por lenguaje
- **Historial de Edición**: Deshacer/Rehacer con `Ctrl+Z` / `Ctrl+Y`
- **Búsqueda y Reemplazo**: `Ctrl+F` para buscar, `Ctrl+H` para reemplazar
- **Exportación Rápida**: `Ctrl+R` para exportar selección a parámetros

### 🔗 Sistema de Nodos

- **Tipos de Nodos**:
  - `Auto`: Detecta automáticamente el lenguaje
  - `Asm`: Assembly NASM (x86_64) - Linux y Windows
  - `Rust`: Rust moderno y seguro
  - `Java`: Java 25 para aplicaciones enterprise
  - `Python`: Python 3.12 para scripting y automatización
  - `Cpp`: C++ (11, 14, 17) para legacy y alto rendimiento
  - `Text`: Texto/Documentación (no se compila)

- **Pines de Conexión**: Entradas y salidas para conectar nodos
- **Herencia de Código**: Los nodos heredan código de sus nodos padre automáticamente
- **Código Propio vs Heredado**: Visualiza y edita solo el código propio de cada nodo

### 📦 Sistema de Templates

**91+ templates pre-construidos listos para usar:**

- **Rust (22 templates)**: Core del sistema, aplicaciones de alto rendimiento
- **Assembly (26 templates)**: Bootloaders, código de bajo nivel (Linux + Windows)
- **Java (28 templates)**: Aplicaciones enterprise, multiplataforma
- **Python (6 templates)**: Scripting, automatización, prototipado rápido
- **C++ (9 templates)**: Legacy y moderno (3 por versión: 11, 14, 17)

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
  - **NASM** (para Assembly x64)
  - **Rustc** (para Rust)
  - **JDK 25** (para Java 25) - Ver [Instalación de Java 25](#instalación-de-java-25)
  - **Python 3.12** (para Python)
- **Dependencias del Sistema**:
  - Linux: `libxcb`, `libx11`, `libxrandr`, `libasound2`
  - Windows: No requiere dependencias adicionales
  - macOS: No requiere dependencias adicionales

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

### Instalación con Soporte Extendido (Opcional)

```bash
# Compilar con features adicionales cuando estén disponibles
cargo build --release --features [feature-name]
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
2. **Seleccionar Template**: Elige entre 82+ templates disponibles
   - **Rust**: Aplicaciones de alto rendimiento
   - **Assembly**: Bootloaders y código de bajo nivel
   - **Java**: Aplicaciones enterprise multiplataforma
   - **Python**: Scripts y automatización
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

### Ejemplos Rápidos por Lenguaje

#### Rust - Hello World
```rust
// Nodo: main_rust
fn main() {
    println!("Hello from Ultra-Omega!");
}
```

#### Assembly - Hello World (Linux)
```asm
; Nodo: main_asm
section .data
    msg db 'Hello from Assembly!', 0x0A
    len equ $ - msg

section .text
    global _start

_start:
    mov rax, 1          ; syscall write
    mov rdi, 1          ; stdout
    mov rsi, msg        ; message
    mov rdx, len        ; length
    syscall
    mov rax, 60         ; syscall exit
    xor rdi, rdi        ; status 0
    syscall
```

#### Java - Hello World
```java
// Nodo: Main.java
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello from Java 25!");
    }
}
```

#### Python - Hello World
```python
# Nodo: main.py
print("Hello from Python 3.12!")
```

#### C++11 - Hello Modern
```cpp
// Nodo: main_cpp11.cpp
#include <iostream>
#include <vector>
#include <memory>

int main() {
    auto numbers = std::make_unique<std::vector<int>>(std::vector<int>{1,2,3,4,5});
    
    for (const auto& num : *numbers) {
        std::cout << "C++11: " << num << std::endl;
    }
    
    return 0;
}
```

### Combinar Lenguajes con Herencia

```rust
// Nodo: base (Rust)
pub fn common_function() {
    println!("Base functionality");
}
```

```rust
// Nodo: app (Rust) - hereda de base
use crate::base::*;

fn main() {
    common_function();  // Heredado
    println!("App specific");
}
```

---

## 🔧 Soporte para C++ Legacy

### ¿Por qué Rust en lugar de C++?

Ultra-Omega ha sido diseñado con **Rust como lenguaje principal** en lugar de C++ por estas razones:

- **Seguridad de Memoria**: Rust previene errores comunes de C++ (null pointers, buffer overflows)
- **Rendimiento**: Rust ofrece rendimiento comparable a C++ con garantías de seguridad
- **Modernidad**: Rust tiene características modernas que C++ carece (pattern matching, ownership system)
- **Interoperabilidad**: Rust puede llamar código C++ existente mediante FFI

### Migrar desde C++ Legacy

Si tienes proyectos C++ existentes, puedes integrarlos de varias formas:

#### Opción 1: Interoperabilidad FFI
```rust
// Nodo: rust_wrapper
extern "C" {
    fn cpp_function(arg: i32) -> i32;
}

fn main() {
    let result = unsafe { cpp_function(42) };
    println!("Resultado de C++: {}", result);
}
```

#### Opción 2: Reescribir Gradualmente
- Convierte módulos C++ a Rust uno por uno
- Usa nodos Rust para nueva funcionalidad
- Mantén código C++ crítico en nodos Assembly para máximo control

#### Opción 3: Templates de Transición
Ultra-Omega incluye templates específicos para migración:
- **cpp-to-rust/**: Plantillas para reescribir código C++ en Rust
- **legacy-bridge/**: Puentes entre código existente y nuevo

### Beneficios de la Migración

| Característica | C++ Legacy | Rust (Ultra-Omega) |
|---------------|------------|-------------------|
| **Seguridad** | Manual (RAII) | Automática (ownership) |
| **Concurrencia** | Compleja | Segura por diseño |
| **Memory Leaks** | Posibles | Eliminados |
| **Tooling** | Variado | Integrado en Ultra-Omega |
| **Performance** | Excelente | Excelente + Seguridad |

### Ejemplo: Sistema C++ → Ultra-Omega

**Antes (C++ puro):**
```cpp
// main.cpp
#include <iostream>
#include <vector>
#include <memory>

class Processor {
    std::vector<int> data;
public:
    void add(int value) { data.push_back(value); }
    void process() {
        for(auto& item : data) {
            std::cout << item * 2 << std::endl;
        }
    }
};

int main() {
    auto proc = std::make_unique<Processor>();
    proc->add(1);
    proc->add(2);
    proc->process();
    return 0;
}
```

**Después (Ultra-Omega con Rust):**
```rust
// Nodo: data_processor (Rust)
pub struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    
    pub fn add(&mut self, value: i32) {
        self.data.push(value);
    }
    
    pub fn process(&self) {
        for item in &self.data {
            println!("{}", item * 2);
        }
    }
}
```

```rust
// Nodo: main_app (Rust) - hereda de data_processor
use crate::data_processor::*;

fn main() {
    let mut proc = DataProcessor::new();
    proc.add(1);
    proc.add(2);
    proc.process();
}
```

### Recursos de Migración

- **Book**: "The Rust Programming Language" - Capítulo sobre FFI
- **Templates**: Ver sección de templates de migración en Ultra-Omega
- **Comunidad**: Discord/Rust para preguntas específicas de migración

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
│   ├── templates/           # Templates pre-construidos (91 archivos)
│   │   ├── rust/            # 🦀 22 templates Rust
│   │   ├── asm-linux/       # ⚙️ 13 templates Assembly Linux
│   │   ├── asm-windows/     # ⚙️ 13 templates Assembly Windows
│   │   ├── java/            # ☕ 28 templates Java 25
│   │   ├── python/          # 🐍 6 templates Python 3.12
│   │   ├── cpp/             # 🔷 9 templates C++ (11/14/17)
│   │   │   ├── cpp11/       # 3 templates C++11
│   │   │   ├── cpp14/       # 3 templates C++14
│   │   │   └── cpp17/       # 3 templates C++17
│   │   └── mod.rs           # Registro de templates
│   ├── storage/             # Sistema de almacenamiento
│   │   ├── workspace.rs     # Gestión de workspace
│   │   ├── node_storage.rs  # Código separado por archivos
│   │   └── migration.rs     # Migración de proyectos
│   ├── compilation/         # Sistema de compilación
│   │   ├── terminal.rs      # Terminal integrada
│   │   └── compiler_detector.rs
│   ├── inheritance/         # Sistema de herencia de código
│   └── utils/               # Utilidades varias
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

### 🦀 Templates Rust (22 archivos)

Aplicaciones de alto rendimiento y sistemas core:

- **Básicos**: hello_world, cli_app, web_server
- **Sistemas**: kernel_module, bootloader_rust, os_dev
- **Avanzados**: game_engine, compiler, database
- **Librerías**: math_lib, crypto_lib, network_lib

**Uso**:
```rust
let graph = NodeGraph::create_rust_project("hello_world");
```

### ⚙️ Templates Assembly (26 archivos)

Código de bajo nivel para Linux y Windows:

#### Linux (13 templates)
- **Bootloaders**: bootloader_linux, stage2, kernel_entry
- **Syscalls**: syscalls_basic, file_operations, memory_mgmt
- **Sistemas**: minimal_kernel, driver_template, interrupt_handler

#### Windows (13 templates)
- **Bootloaders**: bootloader_windows, pe_loader, stage2_win
- **Sistemas**: driver_windows, service_template, registry_ops
- **Utilidades**: dll_template, syscalls_win, memory_win

**Uso**:
```rust
let graph = NodeGraph::create_asm_project("bootloader_linux", Platform::Linux);
```

### ☕ Templates Java (28 archivos)

Aplicaciones enterprise y multiplataforma:

- **Básicos**: hello_java, console_app, basic_oop
- **Web**: spring_boot, web_api, microservice
- **Desktop**: javafx_app, swing_app, gui_template
- **Enterprise**: enterprise_app, database_jdbc, jpa_template
- **Avanzados**: concurrent_app, network_server, security_app
- **Librerías**: math_lib, json_parser, logging_lib

**Uso**:
```rust
let graph = NodeGraph::create_java_project("spring_boot");
```

### �️ C++ Templates (9 archivos)

Soporte completo para C++ legacy y moderno:

#### C++11 (3 templates)
- **Básicos**: hello_modern (auto, lambda, smart pointers)
- **Intermedio**: threading_demo (std::thread, mutex, atomic)
- **Avanzado**: stl_features (array, forward_list, unordered)

#### C++14 (3 templates)
- **Básicos**: generic_features (generic lambdas, variable templates)
- **Intermedio**: stl_improvements (make_unique, chrono literals)
- **Avanzado**: template_meta (constexpr, decltype(auto))

#### C++17 (3 templates)
- **Básicos**: core_features (structured bindings, optional, variant)
- **Intermedio**: parallel_filesystem (parallel algorithms, std::filesystem)
- **Avanzado**: advanced_features (fold expressions, CTAD, constexpr lambda)

**Uso**:
```bash
# C++11
cd cpp11/
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
./UltraOmega_CPP11

# C++14
cd ../cpp14/
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
./UltraOmega_CPP14

# C++17
cd ../cpp17/
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release -DENABLE_PARALLEL=ON -DENABLE_FILESYSTEM=ON
make
./UltraOmega_CPP17
```

### Combinación de Templates

Puedes combinar templates de diferentes lenguajes:

```rust
// Proyecto híbrido: Bootloader ASM + Kernel Rust + App Java
let mut graph = NodeGraph::new();
graph.add_template("bootloader_linux", Language::Asm);
graph.add_template("kernel_rust", Language::Rust);
graph.add_template("user_app", Language::Java);
```

### 🛠️ Configuración y Compilación

#### Requisitos del Sistema

- **CMake**: Versión 3.10 o superior (3.12+ para C++17)
- **Compilador C++**: 
  - GCC 4.8.1+ (C++11), 4.9+ (C++14), 7+ (C++17)
  - Clang 3.3+ (C++11), 3.4+ (C++14), 5+ (C++17)
  - MSVC 2013+ (C++11), 2015+ (C++14), 2017+ (C++17)

#### Compilación Rápida

```bash
# Para cualquier versión de C++
cd [version]/  # cpp11/, cpp14/, o cpp17/
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)  # Linux/macOS
# o
cmake --build . --config Release  # Windows
```

#### Opciones de CMake

**Opciones Generales:**
- `-DCMAKE_BUILD_TYPE=Debug|Release|RelWithDebInfo`
- `-DENABLE_WARNINGS=ON|OFF` (default: ON)
- `-DENABLE_DEBUG=ON|OFF` (default: ON)

**Opciones Específicas de C++17:**
- `-DENABLE_PARALLEL=ON|OFF` (default: ON)
- `-DENABLE_FILESYSTEM=ON|OFF` (default: ON)
- `-DENABLE_OPTIMIZATIONS=ON|OFF` (default: ON)

#### Targets Personalizados

```bash
# Ejecutar el programa
make run
# o
cmake --build . --target run

# Limpiar y recompilar
make rebuild

# Ver información del compilador
make compiler_info

# Verificar soporte de versión (C++14/17)
make check_cpp14  # o check_cpp17

# Probar filesystem (solo C++17)
make test_filesystem

# Benchmark de algoritmos paralelos (solo C++17)
make benchmark_parallel
```

#### Detección Automática de Versión

Todos los templates incluyen `cpp_version_detection.hpp` que:
- Detecta automáticamente la versión de C++ en tiempo de compilación
- Muestra información detallada del compilador y características disponibles
- Proporciona macros para verificar características específicas
- Incluye headers apropiados según la versión detectada

**Ejemplo de salida:**
```
╔══════════════════════════════════════════════════════════════╗
║ Ultra-Omega C++ Template - Versión Detectada: C++17           ║
║ Compilado con estándar: 201703                                   ║
╚══════════════════════════════════════════════════════════════╝

🔍 Características C++ Disponibles:
   ✓ Auto keyword disponible
   ✓ Lambda expressions disponibles
   ✓ Smart pointers disponibles
   ✓ Generic lambdas disponibles
   ✓ Structured bindings disponibles
   ✓ std::optional disponible
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
# Con soporte extendido cuando esté disponible
cargo run --features [feature-name]
```

### Estructura de Código

- **Rust 2021 Edition**: Usa la edición 2021 de Rust
- **eframe**: Framework de UI inmediata con wgpu
- **serde**: Serialización/Deserialización JSON
- **rfd**: Diálogos de archivos nativos
- **dirs**: Gestión de directorios del sistema
- **which**: Detección de compiladores
- **glob**: Búsqueda de archivos con patrones
- **chrono**: Manejo de fechas y tiempos

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
- [Templates Rust](src/templates/rust/README.md)
- [Templates Assembly](src/templates/asm-linux/README.md) / [Assembly Windows](src/templates/asm-windows/README.md)
- [Templates Java](src/templates/java/README.md)
- [Templates Python](src/templates/python/README.md)
- [IDEAS y Arquitectura](IDEAS.md)

---

## 🐛 Problemas Conocidos

- El sistema de expresiones puede tener problemas con referencias circulares (en desarrollo)
- Algunos templates pueden requerir herramientas externas (NASM, GCC, JDK, Python)
- El rendimiento puede degradarse con grafos muy grandes (>1000 nodos)
- La detección automática de compiladores puede fallar en sistemas no estándar

---

## 🔮 Roadmap

### Próximas Características

- [ ] Validación de referencias circulares en expresiones
- [ ] Cache de resultados de expresiones
- [ ] Sistema de subnetworks (nodos que contienen grafos)
- [ ] Sistema de plugins para extender lenguajes
- [ ] Exportación a código fuente compilable
- [ ] Integración con debuggers externos
- [ ] Sistema de debugging visual
- [ ] Mejores templates de migración C++ → Rust
- [ ] Colaboración en tiempo real (multiusuario)
- [ ] Minimap/Navigator del grafo

### Prioridades 2026

1. **Q1**: Sistema de expresiones robusto
2. **Q2**: Templates de migración C++
3. **Q3**: Subnetworks y navegación jerárquica
4. **Q4**: Sistema de plugins base

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

- **Rust**: Lenguaje de programación principal
- **eframe**: Framework de UI inmediata con wgpu
- **serde**: Serialización JSON
- **rfd**: Diálogos de archivos
- **NASM**: Ensamblador para templates Assembly
- **JDK 25**: Compilador para templates Java
- **Python 3.12**: Intérprete para templates Python
- **G++/Clang**: Compiladores para templates C++ (11/14/17)

---

## 📞 Contacto

Para preguntas, sugerencias o colaboraciones, por favor abre un issue en el repositorio.

---

<div align="center">

**Hecho con ❤️ en Perú 🇵🇪**

![Peru Flag](https://img.shields.io/badge/Peru-🇵🇪-red?style=flat-square)

</div>

