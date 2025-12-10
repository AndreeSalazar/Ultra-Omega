# 📁 Ideas: Sistema de Carpetas Virtuales para Nodos

## 🎯 Objetivo Principal

Crear un sistema de **carpetas virtuales** que permita organizar nodos visualmente en el explorador, con **sincronización bidireccional en tiempo real** entre:
- **Vista del explorador** (sidebar)
- **Vista del grafo** (viewport)
- **Estructura de archivos físicos** (`nodes/`)

---

## 💡 Idea 1: Carpetas Virtuales en el Explorador

### Concepto
Permitir crear **carpetas virtuales** en el sidebar que agrupen nodos relacionados, similar a VS Code pero adaptado a nodos.

### Estructura Visual Propuesta:
```
📂 Explorador
├── 📁 Core (carpeta virtual)
│   ├── 🦀 node_000001.rs (Kernel Entry)
│   ├── 🦀 node_000002.rs (Memory Manager)
│   └── 🦀 node_000003.rs (Task Scheduler)
├── 📁 Graphics (carpeta virtual)
│   ├── 🔵 node_000004.cpp (DirectX12 Init)
│   ├── 🔵 node_000005.cpp (Render Loop)
│   └── 🔵 node_000006.cpp (Shader Compiler)
├── 📁 Utils (carpeta virtual)
│   ├── 🦀 node_000007.rs (String Utils)
│   └── 🦀 node_000008.rs (Math Utils)
└── 📄 node_000009.rs (sin carpeta)
```

### Características:
- **Crear carpeta**: Click derecho → "Nueva carpeta"
- **Mover nodos**: Drag & drop de nodos a carpetas
- **Expandir/Colapsar**: Click en carpeta para mostrar/ocultar contenido
- **Renombrar carpeta**: Click derecho → "Renombrar"
- **Eliminar carpeta**: Click derecho → "Eliminar" (no elimina nodos, solo organización)

---

## 💡 Idea 2: Sincronización con Estructura de Archivos

### Concepto
Las carpetas virtuales se reflejan en la estructura física de archivos, creando subdirectorios en `nodes/`.

### Mapeo Propuesto:
```
Carpeta Virtual "Core" → nodes/Core/
Carpeta Virtual "Graphics" → nodes/Graphics/
Carpeta Virtual "Utils" → nodes/Utils/
```

### Estructura Física Resultante:
```
proyecto/
├── node_map.json
└── nodes/
    ├── Core/
    │   ├── node_000001.rs
    │   ├── node_000002.rs
    │   └── node_000003.rs
    ├── Graphics/
    │   ├── node_000004.cpp
    │   ├── node_000005.cpp
    │   └── node_000006.cpp
    ├── Utils/
    │   ├── node_000007.rs
    │   └── node_000008.rs
    └── node_000009.rs (sin carpeta)
```

### Sincronización Bidireccional:
1. **Explorador → Archivos**: Al mover nodo a carpeta, mover archivo físicamente
2. **Archivos → Explorador**: Si se crea carpeta en `nodes/`, aparece en explorador
3. **Tiempo Real**: Cambios se reflejan inmediatamente

---

## 💡 Idea 3: Carpetas en el Viewport (Grafo Visual)

### Concepto
Mostrar **regiones visuales** en el viewport que representen carpetas, agrupando nodos visualmente.

### Visualización Propuesta:
```
┌─────────────────────────────────────┐
│  📁 Core                            │
│  ┌─────────┐  ┌─────────┐          │
│  │ Node 1  │  │ Node 2  │          │
│  └─────────┘  └─────────┘          │
│  ┌─────────┐                        │
│  │ Node 3  │                        │
│  └─────────┘                        │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│  📁 Graphics                         │
│  ┌─────────┐  ┌─────────┐          │
│  │ Node 4  │  │ Node 5  │          │
│  └─────────┘  └─────────┘          │
└─────────────────────────────────────┘
```

### Características:
- **Región visual**: Fondo semitransparente con borde
- **Título de carpeta**: Visible en la parte superior
- **Agrupación automática**: Nodos dentro de la región se agrupan visualmente
- **Colapsar/Expandir**: Click en título para ocultar/mostrar nodos
- **Mover carpeta**: Mover toda la región (y sus nodos) juntos

---

## 💡 Idea 4: Organización Automática por Categorías ⭐ MEJORADA

### Concepto
Permitir que el sistema **agrupe automáticamente** nodos y **sugiera crear Nodos Carpeta o Subnetworks** basándose en:
- **Lenguaje** (Rust, C++, ASM, etc.)
- **Tipo** (Kernel, Graphics, Utils, etc.)
- **Conexiones** (nodos conectados juntos) ⭐ **Clave para Nodo Carpeta/Subnetwork**
- **Tags/Metadatos** (etiquetas personalizadas)

### Opciones de Agrupación:

1. **Por Lenguaje**:
   ```
   📁 Rust
   📁 C++
   📁 ASM
   📁 Zig
   ```
   **Sugerencia**: "¿Crear Nodo Carpeta 'Rust Utils' para estos 5 nodos Rust?"

2. **Por Tipo (basado en título)**:
   ```
   📁 Kernel (nodos con "Kernel" en título)
   📁 Graphics (nodos con "Render", "Shader", etc.)
   📁 Utils (nodos con "Utils", "Helper", etc.)
   ```
   **Sugerencia**: "¿Crear Nodo Carpeta 'Graphics Core' para estos nodos relacionados?"

3. **Por Conexiones** ⭐ **LA MÁS IMPORTANTE**:
   - **Detectar grupos de nodos conectados**
   - **Sugerir automáticamente**:
     - **Nodo Carpeta** si los nodos forman una "librería" (funciones relacionadas)
     - **Subnetwork** si los nodos forman un "proceso" (flujo de trabajo)
   - **Criterios de detección**:
     - 3+ nodos conectados entre sí
     - Forman un subgrafo aislado o semi-aislado
     - Comparten patrones de conexión similares

   **Ejemplo Automático**:
   ```
   Sistema detecta: 4 nodos conectados (Init → Render → Shader → Texture)
   Sugerencia: "¿Crear Nodo Carpeta 'Graphics Pipeline' para estos nodos?"
   O: "¿Convertir en Subnetwork 'Graphics Pipeline'?"
   ```

4. **Por Tags Personalizados**:
   - Agregar tags a nodos (ej: `#core`, `#graphics`, `#experimental`)
   - Agrupar por tags
   - **Sugerencia**: "¿Crear Nodo Carpeta para nodos con tag '#graphics'?"

### 🎯 Integración con Nodo Carpeta y Subnetworks:

#### **Detección Inteligente de Grupos**:
```rust
// Algoritmo propuesto:
1. Analizar grafo de conexiones
2. Detectar subgrafos conectados (3+ nodos)
3. Analizar patrón de conexiones:
   - Si es "librería" (muchas funciones, pocas conexiones) → Sugerir Nodo Carpeta
   - Si es "proceso" (flujo lineal, muchas conexiones) → Sugerir Subnetwork
4. Mostrar sugerencia al usuario
5. Si acepta → Crear automáticamente
```

#### **Sugerencias Contextuales**:
- **"Detectados 5 nodos Rust conectados. ¿Crear Nodo Carpeta 'Rust Core'?"**
- **"Grupo de 4 nodos forma un flujo. ¿Convertir en Subnetwork?"**
- **"Nodos con tag '#graphics' detectados. ¿Agrupar en Nodo Carpeta?"**

#### **Conversión Automática**:
- **Carpeta Virtual → Nodo Carpeta**: Convertir carpeta virtual en nodo carpeta
- **Grupo Seleccionado → Nodo Carpeta**: Seleccionar nodos → "Crear Nodo Carpeta"
- **Grupo Seleccionado → Subnetwork**: Seleccionar nodos → "Crear Subnetwork"

### 🚀 Flujo de Trabajo Mejorado:

```
1. Usuario crea varios nodos y los conecta
   └─ Sistema detecta: "5 nodos conectados detectados"

2. Sistema sugiere:
   └─ "¿Crear Nodo Carpeta 'Graphics Core' para estos nodos?"
   └─ O: "¿Convertir en Subnetwork 'Render Pipeline'?"

3. Usuario acepta → Sistema crea automáticamente:
   ├─ Nodo Carpeta creado
   ├─ Nodos movidos dentro
   ├─ Conexiones preservadas
   └─ Listo para usar/heredar

4. Si es Nodo Carpeta:
   └─ Otros nodos pueden heredar: ch("Graphics Core")

5. Si es Subnetwork:
   └─ Pines expuestos automáticamente
```

### 💡 Ventajas de la Automatización:

1. **Facilita Creación**: No necesitas crear manualmente nodo carpeta/subnetwork
2. **Detección Inteligente**: Sistema identifica grupos lógicos automáticamente
3. **Ahorra Tiempo**: Un clic en lugar de crear manualmente
4. **Mejor Organización**: Agrupa nodos relacionados sin esfuerzo
5. **Aprendizaje**: Sistema aprende patrones de tu proyecto

---

## 💡 Idea 5: Vista de Árbol Expandible (Tree View)

### Concepto
Mostrar estructura jerárquica completa con árbol expandible, similar a VS Code.

### Interfaz Propuesta:
```
📂 ULTRA-OMEGA
  ├─ 📁 Core
  │   ├─ 🦀 Kernel Entry
  │   ├─ 🦀 Memory Manager
  │   └─ 🦀 Task Scheduler
  ├─ 📁 Graphics
  │   ├─ 🔵 DirectX12 Init
  │   ├─ 🔵 Render Loop
  │   └─ 📁 Shaders (subcarpeta)
  │       ├─ 🔵 Vertex Shader
  │       └─ 🔵 Fragment Shader
  ├─ 📁 Utils
  │   └─ 🦀 String Utils
  └─ 📄 Standalone Node
```

### Características:
- **Expandir/Colapsar**: Click en flecha o carpeta
- **Indentación visual**: Muestra jerarquía claramente
- **Subcarpetas**: Carpetas dentro de carpetas
- **Drag & Drop**: Mover entre niveles
- **Búsqueda**: Filtrar por nombre de carpeta o nodo

---

## 💡 Idea 6: Sincronización con node_map.json

### Concepto
Guardar la organización de carpetas en `node_map.json` para persistencia.

### Estructura JSON Propuesta:
```json
{
  "folders": [
    {
      "id": "folder_001",
      "name": "Core",
      "parent": null,
      "node_ids": [1, 2, 3],
      "position": {"x": 100, "y": 100},
      "collapsed": false
    },
    {
      "id": "folder_002",
      "name": "Graphics",
      "parent": null,
      "node_ids": [4, 5, 6],
      "position": {"x": 500, "y": 100},
      "collapsed": false
    }
  ],
  "nodes": [
    {
      "id": 1,
      "title": "Kernel Entry",
      "folder_id": "folder_001",
      "code_path": "nodes/Core/node_000001.rs",
      ...
    }
  ]
}
```

### Ventajas:
- **Persistencia**: La organización se guarda entre sesiones
- **Versionado**: Cambios de organización visibles en Git
- **Migración**: Fácil migrar organización entre proyectos

---

## 💡 Idea 7: Organización Inteligente con IA/Heurísticas

### Concepto
Sugerir automáticamente cómo organizar nodos basándose en:
- **Análisis de código**: Detectar funciones/clases y agrupar
- **Patrones de conexión**: Nodos que se conectan frecuentemente
- **Nombres similares**: Nodos con nombres relacionados
- **Lenguaje**: Agrupar por lenguaje automáticamente

### Flujo Propuesto:
1. Usuario crea varios nodos
2. Sistema analiza y sugiere: "¿Crear carpeta 'Graphics' para estos 3 nodos?"
3. Usuario acepta o rechaza
4. Organización se aplica automáticamente

---

## 💡 Idea 8: Vista Dual: Explorador + Grafo

### Concepto
Mostrar **dos vistas sincronizadas**:
1. **Vista de Árbol** (sidebar): Estructura jerárquica
2. **Vista de Grafo** (viewport): Visualización espacial

### Sincronización:
- **Selección**: Seleccionar nodo en árbol → se enfoca en grafo
- **Mover**: Mover nodo en grafo → se actualiza posición en árbol
- **Carpeta**: Crear carpeta en árbol → aparece región en grafo
- **Colapsar**: Colapsar carpeta en árbol → oculta nodos en grafo

---

## 💡 Idea 9: Filtros y Vistas Personalizadas

### Concepto
Permitir crear **vistas personalizadas** que muestren solo ciertos nodos/carpetas.

### Ejemplos de Vistas:
1. **Solo Rust**: Mostrar solo nodos Rust
2. **Solo Core**: Mostrar solo carpeta "Core"
3. **Sin Carpetas**: Vista plana de todos los nodos
4. **Por Conexiones**: Agrupar por nodos conectados
5. **Recientes**: Mostrar nodos modificados recientemente

---

## 💡 Idea 10: Integración con Subnetworks

### Concepto
Combinar carpetas virtuales con subnetworks existentes.

### Propuesta:
- **Carpeta → Subnetwork**: Convertir carpeta en subnetwork
- **Subnetwork → Carpeta**: Mostrar subnetwork como carpeta en explorador
- **Jerarquía**: Carpetas pueden contener subnetworks

### Estructura:
```
📁 Core
  ├─ 🦀 Kernel Entry
  ├─ 🦀 Memory Manager
  └─ 📁 Subnetwork: Task Management
      ├─ 🦀 Task Scheduler
      └─ 🦀 Task Queue
```

---

____________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________

## 💡 Idea 11: Nodo Carpeta - Contenedor de Trabajo y Herencia 🆕

### Concepto
Crear un **"Nodo Carpeta"** especial que funcione como:
1. **Contenedor de trabajo**: Espacio donde trabajar con múltiples nodos
2. **Unidad de herencia**: Otros nodos pueden heredar todo el contenido del nodo carpeta

### Visualización Propuesta:
```
┌─────────────────────────────────────┐
│  📁 Nodo Carpeta: "Graphics Core"  │
│  ┌───────────────────────────────┐  │
│  │  🦀 Init Graphics            │  │
│  │  🔵 Render Loop              │  │
│  │  🔵 Shader Compiler          │  │
│  │  🔵 Texture Manager          │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
         │
         │ (herencia)
         ▼
┌─────────────────────────────────────┐
│  🔵 Nodo B: "My Renderer"          │
│  (hereda todo de "Graphics Core")  │
│  + código adicional propio         │
└─────────────────────────────────────┘
```

### Características Principales:

#### 1. **Nodo Carpeta como Workspace**
- **Doble clic** en nodo carpeta → Abre vista interna con sus nodos
- **Trabajar dentro**: Crear, editar, conectar nodos dentro del nodo carpeta
- **Aislado**: Los nodos dentro no afectan el grafo principal directamente
- **Guardado**: El contenido se guarda como parte del nodo carpeta

#### 2. **Herencia Completa**
- **Nodo B hereda Nodo Carpeta A**: 
  - Acceso a **todos los nodos** dentro de la carpeta
  - Acceso a **todo el código** de esos nodos
  - Acceso a **todas las funciones/clases** definidas
- **Sintaxis de herencia**: `ch("Graphics Core")` → accede a todo el contenido
- **Combinación**: El nodo B puede usar código de la carpeta + su propio código

#### 3. **Dos Modos de Uso**

**Modo 1: Solo Organización (Guardar)**
- Nodo carpeta como contenedor visual
- Nodos dentro se guardan pero no se exponen
- Útil para organizar código relacionado

**Modo 2: Herencia Funcional (Convertir)**
- Nodo carpeta se convierte en "librería heredable"
- Otros nodos pueden heredar su contenido completo
- Similar a un módulo o namespace

### Flujo de Trabajo Propuesto:

```
1. Crear "Nodo Carpeta: Graphics Core"
   └─ Doble clic → Abre vista interna

2. Dentro del nodo carpeta:
   ├─ Crear nodo: "Init Graphics"
   ├─ Crear nodo: "Render Loop"
   ├─ Crear nodo: "Shader Compiler"
   └─ Conectar nodos entre sí

3. Opción A: Guardar como organización
   └─ Los nodos quedan encapsulados

4. Opción B: Convertir en heredable
   └─ Otros nodos pueden usar: ch("Graphics Core")

5. Crear "Nodo B: My Renderer"
   └─ Conectar a "Graphics Core" (herencia)
   └─ Accede a todo el código de la carpeta
   └─ Agrega su propio código adicional
```

### Estructura de Datos Propuesta:
```rust
// src/core/node_graph.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    // ... campos existentes
    
    // NUEVO: Nodo Carpeta
    pub is_folder_node: bool,
    pub folder_content: Option<NodeGraph>,  // Grafo interno (como subnetwork)
    pub is_inheritable: bool,  // Si otros nodos pueden heredarlo
    
    // Herencia desde nodo carpeta
    pub inherits_from_folder: Option<NodeId>,  // ID del nodo carpeta que hereda
}

// Funciones para herencia
impl NodeGraph {
    // Obtener todo el código de un nodo carpeta (para herencia)
    pub fn get_folder_node_code(&self, folder_node_id: NodeId) -> String {
        // Combina código de todos los nodos dentro de la carpeta
    }
    
    // Aplicar herencia: combinar código de carpeta + código propio
    pub fn apply_folder_inheritance(&self, node_id: NodeId) -> String {
        // Si el nodo hereda de una carpeta, combina códigos
    }
}
```

### Ventajas:
1. **Organización Modular**: Agrupar funcionalidad relacionada
2. **Reutilización**: Un nodo carpeta puede ser heredado por múltiples nodos
3. **Encapsulación**: Lógica compleja contenida en un solo lugar
4. **Herencia Potente**: Acceso a múltiples nodos/funciones de una vez
5. **Flexibilidad**: Usar como organización o como librería

### Ejemplo de Uso:

**Nodo Carpeta "Math Utils":**
```rust
// Contiene:
// - node_math_vector.rs (funciones de vectores)
// - node_math_matrix.rs (funciones de matrices)
// - node_math_quaternion.rs (funciones de quaterniones)
```

**Nodo "Physics Engine" hereda "Math Utils":**
```rust
// Accede automáticamente a:
// - ch("Math Utils") → todas las funciones de vectores, matrices, quaterniones
// + su propio código de física
```

### Integración con Sistema Existente:
- **Compatible con Subnetworks**: Un nodo carpeta puede contener subnetworks
- **Compatible con Carpetas Virtuales**: Nodos carpeta pueden estar en carpetas virtuales
- **Compatible con Herencia Actual**: Extiende el sistema de herencia existente

---

## 🎨 Priorización de Ideas

### Fase 1: Fundamentos (Alta Prioridad)
1. ✅ **Idea 1**: Carpetas virtuales en explorador
2. ✅ **Idea 2**: Sincronización con archivos físicos
3. ✅ **Idea 6**: Persistencia en node_map.json

### Fase 2: Visualización (Media Prioridad)
4. ✅ **Idea 3**: Carpetas en viewport
5. ✅ **Idea 5**: Vista de árbol expandible
6. ✅ **Idea 8**: Vista dual sincronizada

### Fase 3: Nodo Carpeta - Herencia Avanzada (Alta Prioridad) 🆕
11. ⏳ **Idea 11**: Nodo Carpeta como contenedor y unidad de herencia

### Fase 4: Automatización Inteligente (Alta Prioridad) ⭐ MEJORADA
4. ⏳ **Idea 4**: Organización automática + **Sugerencias para Nodo Carpeta/Subnetwork**
   - Detección automática de grupos conectados
   - Sugerencias contextuales (Nodo Carpeta vs Subnetwork)
   - Conversión automática de grupos seleccionados
7. ⏳ **Idea 7**: Sugerencias inteligentes (extendida con Idea 4)
10. ⏳ **Idea 10**: Integración con subnetworks

### Fase 5: Visualización y Filtros (Media Prioridad)
9. ⏳ **Idea 9**: Filtros y vistas personalizadas

---

## 🛠️ Implementación Técnica Propuesta

### Estructura de Datos:
```rust
// src/core/node_graph.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub parent_id: Option<FolderId>,
    pub node_ids: Vec<NodeId>,
    pub position: Pos2,  // Posición en viewport
    pub size: Vec2,      // Tamaño de la región
    pub collapsed: bool,
    pub color: Color32,  // Color de la región
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FolderId(pub u32);

// Agregar a Node
pub struct Node {
    // ... campos existentes
    pub folder_id: Option<FolderId>,  // Carpeta a la que pertenece
}
```

### Funciones Clave:
```rust
impl NodeGraph {
    // Crear carpeta
    pub fn create_folder(&mut self, name: String, parent: Option<FolderId>) -> FolderId;
    
    // Mover nodo a carpeta
    pub fn move_node_to_folder(&mut self, node_id: NodeId, folder_id: Option<FolderId>);
    
    // Obtener nodos de carpeta
    pub fn get_folder_nodes(&self, folder_id: FolderId) -> Vec<NodeId>;
    
    // Eliminar carpeta (no elimina nodos)
    pub fn remove_folder(&mut self, folder_id: FolderId);
}
```

### Sincronización con Archivos:
```rust
// src/storage/node_storage.rs
impl NodeStorage {
    // Mover archivo a subdirectorio
    pub fn move_node_to_folder(&self, node_id: NodeId, folder_name: &str) -> Result<()>;
    
    // Obtener ruta con carpeta
    pub fn get_node_code_path_with_folder(&self, node_id: NodeId, folder: Option<&str>) -> PathBuf;
}
```

---

## 📋 Checklist de Implementación

### Fase 1: Fundamentos
- [ ] Agregar struct `Folder` y `FolderId` a `node_graph.rs`
- [ ] Agregar campo `folder_id` a `Node`
- [ ] Implementar funciones básicas (crear, mover, eliminar carpetas)
- [ ] Guardar/cargar carpetas en `node_map.json`
- [ ] UI básica en sidebar para crear/mostrar carpetas

### Fase 2: Sincronización
- [ ] Sincronizar carpetas con estructura de archivos
- [ ] Mover archivos físicamente al cambiar carpeta
- [ ] Detectar carpetas creadas externamente
- [ ] Actualizar `code_path` en nodos al mover

### Fase 3: Visualización
- [ ] Mostrar carpetas en viewport como regiones
- [ ] Agrupar nodos visualmente dentro de regiones
- [ ] Implementar colapsar/expandir en viewport
- [ ] Sincronizar selección entre sidebar y viewport

### Fase 3: Nodo Carpeta
- [ ] Agregar campo `is_folder_node` y `folder_content` a `Node`
- [ ] Agregar campo `is_inheritable` para controlar herencia
- [ ] Agregar campo `inherits_from_folder` para herencia
- [ ] Implementar vista interna del nodo carpeta (similar a subnetwork)
- [ ] Implementar funciones de herencia (`get_folder_node_code`, `apply_folder_inheritance`)
- [ ] UI para crear nodo carpeta
- [ ] UI para convertir entre modo organización y modo heredable
- [ ] UI para conectar nodo a nodo carpeta (herencia)
- [ ] Sincronización con sistema de archivos (carpeta física para nodo carpeta)

### Fase 4: Automatización Inteligente (Idea 4 Mejorada) ⭐
- [ ] Implementar detección de grupos conectados en el grafo
- [ ] Algoritmo para identificar subgrafos (3+ nodos conectados)
- [ ] Análisis de patrones (librería vs proceso)
- [ ] Sistema de sugerencias contextuales
- [ ] UI para mostrar sugerencias ("¿Crear Nodo Carpeta/Subnetwork?")
- [ ] Conversión automática: grupo seleccionado → Nodo Carpeta
- [ ] Conversión automática: grupo seleccionado → Subnetwork
- [ ] Detección por lenguaje (agrupar nodos del mismo lenguaje)
- [ ] Detección por tags/metadatos
- [ ] Detección por nombres similares (análisis de títulos)

### Fase 5: Avanzado
- [ ] Organización automática por categorías
- [ ] Sugerencias inteligentes
- [ ] Filtros y vistas personalizadas
- [ ] Integración completa con subnetworks

---

## 🎯 Beneficios Esperados

1. **Organización Visual**: Nodos agrupados lógicamente
2. **Navegación Rápida**: Encontrar nodos más fácilmente
3. **Estructura Física**: Reflejada en sistema de archivos
4. **Escalabilidad**: Manejar proyectos con muchos nodos
5. **Colaboración**: Estructura clara para equipos
6. **Versionado**: Cambios de organización visibles en Git

---

## 💬 Próximos Pasos

1. **Revisar ideas** y priorizar según necesidades
2. **Prototipo rápido** de Idea 1 (carpetas virtuales básicas)
3. **Implementar Idea 11** (Nodo Carpeta) - Alta prioridad por funcionalidad única
4. **Testing** con proyectos reales
5. **Iterar** basándose en feedback

---

## 🎯 Comparación: Nodo Carpeta vs Subnetwork

| Característica | Nodo Carpeta | Subnetwork |
|---------------|--------------|------------|
| **Propósito** | Organización + Herencia | Encapsulación de lógica |
| **Herencia** | ✅ Otros nodos heredan contenido | ❌ No diseñado para herencia |
| **Vista Interna** | ✅ Similar a workspace | ✅ Grafo interno |
| **Pines Expuestos** | ❌ No expone pines | ✅ Expone inputs/outputs |
| **Uso Principal** | Librerías reutilizables | Lógica encapsulada |
| **Modo Organización** | ✅ Solo guardar | ❌ Siempre funcional |

**Conclusión**: Nodo Carpeta complementa Subnetworks, enfocándose en **organización y herencia**, mientras que Subnetworks se enfoca en **encapsulación funcional**.

