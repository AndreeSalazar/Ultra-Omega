# 🔧 Plan de Refactorización - Ultra-Omega

## 🎯 Objetivos

1. **Separar código del mapa de nodos** (Propuesta 1) ✅
2. **Reorganizar estructura de `src/`** para mejor gestión ✅
3. **Preparar para repositorios en la nube** (GitHub/GitLab) ⏳
4. **Inspiración Houdini**: Sistema avanzado de nodos jerárquicos, subnetworks, y workflows profesionales 🆕
5. **Inspiración VS Code**: Explorador de archivos en tiempo real, búsqueda avanzada, organización visual 🆕

---

## 📁 Nueva Estructura del Proyecto

### Estructura de Archivos del Proyecto
```
proyecto/
├── node_map.json          # Solo estructura, posición, links (sin código)
├── nodes/                 # Código fuente de los nodos
│   ├── node_000001.rs
│   ├── node_000002.asm
│   ├── node_000003.cpp
│   ├── node_000004.zig
│   └── ...
├── .ultra-omega/          # Metadatos y config (opcional)
│   └── project.json       # Configuración del proyecto
└── README.md              # Documentación del proyecto
```

### Nueva Estructura de `src/`
```
src/
├── main.rs                # Punto de entrada
├── config.rs              # Configuración de la app
│
├── core/                  # Núcleo del sistema
│   ├── mod.rs
│   ├── app.rs            # Aplicación principal (movido desde src/)
│   └── node_graph.rs     # Estructura de nodos (movido desde src/)
│
├── storage/               # Sistema de almacenamiento
│   ├── mod.rs
│   ├── workspace.rs      # Workspace (movido desde src/)
│   ├── project.rs        # Gestión de proyectos
│   ├── node_storage.rs   # Guardado/carga de código separado
│   └── migration.rs      # Migración de proyectos antiguos
│
├── compilation/           # Sistema de compilación
│   ├── mod.rs
│   ├── terminal.rs       # Terminal manager (movido desde src/)
│   ├── auto_linker.rs    # Auto-linker (movido desde src/)
│   └── compiler_detector.rs  # Detección de compiladores (movido desde src/)
│
├── ui/                    # Interfaz de usuario
│   ├── mod.rs
│   ├── nodes.rs          # Renderizado de nodos
│   ├── sidebar.rs        # Sidebar
│   ├── viewport.rs       # Viewport 2D
│   ├── layout.rs         # Sistema de layout
│   ├── menu/             # Menús
│   ├── connectors/       # Conectores
│   ├── cut/              # Herramienta de corte
│   └── nodes_semantic/   # Mapa semántico
│
├── expressions/           # Sistema de expresiones (ya existe)
│   └── ...
│
├── templates/             # Templates de código (ya existe)
│   └── ...
│
└── utils/                 # Utilidades
    ├── mod.rs
    ├── editor_history.rs  # Historial del editor (movido desde src/)
    └── serialization.rs   # Helpers de serialización (extraído de node_graph.rs)
```

---

## ✅ FASES COMPLETADAS

### Fase 1: Crear Sistema de Storage Separado ✅ COMPLETADO

#### ✅ **COMPLETADO:**

**Archivos creados:**
- ✅ `src/storage/mod.rs` - Módulo principal de storage
- ✅ `src/storage/node_storage.rs` - Gestión de código separado
- ✅ `src/storage/project.rs` - Gestión de proyectos y metadatos
- ✅ `src/storage/migration.rs` - Migración de proyectos antiguos

**Modificaciones realizadas:**
- ✅ `src/core/node_graph.rs` - Agregado campo `code_path: Option<String>`, implementado `Clone`
- ✅ `src/storage/workspace.rs` (movido desde `src/workspace.rs`) - Actualizado para usar nuevo sistema
- ✅ `src/core/app.rs` - Actualizado para usar nuevo sistema de storage
- ✅ `src/main.rs` - Actualizados imports para usar `storage::`
- ✅ `Cargo.toml` - Agregada dependencia `chrono`

**Resultado:**
- ✅ Sistema funcional, compila correctamente
- ✅ Código de nodos separado en archivos individuales
- ✅ Compatibilidad con formato antiguo mantenida

**Esfuerzo real**: ~3 horas

---

### Fase 2: Reorganizar Estructura de `src/` ✅ COMPLETADO

#### ✅ **COMPLETADO:**

**Estructura creada:**
- ✅ Directorios `core/`, `compilation/`, `utils/` creados
- ✅ Archivos `mod.rs` creados para cada módulo

**Archivos movidos:**
- ✅ `app.rs` → `core/app.rs`
- ✅ `node_graph.rs` → `core/node_graph.rs`
- ✅ `terminal.rs` → `compilation/terminal.rs`
- ✅ `auto_linker.rs` → `compilation/auto_linker.rs`
- ✅ `compiler_detector.rs` → `compilation/compiler_detector.rs`
- ✅ `editor_history.rs` → `utils/editor_history.rs`

**Imports actualizados:**
- ✅ `src/main.rs` - Usa nuevos módulos
- ✅ `src/core/app.rs` - Imports actualizados
- ✅ Todos los archivos en `ui/` - Actualizados a `crate::core::`
- ✅ Todos los archivos en `storage/` - Actualizados a `crate::core::node_graph`
- ✅ Todos los archivos en `expressions/` - Actualizados
- ✅ `templates/mod.rs` - Actualizado
- ✅ Archivos de compilación - Actualizados internamente

**Verificación:**
- ✅ Compilación exitosa verificada con `cargo check`
- ✅ Sin errores de compilación
- ✅ Estructura modular y organizada

**Esfuerzo real**: ~2 horas

---

## ⏳ FASES PENDIENTES / EN PROGRESO

### Fase 3: Migración de Proyectos Existentes 🔄 PARCIAL

#### ✅ **COMPLETADO:**
- ✅ `needs_migration()` - Detecta si un proyecto usa formato antiguo
- ✅ `migrate_project()` - Migra automáticamente a formato nuevo
- ✅ `create_backup()` - Crea backup del proyecto antes de migrar
- ✅ `copy_directory()` - Helper para backup
- ✅ Funciones de migración en `src/storage/migration.rs`

#### ⏳ **PENDIENTE:**
- ⏳ Validación post-migración
- ⏳ Testing con proyectos reales:
  - ⏳ Proyecto con código embebido
  - ⏳ Proyecto con muchos nodos
  - ⏳ Validar que backup se crea correctamente

**Estado**: 🔄 **PARCIAL** - Integración y UI completadas, falta testing

**Esfuerzo restante**: 30 min - 1 hora (testing manual)

---

### Fase 4: Preparar para GitHub ⏳ PENDIENTE

#### ✅ **COMPLETADO:**
- ✅ `LICENSE` - Ya existe el archivo LICENSE

#### ⏳ **PENDIENTE:**

**`.gitignore`:**
- ⏳ Verificar y actualizar `.gitignore`:
  - ⏳ Excluir `nodes/` (código de nodos - puede incluirse o excluirse según preferencia)
  - ⏳ Excluir `.ultra-omega/` (configuración local)
  - ⏳ Excluir `target/`, `Cargo.lock` (build artifacts)
  - ⏳ Excluir `*.exe`, `*.obj`, `*.o` (binarios compilados)

**CI/CD:**
- ⏳ Crear `.github/workflows/ci.yml` básico:
  - ⏳ Build y test en Windows/Linux
  - ⏳ Verificación de compilación
  - ⏳ Linting básico (opcional)

**Documentación:**
- ⏳ Actualizar `README.md` con:
  - ⏳ Nueva estructura de proyectos
  - ⏳ Instrucciones de uso
  - ⏳ Información sobre formato de código separado
  - ⏳ Ejemplos de uso
  - ⏳ Información sobre migración de proyectos antiguos
- ⏳ Crear `CHANGELOG.md` con versión 0.2.0 (nuevo formato)
- ⏳ Documentar estructura de directorios

**Opcional:**
- ⏳ Crear `.github/ISSUE_TEMPLATE/` (opcional pero recomendado)
- ⏳ Crear `.github/PULL_REQUEST_TEMPLATE.md` (opcional)

**Estado**: ⏳ **PENDIENTE**

**Esfuerzo estimado**: 1-2 horas

---

## 📋 Checklist de Tareas

### ✅ Fase 1: Storage Separado - COMPLETADO

- [x] Crear `src/storage/mod.rs` ✅
- [x] Crear `src/storage/node_storage.rs` con funciones:
  - [x] `get_node_code_path()` - Obtiene ruta del código
  - [x] `save_node_code()` - Guarda código en archivo separado
  - [x] `load_node_code()` - Carga código desde archivo
  - [x] `ensure_nodes_directory()` - Asegura directorio nodes/ existe
  - [x] `code_file_exists()` - Verifica si archivo existe
- [x] Crear `src/storage/project.rs` con:
  - [x] `ProjectMetadata` struct ✅
  - [x] `ProjectConfig` struct ✅
  - [x] `save()` - Guarda configuración del proyecto ✅
  - [x] `load()` - Carga configuración del proyecto ✅
- [x] Modificar `Node` para tener `code_path: Option<String>` ✅
- [x] Modificar `workspace.rs` para usar nuevo sistema ✅
- [x] Actualizar `app.rs` para cargar código desde archivos ✅
- [x] Agregar `Clone` a `NodeGraph` para soporte de serialización ✅

---

### ✅ Fase 2: Reorganización - COMPLETADO

- [x] Crear directorios `core/`, `compilation/`, `utils/` ✅
- [x] Crear archivos `mod.rs` para cada módulo nuevo ✅
- [x] Mover `app.rs` → `core/app.rs` ✅
- [x] Mover `node_graph.rs` → `core/node_graph.rs` ✅
- [x] Mover `terminal.rs` → `compilation/terminal.rs` ✅
- [x] Mover `auto_linker.rs` → `compilation/auto_linker.rs` ✅
- [x] Mover `compiler_detector.rs` → `compilation/compiler_detector.rs` ✅
- [x] Mover `editor_history.rs` → `utils/editor_history.rs` ✅
- [x] Actualizar todos los imports en todos los archivos ✅
- [x] Verificar compilación exitosa ✅

#### ⏳ Pendiente (Testing Manual):
- [ ] Probar funcionalidad básica (abrir, crear, guardar proyecto)

---

### 🔄 Fase 3: Migración - PARCIAL

#### ✅ Completado:
- [x] Crear `src/storage/migration.rs` ✅
- [x] Implementar `needs_migration()` - Detecta formato antiguo ✅
- [x] Implementar `migrate_project()` - Migra a formato nuevo ✅
- [x] Implementar `create_backup()` - Crea backup antes de migrar ✅
- [x] Implementar `copy_directory()` - Helper para backup ✅

#### ✅ Completado Recientemente:
- [x] Integrar migración en `app.rs` al cargar proyecto ✅
  - [x] Llamar `needs_migration()` al abrir proyecto ✅
  - [x] Mostrar diálogo al usuario preguntando si migrar ✅
  - [x] Ejecutar `migrate_project()` si usuario acepta ✅
  - [x] Mostrar resultado de migración ✅
- [x] Agregar UI para mostrar progreso de migración ✅
- [x] Crear backup automático antes de migrar ✅

#### ⏳ Pendiente:
- [ ] Testing con proyectos existentes:
  - [ ] Proyecto con código embebido
  - [ ] Proyecto con muchos nodos
  - [ ] Validar que backup se crea correctamente
- [ ] Validación post-migración

---

### 🆕 Fase 5: Inspiración Houdini - Sistema Avanzado de Nodos 🆕

#### 🎨 Ideas de Houdini para implementar:

**Sistema de Subgrafos/Subnetworks:**
- [ ] **Subnetwork Nodes**: Nodos que contienen un grafo completo dentro
  - Abrir subnetwork en nueva pestaña/ventana
  - Exportar subnetwork como asset reutilizable
  - Parámetros expuestos al nivel padre
  - Jerarquía visual clara (breadcrumbs)
  
- [ ] **HDAs (Houdini Digital Assets)**: Assets exportables
  - Exportar grupo de nodos como asset
  - Importar assets en otros proyectos
  - Parámetros configurables expuestos
  - Documentación integrada en el asset

**Sistema de Parámetros Avanzado:**
- [ ] **Parameter Editor**: Panel dedicado para editar parámetros
  - Parámetros dinámicos según tipo de nodo
  - Expresiones en parámetros (como HScript/Python)
  - Referencias a otros nodos (channels)
  - Validación de tipos en tiempo real
  
- [ ] **Channel References**: Sistema mejorado de referencias
  - `ch("../node_name/parameter")` - Referencias relativas
  - `ch("/absolute/path/to/node/param")` - Referencias absolutas
  - Autocompletado de rutas
  - Validación de referencias

**Vistas y Workflows:**
- [ ] **Network View Tabs**: Múltiples pestañas de grafos
  - Abrir subnetworks en nuevas pestañas
  - Navegación rápida entre niveles
  - Breadcrumbs para jerarquía
  
- [ ] **Minimap/Navigator**: Vista general del grafo
  - Mini mapa en esquina
  - Navegación rápida
  - Indicador de vista actual

- [ ] **Copy/Paste Mejorado**:
  - Copiar grupo de nodos con todas sus conexiones
  - Pegar preservando IDs únicos o generando nuevos
  - Copiar desde un subnetwork al nivel padre

**Hot Reload y Live Editing:**
- [ ] **File Watcher**: Detectar cambios en archivos externos
  - Recargar código automáticamente cuando cambia en editor externo
  - Notificar cambios no guardados
  - Merge inteligente de cambios

---

### 🆕 Fase 6: Inspiración VS Code - Organización y Exploración 🆕

#### 📁 Ideas de VS Code para implementar:

**Explorador de Archivos Avanzado:**
- [ ] **File Explorer Mejorado**: Sidebar con estructura completa
  - Mostrar estructura completa de `nodes/`
  - Agrupar por tipo de archivo (Rust, ASM, C++, etc.)
  - Iconos diferenciados por lenguaje
  - Indicador de nodos modificados/no guardados
  
- [ ] **Quick File Search**: Búsqueda rápida de archivos
  - Atajo `Ctrl+P` para buscar archivos por nombre
  - Filtrar por tipo de archivo
  - Abrir directamente desde búsqueda

- [ ] **Outline View**: Vista de estructura del código
  - Mostrar funciones/clases en archivos
  - Navegación rápida dentro del archivo
  - Resaltar definiciones

**Editor Avanzado:**
- [ ] **Multi-Editor Tabs**: Abrir múltiples archivos simultáneamente
  - Tabs para cada archivo abierto
  - Split view (horizontal/vertical)
  - Grupos de editores
  
- [ ] **Preview Mode**: Vista previa sin abrir editor completo
  - Click derecho → "Preview" para vista rápida
  - Auto-cierre al abrir otro preview
  - Hover preview con información

**Workspace y Organización:**
- [ ] **Workspace Files**: Archivos de configuración de workspace
  - Guardar configuración de ventanas/tabs abiertos
  - Restaurar estado al abrir proyecto
  - Multi-root workspaces
  
- [ ] **Folder Organization**: Organización visual mejorada
  - Crear carpetas virtuales (como VS Code)
  - Agrupar nodos por categoría/carpeta
  - Folders en el grafo visual

**Búsqueda y Navegación:**
- [ ] **Global Search**: Buscar en todos los archivos
  - `Ctrl+Shift+F` para búsqueda global
  - Buscar texto, nombres de nodos, etc.
  - Reemplazar en múltiples archivos
  
- [ ] **Go to Definition**: Navegación inteligente
  - `F12` para ir a definición
  - `Alt+F12` para peek definition
  - Navegar entre referencias

---

### ⏳ Fase 4: GitHub Ready - PENDIENTE

#### ✅ Completado:
- [x] `LICENSE` - Ya existe el archivo LICENSE ✅

#### ⏳ Pendiente:

**`.gitignore`:**
- [ ] Verificar y actualizar `.gitignore`:
  - [ ] Excluir `nodes/` (código de nodos - puede incluirse o excluirse según preferencia)
  - [ ] Excluir `.ultra-omega/` (configuración local)
  - [ ] Excluir `target/`, `Cargo.lock` (build artifacts)
  - [ ] Excluir `*.exe`, `*.obj`, `*.o` (binarios compilados)

**CI/CD:**
- [ ] Crear `.github/workflows/ci.yml` básico:
  - [ ] Build en Windows
  - [ ] Build en Linux (opcional)
  - [ ] Verificación de compilación
  - [ ] Linting básico (opcional)

**Documentación:**
- [ ] Actualizar `README.md` con:
  - [ ] Descripción del nuevo formato de proyectos
  - [ ] Estructura de directorios explicada
  - [ ] Instrucciones de instalación
  - [ ] Ejemplos de uso
  - [ ] Información sobre migración de proyectos antiguos
- [ ] Crear `CHANGELOG.md`:
  - [ ] Versión 0.2.0: Nuevo formato de código separado
  - [ ] Notas de migración
  - [ ] Breaking changes documentados
- [ ] Documentar estructura de directorios

**Opcional:**
- [ ] Crear `.github/ISSUE_TEMPLATE/` (opcional pero recomendado)
- [ ] Crear `.github/PULL_REQUEST_TEMPLATE.md` (opcional)

---

## 🆕 Nuevas Ideas Detalladas

### 🎯 Sistema de Subnetworks (Inspiración Houdini)

**Concepto**: Nodos especiales que contienen un grafo completo dentro.

**Implementación propuesta:**
```rust
// src/core/node_graph.rs
pub struct SubnetworkNode {
    pub id: NodeId,
    pub title: String,
    pub inner_graph: NodeGraph, // Grafo interno
    pub exposed_inputs: Vec<ExposedPin>, // Pines expuestos al nivel padre
    pub exposed_outputs: Vec<ExposedPin>,
}

// Navegación entre niveles
pub struct NetworkLevel {
    pub graph: NodeGraph,
    pub parent_node_id: Option<NodeId>, // None = nivel raíz
    pub breadcrumbs: Vec<String>, // Para mostrar ruta
}
```

**UI Propuesta:**
- Botón en nodo subnetwork para "entrar" al nivel interno
- Nueva pestaña/ventana para el grafo interno
- Breadcrumbs en la parte superior: `Root > Subnetwork1 > Subnetwork2`
- Botón "Subir" para volver al nivel padre

**Beneficios:**
- Organización de grafos complejos
- Reutilización de lógica encapsulada
- Mejor navegación en proyectos grandes

---

### 📁 File Explorer Mejorado (Inspiración VS Code)

**Estructura propuesta:**
```
Explorador
├── 📁 nodes/
│   ├── 🔴 node_000001.rs (Rust)
│   ├── 🟠 node_000002.asm (ASM)
│   ├── 🔵 node_000003.cpp (C++)
│   └── ⚡ node_000004.zig (Zig)
├── 📄 node_map.json
└── 📁 .ultra-omega/
    └── 📄 project.json
```

**Features:**
- Click para abrir en editor
- Click derecho para menú contextual (Preview, Delete, Rename)
- Indicadores visuales:
  - 🔴 Modificado (no guardado)
  - ✓ Guardado
  - 📁 Carpeta expandida/colapsada

**Quick Search (`Ctrl+P`):**
- Buscar archivos por nombre
- Filtrar por extensión: `@rs` solo Rust, `@asm` solo ASM
- Abrir directamente desde resultados

---

### ⚡ Hot Reload / File Watcher

**Implementación:**
```rust
// src/storage/file_watcher.rs
pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    callbacks: HashMap<PathBuf, Vec<Box<dyn Fn()>>>,
}

impl FileWatcher {
    pub fn watch_node_code(&mut self, path: PathBuf, callback: Box<dyn Fn()>) {
        // Registrar callback para recargar código cuando cambie
    }
}
```

**Comportamiento:**
- Detectar cambios en archivos de `nodes/`
- Notificar al usuario: "Archivo modificado externamente, ¿Recargar?"
- Auto-recargar opcional (configurable)
- Merge inteligente si hay cambios no guardados

---

### 📝 Parameter Editor (Inspiración Houdini)

**Panel dedicado para parámetros de nodos:**
```
Parameter Editor
├── Nodo: Kernel Entry (ASM)
├── ────────────────────────
├── 📌 Entrada
│   └── source: ../Bootloader/code
├── ⚙️ Configuración
│   ├── stack_size: 4096
│   └── heap_size: 8192
└── 🔗 Referencias
    └── kernel_path: ch("../Kernel/code")
```

**Features:**
- Edición directa de parámetros
- Autocompletado de referencias `ch()`
- Validación de tipos en tiempo real
- Referencias relativas/absolutas
- Preview de valores resueltos

---

### 🗂️ Multi-Editor Tabs (Inspiración VS Code)

**Sistema de tabs:**
- Cada archivo abierto tiene un tab
- Click en tab para cambiar de archivo
- `Ctrl+Tab` para cambiar entre tabs
- Split view: dividir editor horizontal/verticalmente
- Cerrar tab: `Ctrl+W` o click en X

**Grupos de editores:**
- Editor principal + editor secundario (split)
- Útil para comparar archivos o copiar código

---

### 🔍 Global Search (`Ctrl+Shift+F`)

**Buscar en todo el proyecto:**
```
Buscar: "printf"
├── node_000001.rs (2 matches)
│   └── línea 45, 67
├── node_000002.c (1 match)
│   └── línea 12
└── node_000003.cpp (3 matches)
    └── líneas 23, 44, 89
```

**Features:**
- Búsqueda de texto en todos los archivos
- Buscar por nombre de nodo
- Reemplazar en múltiples archivos
- Filtros: por tipo de archivo, por carpeta

---

## 🔄 Formato del node_map.json (Nuevo)

### Antes (Código Embebido):
```json
{
  "nodes": [
    {
      "id": 1,
      "title": "Mi Nodo",
      "code": "fn main() { println!(\"Hola\"); }",
      ...
    }
  ]
}
```

### Después (Código Separado):
```json
{
  "nodes": [
    {
      "id": 1,
      "title": "Mi Nodo",
      "code_path": "nodes/node_000001.rs",
      "language": "Rust",
      ...
    }
  ]
}
```

---

## 🛠️ Implementación Detallada

### 1. Node Storage

```rust
// src/storage/node_storage.rs
pub struct NodeStorage {
    workspace: Workspace,
}

impl NodeStorage {
    pub fn get_node_code_path(&self, node_id: NodeId) -> PathBuf {
        self.workspace.root_path
            .as_ref()
            .unwrap()
            .join("nodes")
            .join(format!("node_{:06}.rs", node_id.0))
    }
    
    pub fn save_node_code(&self, node_id: NodeId, code: &str, lang: NodeLanguage) -> Result<()> {
        let path = self.get_node_code_path_for_lang(node_id, lang);
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, code)?;
        Ok(())
    }
    
    pub fn load_node_code(&self, code_path: &str) -> Result<String> {
        let full_path = self.workspace.root_path
            .as_ref()
            .unwrap()
            .join(code_path);
        Ok(std::fs::read_to_string(full_path)?)
    }
    
    fn get_node_code_path_for_lang(&self, node_id: NodeId, lang: NodeLanguage) -> PathBuf {
        let ext = match lang {
            NodeLanguage::Rust => "rs",
            NodeLanguage::Asm => "asm",
            NodeLanguage::C => "c",
            NodeLanguage::Cpp => "cpp",
            NodeLanguage::Zig => "zig",
            _ => "txt",
        };
        self.workspace.root_path
            .as_ref()
            .unwrap()
            .join("nodes")
            .join(format!("node_{:06}.{}", node_id.0, ext))
    }
}
```

### 2. Modificación de Node

```rust
// src/core/node_graph.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub title: String,
    pub position: Pos2,
    pub color: Color32,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    
    // NUEVO: Path al código (si está separado)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_path: Option<String>,  // Ejemplo: "nodes/node_000001.rs"
    
    // DEPRECATED: Mantener por compatibilidad durante migración
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub code: String,  // Se usará solo si code_path es None
    
    pub language: NodeLanguage,
}
```

### 3. Workspace Actualizado

```rust
// src/storage/workspace.rs
impl Workspace {
    pub fn save_graph(&self, graph: &mut NodeGraph, storage: &NodeStorage) -> Result<()> {
        // 1. Guardar código de cada nodo en archivos separados
        for node in graph.nodes_mut() {
            if let Some(path) = &node.code_path {
                // Ya está separado, solo guardar si cambió
                storage.save_node_code(node.id, &node.code, node.language)?;
            } else if !node.code.is_empty() {
                // Formato antiguo, migrar
                let code_path = storage.save_node_code(node.id, &node.code, node.language)?;
                node.code_path = Some(code_path);
            }
        }
        
        // 2. Guardar node_map.json (sin código embebido para nodos con code_path)
        let map_path = self.get_node_map_path()?;
        let mut graph_for_serialization = graph.clone();
        for node in graph_for_serialization.nodes_mut() {
            if node.code_path.is_some() {
                node.code = String::new(); // Limpiar código embebido
            }
        }
        let json = serde_json::to_string_pretty(&graph_for_serialization)?;
        std::fs::write(map_path, json)?;
        
        Ok(())
    }
    
    pub fn load_graph(&self, storage: &NodeStorage) -> Result<NodeGraph> {
        let map_path = self.get_node_map_path()?;
        if !map_path.exists() {
            return Ok(NodeGraph::default());
        }
        
        let json = std::fs::read_to_string(&map_path)?;
        let mut graph: NodeGraph = serde_json::from_str(&json)?;
        
        // Cargar código desde archivos separados
        for node in graph.nodes_mut() {
            if let Some(code_path) = &node.code_path {
                node.code = storage.load_node_code(code_path)?;
            }
            // Si no hay code_path pero hay code, está en formato antiguo
            // Se migrará en el próximo guardado
        }
        
        Ok(graph)
    }
}
```

---

## ✅ Beneficios Esperados

### Beneficios Actuales (Ya Implementados):
1. **Archivos editables externamente**: Podrás editar código con tu editor favorito
2. **Versionado Git-friendly**: Cambios de código se ven claramente en Git
3. **Node map más rápido**: JSON mucho más pequeño
4. **Backup incremental**: Solo respaldar archivos modificados
5. **Reutilización**: Compartir código entre proyectos
6. **Mejor estructura**: Código más organizado y mantenible
7. **Migración automática**: Proyectos antiguos se actualizan automáticamente

### Beneficios Futuros (Houdini + VS Code):

**Organización y Navegación:**
- **Subnetworks**: Encapsular lógica compleja en nodos jerárquicos
- **Assets reutilizables (HDAs)**: Compartir subgrafos entre proyectos
- **File Explorer avanzado**: Ver y organizar todos los archivos del proyecto
- **Quick Search**: Encontrar archivos instantáneamente (`Ctrl+P`)
- **Minimap**: Navegación rápida en grafos grandes

**Productividad:**
- **Multi-Editor Tabs**: Editar múltiples archivos simultáneamente
- **Hot Reload**: Sincronización automática con editores externos
- **Parameter Editor**: Editar parámetros de nodos de forma profesional
- **Global Search**: Buscar y reemplazar en todo el proyecto (`Ctrl+Shift+F`)

**Workflow Profesional:**
- **Breadcrumbs**: Navegación clara entre niveles de subnetwork
- **Workspace State**: Restaurar configuración al reabrir proyecto
- **Preview Mode**: Vista rápida de archivos sin abrir editor completo
- **Go to Definition**: Navegación inteligente entre referencias

---

## 🔍 Testing

### Casos de Prueba

1. **Migración automática**
   - Crear proyecto con formato antiguo
   - Abrir en nueva versión
   - Verificar que migra correctamente
   - Verificar que el código se carga bien

2. **Nuevos proyectos**
   - Crear proyecto nuevo
   - Verificar que usa formato nuevo
   - Verificar que se crean archivos en `nodes/`

3. **Compatibilidad hacia atrás**
   - Abrir proyecto antiguo
   - No debe fallar
   - Debe migrar automáticamente

4. **Guardado/Carga**
   - Modificar código de nodo
   - Guardar proyecto
   - Cerrar y reabrir
   - Verificar que el código se mantiene

---

## 📝 Notas Importantes

- **Compatibilidad**: Mantener soporte para formato antiguo durante migración
- **Backwards compatibility**: No romper proyectos existentes
- **Performance**: Cargar código bajo demanda (lazy loading) para proyectos grandes
- **Seguridad**: Validar rutas de archivos para prevenir path traversal

---

## 📊 Estado General del Proyecto

**Última actualización**: 2025-01-07 (Fase 2 completada, nuevas fases agregadas)

### Progreso por Fases:

| Fase | Estado | Progreso | Prioridad |
|------|--------|----------|-----------|
| **Fase 1: Storage Separado** | ✅ COMPLETADO | 100% | Alta |
| **Fase 2: Reorganización** | ✅ COMPLETADO | 100% | Media |
| **Fase 3: Migración** | 🔄 PARCIAL | 85% | Media |
| **Fase 4: GitHub Ready** | ⏳ PENDIENTE | 10% | Baja |
| **Fase 5: Inspiración Houdini** | 🆕 PLANIFICADO | 0% | Media-Alta |
| **Fase 6: Inspiración VS Code** | 🆕 PLANIFICADO | 0% | Media |

### Resumen:
- ✅ **Sistema de código separado funcionando** - Los nodos ahora guardan código en archivos separados
- ✅ **Compatibilidad con formato antiguo** - Proyectos antiguos se pueden cargar y migrar automáticamente
- ✅ **Estructura reorganizada** - Código organizado en módulos lógicos (`core/`, `compilation/`, `utils/`, `storage/`)
- ✅ **Migración automática implementada** - Con UI completa y diálogo de usuario
- ✅ **Tema visual mejorado** - Fondo negro con líneas blancas para mejor visibilidad
- 🆕 **Nuevas fases planificadas** - Inspiración Houdini (subnetworks, HDA) y VS Code (explorador, búsqueda, tabs)
- ⏳ **Testing pendiente** - Validar migración con proyectos reales
- ⏳ **Preparación para GitHub pendiente** - Documentación y CI/CD

### Próximos Pasos Recomendados:
1. **Inmediato**: Completar testing de migración automática (Fase 3)
2. **Corto plazo**: Implementar File Explorer mejorado y Quick Search (Fase 6)
3. **Medio plazo**: Sistema de Subnetworks/HDA básico (Fase 5)
4. **Largo plazo**: Preparar para GitHub con documentación completa (Fase 4)

### 🎨 Roadmap Inspirado en Houdini + VS Code:

**Prioridad Alta (Inspiración Houdini):**
1. Sistema de Subnetworks - Nodos que contienen grafos
2. Parameter Editor avanzado con expresiones
3. File Watcher para hot reload de archivos externos

**Prioridad Media (Inspiración VS Code):**
1. File Explorer completo con estructura de `nodes/`
2. Quick File Search (`Ctrl+P`)
3. Multi-Editor Tabs para múltiples archivos

**Prioridad Media (Ambos):**
1. Copy/Paste mejorado de grupos de nodos
2. Minimap/Navigator para grafos grandes
3. Workspace state persistence

**Organización en Tiempo Real (VS Code Style):**
- File watcher para detectar cambios externos automáticamente
- Sincronización bidireccional entre editor interno y externo
- Indicadores visuales de archivos modificados
- Auto-refresh del explorador cuando se crean/eliminan archivos

---

## 🎉 Logros Completados

✅ Sistema de storage modular y extensible  
✅ Separación completa de código del mapa de nodos  
✅ Soporte para múltiples lenguajes (extensión automática)  
✅ Migración automática implementada con UI completa  
✅ Compatibilidad hacia atrás mantenida  
✅ Compilación exitosa verificada  
✅ Estructura de código reorganizada y modularizada  
✅ Separación clara de responsabilidades (core, compilation, storage, utils, ui)  
✅ Mejor mantenibilidad y extensibilidad del código  
✅ Tema visual negro profesional con líneas blancas (mejora UX)

---

## 🚀 Visiones Futuras (Houdini + VS Code)

### 🎨 Houdini-Style Features:

**Workflow Profesional:**
- Sistema de subnetworks para encapsular lógica compleja
- Assets exportables y reutilizables (HDAs)
- Parámetros dinámicos con expresiones poderosas
- Navegación jerárquica fluida entre niveles de red

**Live Editing:**
- Hot reload automático al editar archivos externamente
- File watchers para sincronización en tiempo real
- Merge inteligente de cambios concurrentes

### 📁 VS Code-Style Features:

**Organización Visual:**
- Explorador de archivos completo con iconos y agrupación
- Búsqueda rápida de archivos y contenido
- Tabs múltiples para editar varios archivos
- Outline view para navegación dentro de archivos

**Productividad:**
- Atajos de teclado familiares (`Ctrl+P`, `Ctrl+Shift+F`)
- Workspace state persistence
- Preview mode para vistas rápidas
- Go to definition y referencias
