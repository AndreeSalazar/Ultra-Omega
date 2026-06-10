# Ultra-Omega — arquitectura objetivo Rust/Vulkan para editor profesional de nodos

Este documento resume el estado actual del proyecto y propone una arquitectura nueva para convertir Ultra-Omega en un gestor visual de carpetas, código y grafos de nodos profesionales: algo con la productividad de Visual Studio Code, pero centrado en nodos precisos, directos, reutilizables y renderizados con Vulkan.

## 1. Lectura rápida del estado actual

Ultra-Omega ya tiene una base buena para moverse hacia una versión 100% Rust:

- `src/main.rs` crea una ventana con `winit` y arranca `VulkanContext`.
- `src/vulkan/` contiene inicialización Vulkan con `ash`, swapchain, render pass, pipeline y un renderer básico que dibuja un rectángulo.
- `src/core/` contiene el modelo principal de nodos: nodos, pines, links, carpetas/subnetworks e herencia de código.
- `src/storage/` ya piensa en workspace real: `node_map.json`, archivos de código separados por nodo, migraciones, HDA/assets y file watcher.
- `src/expressions/` implementa un sistema tipo Houdini con `ch()` para conectar valores entre nodos.
- `src/templates/` contiene muchas plantillas Rust reutilizables.
- `src/compilation/` tiene detección/ejecución de compiladores y una terminal lógica.
- `src/ui/` conserva tema/colores, pero la UI real todavía no está integrada al renderer Vulkan.

El punto clave: la lógica de nodos existe, pero el runtime visual actual todavía no la usa. La app gráfica está en modo “motor Vulkan mínimo”.

```text
Estado actual:

╭────────────╮      ╭────────────────╮
│ main.rs    │─────▶│ VulkanContext  │─────▶ rectángulo demo
╰────────────╯      ╰────────────────╯

╭──────────────╮   ╭──────────────╮   ╭──────────────╮
│ NodeGraphApp │   │ Workspace    │   │ Expressions  │
╰──────┬───────╯   ╰──────┬───────╯   ╰──────┬───────╯
       │                  │                  │
       └────────── existen, pero no están conectados al loop Vulkan todavía
```

## 2. Visión recomendada

Ultra-Omega debería ser un editor profesional de nodos donde cada carpeta, archivo, módulo, función, dependencia o flujo pueda representarse como nodo. El objetivo no es copiar VS Code, sino tomar su precisión de edición y convertirla en una interfaz visual de arquitectura.

Prioridad máxima:

1. **Rust primero**: todo nodo ejecutable, plantilla, compilación, introspección y análisis debe favorecer Rust.
2. **Vulkan primero**: nada de frameworks UI pesados; Vulkan debe renderizar canvas, nodos, conexiones, texto, minimap y paneles.
3. **Grafo como fuente de verdad**: el editor no debe ser una colección de ventanas; debe ser un grafo profesional persistente.
4. **Archivos reales, no caja cerrada**: cada nodo con código debe mapearse a `.rs` o archivos reales para Git, Cargo y rust-analyzer.
5. **Reutilización fuerte**: templates, HDAs/assets, subnetworks y snippets deben poder reutilizarse entre proyectos.

## 3. Arquitectura objetivo por capas

```text
╭────────────────────────────────────────────────────────────╮
│                    Ultra-Omega App Runtime                 │
├────────────────────────────────────────────────────────────┤
│ Input + Commands + Selection + Panels + Shortcuts          │
├────────────────────────────────────────────────────────────┤
│ Node Workspace: Graph, Folders, Assets, History, Search    │
├────────────────────────────────────────────────────────────┤
│ Rust Intelligence: Cargo, rust-analyzer, Compiler, Runner  │
├────────────────────────────────────────────────────────────┤
│ Vulkan UI: Canvas, Nodes, Links, Text, Icons, Minimap      │
├────────────────────────────────────────────────────────────┤
│ Platform: winit, ash, filesystem, config, logs             │
╰────────────────────────────────────────────────────────────╯
```

### Capa 1 — Runtime de aplicación

Responsabilidad: unir ventana, eventos, estado global, grafo y renderer.

Estado actual:

- `main.rs` tiene un `App` local con `window` y `vulkan_ctx`.
- `NodeGraphApp` existe en `src/core/app.rs`, pero no se instancia en el runtime principal.

Meta:

- Crear un estado único de app con:
  - `NodeGraphApp`
  - `VulkanContext`
  - estado de input/cámara/selección
  - comandos del usuario
  - estado de paneles: explorer, inspector, terminal, editor, assets

Idea de estructura:

```rust
pub struct UltraOmegaRuntime {
    pub graph_app: NodeGraphApp,
    pub viewport: ViewportState,
    pub selection: SelectionState,
    pub commands: CommandQueue,
    pub panels: PanelState,
}
```

### Capa 2 — Modelo profesional de nodos

Responsabilidad: mantener el grafo como verdad del proyecto.

Estado actual:

- `NodeGraph` ya tiene `nodes`, `links`, ids, pines, subnetworks y funciones de herencia.
- `FolderNode` ya representa carpetas/subredes.
- `NodeLanguage` fue reducido a `Auto`, `Rust`, `Text`, lo cual encaja con la prioridad Rust.

Mejora recomendada:

- Separar datos visuales de datos semánticos:
  - visual: posición, tamaño, color, colapso, selección
  - semántico: ruta, lenguaje, símbolo Rust, dependencia, tipo de nodo
  - ejecución: estado compilado, errores, salida, cache

Tipos de nodos propuestos:

| Tipo | Uso |
|---|---|
| `Folder` | Representa carpeta real o subnetwork visual. |
| `RustFile` | Archivo `.rs` completo. |
| `RustModule` | Módulo Rust dentro de Cargo. |
| `RustFunction` | Función reutilizable como nodo. |
| `RustStruct` | Struct/enum/trait representado visualmente. |
| `CargoCrate` | Crate/dependencia. |
| `Command` | Comando `cargo check`, `cargo run`, tests, scripts. |
| `Asset/HDA` | Subgrafo empaquetado y reutilizable. |
| `Text/Note` | Documentación visual no compilable. |

### Capa 3 — Workspace como gestor de carpetas

Responsabilidad: abrir una carpeta, mapearla a nodos, guardar cambios y sincronizar filesystem ⇄ grafo.

Estado actual:

- `Workspace` guarda/carga `node_map.json`.
- `NodeStorage` separa código por nodo en `nodes/node_000001.rs`.
- `FileWatcherState` detecta archivos y carpetas.
- `ProjectConfig` y migraciones ya existen.

Arquitectura recomendada:

```text
╭───────────────╮       scan/watch       ╭───────────────╮
│ Carpeta real  │───────────────────────▶│ Workspace     │
╰──────┬────────╯                         ╰──────┬────────╯
       │                                         │
       │ archivos .rs/.toml/json                 │ node_map.json
       ▼                                         ▼
╭───────────────╮       sincroniza       ╭───────────────╮
│ Cargo Project │◀──────────────────────▶│ NodeGraph     │
╰───────────────╯                         ╰───────────────╯
```

Regla recomendada:

- Si el usuario abre una carpeta Cargo, Ultra-Omega debe crear nodos desde:
  - `Cargo.toml`
  - `src/main.rs`, `src/lib.rs`
  - módulos `mod.rs`
  - archivos `.rs`
  - funciones públicas importantes
- Si crea nodos nuevos, deben poder materializarse como archivos Rust reales.

### Capa 4 — Renderer Vulkan para nodos de alta calidad

Responsabilidad: dibujar miles de nodos, links y texto con precisión profesional.

Estado actual:

- `VulkanContext` crea instance/device/swapchain/render pass/framebuffers/command buffers.
- `GraphicsPipeline` carga shaders compilados por `build.rs`.
- `Renderer` dibuja un rectángulo fijo.

Arquitectura objetivo del renderer:

```text
╭─────────────────────╮
│ RenderGraphFrame    │ datos preparados por CPU
╰──────────┬──────────╯
           ▼
╭─────────────────────╮
│ GPU Buffers         │ nodes, links, text, icons, selection
╰──────────┬──────────╯
           ▼
╭─────────────────────╮
│ Vulkan Passes       │ background → links → nodes → text → overlays
╰──────────┬──────────╯
           ▼
╭─────────────────────╮
│ Swapchain Present   │
╰─────────────────────╯
```

Pipelines recomendados:

| Pipeline | Uso |
|---|---|
| `background_grid` | Grid infinito tipo editor profesional. |
| `node_rects` | Rectángulos redondeados, headers, estados. |
| `links` | Curvas Bézier, flechas, grosor por tipo. |
| `ports` | Pines de entrada/salida. |
| `text_sdf` | Texto nítido con atlas SDF/MSDF. |
| `icons` | Iconos Rust, carpeta, archivo, crate, error. |
| `selection_overlay` | Selección, hover, búsqueda, minimap. |

Prioridad visual:

1. Canvas con cámara pan/zoom.
2. Nodos rectangulares instanciados desde `NodeGraph`.
3. Links entre pines.
4. Texto legible.
5. Inspector/editor lateral.
6. Minimap y overlays.

### Capa 5 — Rust Intelligence

Responsabilidad: que Ultra-Omega sea un editor visual real para Rust, no solo un canvas.

Estado actual:

- Existen templates Rust.
- Existe `TerminalManager` para compilar/ejecutar código.
- Falta integración con rust-analyzer/LSP.

Meta:

- Cada nodo Rust debe tener:
  - `cargo check` contextual
  - errores inline
  - formato `rustfmt`
  - navegación a símbolo
  - dependencias detectadas
  - pruebas unitarias asociadas

Arquitectura recomendada:

```text
╭──────────────╮    guarda     ╭──────────────╮
│ Nodo Rust    │───────── ────▶│ archivo .rs  │
╰──────┬───────╯               ╰──────┬───────╯
       │                              │
       │ consulta                     │ analiza
       ▼                              ▼
╭──────────────╮               ╭──────────────╮
│ UI errores   │◀──────────────│ rust-analyzer│
╰──────────────╯               ╰──────────────╯
```

## 4. Roadmap recomendado por prioridad

### P0 — Base estable Rust/Vulkan

- Mantener `cargo check` pasando siempre.
- Resolver warnings críticos cuando oculten código muerto real.
- Manejar resize de swapchain.
- Evitar `unwrap()` en zonas Vulkan críticas y convertir errores en logs claros.
- Separar creación Vulkan en bloques: instance, device, swapchain, render pass, sync, pipelines.

### P1 — Conectar grafo real al canvas

- Instanciar `NodeGraphApp` en `main.rs`.
- Convertir `NodeGraph` a datos de render por frame.
- Dibujar todos los nodos como rectángulos Vulkan.
- Dibujar links básicos entre pines.
- Añadir cámara 2D: pan, zoom, transformar coordenadas mundo/pantalla.

### P2 — Gestor de carpetas visual

- Abrir carpeta real como workspace.
- Usar `FileWatcherState` para construir nodos de carpetas/archivos.
- Persistir layout visual en `node_map.json`.
- Sincronizar cambios externos sin destruir layout manual.

### P3 — Edición Rust profesional

- Panel editor de código para el nodo seleccionado.
- Guardar cada nodo Rust como `.rs` real.
- Integrar `rustfmt`.
- Integrar `cargo check` por proyecto.
- Diseñar puente con rust-analyzer.

### P4 — Subnetworks y reutilización avanzada

- Convertir carpetas en subnetworks navegables.
- Exportar/importar HDAs como assets visuales.
- Permitir “plantillas vivas” con parámetros.
- Hacer que un subgrafo pueda convertirse en crate, módulo o función Rust.

### P5 — Calidad profesional tipo IDE

- Command palette.
- Búsqueda global de nodos, símbolos y archivos.
- Minimap.
- Terminal integrada real.
- Debug visual de flujo.
- Profiler de nodos/render.
- Snap/grid/auto-layout por compute shader.

## 5. Decisiones técnicas recomendadas

### Mantener Vulkan directo, pero ordenar responsabilidades

No conviene meter `egui`, `imgui` o `wgpu` si la visión es control máximo. Pero sí conviene crear capas internas pequeñas:

- `VulkanContext`: recursos base y swapchain.
- `GpuResources`: buffers, memoria, descriptors.
- `PipelineRegistry`: pipelines por tipo visual.
- `FrameRenderer`: graba command buffers.
- `SceneBuilder`: transforma `NodeGraph` en batches GPU.

### Usar datos GPU-friendly

Los nodos no deberían mandarse uno por uno. Deben agruparse:

```rust
#[repr(C)]
pub struct GpuNodeInstance {
    pub rect: [f32; 4],
    pub color: [f32; 4],
    pub flags: u32,
    pub _pad: [u32; 3],
}
```

Eso permite renderizar muchos nodos con instancing.

### Preservar archivos reales

El proyecto ya va bien con `NodeStorage`. La regla debe ser:

- El grafo guarda estructura, layout y metadatos.
- El código vive en archivos reales.
- Los nodos enlazan a rutas, símbolos o rangos de código.

### No convertir todo en “multi-lenguaje”

Para la etapa actual, mantener foco:

- Rust = prioridad máxima.
- Text = documentación.
- Auto = detección.

Otros lenguajes pueden esperar. Si se agregan, deben ser plugins o adapters, no contaminar el core.

## 6. Riesgos detectados

| Riesgo | Impacto | Solución recomendada |
|---|---:|---|
| Lógica de nodos desconectada de `main.rs` | Alto | Integrar `NodeGraphApp` al runtime principal. |
| Vulkan creciendo en un solo archivo | Alto | Separar context/swapchain/pipeline/resources/frame. |
| UI textual difícil en Vulkan puro | Alto | Planear temprano atlas SDF/MSDF. |
| Warnings masivos por código no usado | Medio | Reducir después de conectar módulos, no borrar antes. |
| `unwrap()` en Vulkan | Medio | Convertir a errores/logs controlados. |
| Workspace vs archivos externos | Alto | Definir sincronización bidireccional con IDs estables. |

## 7. Modelo mental final

Ultra-Omega debería sentirse así:

```text
╭──────────────────────────────────────────────────────────╮
│ Explorer visual de carpeta                               │
│  ╭────────────╮   ╭────────────╮   ╭────────────╮        │
│  │ Cargo.toml │──▶│ src/main.rs│──▶│ función run│       │
│  ╰────────────╯   ╰────────────╯   ╰────────────╯        │
│         │               │                │               │
│         ▼               ▼                ▼               │
│  dependencias      módulos Rust      nodos ejecutables   │
├──────────────────────────────────────────────────────────┤
│ Panel inspector + editor Rust + terminal + errores LSP   │
╰──────────────────────────────────────────────────────────╯
```

La ventaja de Ultra-Omega frente a VS Code no debe ser “tener pestañas”. Debe ser poder ver arquitectura, carpetas, código, dependencias y ejecución como un grafo editable de alta precisión.

## 8. Estado de implementación por fases

### Fase P1 activa — base Vulkan limpia para nodos Rust

Implementado:

- `main.rs` usa `NodeGraph` directo como estado activo de aplicación.
- Los módulos antiguos (`storage`, `templates`, `expressions`, `inheritance`, `utils`, `config`) quedan desconectados temporalmente del runtime para reescribir la base Vulkan sin ruido.
- `VulkanContext::draw_frame()` recibe `&NodeGraph`.
- `Renderer` deja de depender de un rectángulo hardcodeado.
- El renderer genera vértices desde los nodos del grafo:
  - cuerpo del nodo
  - header coloreado
  - borde/sombra
  - pines de entrada/salida
- El canvas tiene grid infinito básico afectado por pan/zoom.
- El renderer dibuja conexiones reales entre pines usando los `links` del `NodeGraph`.
- El runtime tiene cámara 2D básica:
  - rueda del mouse para zoom hacia el cursor
  - botón central del mouse para pan
- El runtime tiene interacción básica:
  - hover de nodos
  - selección con click izquierdo
  - borde azul para hover
  - borde amarillo para selección
  - drag con click izquierdo para mover nodos
- El runtime permite edición inicial del grafo:
  - `N` crea un nodo Rust nuevo en el centro de la vista
  - `Delete` elimina el nodo seleccionado
  - `Esc` limpia selección
  - `R` reinicia pan/zoom
  - click sobre un pin de salida inicia una conexión
  - click sobre un pin de entrada termina la conexión
  - `C` marca la salida `0` del nodo seleccionado como origen de conexión rápido
- Los links se renderizan como curvas Bézier segmentadas en vez de líneas rectas.
- Corregida la conversión vertical a NDC de Vulkan: render, hit-test y drag quedan sincronizados; arriba/abajo ya no están invertidos.
- El primer frame se solicita con `window.request_redraw()`.
- `cargo check` pasa sin warnings en la app activa.

Pendiente dentro de P1:

- Empezar texto GPU para mostrar títulos de nodos.
- Migrar de vértices reconstruidos por frame a instancing GPU-friendly.
- Añadir resize robusto de swapchain.
- Añadir feedback visual de pin hover y preview de conexión antes de soltar/click final.

### Próximo paso inmediato

El siguiente cambio recomendado es **texto GPU para títulos de nodos y mejor sistema de recursos/pipelines**. Eso convierte los rectángulos actuales en nodos identificables y prepara la transición hacia un editor visual profesional de Rust.
