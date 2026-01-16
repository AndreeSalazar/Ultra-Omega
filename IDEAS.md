# 🚀 Ultra-Omega - Ideas y Arquitectura

## 🎯 Visión del Proyecto

**Ultra-Omega** es un sistema de desarrollo visual basado en nodos enfocado en **4 lenguajes principales**:

| Lenguaje | Propósito | Icono |
|----------|-----------|-------|
| **Rust** | Core del sistema, rendimiento, seguridad | 🦀 |
| **ASM** | Bajo nivel, bootloaders, optimización | ⚙️ |
| **Java** | Aplicaciones empresariales, multiplataforma | ☕ |
| **Python** | Scripting, automatización, prototipado | 🐍 |

---

## 📁 Estructura de Templates

```
src/templates/
├── rust/           # 🦀 Templates Rust (22 archivos)
├── asm-linux/      # ⚙️ Templates ASM Linux (13 archivos)
├── asm-windows/    # ⚙️ Templates ASM Windows (13 archivos)
├── java/           # ☕ Templates Java 25 (28 archivos)
├── python/         # 🐍 Templates Python 3.12 (6 archivos)
└── mod.rs          # Registro de templates
```

---

## 🔧 Sistema de Nodos

### Tipos de Nodos Soportados

```rust
pub enum NodeLanguage {
    Rust,       // 🦀 Rust - Core del sistema
    Asm,        // ⚙️ Assembly NASM (x86_64)
    Java,       // ☕ Java 25
    Python,     // 🐍 Python 3.12
    Auto,       // Detección automática
    Text,       // Documentación
}
```

### Sistema de Herencia

Los nodos pueden heredar código de nodos padre mediante conexiones:

```
Nodo A (código base)
    ↓ (conexión)
Nodo B (hereda A + código propio)
    ↓ (conexión)
Nodo C (hereda A + B + código propio)
```

### Expresiones `ch()`

Sistema inspirado en Houdini para referenciar valores:

```rust
ch("nodo_padre")           // Código completo del nodo
ch("carpeta/nodo")         // Nodo dentro de carpeta
ch("nodo", "parametro")    // Parámetro específico
```

---

## 📂 Sistema de Carpetas (Nodo Carpeta)

### Dos Modos de Operación

1. **Modo Organización**: Solo agrupar nodos visualmente
2. **Modo Heredable**: Librería reutilizable con enforcement de lenguaje

### Características

- **Enforcement de Lenguaje**: Carpetas heredables solo aceptan un lenguaje
- **Vista Interna**: Entrar a carpeta con `Enter` o doble clic
- **Navegación**: Breadcrumbs y botón "Subir"
- **Herencia**: `ch("nombre_carpeta")` para heredar todo el contenido

---

## 🗂️ Sistema de Subnetworks

Inspirado en Houdini, permite crear nodos que contienen grafos completos:

### Características

- **Navegación Jerárquica**: Entrar/salir de subnetworks
- **Breadcrumbs**: Ruta visual `Root > Subnetwork1 > Subnetwork2`
- **Pines Expuestos**: Inputs/outputs del grafo interno
- **Guardado Recursivo**: Subnetworks anidados se guardan correctamente

### Atajos

- `Enter` - Entrar al subnetwork seleccionado
- `Esc` / `Backspace` - Salir del subnetwork actual

---

## 💾 Sistema de Storage

### Estructura de Proyecto

```
proyecto/
├── node_map.json          # Estructura y metadatos (sin código)
├── nodes/                 # Código fuente separado
│   ├── node_000001.rs
│   ├── node_000002.asm
│   ├── node_000003.java
│   └── node_000004.py
├── .ultra-omega/          # Configuración del proyecto
│   └── project.json
└── README.md
```

### Ventajas

- ✅ Código editable con syntax highlighting externo
- ✅ Versionado fácil con Git
- ✅ Backup incremental posible
- ✅ Node map más pequeño y rápido

---

## 🎯 Roadmap

### Prioridad Alta

- [ ] Completar integración de Python 3.12
- [ ] Sistema de herencia `ch()` completo
- [ ] Validación de dependencias circulares
- [ ] Hot reload de archivos externos

### Prioridad Media

- [ ] Sistema de HDAs (assets exportables)
- [ ] Búsqueda global en todos los nodos
- [ ] Minimap/Navigator del grafo

### Prioridad Baja

- [ ] Colaboración en tiempo real
- [ ] Sistema de plugins
- [ ] Integración con debuggers

---

## ⌨️ Atajos de Teclado

| Atajo | Acción |
|-------|--------|
| `Tab` | Menú de creación de nodos |
| `F3` | Búsqueda rápida |
| `F` | Enfocar viewport |
| `Ctrl+Z` | Deshacer |
| `Ctrl+Y` | Rehacer |
| `Ctrl+S` | Guardar |
| `Ctrl+I` | Ver cadena de herencia |
| `Enter` | Entrar a subnetwork |
| `Esc` | Salir de subnetwork |

---

## 🏗️ Arquitectura del Código

```
src/
├── main.rs                # Punto de entrada
├── core/                  # Núcleo del sistema
│   ├── app.rs            # Aplicación principal
│   ├── node_graph.rs     # Estructura de nodos
│   └── folder_node.rs    # Nodos carpeta
├── storage/               # Sistema de almacenamiento
│   ├── workspace.rs      # Gestión de workspace
│   ├── node_storage.rs   # Código separado
│   └── migration.rs      # Migración de proyectos
├── compilation/           # Sistema de compilación
│   ├── terminal.rs       # Terminal manager
│   └── compiler_detector.rs
├── ui/                    # Interfaz de usuario
│   ├── nodes.rs          # Renderizado de nodos
│   ├── sidebar.rs        # Sidebar
│   └── viewport.rs       # Viewport 2D
├── expressions/           # Sistema de expresiones ch()
└── templates/             # Templates de código
    ├── rust/
    ├── asm-linux/
    ├── asm-windows/
    ├── java/
    └── python/
```

---

## 📝 Notas de Desarrollo

### Lenguajes Eliminados (ya no soportados)

- ~~C~~ → Usar Rust
- ~~C++~~ → Usar Rust
- ~~Zig~~ → Usar Rust
- ~~DirectX12~~ → Enfoque en Rust/ASM
- ~~Vulkan~~ → Enfoque en Rust/ASM

### Principios de Diseño

1. **Rust First**: El core siempre en Rust
2. **ASM para bajo nivel**: Bootloaders, optimización crítica
3. **Java para enterprise**: Aplicaciones multiplataforma
4. **Python para scripting**: Automatización y prototipado

---

**Última actualización**: Enero 2026  
**Desarrollado por**: Eddi Andreé Salazar Matos 🇵🇪
