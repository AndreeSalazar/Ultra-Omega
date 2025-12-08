use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2, Visuals, pos2, PointerButton};
use eframe::egui::text_selection::CursorRange;
use super::node_graph::{self, Link, Node, NodeGraph, NodeId, NodeLanguage, PinId, PinKind};
use crate::compilation::terminal::{TerminalManager, TerminalTab};
use crate::ui::viewport::Viewport2D;
use crate::ui::layout::LayoutConfig;
use crate::storage::{Workspace, migration::{needs_migration, migrate_project, create_backup, MigrationResult}};
use crate::config::AppConfig;
use crate::compilation::compiler_detector::{CompilerStatus, detect_all_compilers};

pub struct NodeGraphApp {
    pub graph: NodeGraph,
    pub viewport: Viewport2D,
    pub interaction: InteractionState,
    pub terminal: TerminalManager,
    pub show_node_menu: bool,
    pub node_menu_pos: Pos2,
    pub new_node_title: String, // Título para el nuevo nodo
    pub workspace: Workspace,
    pub last_save_hash: u64,
    pub last_save_time: Option<std::time::Instant>,
    pub channel_manager: crate::expressions::ChannelManager, // Sistema de canales para ch()
    // Menú de búsqueda F3 estilo Blender
    pub show_search_menu: bool,
    pub search_query: String,
    pub selected_category: Option<String>,
    // Sistema de Layout para nodos
    pub layout_config: LayoutConfig,
    // Detector de compiladores
    pub compiler_status: Option<CompilerStatus>,
    pub show_compiler_status: bool,
    // Sistema de migración
    pub migration_dialog: Option<MigrationDialogState>,
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 SISTEMA DE SUBNETWORKS (Inspiración Houdini)
    // ═══════════════════════════════════════════════════════════════════
    /// Pila de niveles de red (para navegación jerárquica)
    pub network_levels: Vec<NetworkLevel>,
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 SISTEMA DE HDAs (Houdini Digital Assets)
    // ═══════════════════════════════════════════════════════════════════
    pub show_hda_export_dialog: bool,
    pub show_hda_import_dialog: bool,
    pub hda_export_name: String,
    pub hda_export_label: String,
    pub hda_export_description: String,
    pub hda_export_author: String,
    pub hda_export_category: String,
    pub hda_export_to_global: bool,
    // Parámetros del HDA que se está exportando
    pub hda_export_parameters: Vec<crate::storage::HDAParameter>,
    // Parámetros del HDA que se está importando (nombre -> valor)
    pub hda_import_parameter_values: std::collections::HashMap<String, String>,
    pub hda_import_selected_asset: Option<(std::path::PathBuf, crate::storage::HDAInfo)>,
}

/// Representa un nivel de red en la jerarquía de subnetworks
#[derive(Clone, Debug)]
pub struct NetworkLevel {
    /// El grafo en este nivel
    pub graph: NodeGraph,
    /// ID del nodo subnetwork que contiene este nivel (None = nivel raíz)
    pub parent_subnetwork_id: Option<NodeId>,
    /// Breadcrumbs para mostrar la ruta de navegación
    pub breadcrumbs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MigrationDialogState {
    pub show: bool,
    pub needs_migration: bool,
    pub backup_path: Option<std::path::PathBuf>,
    pub result: Option<MigrationResult>,
    pub error: Option<String>,
}

#[derive(Default)]
pub struct InteractionState {
    pub dragging_node: Option<NodeId>,
    pub editing_node: Option<NodeId>,
    pub selected_nodes: std::collections::HashSet<NodeId>,
    pub box_selection_start: Option<Pos2>,
    pub box_selection_current: Option<Pos2>,
    pub connecting_from: Option<PinId>,
    pub viewing_inheritance: Option<NodeId>,
    pub cut_tool: crate::ui::cut::CutTool,
    pub editor_history: Option<crate::utils::editor_history::EditorHistory>, // Historial del editor
    pub show_r_menu: bool, // Menú Ctrl+R para exportación rápida a parámetros
    pub multi_param_mode: bool, // Modo de múltiples parámetros (Ctrl+Shift+P)
    pub r_menu_selection: Option<(usize, usize, String)>, // Selección guardada para menú Ctrl+R (start, end, text)
    pub r_menu_pos: Option<egui::Pos2>, // Posición del menú Ctrl+R
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 DOBLE CLIC PARA SUBNETWORKS
    // ═══════════════════════════════════════════════════════════════════
    pub last_click_node: Option<(NodeId, std::time::Instant)>, // Último nodo clickeado y tiempo
}

#[derive(Clone, Copy)]
struct PointerSnapshot {
    pos: Option<Pos2>,
    delta: Vec2,
    primary_pressed: bool,
    primary_down: bool,
    secondary_pressed: bool,
    middle_down: bool,
    scroll_delta: f32,
    ctrl_scroll: f32,
    modifiers: egui::Modifiers,
}

impl Default for NodeGraphApp {
    fn default() -> Self {
        Self::from_config(AppConfig::default())
    }
}

impl NodeGraphApp {
    /// Determina el color de sintaxis para una línea de código Assembly
    fn get_asm_line_color(line: &str) -> Color32 {
        let trimmed = line.trim_start();
        if trimmed.starts_with(';') {
            // Comentarios en verde
            Color32::from_rgb(106, 153, 85)
        } else if trimmed.contains("section") || trimmed.contains("global") || trimmed.contains("extern") || trimmed.contains("default") {
            // Keywords en azul
            Color32::from_rgb(86, 156, 214)
        } else if trimmed.ends_with(':') {
            // Labels en amarillo/naranja
            Color32::from_rgb(220, 220, 170)
        } else if trimmed.starts_with("db") || trimmed.starts_with("dw") || trimmed.starts_with("dd") || trimmed.starts_with("dq") {
            // Directivas de datos en púrpura
            Color32::from_rgb(197, 134, 192)
        } else {
            // Código normal en blanco/gris claro
            Color32::from_rgb(212, 212, 212)
        }
    }

    pub fn from_config(config: AppConfig) -> Self {
        let mut workspace = Workspace::new();
        workspace.auto_save = config.auto_save;
        
        let initial_graph = if let Some(workspace_path) = config.workspace_path.as_ref() {
            let path = std::path::PathBuf::from(workspace_path);
            if path.exists() {
                let mut workspace = Workspace::new();
                workspace.set_root(path.clone());
                workspace.load_graph().unwrap_or_else(|_| {
                    eprintln!("Error loading workspace, using demo");
                    NodeGraph::demo()
                })
            } else {
                NodeGraph::demo()
            }
        } else {
            NodeGraph::demo()
        };
        
        let mut app = Self {
            graph: initial_graph.clone(),
            viewport: Viewport2D::default(),
            interaction: InteractionState::default(),
            terminal: TerminalManager::default(),
            show_node_menu: false,
            node_menu_pos: Pos2::ZERO,
            new_node_title: String::new(),
            workspace,
            last_save_hash: 0,
            last_save_time: None,
            channel_manager: crate::expressions::ChannelManager::new(),
            show_search_menu: false,
            search_query: String::new(),
            selected_category: None,
            layout_config: LayoutConfig::default(),
            compiler_status: None,
            show_compiler_status: false,
            migration_dialog: None,
            // Inicializar con nivel raíz (el grafo principal es el nivel 0)
            network_levels: vec![NetworkLevel {
                graph: initial_graph,
                parent_subnetwork_id: None,
                breadcrumbs: vec!["Root".to_string()],
            }],
            // HDA system
            show_hda_export_dialog: false,
            show_hda_import_dialog: false,
            hda_export_name: String::new(),
            hda_export_label: String::new(),
            hda_export_description: String::new(),
            hda_export_author: String::new(),
            hda_export_category: String::from("General"),
            hda_export_to_global: false,
            hda_export_parameters: Vec::new(),
            hda_import_parameter_values: std::collections::HashMap::new(),
            hda_import_selected_asset: None,
        };
        
        // Load workspace if configured
        if let Some(workspace_path) = config.workspace_path {
            let path = std::path::PathBuf::from(&workspace_path);
            if path.exists() {
                app.workspace.set_root(path);
                if let Err(e) = app.load_graph_from_workspace() {
                    eprintln!("Error loading workspace: {}", e);
                    // Fallback to demo if load fails
                    app.graph = NodeGraph::demo();
                    app.graph.recalculate_ids();
                    // Actualizar nivel raíz también
                    if let Some(level) = app.network_levels.get_mut(0) {
                        level.graph = app.graph.clone();
                    }
                } else {
                    // Actualizar nivel raíz con el grafo cargado
                    if let Some(level) = app.network_levels.get_mut(0) {
                        level.graph = app.graph.clone();
                    }
                }
            } else {
                app.graph = NodeGraph::demo();
                app.graph.recalculate_ids();
                if let Some(level) = app.network_levels.get_mut(0) {
                    level.graph = app.graph.clone();
                }
            }
        } else {
            app.graph = NodeGraph::demo();
            app.graph.recalculate_ids();
            if let Some(level) = app.network_levels.get_mut(0) {
                level.graph = app.graph.clone();
            }
        }
        
        // Registrar todos los nodos existentes en el sistema de canales
        let nodes_to_register: Vec<_> = app.graph.nodes().iter().map(|n| (n.id, n.title.clone(), n.code.clone())).collect();
        for (node_id, title, code) in nodes_to_register {
            use crate::expressions::ChannelValue;
            app.channel_manager.set_channel(title.clone(), ChannelValue::Code(code.clone()));
            app.channel_manager.set_channel(format!("{}/code", title.clone()), ChannelValue::Code(code.clone()));
            app.channel_manager.set_node_channel(node_id, "code".to_string(), ChannelValue::Code(code));
        }
        
        app
    }
    
    pub fn save_config(&self) {
        let mut config = AppConfig::load();
        config.workspace_path = self.workspace.root_path.as_ref()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string());
        config.auto_save = self.workspace.auto_save;
        
        if let Err(e) = config.save() {
            eprintln!("Error saving config: {}", e);
        }
    }
}

impl eframe::App for NodeGraphApp {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save config on exit
        self.save_config();
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle global shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
            self.show_node_menu = !self.show_node_menu;
            self.show_search_menu = false;
            if self.show_node_menu {
                self.node_menu_pos = ctx.pointer_hover_pos().unwrap_or(pos2(200.0, 200.0));
                self.selected_category = None;
            }
        }

        // F3 para búsqueda rápida estilo Blender
        if ctx.input(|i| i.key_pressed(egui::Key::F3)) {
            self.show_search_menu = !self.show_search_menu;
            self.show_node_menu = false;
            if self.show_search_menu {
                self.node_menu_pos = ctx.pointer_hover_pos().unwrap_or(pos2(200.0, 200.0));
                self.search_query.clear();
            }
        }

        if ctx.input(|i| i.key_pressed(egui::Key::F)) {
            self.focus_view(ctx.screen_rect());
        }

        // Handle Ctrl+S for save
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && !i.modifiers.shift) {
            if self.workspace.has_root() {
                let _ = self.save_current_graph();
            }
        }

        // Handle Ctrl+Shift+S for save as
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl && i.modifiers.shift) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Node Map", &["json"])
                .set_file_name("node_map.json")
                .save_file()
            {
                if let Some(parent) = path.parent() {
                    self.workspace.set_root(parent.to_path_buf());
                    self.workspace.current_file = Some(path);
                    let _ = self.save_current_graph();
                }
            }
        }

        // Handle Ctrl+Shift+C for compiler status
        if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl && i.modifiers.shift) {
            self.show_compiler_status = !self.show_compiler_status;
            if self.show_compiler_status {
                // Actualizar estado de compiladores
                self.compiler_status = Some(detect_all_compilers());
                // Mostrar en terminal también
                if let Some(status) = &self.compiler_status {
                    self.terminal.rust_output = status.summary();
                    self.terminal.active_tab = crate::compilation::terminal::TerminalTab::Rust;
                    self.terminal.visible = true;
                }
            }
        }

        // Detectar compiladores al inicio si no se ha hecho
        if self.compiler_status.is_none() {
            self.compiler_status = Some(detect_all_compilers());
        }

        // Handle Delete key to remove selected nodes
        // Backspace también se usa para salir de subnetworks, así que verificar primero
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            if !self.interaction.selected_nodes.is_empty() {
                // Don't delete if editing a node
                if self.interaction.editing_node.is_none() {
                    // Limpiar canales de los nodos que se van a eliminar
                    for node_id in &self.interaction.selected_nodes {
                        self.channel_manager.clear_node_channels(*node_id);
                        // También limpiar por nombre si existe
                        if let Some(_node) = self.graph.nodes().iter().find(|n| n.id == *node_id) {
                            // Nota: No podemos eliminar del HashMap directamente aquí, pero el canal quedará obsoleto
                            // Se actualizará cuando se registre un nuevo nodo con el mismo nombre
                        }
                    }
                    self.graph.remove_nodes(&self.interaction.selected_nodes);
                    self.interaction.selected_nodes.clear();
                    // Sincronizar cambios al nivel actual
                    self.sync_current_level_to_graph();
                    // Auto-save after deletion
                    self.check_and_auto_save();
                }
            }
        }

        // ═══════════════════════════════════════════════════════════════════
        // 🆕 ATAJOS DE TECLADO PARA SUBNETWORKS
        // ═══════════════════════════════════════════════════════════════════
        
        // Enter: Entrar al subnetwork seleccionado
        if ctx.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.ctrl && !i.modifiers.shift) {
            if !self.interaction.selected_nodes.is_empty() && self.interaction.editing_node.is_none() {
                if let Some(&selected_id) = self.interaction.selected_nodes.iter().next() {
                    if let Some(node) = self.graph.node(selected_id) {
                        if node.subnetwork_graph.is_some() {
                            if let Err(e) = self.enter_subnetwork(selected_id) {
                                eprintln!("Error entering subnetwork: {}", e);
                            }
                        }
                    }
                }
            }
        }

        // Esc o Backspace: Salir del subnetwork actual (solo si no estamos editando)
        if ctx.input(|i| (i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace)) 
            && !i.modifiers.ctrl && !i.modifiers.shift) {
            if self.interaction.editing_node.is_none() && !self.is_at_root() {
                if let Err(e) = self.exit_subnetwork() {
                    eprintln!("Error exiting subnetwork: {}", e);
                }
            }
        }

        // Handle Y key for cut tool (Houdini style) - toggle mode
        // Usar key_pressed para toggle, pero verificar que no haya click simultáneo
        if ctx.input(|i| i.key_pressed(egui::Key::Y) && !i.pointer.primary_down()) {
            self.interaction.cut_tool.active = !self.interaction.cut_tool.active;
            if !self.interaction.cut_tool.active {
                // Desactivar: limpiar puntos y cancelar cualquier operación en curso
                self.interaction.cut_tool.clear();
                // También limpiar selección/drag si estaban activos
                self.interaction.dragging_node = None;
                self.interaction.box_selection_start = None;
                self.interaction.box_selection_current = None;
            } else {
                // Al activar, limpiar cualquier selección/drag en curso
                self.interaction.dragging_node = None;
                self.interaction.box_selection_start = None;
                self.interaction.box_selection_current = None;
            }
        }

        // Auto-save and file change detection
        self.check_and_auto_save();
        self.check_file_changes();
        
        // Save config periodically (every 5 seconds or on close)
        // This will be handled on close via save_config

        // 1. Top Menu Bar (Always visible, no animation)
        crate::ui::menu::draw_menu_bar(self, ctx, 1.0);

        // 2. Bottom Terminal
        // Logic handled inside terminal_panel now (timer based)
        self.terminal_panel(ctx, 1.0);

        // 3. Left Sidebar (Always visible and pinned)
        crate::ui::sidebar::draw_sidebar(self, ctx, 1.0);

        // 4. Central Canvas (Remaining space)
        self.canvas_ui(ctx);

            // 5. Overlays
            self.editor_ui(ctx);
            self.node_menu_ui(ctx);
            self.inheritance_view_ui(ctx);
            self.draw_compiler_status(ctx);
            self.draw_migration_dialog(ctx);
        }
}

impl NodeGraphApp {
    pub fn focus_view(&mut self, screen_rect: Rect) {
        // If a node is explicitly selected (single selection for focus), focus on it
        // If multiple, focus on bounds of all selected
        if !self.interaction.selected_nodes.is_empty() {
             let mut bounds = Rect::NOTHING;
             for id in &self.interaction.selected_nodes {
                 if let Some(node) = self.graph.nodes().iter().find(|n| n.id == *id) {
                     bounds = bounds.union(self.node_rect_at_scale(node, 1.0));
                 }
             }
             if bounds.is_positive() {
                 self.viewport.focus_on(bounds, screen_rect);
                 return;
             }
        }

        // Fallback to previous logic (editing or dragging)
        if let Some(target_id) = self.interaction.editing_node.or(self.interaction.dragging_node) {
             let mut target_rect = Rect::NOTHING;
             if let Some(node) = self.graph.nodes().iter().find(|n| n.id == target_id) {
                 target_rect = self.node_rect_at_scale(node, 1.0);
             }
             if target_rect.is_positive() {
                 self.viewport.focus_on(target_rect, screen_rect);
                 return;
             }
        }

        // Otherwise focus all
        if self.graph.nodes().is_empty() {
            self.viewport = Viewport2D::default();
            return;
        }

        let mut bounds = Rect::NOTHING;
        for node in self.graph.nodes() {
            let rect = self.node_rect_at_scale(node, 1.0);
            bounds = bounds.union(rect);
        }
        
        if bounds.is_positive() {
            self.viewport.focus_on(bounds, screen_rect);
        }
    }

    fn node_rect_at_scale(&self, node: &Node, scale: f32) -> Rect {
        let size = self.node_size(node) * scale;
        // Position is already in world coordinates
        Rect::from_min_size(node.position, size)
    }

    fn add_template_node(&mut self, ctx: &egui::Context, title: &str, code: &str, color: Color32, language: NodeLanguage) {
        let world_pos = self.viewport.screen_to_world(self.node_menu_pos, Rect::from_min_size(Pos2::ZERO, Vec2::new(10000.0, 10000.0)));
        
        // Todos los nodos tienen una entrada "Entrada" y una salida "Código"
        let id = self.graph.add_node(title, world_pos, color, &["Entrada"], &["Código"], language);
        let (node_title, node_code) = {
            if let Some(node) = self.graph.node_mut(id) {
                node.code = code.to_string();
                node.language = language;
                (node.title.clone(), node.code.clone())
            } else {
                return;
            }
        };
        // Registrar nodo en el sistema de canales (después de soltar el borrow)
        use crate::expressions::ChannelValue;
        self.channel_manager.set_channel(node_title.clone(), ChannelValue::Code(node_code.clone()));
        self.channel_manager.set_channel(format!("{}/code", node_title.clone()), ChannelValue::Code(node_code.clone()));
        self.channel_manager.set_node_channel(id, "code".to_string(), ChannelValue::Code(node_code));
        
        // Sincronizar cambios al nivel actual
        self.sync_current_level_to_graph();
        
        // Auto-save immediately when a node is created
        if self.workspace.has_root() {
            if let Err(e) = self.save_current_graph() {
                eprintln!("Error auto-saving after node creation: {}", e);
            } else {
                // Request repaint to update sidebar
                ctx.request_repaint();
            }
        }
    }

    fn resolve_effective_language(&self, node_id: NodeId, inheritance_chain: &[(NodeId, String, String)]) -> NodeLanguage {
        if let Some(node) = self.graph.node(node_id) {
            if node.language != NodeLanguage::Auto {
                return node.language;
            }
        }

        for (ancestor_id, _title, _code) in inheritance_chain.iter().rev() {
            if let Some(node) = self.graph.node(*ancestor_id) {
                if node.language != NodeLanguage::Auto {
                    return node.language;
                }
            }
        }

        NodeLanguage::Auto
    }

    fn language_to_terminal(language: NodeLanguage, node_title: &str) -> crate::compilation::terminal::Language {
        match language {
            NodeLanguage::Asm => crate::compilation::terminal::Language::Nasm,
            NodeLanguage::C => crate::compilation::terminal::Language::C,
            NodeLanguage::Cpp => crate::compilation::terminal::Language::Cpp,
            NodeLanguage::Rust => crate::compilation::terminal::Language::Rust,
            NodeLanguage::Zig => crate::compilation::terminal::Language::Zig,
            NodeLanguage::Java => crate::compilation::terminal::Language::Java,
            NodeLanguage::Text => crate::compilation::terminal::Language::C, // Text no se compila realmente
            NodeLanguage::Mojo => crate::compilation::terminal::Language::Mojo,
            NodeLanguage::MojoAI => crate::compilation::terminal::Language::Mojo, // MojoAI también usa Mojo
            NodeLanguage::Auto => {
                let lower = node_title.to_lowercase();
                if lower.contains("asm") {
                    crate::compilation::terminal::Language::Nasm
                } else if lower.contains("java") {
                    crate::compilation::terminal::Language::Java
                } else if lower.contains("zig") {
                    crate::compilation::terminal::Language::Zig
                } else if lower.contains("cpp") || lower.contains("c++") {
                    crate::compilation::terminal::Language::Cpp
                } else if lower.contains("rust") {
                    crate::compilation::terminal::Language::Rust
                } else if lower.contains("c ") || lower.ends_with('c') {
                    crate::compilation::terminal::Language::C
                } else {
                    crate::compilation::terminal::Language::C
                }
            }
        }
    }
    
    /// Registrar un nodo en el sistema de canales para acceso mediante ch()
    fn register_node_in_channels(&mut self, node_id: NodeId, node: &Node) {
        use crate::expressions::ChannelValue;
        // Registrar por nombre del nodo
        self.channel_manager.set_channel(
            node.title.clone(),
            ChannelValue::Code(node.code.clone()),
        );
        self.channel_manager.set_channel(
            format!("{}/code", node.title.clone()),
            ChannelValue::Code(node.code.clone()),
        );
        // Registrar por ID del nodo
        self.channel_manager.set_node_channel(
            node_id,
            "code".to_string(),
            ChannelValue::Code(node.code.clone()),
        );
    }
    
    /// Actualizar canales cuando un nodo cambia
    pub fn update_node_channels(&mut self, node_id: NodeId) {
        // Clonar datos para evitar problemas de borrow
        if let Some(node) = self.graph.nodes().iter().find(|n| n.id == node_id) {
            let title = node.title.clone();
            let code = node.code.clone();
            use crate::expressions::ChannelValue;
            self.channel_manager.set_channel(title.clone(), ChannelValue::Code(code.clone()));
            self.channel_manager.set_channel(format!("{}/code", title.clone()), ChannelValue::Code(code.clone()));
            self.channel_manager.set_node_channel(node_id, "code".to_string(), ChannelValue::Code(code));
        }
    }

    fn generate_unique_parameter_name(&self) -> String {
        let mut index = 1;
        loop {
            let name = format!("Parámetro {}", index);
            if self.graph.nodes().iter().all(|n| n.title != name) {
                break name;
            }
            index += 1;
        }
    }

    fn create_parameter_node_from_selection(&mut self, target_node_id: NodeId, value: &str) -> Option<String> {
        let title = self.generate_unique_parameter_name();
        let base_pos = self.graph.node(target_node_id).map(|n| n.position).unwrap_or(Pos2::new(0.0, 0.0));
        
        // Posición ADELANTE (derecha) del nodo original
        // El parámetro hereda del nodo A y permite modificar valores
        let new_pos = Pos2::new(base_pos.x + 300.0, base_pos.y);
        let param_color = Color32::from_rgb(0x29, 0x7d, 0xf0);
        
        let inheritance_chain = self.graph.get_inheritance_chain(target_node_id);
        let effective_language = self.resolve_effective_language(target_node_id, &inheritance_chain);

        // El parámetro tiene Entrada (para heredar de A) y Código (para exportar)
        let new_node_id = self.graph.add_node(
            &title,
            new_pos,
            param_color,
            &["Entrada"],   // Recibe herencia del nodo A
            &["Código"],    // Exporta el valor modificado
            effective_language,
        );

        if let Some(node) = self.graph.node_mut(new_node_id) {
            node.code = value.to_string();
        }

        if let Some(snapshot) = self.graph.node(new_node_id).cloned() {
            self.register_node_in_channels(new_node_id, &snapshot);
            
            // Conectar: Nodo A (Código) -> Parámetro B (Entrada)
            // Así el parámetro hereda todo del nodo A
            if let Some(target_node) = self.graph.node(target_node_id) {
                if let Some(output_pin) = target_node.outputs.first() {
                    if let Some(input_pin) = snapshot.inputs.first() {
                         self.graph.add_link(output_pin.id, input_pin.id, Color32::from_rgb(150, 200, 255));
                    }
                }
            }
        }

        Some(title)
    }

    fn extract_selected_text(text: &str, range: &CursorRange) -> Option<(String, usize, usize)> {
        if range.is_empty() {
            return None;
        }
        let char_range = range.as_sorted_char_range();
        if char_range.start == char_range.end {
            return None;
        }
        let (start_byte, end_byte) = Self::char_range_to_byte_range(text, char_range);
        if start_byte >= end_byte || start_byte > text.len() || end_byte > text.len() {
            return None;
        }
        Some((text[start_byte..end_byte].to_string(), start_byte, end_byte))
    }

    fn char_range_to_byte_range(text: &str, char_range: std::ops::Range<usize>) -> (usize, usize) {
        if char_range.start >= char_range.end {
            let idx = char_range.start.min(text.chars().count());
            return (text.len().min(idx), text.len().min(idx));
        }

        let mut current_char = 0;
        let mut start_byte = text.len();
        let mut end_byte = text.len();
        for (byte_idx, _) in text.char_indices() {
            if current_char == char_range.start && start_byte == text.len() {
                start_byte = byte_idx;
            }
            if current_char == char_range.end {
                end_byte = byte_idx;
                break;
            }
            current_char += 1;
        }

        if start_byte == text.len() {
            start_byte = text.len();
        }
        if end_byte == text.len() && char_range.end >= text.chars().count() {
            end_byte = text.len();
        }

        (start_byte.min(text.len()), end_byte.min(text.len()))
    }
    
    fn terminal_panel(&mut self, ctx: &egui::Context, _open_factor: f32) {
        // Terminal visibility logic
        if !self.terminal.visible { return; }

        // Timer logic: decrement if not pinned
        if !self.terminal.pinned {
            let dt = ctx.input(|i| i.stable_dt);
            self.terminal.hide_timer -= dt;
            if self.terminal.hide_timer <= 0.0 {
                self.terminal.visible = false;
                return;
            }
            // Keep refreshing while timer is active
            ctx.request_repaint();
        }

        let height = 180.0;

        egui::TopBottomPanel::bottom("terminal_panel")
            .resizable(false)
            .exact_height(height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::Nasm, "Terminal NASM");
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::C, "Terminal C");
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::Cpp, "Terminal C++");
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::Rust, "Terminal Rust");
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::Zig, "Terminal Zig");
                    ui.selectable_value(&mut self.terminal.active_tab, TerminalTab::Java, "Terminal Java");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Pin button
                        let pin_text = if self.terminal.pinned { "📌 Anclado" } else { "⚓ Anclar" };
                        if ui.button(pin_text).clicked() {
                            self.terminal.pinned = !self.terminal.pinned;
                            // Reset timer if unpinning to give user a chance
                            if !self.terminal.pinned {
                                self.terminal.hide_timer = 10.0;
                            }
                        }
                        // Close button
                        if ui.button("❌").clicked() {
                            self.terminal.visible = false;
                        }
                    });
                });
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let text = match self.terminal.active_tab {
                        TerminalTab::Nasm => &mut self.terminal.asm_output,
                        TerminalTab::C => &mut self.terminal.c_output,
                        TerminalTab::Cpp => &mut self.terminal.cpp_output,
                        TerminalTab::Rust => &mut self.terminal.rust_output,
                        TerminalTab::Zig => &mut self.terminal.zig_output,
                        TerminalTab::Java => &mut self.terminal.java_output,
                        TerminalTab::Mojo => &mut self.terminal.rust_output, // Mojo usa el buffer de Rust por ahora
                    };
                    
                    ui.add(
                        egui::TextEdit::multiline(text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(8),
                    );
                });
            });
    }

    fn node_menu_ui(&mut self, ctx: &egui::Context) {
        // ═══════════════════════════════════════════════════════════════
        // MENÚ TAB - Estilo Blender con categorías y subcategorías
        // ═══════════════════════════════════════════════════════════════
        if self.show_node_menu {
            let mut close_menu = false;
            let mut template_to_add: Option<crate::templates::Template> = None;
            
            egui::Area::new("node_menu_area".into())
                .fixed_pos(self.node_menu_pos)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    let frame = egui::Frame::window(ui.style())
                        .shadow(eframe::egui::epaint::Shadow {
                            offset: Vec2::new(4.0, 8.0),
                            blur: 12.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(120),
                        })
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(8.0));
                        
                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Panel izquierdo: Categorías
                            ui.vertical(|ui| {
                                ui.set_width(200.0);
                                
                                // Header mejorado
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(40, 45, 55, 200))
                                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(100, 150, 255)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("➕").size(18.0).color(Color32::from_rgb(100, 200, 255)));
                                            ui.label(egui::RichText::new("Add").strong().size(14.0));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(egui::RichText::new("Shift+A").small().color(Color32::from_rgb(150, 180, 200)));
                                            });
                                        });
                                    });
                                ui.add_space(8.0);
                                
                                // Nodo nuevo con mejor diseño
                                let new_selected = self.selected_category.as_deref() == Some("new");
                                let new_bg = if new_selected {
                                    Color32::from_rgba_unmultiplied(100, 150, 255, 120)
                                } else {
                                    Color32::from_rgba_unmultiplied(40, 45, 55, 100)
                                };
                                let new_border = if new_selected {
                                    Color32::from_rgb(150, 200, 255)
                                } else {
                                    Color32::from_rgb(80, 90, 100)
                                };
                                
                                let new_response = egui::Frame::none()
                                    .fill(new_bg)
                                    .stroke(egui::Stroke::new(if new_selected { 2.0 } else { 1.0 }, new_border))
                                    .rounding(egui::Rounding::same(5.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("✨").size(16.0));
                                            ui.label(egui::RichText::new("Nuevo Nodo")
                                                .size(13.0)
                                                .color(if new_selected { Color32::WHITE } else { Color32::from_rgb(220, 220, 230) }));
                                        });
                                    }).response;
                                
                                if new_response.clicked() {
                                    self.selected_category = Some("new".to_string());
                                }
                                
                                ui.add_space(6.0);
                                
                                // Separador visual mejorado
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), 1.0),
                                    egui::Sense::hover()
                                );
                                ui.painter().rect_filled(
                                    rect,
                                    0.0,
                                    Color32::from_rgba_unmultiplied(100, 150, 255, 60)
                                );
                                ui.add_space(6.0);
                                
                                // Inicializar categoría por defecto si no hay ninguna seleccionada
                                if self.selected_category.is_none() {
                                    self.selected_category = Some("C++".to_string());
                                }
                                
                                // Categorías de templates con mejor diseño
                                let categories = ["Assembler (Windows)", "Assembler (Linux)", "C", "C++", "Rust", "Zig", "Java", "FastOS ASM+Rust+Zig", "Vulkan", "DirectX12"];
                                let category_icons = ["🔧", "🐧", "📘", "📗", "🦀", "⚡", "☕", "🚀", "🎮", "💎"];
                                let category_colors = [
                                    Color32::from_rgb(0xff, 0x47, 0x00), // Naranja para Windows
                                    Color32::from_rgb(0x00, 0xaa, 0xff), // Cyan para Linux
                                    Color32::from_rgb(0x00, 0x59, 0x9C),
                                    Color32::from_rgb(0x00, 0x44, 0x82),
                                    Color32::from_rgb(0xde, 0x39, 0x00),
                                    Color32::from_rgb(0xf0, 0xaa, 0x00), // Amarillo/naranja para Zig
                                    Color32::from_rgb(0xed, 0x8b, 0x00), // Naranja Java (Java orange)
                                    Color32::from_rgb(0xFF, 0x44, 0x00), // Naranja/Rojo para FastOS ASM+Rust+Zig
                                    Color32::from_rgb(0xac, 0x14, 0x2c), // Rojo Vulkan
                                    Color32::from_rgb(0x00, 0x7a, 0xcc), // Azul DirectX12
                                ];
                                
                                for (i, cat) in categories.iter().enumerate() {
                                    let selected = self.selected_category.as_deref() == Some(*cat);
                                    
                                    // Crear área interactiva primero
                                    let (rect, response) = ui.allocate_exact_size(
                                        egui::vec2(ui.available_width(), 38.0),
                                        egui::Sense::click()
                                    );
                                    
                                    let is_hovered = response.hovered();
                                    
                                    // Determinar colores según estado
                                    let bg_color = if selected {
                                        Color32::from_rgba_unmultiplied(category_colors[i].r(), category_colors[i].g(), category_colors[i].b(), 150)
                                    } else if is_hovered {
                                        Color32::from_rgba_unmultiplied(category_colors[i].r(), category_colors[i].g(), category_colors[i].b(), 70)
                                    } else {
                                        Color32::from_rgba_unmultiplied(40, 45, 55, 100)
                                    };
                                    let border_color = if selected {
                                        category_colors[i]
                                    } else if is_hovered {
                                        Color32::from_rgba_unmultiplied(category_colors[i].r(), category_colors[i].g(), category_colors[i].b(), 200)
                                    } else {
                                        Color32::from_rgb(70, 75, 85)
                                    };
                                    
                                    // Dibujar fondo y borde
                                    ui.painter().rect_filled(
                                        rect,
                                        5.0,
                                        bg_color
                                    );
                                    ui.painter().rect_stroke(
                                        rect,
                                        5.0,
                                        egui::Stroke::new(if selected { 2.5 } else { 1.0 }, border_color)
                                    );
                                    
                                    // Dibujar contenido
                                    let icon_pos = rect.left_top() + egui::vec2(12.0, 19.0);
                                    ui.painter().text(
                                        icon_pos,
                                        egui::Align2::LEFT_CENTER,
                                        category_icons[i],
                                        egui::FontId::proportional(18.0),
                                        category_colors[i]
                                    );
                                    
                                    let text_pos = rect.left_top() + egui::vec2(40.0, 19.0);
                                    ui.painter().text(
                                        text_pos,
                                        egui::Align2::LEFT_CENTER,
                                        *cat,
                                        egui::FontId::proportional(12.5),
                                        if selected {
                                            Color32::WHITE
                                        } else {
                                            Color32::from_rgb(220, 220, 230)
                                        }
                                    );
                                    
                                    if response.clicked() {
                                        self.selected_category = Some(cat.to_string());
                                    }
                                    
                                    ui.add_space(2.0);
                                }
                                
                                ui.add_space(8.0);
                                
                                // Separador visual
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(ui.available_width(), 1.0),
                                    egui::Sense::hover()
                                );
                                ui.painter().rect_filled(
                                    rect,
                                    0.0,
                                    Color32::from_rgba_unmultiplied(100, 150, 255, 60)
                                );
                                ui.add_space(6.0);
                                
                                // Shortcuts mejorados
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(30, 35, 45, 150))
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("🔍 Buscar").small().size(11.0).color(Color32::from_rgb(150, 180, 200)));
                                        ui.label(egui::RichText::new("F3").small().size(10.0).color(Color32::from_rgb(100, 200, 255)));
                                    });
                            });
                            
                            ui.separator();
                            
                            // Panel derecho: Subcategorías y templates
                            ui.vertical(|ui| {
                                ui.set_min_width(200.0);
                                
                                if let Some(ref cat) = self.selected_category.clone() {
                                    if cat == "new" {
                                        // Panel para crear nodo nuevo
                                        ui.heading("Crear Nodo");
                                        ui.add_space(8.0);
                                        
                                        ui.horizontal(|ui| {
                                            ui.label("Título:");
                                            ui.text_edit_singleline(&mut self.new_node_title);
                                        });
                                        
                                        ui.add_space(8.0);
                                        
                                        if ui.button("✨ Crear Nodo Vacío").clicked() {
                                            let title = if self.new_node_title.trim().is_empty() {
                                                format!("Nodo {}", self.graph.nodes().len() + 1)
                                            } else {
                                                self.new_node_title.trim().to_string()
                                            };
                                            
                                            let world_pos = self.viewport.screen_to_world(self.node_menu_pos, Rect::from_min_size(Pos2::ZERO, Vec2::new(10000.0, 10000.0)));
                                            let id = self.graph.add_node(&title, world_pos, Color32::from_rgb(100, 150, 200), &["Entrada"], &["Código"], NodeLanguage::Auto);
                                            
                                            if let Some(node) = self.graph.nodes().iter().find(|n| n.id == id) {
                                                let title_clone = node.title.clone();
                                                let code_clone = node.code.clone();
                                                use crate::expressions::ChannelValue;
                                                self.channel_manager.set_channel(title_clone.clone(), ChannelValue::Code(code_clone.clone()));
                                                self.channel_manager.set_channel(format!("{}/code", title_clone.clone()), ChannelValue::Code(code_clone.clone()));
                                                self.channel_manager.set_node_channel(id, "code".to_string(), ChannelValue::Code(code_clone));
                                            }
                                            
                                            self.new_node_title.clear();
                                            
                                            if self.workspace.has_root() {
                                                let _ = self.save_current_graph();
                                            }
                                            
                                            close_menu = true;
                                        }
                                    } else if cat == "FastOS ASM+Rust+Zig" {
                                        // Panel especial para FastOS ASM+Rust+Zig
                                        ui.heading(egui::RichText::new("🚀 FastOS ASM+Rust+Zig").color(Color32::from_rgb(0xFF, 0x44, 0x00)));
                                        ui.add_space(8.0);
                                        
                                        ui.label(egui::RichText::new("Sistema Operativo Multi-Lenguaje").color(Color32::GRAY));
                                        ui.add_space(8.0);
                                        
                                        // Mostrar templates de FastOS ASM+Rust+Zig
                                        let templates = crate::templates::all_templates();
                                        let filtered: Vec<_> = templates.iter()
                                            .filter(|t| t.category == "FastOS ASM+Rust+Zig")
                                            .collect();
                                        
                                        // Agrupar por subcategoría
                                        let mut subcats: Vec<&str> = filtered.iter()
                                            .map(|t| t.subcategory)
                                            .collect();
                                        subcats.sort();
                                        subcats.dedup();
                                        
                                        egui::ScrollArea::vertical()
                                            .max_height(400.0)
                                            .show(ui, |ui| {
                                                for subcat in subcats {
                                                    // Header de subcategoría mejorado
                                                    egui::Frame::none()
                                                        .fill(Color32::from_rgba_unmultiplied(50, 55, 65, 150))
                                                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(100, 150, 255)))
                                                        .rounding(egui::Rounding::same(4.0))
                                                        .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                                        .show(ui, |ui| {
                                                            ui.label(egui::RichText::new(subcat)
                                                                .strong()
                                                                .size(13.0)
                                                                .color(Color32::from_rgb(200, 210, 220)));
                                                        });
                                                    ui.add_space(6.0);
                                                    
                                                    // Templates con mejor diseño
                                                    for template in filtered.iter().filter(|t| t.subcategory == subcat) {
                                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                                        let btn_text = format!("{} {}", template.icon, template.name);
                                                        
                                                        let btn_response = egui::Frame::none()
                                                            .fill(Color32::from_rgba_unmultiplied(40, 45, 55, 150))
                                                            .stroke(egui::Stroke::new(1.5, color))
                                                            .rounding(egui::Rounding::same(5.0))
                                                            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    ui.label(egui::RichText::new(template.icon).size(16.0).color(color));
                                                                    ui.add_space(6.0);
                                                                    ui.label(egui::RichText::new(template.name)
                                                                        .size(12.0)
                                                                        .color(Color32::from_rgb(230, 230, 235)));
                                                                });
                                                            }).response;
                                                        
                                                        // Efecto hover visual mejorado
                                                        if btn_response.hovered() {
                                                            let hover_rect = btn_response.rect.expand(2.0);
                                                            ui.painter().rect_filled(
                                                                hover_rect,
                                                                3.0,
                                                                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 50)
                                                            );
                                                        }
                                                        
                                                        if btn_response.clicked() {
                                                            template_to_add = Some((*template).clone());
                                                            close_menu = true;
                                                        }
                                                        
                                                        ui.add_space(4.0);
                                                    }
                                                    
                                                    ui.add_space(10.0);
                                                }
                                            });
                                    } else if cat == "Vulkan" {
                                        // Panel especial para Vulkan mejorado
                                        let vulkan_color = Color32::from_rgb(0xac, 0x14, 0x2c);
                                        let vulkan_gold = Color32::from_rgb(0xff, 0xd7, 0x00);
                                        
                                        // Header mejorado de Vulkan
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(172, 20, 44, 30))
                                            .stroke(egui::Stroke::new(2.0, vulkan_color))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("🎮").size(24.0));
                                                    ui.add_space(8.0);
                                                    ui.vertical(|ui| {
                                                        ui.label(egui::RichText::new("Vulkan API")
                                                            .strong()
                                                            .size(18.0)
                                                            .color(vulkan_color));
                                                        ui.label(egui::RichText::new("Graphics API de bajo nivel")
                                                            .small()
                                                            .color(Color32::from_rgb(180, 180, 190)));
                                                    });
                                                });
                                            });
                                        ui.add_space(12.0);
                                        
                                        // Botón principal mejorado para crear proyecto completo
                                        let create_project_response = egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(
                                                vulkan_gold.r(),
                                                vulkan_gold.g(),
                                                vulkan_gold.b(),
                                                40
                                            ))
                                            .stroke(egui::Stroke::new(2.5, vulkan_gold))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(14.0, 12.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("🎮").size(20.0).color(vulkan_gold));
                                                    ui.add_space(10.0);
                                                    ui.vertical(|ui| {
                                                        ui.label(egui::RichText::new("Crear Proyecto Vulkan Completo")
                                                            .strong()
                                                            .size(14.0)
                                                            .color(vulkan_gold));
                                                        ui.label(egui::RichText::new("Incluye todos los componentes necesarios")
                                                            .small()
                                                            .color(Color32::from_rgb(200, 200, 180)));
                                                    });
                                                });
                                            }).response;
                                        
                                        if create_project_response.clicked() || create_project_response.hovered() {
                                            let hover_bg = if create_project_response.hovered() {
                                                Color32::from_rgba_unmultiplied(
                                                    vulkan_gold.r(),
                                                    vulkan_gold.g(),
                                                    vulkan_gold.b(),
                                                    80
                                                )
                                            } else {
                                                Color32::from_rgba_unmultiplied(
                                                    vulkan_gold.r(),
                                                    vulkan_gold.g(),
                                                    vulkan_gold.b(),
                                                    40
                                                )
                                            };
                                            ui.painter().rect_filled(
                                                create_project_response.rect.expand(2.0),
                                                8.0,
                                                hover_bg
                                            );
                                        }
                                        
                                        if create_project_response.clicked() {
                                            // Crear el proyecto Vulkan con todos los nodos
                                            self.graph = NodeGraph::create_vulkan_project();
                                            self.viewport.pan = Vec2::new(0.0, 0.0);
                                            self.viewport.zoom = 0.4; // Zoom out para ver todo
                                            
                                            if self.workspace.has_root() {
                                                let _ = self.save_current_graph();
                                            }
                                            
                                            close_menu = true;
                                        }
                                        
                                        ui.add_space(16.0);
                                        
                                        // Separador visual mejorado
                                        let (sep_rect, _) = ui.allocate_exact_size(
                                            egui::vec2(ui.available_width(), 1.0),
                                            egui::Sense::hover()
                                        );
                                        ui.painter().rect_filled(
                                            sep_rect,
                                            0.0,
                                            Color32::from_rgba_unmultiplied(
                                                vulkan_color.r(),
                                                vulkan_color.g(),
                                                vulkan_color.b(),
                                                60
                                            )
                                        );
                                        
                                        ui.add_space(12.0);
                                        
                                        // Header de componentes individuales mejorado
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(172, 20, 44, 25))
                                            .stroke(egui::Stroke::new(1.5, vulkan_color))
                                            .rounding(egui::Rounding::same(6.0))
                                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("📦").size(18.0).color(Color32::from_rgb(200, 150, 255)));
                                                    ui.add_space(10.0);
                                                    ui.label(egui::RichText::new("Componentes Individuales")
                                                        .strong()
                                                        .size(14.5)
                                                        .color(Color32::from_rgb(230, 230, 240)));
                                                });
                                            });
                                        ui.add_space(10.0);
                                        
                                        // Mostrar templates de Vulkan con organización mejorada
                                        let templates = crate::templates::all_templates();
                                        let filtered: Vec<_> = templates.iter()
                                            .filter(|t| t.category == "Vulkan")
                                            .collect();
                                        
                                        // Agrupar por subcategoría y ordenar lógicamente
                                        let mut subcats: Vec<&str> = filtered.iter()
                                            .map(|t| t.subcategory)
                                            .collect();
                                        subcats.dedup();
                                        
                                        // Ordenar subcategorías para Vulkan: Base, Inicialización, Pipeline, Recursos, Ejecución, Librerías, Renderizado, Build
                                        let vulkan_order = ["Base", "Inicialización", "Pipeline", "Recursos", "Ejecución", "Librerías", "Renderizado", "Build"];
                                        subcats.sort_by(|a, b| {
                                            let a_pos = vulkan_order.iter().position(|&x| x == *a).unwrap_or(999);
                                            let b_pos = vulkan_order.iter().position(|&x| x == *b).unwrap_or(999);
                                            if a_pos == b_pos {
                                                a.cmp(b)
                                            } else {
                                                a_pos.cmp(&b_pos)
                                            }
                                        });
                                        
                                        egui::ScrollArea::vertical()
                                            .max_height(400.0)
                                            .show(ui, |ui| {
                                                for (subcat_idx, subcat) in subcats.iter().enumerate() {
                                                    // Determinar color y estilo según tipo de subcategoría para Vulkan
                                                    let (subcat_icon, subcat_color, subcat_bg) = match *subcat {
                                                        "Base" => ("🔰", Color32::from_rgb(100, 200, 100), Color32::from_rgba_unmultiplied(50, 100, 50, 80)),
                                                        "Inicialización" => ("🔌", Color32::from_rgb(150, 150, 255), Color32::from_rgba_unmultiplied(50, 50, 100, 80)),
                                                        "Pipeline" => ("🔧", Color32::from_rgb(200, 100, 255), Color32::from_rgba_unmultiplied(100, 40, 100, 80)),
                                                        "Recursos" => ("📦", Color32::from_rgb(100, 200, 200), Color32::from_rgba_unmultiplied(40, 100, 100, 80)),
                                                        "Ejecución" => ("💻", Color32::from_rgb(255, 170, 80), Color32::from_rgba_unmultiplied(100, 60, 30, 80)),
                                                        "Librerías" => ("📚", Color32::from_rgb(200, 150, 80), Color32::from_rgba_unmultiplied(100, 60, 40, 80)),
                                                        "Renderizado" => ("🎨", Color32::from_rgb(80, 200, 255), Color32::from_rgba_unmultiplied(30, 80, 100, 80)),
                                                        "Build" => ("🔧", Color32::from_rgb(100, 150, 255), Color32::from_rgba_unmultiplied(40, 60, 100, 80)),
                                                        _ => ("📋", Color32::from_rgb(150, 150, 150), Color32::from_rgba_unmultiplied(50, 50, 50, 80)),
                                                    };
                                                    
                                                    // Header de subcategoría mejorado y destacado
                                                    let subcat_templates: Vec<_> = filtered.iter()
                                                        .filter(|t| t.subcategory == *subcat)
                                                        .collect();
                                                    
                                                    // Header de subcategoría con diseño más profesional
                                                    egui::Frame::none()
                                                        .fill(subcat_bg)
                                                        .stroke(egui::Stroke::new(2.5, subcat_color))
                                                        .rounding(egui::Rounding::same(8.0))
                                                        .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                                                        .show(ui, |ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.label(egui::RichText::new(subcat_icon).size(20.0).color(subcat_color));
                                                                ui.add_space(10.0);
                                                                ui.label(egui::RichText::new(*subcat)
                                                                    .strong()
                                                                    .size(15.0)
                                                                    .color(Color32::from_rgb(255, 255, 255)));
                                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                    ui.label(egui::RichText::new(format!("({})", subcat_templates.len()))
                                                                        .size(12.0)
                                                                        .color(Color32::from_rgb(200, 200, 220)));
                                                                });
                                                            });
                                                        });
                                                    ui.add_space(10.0);
                                                    
                                                    // Templates organizados con diseño profesional mejorado (Vulkan)
                                                    for template in subcat_templates.iter() {
                                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                                        let template_cloned = (**template).clone();
                                                        
                                                        // Crear área interactiva más grande y profesional
                                                        let (rect, response) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 44.0),
                                                            egui::Sense::click()
                                                        );
                                                        
                                                        let is_hovered = response.hovered();
                                                        
                                                        // Colores mejorados con gradientes sutiles
                                                        let template_bg = if is_hovered {
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.g() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.b() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                200
                                                            )
                                                        } else {
                                                            Color32::from_rgba_unmultiplied(38, 42, 52, 180)
                                                        };
                                                        
                                                        let template_border = if is_hovered {
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 13 / 10).min(255) as u8,
                                                                (color.g() as u32 * 13 / 10).min(255) as u8,
                                                                (color.b() as u32 * 13 / 10).min(255) as u8,
                                                                255
                                                            )
                                                        } else {
                                                            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 180)
                                                        };
                                                        
                                                        // Sombra profesional cuando está hover
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                rect.translate(egui::vec2(0.0, 2.0)),
                                                                8.0,
                                                                Color32::from_black_alpha(60)
                                                            );
                                                        }
                                                        
                                                        ui.painter().rect_filled(rect, 8.0, template_bg);
                                                        ui.painter().rect_stroke(
                                                            rect,
                                                            8.0,
                                                            egui::Stroke::new(if is_hovered { 2.5 } else { 1.5 }, template_border)
                                                        );
                                                        
                                                        let indicator_width = if is_hovered { 5.0 } else { 4.0 };
                                                        ui.painter().rect_filled(
                                                            egui::Rect::from_min_size(
                                                                rect.left_top(),
                                                                egui::vec2(indicator_width, rect.height())
                                                            ),
                                                            if is_hovered { 8.0 } else { 0.0 },
                                                            color
                                                        );
                                                        
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                egui::Rect::from_min_size(
                                                                    rect.left_top(),
                                                                    egui::vec2(indicator_width, rect.height() * 0.5)
                                                                ),
                                                                0.0,
                                                                Color32::from_rgba_unmultiplied(255, 255, 255, 40)
                                                            );
                                                        }
                                                        
                                                        let icon_size = if is_hovered { 20.0 } else { 18.0 };
                                                        let icon_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            icon_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.icon,
                                                            egui::FontId::proportional(icon_size),
                                                            color
                                                        );
                                                        
                                                        let text_pos = rect.left_top() + egui::vec2(54.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            text_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.name,
                                                            egui::FontId::proportional(if is_hovered { 13.5 } else { 13.0 }),
                                                            Color32::from_rgb(245, 245, 250)
                                                        );
                                                        
                                                        if is_hovered {
                                                            let glow_rect = egui::Rect::from_min_size(
                                                                rect.right_top() - egui::vec2(40.0, 0.0),
                                                                egui::vec2(40.0, 20.0)
                                                            );
                                                            ui.painter().rect_filled(
                                                                glow_rect,
                                                                4.0,
                                                                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 30)
                                                            );
                                                            
                                                            let click_hint_pos = rect.right_top() - egui::vec2(8.0, rect.height() / 2.0);
                                                            ui.painter().text(
                                                                click_hint_pos,
                                                                egui::Align2::RIGHT_CENTER,
                                                                "→",
                                                                egui::FontId::proportional(16.0),
                                                                Color32::from_rgba_unmultiplied(200, 200, 200, 180)
                                                            );
                                                        }
                                                        
                                                        if response.clicked() {
                                                            template_to_add = Some(template_cloned);
                                                            close_menu = true;
                                                        }
                                                        
                                                        ui.add_space(6.0);
                                                    }
                                                    
                                                    // Separador entre subcategorías (excepto la última)
                                                    if subcat_idx < subcats.len() - 1 {
                                                        ui.add_space(12.0);
                                                        let (sep_rect, _) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 1.0),
                                                            egui::Sense::hover()
                                                        );
                                                        ui.painter().rect_filled(
                                                            sep_rect,
                                                            0.0,
                                                            Color32::from_rgba_unmultiplied(
                                                                vulkan_color.r(),
                                                                vulkan_color.g(),
                                                                vulkan_color.b(),
                                                                40
                                                            )
                                                        );
                                                        ui.add_space(12.0);
                                                    } else {
                                                        ui.add_space(4.0);
                                                    }
                                                }
                                                
                                                // Si no hay templates, mostrar mensaje
                                                if filtered.is_empty() {
                                                    ui.centered_and_justified(|ui| {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgba_unmultiplied(50, 50, 50, 100))
                                                            .rounding(egui::Rounding::same(6.0))
                                                            .inner_margin(egui::Margin::symmetric(20.0, 15.0))
                                                            .show(ui, |ui| {
                                                                ui.label(egui::RichText::new("📭 No hay componentes disponibles")
                                                                    .size(13.0)
                                                                    .color(Color32::GRAY));
                                                            });
                                                    });
                                                }
                                            });
                                    } else if cat == "DirectX12" {
                                        // Panel especial para DirectX12 (similar a Vulkan)
                                        let dx12_color = Color32::from_rgb(0x00, 0x7a, 0xcc);
                                        let dx12_gold = Color32::from_rgb(0xff, 0xd7, 0x00);
                                        
                                        // Header mejorado de DirectX12
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(0, 122, 204, 30))
                                            .stroke(egui::Stroke::new(2.0, dx12_color))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("💎").size(24.0));
                                                    ui.add_space(8.0);
                                                    ui.vertical(|ui| {
                                                        ui.label(egui::RichText::new("DirectX 12 API")
                                                            .strong()
                                                            .size(18.0)
                                                            .color(dx12_color));
                                                        ui.label(egui::RichText::new("Graphics API de Microsoft para Windows")
                                                            .small()
                                                            .color(Color32::from_rgb(180, 180, 190)));
                                                    });
                                                });
                                            });
                                        ui.add_space(12.0);
                                        
                                        // Botón principal para crear proyecto completo
                                        let create_project_response = egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(
                                                dx12_gold.r(),
                                                dx12_gold.g(),
                                                dx12_gold.b(),
                                                40
                                            ))
                                            .stroke(egui::Stroke::new(2.5, dx12_gold))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(14.0, 12.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("💎").size(20.0).color(dx12_gold));
                                                    ui.add_space(10.0);
                                                    ui.vertical(|ui| {
                                                        ui.label(egui::RichText::new("Crear Proyecto DirectX12 Completo")
                                                            .strong()
                                                            .size(14.0)
                                                            .color(dx12_gold));
                                                        ui.label(egui::RichText::new("Incluye todos los componentes necesarios")
                                                            .small()
                                                            .color(Color32::from_rgb(200, 200, 180)));
                                                    });
                                                });
                                            }).response;
                                        
                                        if create_project_response.clicked() || create_project_response.hovered() {
                                            let hover_bg = if create_project_response.hovered() {
                                                Color32::from_rgba_unmultiplied(
                                                    dx12_gold.r(),
                                                    dx12_gold.g(),
                                                    dx12_gold.b(),
                                                    80
                                                )
                                            } else {
                                                Color32::from_rgba_unmultiplied(
                                                    dx12_gold.r(),
                                                    dx12_gold.g(),
                                                    dx12_gold.b(),
                                                    40
                                                )
                                            };
                                            ui.painter().rect_filled(
                                                create_project_response.rect.expand(2.0),
                                                8.0,
                                                hover_bg
                                            );
                                        }
                                        
                                        if create_project_response.clicked() {
                                            // TODO: Crear el proyecto DirectX12 con todos los nodos
                                            // self.graph = NodeGraph::create_directx12_project();
                                            close_menu = true;
                                        }
                                        
                                        ui.add_space(16.0);
                                        
                                        // Separador visual
                                        let (sep_rect, _) = ui.allocate_exact_size(
                                            egui::vec2(ui.available_width(), 1.0),
                                            egui::Sense::hover()
                                        );
                                        ui.painter().rect_filled(
                                            sep_rect,
                                            0.0,
                                            Color32::from_rgba_unmultiplied(
                                                dx12_color.r(),
                                                dx12_color.g(),
                                                dx12_color.b(),
                                                60
                                            )
                                        );
                                        
                                        ui.add_space(12.0);
                                        
                                        // Header de componentes individuales mejorado
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(0, 122, 204, 25))
                                            .stroke(egui::Stroke::new(1.5, Color32::from_rgb(0, 122, 204)))
                                            .rounding(egui::Rounding::same(6.0))
                                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("📦").size(18.0).color(Color32::from_rgb(150, 200, 255)));
                                                    ui.add_space(10.0);
                                                    ui.label(egui::RichText::new("Componentes Individuales")
                                                        .strong()
                                                        .size(14.5)
                                                        .color(Color32::from_rgb(230, 230, 240)));
                                                });
                                            });
                                        ui.add_space(10.0);
                                        
                                        // Mostrar templates de DirectX12 con organización mejorada
                                        let templates = crate::templates::all_templates();
                                        let filtered: Vec<_> = templates.iter()
                                            .filter(|t| t.category == "DirectX12")
                                            .collect();
                                        
                                        // Agrupar por subcategoría y ordenar
                                        let mut subcats: Vec<&str> = filtered.iter()
                                            .map(|t| t.subcategory)
                                            .collect();
                                        subcats.dedup();
                                        
                                        let dx12_order = ["Base", "Inicialización", "Comandos", "Pipeline", "Recursos", "Librerías", "Renderizado", "Build"];
                                        subcats.sort_by(|a, b| {
                                            let a_pos = dx12_order.iter().position(|&x| x == *a).unwrap_or(999);
                                            let b_pos = dx12_order.iter().position(|&x| x == *b).unwrap_or(999);
                                            if a_pos == b_pos {
                                                a.cmp(b)
                                            } else {
                                                a_pos.cmp(&b_pos)
                                            }
                                        });
                                        
                                        egui::ScrollArea::vertical()
                                            .max_height(400.0)
                                            .show(ui, |ui| {
                                                for (subcat_idx, subcat) in subcats.iter().enumerate() {
                                                    let (subcat_icon, subcat_color, subcat_bg) = match *subcat {
                                                        "Base" => ("🔰", Color32::from_rgb(100, 200, 100), Color32::from_rgba_unmultiplied(50, 100, 50, 80)),
                                                        "Inicialización" => ("🔌", Color32::from_rgb(150, 150, 255), Color32::from_rgba_unmultiplied(50, 50, 100, 80)),
                                                        "Comandos" => ("⚙️", Color32::from_rgb(100, 200, 255), Color32::from_rgba_unmultiplied(40, 60, 100, 80)),
                                                        "Pipeline" => ("🔧", Color32::from_rgb(200, 100, 255), Color32::from_rgba_unmultiplied(100, 40, 100, 80)),
                                                        "Recursos" => ("📦", Color32::from_rgb(100, 200, 200), Color32::from_rgba_unmultiplied(40, 100, 100, 80)),
                                                        "Librerías" => ("📚", Color32::from_rgb(200, 150, 80), Color32::from_rgba_unmultiplied(100, 60, 40, 80)),
                                                        "Renderizado" => ("🎨", Color32::from_rgb(80, 200, 255), Color32::from_rgba_unmultiplied(30, 80, 100, 80)),
                                                        "Build" => ("🔧", Color32::from_rgb(100, 150, 255), Color32::from_rgba_unmultiplied(40, 60, 100, 80)),
                                                        _ => ("📋", Color32::from_rgb(150, 150, 150), Color32::from_rgba_unmultiplied(50, 50, 50, 80)),
                                                    };
                                                    
                                                    let subcat_templates: Vec<_> = filtered.iter()
                                                        .filter(|t| t.subcategory == *subcat)
                                                        .collect();
                                                    
                                                    // Header de subcategoría con diseño más profesional
                                                    egui::Frame::none()
                                                        .fill(subcat_bg)
                                                        .stroke(egui::Stroke::new(2.5, subcat_color))
                                                        .rounding(egui::Rounding::same(8.0))
                                                        .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                                                        .show(ui, |ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.label(egui::RichText::new(subcat_icon).size(20.0).color(subcat_color));
                                                                ui.add_space(10.0);
                                                                ui.label(egui::RichText::new(*subcat)
                                                                    .strong()
                                                                    .size(15.0)
                                                                    .color(Color32::from_rgb(255, 255, 255)));
                                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                    ui.label(egui::RichText::new(format!("({})", subcat_templates.len()))
                                                                        .size(12.0)
                                                                        .color(Color32::from_rgb(200, 200, 220)));
                                                                });
                                                            });
                                                        });
                                                    ui.add_space(10.0);
                                                    
                                                    // Templates organizados con diseño profesional mejorado (DirectX12)
                                                    for template in subcat_templates.iter() {
                                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                                        let template_cloned = (**template).clone();
                                                        
                                                        let (rect, response) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 44.0),
                                                            egui::Sense::click()
                                                        );
                                                        
                                                        let is_hovered = response.hovered();
                                                        
                                                        let template_bg = if is_hovered {
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.g() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.b() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                200
                                                            )
                                                        } else {
                                                            Color32::from_rgba_unmultiplied(38, 42, 52, 180)
                                                        };
                                                        
                                                        let template_border = if is_hovered {
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 13 / 10).min(255) as u8,
                                                                (color.g() as u32 * 13 / 10).min(255) as u8,
                                                                (color.b() as u32 * 13 / 10).min(255) as u8,
                                                                255
                                                            )
                                                        } else {
                                                            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 180)
                                                        };
                                                        
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                rect.translate(egui::vec2(0.0, 2.0)),
                                                                8.0,
                                                                Color32::from_black_alpha(60)
                                                            );
                                                        }
                                                        
                                                        ui.painter().rect_filled(rect, 8.0, template_bg);
                                                        ui.painter().rect_stroke(
                                                            rect,
                                                            8.0,
                                                            egui::Stroke::new(if is_hovered { 2.5 } else { 1.5 }, template_border)
                                                        );
                                                        
                                                        let indicator_width = if is_hovered { 5.0 } else { 4.0 };
                                                        ui.painter().rect_filled(
                                                            egui::Rect::from_min_size(rect.left_top(), egui::vec2(indicator_width, rect.height())),
                                                            if is_hovered { 8.0 } else { 0.0 },
                                                            color
                                                        );
                                                        
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                egui::Rect::from_min_size(
                                                                    rect.left_top(),
                                                                    egui::vec2(indicator_width, rect.height() * 0.5)
                                                                ),
                                                                0.0,
                                                                Color32::from_rgba_unmultiplied(255, 255, 255, 40)
                                                            );
                                                        }
                                                        
                                                        let icon_size = if is_hovered { 20.0 } else { 18.0 };
                                                        let icon_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            icon_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.icon,
                                                            egui::FontId::proportional(icon_size),
                                                            color
                                                        );
                                                        
                                                        let text_pos = rect.left_top() + egui::vec2(54.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            text_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.name,
                                                            egui::FontId::proportional(if is_hovered { 13.5 } else { 13.0 }),
                                                            Color32::from_rgb(245, 245, 250)
                                                        );
                                                        
                                                        if is_hovered {
                                                            let glow_rect = egui::Rect::from_min_size(
                                                                rect.right_top() - egui::vec2(40.0, 0.0),
                                                                egui::vec2(40.0, 20.0)
                                                            );
                                                            ui.painter().rect_filled(
                                                                glow_rect,
                                                                4.0,
                                                                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 30)
                                                            );
                                                            
                                                            let click_hint_pos = rect.right_top() - egui::vec2(8.0, rect.height() / 2.0);
                                                            ui.painter().text(
                                                                click_hint_pos,
                                                                egui::Align2::RIGHT_CENTER,
                                                                "→",
                                                                egui::FontId::proportional(16.0),
                                                                Color32::from_rgba_unmultiplied(200, 200, 200, 180)
                                                            );
                                                        }
                                                        
                                                        if response.clicked() {
                                                            template_to_add = Some(template_cloned);
                                                            close_menu = true;
                                                        }
                                                        
                                                        ui.add_space(6.0);
                                                    }
                                                    
                                                    if subcat_idx < subcats.len() - 1 {
                                                        ui.add_space(12.0);
                                                        let (sep_rect, _) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 1.0),
                                                            egui::Sense::hover()
                                                        );
                                                        ui.painter().rect_filled(
                                                            sep_rect,
                                                            0.0,
                                                            Color32::from_rgba_unmultiplied(
                                                                dx12_color.r(),
                                                                dx12_color.g(),
                                                                dx12_color.b(),
                                                                40
                                                            )
                                                        );
                                                        ui.add_space(12.0);
                                                    } else {
                                                        ui.add_space(4.0);
                                                    }
                                                }
                                            });
                                    } else {
                                        // Mostrar templates de la categoría seleccionada con organización mejorada
                                        let templates = crate::templates::all_templates();
                                        let filtered: Vec<_> = templates.iter()
                                            .filter(|t| t.category == cat.as_str())
                                            .collect();
                                        
                                        // Agrupar por subcategoría y ordenar lógicamente
                                        let mut subcats: Vec<&str> = filtered.iter()
                                            .map(|t| t.subcategory)
                                            .collect();
                                        subcats.dedup();
                                        
                                        // Ordenar subcategorías de manera lógica: Básico -> Intermedio -> Avanzado -> Librerías -> Otros
                                        let order = ["Básico", "Intermedio", "Avanzado", "Librerías"];
                                        subcats.sort_by(|a, b| {
                                            let a_pos = order.iter().position(|&x| x == *a).unwrap_or(999);
                                            let b_pos = order.iter().position(|&x| x == *b).unwrap_or(999);
                                            if a_pos == b_pos {
                                                a.cmp(b) // Si no están en la lista, orden alfabético
                                            } else {
                                                a_pos.cmp(&b_pos)
                                            }
                                        });
                                        
                                        // Header de categoría mejorado y profesional
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(40, 45, 55, 220))
                                            .stroke(egui::Stroke::new(2.0, Color32::from_rgb(100, 150, 255)))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("📚").size(24.0).color(Color32::from_rgb(150, 200, 255)));
                                                    ui.add_space(12.0);
                                                    ui.label(egui::RichText::new(cat.as_str())
                                                        .strong()
                                                        .size(18.0)
                                                        .color(Color32::from_rgb(240, 240, 245)));
                                                });
                                            });
                                        ui.add_space(12.0);
                                        
                                        egui::ScrollArea::vertical()
                                            .max_height(450.0)
                                            .show(ui, |ui| {
                                                for (subcat_idx, subcat) in subcats.iter().enumerate() {
                                                    // Determinar color y estilo según tipo de subcategoría
                                                    let (subcat_icon, subcat_color, subcat_bg) = match *subcat {
                                                        "Básico" => ("🔰", Color32::from_rgb(100, 200, 100), Color32::from_rgba_unmultiplied(50, 100, 50, 80)),
                                                        "Intermedio" => ("⚡", Color32::from_rgb(200, 180, 80), Color32::from_rgba_unmultiplied(100, 80, 40, 80)),
                                                        "Avanzado" => ("🔥", Color32::from_rgb(200, 80, 80), Color32::from_rgba_unmultiplied(100, 40, 40, 80)),
                                                        "Librerías" => ("📦", Color32::from_rgb(100, 150, 255), Color32::from_rgba_unmultiplied(40, 60, 100, 80)),
                                                        _ => ("📋", Color32::from_rgb(150, 150, 150), Color32::from_rgba_unmultiplied(50, 50, 50, 80)),
                                                    };
                                                    
                                                    // Header de subcategoría mejorado y destacado
                                                    let subcat_templates: Vec<_> = filtered.iter()
                                                        .filter(|t| t.subcategory == *subcat)
                                                        .collect();
                                                    
                                                    // Header de subcategoría con diseño más profesional
                                                    egui::Frame::none()
                                                        .fill(subcat_bg)
                                                        .stroke(egui::Stroke::new(2.5, subcat_color))
                                                        .rounding(egui::Rounding::same(8.0))
                                                        .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                                                        .show(ui, |ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.label(egui::RichText::new(subcat_icon).size(20.0).color(subcat_color));
                                                                ui.add_space(10.0);
                                                                ui.label(egui::RichText::new(*subcat)
                                                                    .strong()
                                                                    .size(15.0)
                                                                    .color(Color32::from_rgb(255, 255, 255)));
                                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                    ui.label(egui::RichText::new(format!("({})", subcat_templates.len()))
                                                                        .size(12.0)
                                                                        .color(Color32::from_rgb(200, 200, 220)));
                                                                });
                                                            });
                                                        });
                                                    ui.add_space(10.0);
                                                    
                                                    // Templates organizados con diseño profesional mejorado
                                                    for (template_idx, template) in subcat_templates.iter().enumerate() {
                                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                                        let template_cloned = (**template).clone();
                                                        
                                                        // Crear área interactiva más grande y profesional
                                                        let (rect, response) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 44.0),
                                                            egui::Sense::click()
                                                        );
                                                        
                                                        let is_hovered = response.hovered();
                                                        
                                                        // Colores mejorados con gradientes sutiles
                                                        let template_bg = if is_hovered {
                                                            // Hover: más brillante y translúcido
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.g() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                (color.b() as u32 * 12 / 255 + 60).min(255) as u8,
                                                                200
                                                            )
                                                        } else {
                                                            // Normal: fondo oscuro elegante
                                                            Color32::from_rgba_unmultiplied(38, 42, 52, 180)
                                                        };
                                                        
                                                        let template_border = if is_hovered {
                                                            // Hover: borde más brillante y más grueso
                                                            Color32::from_rgba_unmultiplied(
                                                                (color.r() as u32 * 13 / 10).min(255) as u8,
                                                                (color.g() as u32 * 13 / 10).min(255) as u8,
                                                                (color.b() as u32 * 13 / 10).min(255) as u8,
                                                                255
                                                            )
                                                        } else {
                                                            // Normal: borde con color del template pero más sutil
                                                            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 180)
                                                        };
                                                        
                                                        // Sombra profesional cuando está hover
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                rect.translate(egui::vec2(0.0, 2.0)),
                                                                8.0,
                                                                Color32::from_black_alpha(60)
                                                            );
                                                        }
                                                        
                                                        // Dibujar fondo principal con bordes redondeados
                                                        ui.painter().rect_filled(
                                                            rect,
                                                            8.0,
                                                            template_bg
                                                        );
                                                        
                                                        // Borde profesional más grueso en hover
                                                        ui.painter().rect_stroke(
                                                            rect,
                                                            8.0,
                                                            egui::Stroke::new(if is_hovered { 2.5 } else { 1.5 }, template_border)
                                                        );
                                                        
                                                        // Indicador lateral izquierdo más grueso y elegante
                                                        let indicator_width = if is_hovered { 5.0 } else { 4.0 };
                                                        ui.painter().rect_filled(
                                                            egui::Rect::from_min_size(
                                                                rect.left_top(),
                                                                egui::vec2(indicator_width, rect.height())
                                                            ),
                                                            if is_hovered { 8.0 } else { 0.0 },
                                                            color
                                                        );
                                                        
                                                        // Efecto de brillo sutil en el indicador cuando hover
                                                        if is_hovered {
                                                            ui.painter().rect_filled(
                                                                egui::Rect::from_min_size(
                                                                    rect.left_top(),
                                                                    egui::vec2(indicator_width, rect.height() * 0.5)
                                                                ),
                                                                0.0,
                                                                Color32::from_rgba_unmultiplied(255, 255, 255, 40)
                                                            );
                                                        }
                                                        
                                                        // Dibujar contenido con mejor tipografía
                                                        let icon_size = if is_hovered { 20.0 } else { 18.0 };
                                                        let icon_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            icon_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.icon,
                                                            egui::FontId::proportional(icon_size),
                                                            color
                                                        );
                                                        
                                                        // Texto del template con mejor tipografía y spacing
                                                        let text_pos = rect.left_top() + egui::vec2(54.0, rect.height() / 2.0);
                                                        ui.painter().text(
                                                            text_pos,
                                                            egui::Align2::LEFT_CENTER,
                                                            template.name,
                                                            egui::FontId::proportional(if is_hovered { 13.5 } else { 13.0 }),
                                                            Color32::from_rgb(245, 245, 250)
                                                        );
                                                        
                                                        // Indicador de hover adicional (brillo en la esquina)
                                                        if is_hovered {
                                                            // Brillo sutil en la esquina superior derecha
                                                            let glow_rect = egui::Rect::from_min_size(
                                                                rect.right_top() - egui::vec2(40.0, 0.0),
                                                                egui::vec2(40.0, 20.0)
                                                            );
                                                            ui.painter().rect_filled(
                                                                glow_rect,
                                                                4.0,
                                                                Color32::from_rgba_unmultiplied(
                                                                    color.r(),
                                                                    color.g(),
                                                                    color.b(),
                                                                    30
                                                                )
                                                            );
                                                            
                                                            // Indicador "clic para agregar" sutil
                                                            let click_hint_pos = rect.right_top() - egui::vec2(8.0, rect.height() / 2.0);
                                                            ui.painter().text(
                                                                click_hint_pos,
                                                                egui::Align2::RIGHT_CENTER,
                                                                "→",
                                                                egui::FontId::proportional(16.0),
                                                                Color32::from_rgba_unmultiplied(200, 200, 200, 180)
                                                            );
                                                        }
                                                        
                                                        if response.clicked() {
                                                            template_to_add = Some(template_cloned);
                                                            close_menu = true;
                                                        }
                                                        
                                                        ui.add_space(6.0);
                                                    }
                                                    
                                                    // Separador entre subcategorías (excepto la última)
                                                    if subcat_idx < subcats.len() - 1 {
                                                        ui.add_space(12.0);
                                                        let (sep_rect, _) = ui.allocate_exact_size(
                                                            egui::vec2(ui.available_width(), 1.0),
                                                            egui::Sense::hover()
                                                        );
                                                        ui.painter().rect_filled(
                                                            sep_rect,
                                                            0.0,
                                                            Color32::from_rgba_unmultiplied(100, 150, 255, 40)
                                                        );
                                                        ui.add_space(12.0);
                                                    } else {
                                                        ui.add_space(4.0);
                                                    }
                                                }
                                                
                                                // Si no hay templates, mostrar mensaje
                                                if filtered.is_empty() {
                                                    ui.centered_and_justified(|ui| {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgba_unmultiplied(50, 50, 50, 100))
                                                            .rounding(egui::Rounding::same(6.0))
                                                            .inner_margin(egui::Margin::symmetric(20.0, 15.0))
                                                            .show(ui, |ui| {
                                                                ui.label(egui::RichText::new("📭 No hay templates disponibles")
                                                                    .size(13.0)
                                                                    .color(Color32::GRAY));
                                                            });
                                                    });
                                                }
                                            });
                                    }
                                } else {
                                    // Sin categoría seleccionada - mostrar mensaje mejorado
                                    ui.centered_and_justified(|ui| {
                                        egui::Frame::none()
                                            .fill(Color32::from_rgba_unmultiplied(50, 50, 50, 100))
                                            .rounding(egui::Rounding::same(8.0))
                                            .inner_margin(egui::Margin::symmetric(20.0, 15.0))
                                            .show(ui, |ui| {
                                                ui.vertical_centered(|ui| {
                                                    ui.label(egui::RichText::new("👆").size(32.0));
                                                    ui.add_space(8.0);
                                                    ui.label(egui::RichText::new("Selecciona una categoría")
                                                        .size(14.0)
                                                        .strong()
                                                        .color(Color32::from_rgb(200, 200, 210)));
                                                    ui.add_space(4.0);
                                                    ui.label(egui::RichText::new("para ver los templates disponibles")
                                                        .size(12.0)
                                                        .color(Color32::GRAY));
                                                });
                                            });
                                    });
                                }
                            });
                        });
                    });
                });
            
            // Agregar template si se seleccionó uno
            if let Some(template) = template_to_add {
                let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                let title = format!("{}: {}", template.category, template.name);
                self.add_template_node(ctx, &title, template.code, color, template.language);
            }
                
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_menu = true;
            }
            
            if close_menu {
                self.show_node_menu = false;
                self.selected_category = None;
            }
        }
        
        // ═══════════════════════════════════════════════════════════════
        // 🆕 DIÁLOGOS DE HDA (Houdini Digital Assets)
        // ═══════════════════════════════════════════════════════════════
        self.hda_export_dialog_ui(ctx);
        self.hda_import_dialog_ui(ctx);
        
        // ═══════════════════════════════════════════════════════════════
        // MENÚ F3 - Búsqueda rápida estilo Blender
        // ═══════════════════════════════════════════════════════════════
        if self.show_search_menu {
            let mut close_menu = false;
            let mut template_to_add: Option<crate::templates::Template> = None;
            
            egui::Area::new("search_menu_area".into())
                .fixed_pos(self.node_menu_pos)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    let frame = egui::Frame::window(ui.style())
                        .shadow(eframe::egui::epaint::Shadow {
                            offset: Vec2::new(4.0, 8.0),
                            blur: 12.0,
                            spread: 0.0,
                            color: Color32::from_black_alpha(120),
                        })
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(8.0));
                        
                    frame.show(ui, |ui| {
                        ui.set_width(350.0);
                        
                        // Barra de búsqueda
                        ui.horizontal(|ui| {
                            ui.label("🔍");
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.search_query)
                                    .desired_width(300.0)
                                    .hint_text("Buscar templates...")
                            );
                            response.request_focus();
                        });
                        
                        ui.separator();
                        
                        // Resultados de búsqueda
                        let templates = crate::templates::all_templates();
                        let query_lower = self.search_query.to_lowercase();
                        
                        let filtered: Vec<_> = if query_lower.is_empty() {
                            templates.iter().take(15).collect()
                        } else {
                            templates.iter()
                                .filter(|t| {
                                    t.name.to_lowercase().contains(&query_lower) ||
                                    t.category.to_lowercase().contains(&query_lower) ||
                                    t.subcategory.to_lowercase().contains(&query_lower)
                                })
                                .collect()
                        };
                        
                        egui::ScrollArea::vertical()
                            .max_height(300.0)
                            .show(ui, |ui| {
                                if filtered.is_empty() {
                                    ui.label(egui::RichText::new("No se encontraron resultados").color(Color32::GRAY));
                                } else {
                                    for template in filtered {
                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                        
                                        ui.horizontal(|ui| {
                                            // Etiqueta de categoría
                                            ui.label(
                                                egui::RichText::new(format!("Add ({}) • {} •", template.category, template.subcategory))
                                                    .small()
                                                    .color(Color32::GRAY)
                                            );
                                            
                                            // Nombre del template
                                            let btn_text = format!("{} {}", template.icon, template.name);
                                            if ui.add(egui::Button::new(
                                                egui::RichText::new(&btn_text).color(color)
                                            )).clicked() {
                                                template_to_add = Some(template.clone());
                                                close_menu = true;
                                            }
                                        });
                                    }
                                }
                            });
                    });
                });
            
            // Agregar template si se seleccionó uno
            if let Some(template) = template_to_add {
                let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                let title = format!("{}: {}", template.category, template.name);
                self.add_template_node(ctx, &title, template.code, color, template.language);
            }
                
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_menu = true;
            }
            
            if close_menu {
                self.show_search_menu = false;
                self.search_query.clear();
            }
        }
    }

    // Removed toolbar_ui, but keeping canvas_ui signature for now
    // The layout is now managed by central panel block in update()
    
    fn canvas_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                // ═══════════════════════════════════════════════════════════════════
                // 🆕 BREADCRUMBS Y BOTÓN SUBIR (SUBNETWORKS)
                // ═══════════════════════════════════════════════════════════════════
                // Solo mostrar breadcrumbs si NO estamos en root (hay más de un nivel)
                if !self.is_at_root() {
                    let breadcrumbs = self.get_breadcrumbs();
                    if !breadcrumbs.is_empty() && breadcrumbs.len() > 1 {
                        egui::Area::new(egui::Id::new("subnetwork_breadcrumbs"))
                            .fixed_pos(egui::pos2(10.0, 10.0))
                            .order(egui::Order::Foreground)
                            .show(ctx, |ui| {
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(20, 20, 20, 200))
                                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(100, 100, 100)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            // Botón "Subir" siempre visible cuando no estamos en root
                                            if ui.button(egui::RichText::new("⬆ Subir").color(Color32::from_rgb(150, 200, 255))).clicked() {
                                                if let Err(e) = self.exit_subnetwork() {
                                                    eprintln!("Error exiting subnetwork: {}", e);
                                                }
                                            }
                                            ui.separator();
                                            
                                            // Breadcrumbs (siempre incluye Root como primer elemento)
                                            for (i, crumb) in breadcrumbs.iter().enumerate() {
                                                if i > 0 {
                                                    ui.label(egui::RichText::new(" > ").color(Color32::from_gray(100)));
                                                }
                                                
                                                let is_last = i == breadcrumbs.len() - 1;
                                                let crumb_text_clone = crumb.clone();
                                                let crumb_text = if is_last {
                                                    egui::RichText::new(&crumb_text_clone).strong().color(Color32::from_rgb(200, 200, 200))
                                                } else {
                                                    egui::RichText::new(&crumb_text_clone).color(Color32::from_rgb(150, 150, 150))
                                                };
                                                
                                                if !is_last && ui.button(crumb_text.clone()).clicked() {
                                                    // Saltar a ese nivel (implementar navegación a nivel específico)
                                                    // Por ahora, simplemente salir hasta llegar al nivel correcto
                                                    while self.network_levels.len() > i + 1 {
                                                        if let Err(e) = self.exit_subnetwork() {
                                                            eprintln!("Error navigating to level: {}", e);
                                                            break;
                                                        }
                                                    }
                                                } else {
                                                    ui.label(crumb_text);
                                                }
                                            }
                                        });
                                    });
                            });
                    }
                }

                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
                let rect = response.rect;

                // Verificar estado de Y en tiempo real para forzar modo de corte
                let y_key_down = ui.input(|i| i.key_down(egui::Key::Y));
                
                // Si Y NO está presionado y el modo de corte está activo, desactivarlo
                if !y_key_down && self.interaction.cut_tool.active {
                    // Solo desactivar si no hay puntos de corte en progreso
                    if self.interaction.cut_tool.is_empty() {
                        self.interaction.cut_tool.active = false;
                    }
                }
                
                // Si Y está presionado pero el modo no está activo, activarlo inmediatamente
                if y_key_down && !self.interaction.cut_tool.active {
                    self.interaction.cut_tool.active = true;
                    // Limpiar cualquier estado de selección/drag en curso
                    self.interaction.dragging_node = None;
                    self.interaction.box_selection_start = None;
                    self.interaction.box_selection_current = None;
                    // NO limpiar selected_nodes aquí para mantener selección existente
                }

                let input = ui.input(|i| PointerSnapshot {
                    pos: i.pointer.interact_pos(),
                    delta: i.pointer.delta(),
                    primary_pressed: i.pointer.button_pressed(PointerButton::Primary),
                    primary_down: i.pointer.button_down(PointerButton::Primary),
                    secondary_pressed: i.pointer.button_pressed(PointerButton::Secondary),
                    middle_down: i.pointer.button_down(PointerButton::Middle),
                    scroll_delta: i.smooth_scroll_delta.y,
                    ctrl_scroll: if i.modifiers.ctrl {
                        i.smooth_scroll_delta.y
                    } else {
                        0.0
                    },
                    modifiers: i.modifiers,
                });

                if response.hovered() {
                    // Prefer ctrl_scroll if held, otherwise normal scroll for dynamic zoom
                    let zoom_delta = if input.ctrl_scroll != 0.0 { input.ctrl_scroll } else { input.scroll_delta };
                    self.viewport.pan_zoom(rect, zoom_delta, if input.middle_down { input.delta } else { Vec2::ZERO }, input.pos);
                }

                self.paint_grid(&painter, rect, ui.visuals());
                self.paint_links(&painter, rect, ui.ctx().input(|i| i.time));
                self.paint_nodes(&painter, rect, ui.visuals());
                
                // Manejar herramientas en orden de prioridad
                let time = ui.ctx().input(|i| i.time);
                self.draw_connecting_line(&painter, rect, input.pos, time);
                
                // PRIORIDAD ABSOLUTA: Verificar Y en tiempo real y modo de corte
                // Si Y está presionado O el modo de corte está activo, forzar modo de corte
                let force_cut_mode = y_key_down || self.interaction.cut_tool.active;
                
                if force_cut_mode {
                    // Asegurar que el modo de corte esté activo
                    if !self.interaction.cut_tool.active {
                        self.interaction.cut_tool.active = true;
                    }
                    
                    // Limpiar estado de selección/drag para prevenir interferencias
                    self.interaction.dragging_node = None;
                    self.interaction.box_selection_start = None;
                    self.interaction.box_selection_current = None;
                    // Limpiar selected_nodes solo cuando se hace click en modo de corte
                    if input.primary_pressed {
                        self.interaction.selected_nodes.clear();
                    }
                    
                    // Manejar solo el modo de corte
                    self.handle_cut_tool(&painter, &input, rect);
                } else {
                    // Solo manejar drag/selección si el modo de corte NO está activo
                    // NO limpiar selected_nodes aquí - permitir selección normal
                    self.draw_box_selection(&painter);
                    self.handle_node_dragging(&input, rect);
                }

                // Request repaint for animations (connectors pulse)
                ctx.request_repaint();
            });
    }

    fn editor_ui(&mut self, ctx: &egui::Context) {
        let mut open = self.interaction.editing_node.is_some();
        let node_id = self.interaction.editing_node;

        // Inicializar historial si se abre un nuevo editor
        if let Some(id) = node_id {
            if self.interaction.editor_history.is_none() || 
               self.interaction.editor_history.as_ref().map(|h| h.node_id) != Some(id) {
                if let Some(node) = self.graph.nodes().iter().find(|n| n.id == id) {
                    // Intentar cargar desde temp primero
                    let initial_code = if let Some(temp_code) = self.load_editor_temp(id) {
                        // Actualizar el código del nodo con el código cargado desde temp
                        if let Some(node_mut) = self.graph.node_mut(id) {
                            node_mut.code = temp_code.clone();
                        }
                        temp_code
                    } else {
                        node.code.clone()
                    };
                    self.interaction.editor_history = Some(crate::utils::editor_history::EditorHistory::new(id, initial_code));
                }
            }
        }

        // Check for Esc key to close editor
        if open && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            open = false;
        }

        // Handle Ctrl+S for save
        if open && ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            if let Some(id) = node_id {
                self.save_editor_code(id);
            }
        }

        // Handle Ctrl+Z for undo, Ctrl+Y/Ctrl+Shift+Z for redo
        if open {
            if ctx.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && !i.modifiers.shift) {
                if let Some(history) = &mut self.interaction.editor_history {
                    if let Some(undo_code) = history.undo() {
                        if let Some(id) = node_id {
                            if let Some(node) = self.graph.node_mut(id) {
                                node.code = undo_code;
                            }
                        }
                    }
                }
            }
            if ctx.input(|i| (i.key_pressed(egui::Key::Y) && i.modifiers.ctrl) || 
                              (i.key_pressed(egui::Key::Z) && i.modifiers.ctrl && i.modifiers.shift)) {
                if let Some(history) = &mut self.interaction.editor_history {
                    if let Some(redo_code) = history.redo() {
                        if let Some(id) = node_id {
                            if let Some(node) = self.graph.node_mut(id) {
                                node.code = redo_code;
                            }
                        }
                    }
                }
            }
        }

        if open {
            let mut should_close = false;
            egui::Window::new("Editor de Código")
                .open(&mut open)
                .resizable(true)
                .default_size([700.0, 550.0])
                .frame(egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(37, 37, 38))
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 4.0),
                        blur: 12.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(200),
                    }))
                .show(ctx, |ui| {
                    // Check Esc key inside the window too
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        should_close = true;
                    }
                    if let Some(id) = node_id {
                        // Obtener información del nodo antes del closure
                        let node_title = self.graph.nodes().iter()
                            .find(|n| n.id == id)
                            .map(|n| n.title.clone())
                            .unwrap_or_default();
                        let _node_code_len = self.graph.nodes().iter()
                            .find(|n| n.id == id)
                            .map(|n| n.code.lines().count())
                            .unwrap_or(1);
                        let inheritance_chain = self.graph.get_inheritance_chain(id);
                        
                        // Estado para acciones del UI
                        let (can_undo, can_redo) = if let Some(history) = &self.interaction.editor_history {
                            (history.can_undo(), history.can_redo())
                        } else {
                            (false, false)
                        };
                        
                        let mut undo_clicked = false;
                        let mut redo_clicked = false;
                        let mut save_clicked = false;
                        let mut execute_clicked = false;
                        
                        ui.horizontal(|ui| {
                            ui.heading(&node_title);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Indicador de auto-save
                                if let Some(history) = &self.interaction.editor_history {
                                    if history.should_auto_save() {
                                        ui.label(egui::RichText::new("● Guardando...").color(egui::Color32::YELLOW).small());
                                    } else if let Some(last_save) = history.last_save_time {
                                        let elapsed = last_save.elapsed().as_secs();
                                        if elapsed < 2 {
                                            ui.label(egui::RichText::new("✓ Guardado").color(egui::Color32::GREEN).small());
                                        } else {
                                            ui.label(egui::RichText::new(format!("Guardado hace {}s", elapsed)).color(egui::Color32::GRAY).small());
                                        }
                                    }
                                }
                                
                                if ui.button("Cerrar").clicked() {
                                    should_close = true;
                                }
                                if ui.button("▶ Ejecutar").clicked() {
                                    execute_clicked = true;
                                }
                            });
                        });
                        ui.separator();

                        // Barra de herramientas con undo/redo
                        ui.horizontal(|ui| {
                            if ui.add_enabled(can_undo, egui::Button::new("↶ Deshacer (Ctrl+Z)")).clicked() {
                                undo_clicked = true;
                            }
                            if ui.add_enabled(can_redo, egui::Button::new("↷ Rehacer (Ctrl+Y)")).clicked() {
                                redo_clicked = true;
                            }
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("💾 Guardar (Ctrl+S)").clicked() {
                                    save_clicked = true;
                                }
                            });
                        });
                        ui.separator();

                        // Procesar acciones después del closure
                        if undo_clicked {
                            if let Some(history) = &mut self.interaction.editor_history {
                                if let Some(undo_code) = history.undo() {
                                    if let Some(node) = self.graph.node_mut(id) {
                                        node.code = undo_code;
                                    }
                                }
                            }
                        }
                        if redo_clicked {
                            if let Some(history) = &mut self.interaction.editor_history {
                                if let Some(redo_code) = history.redo() {
                                    if let Some(node) = self.graph.node_mut(id) {
                                        node.code = redo_code;
                                    }
                                }
                            }
                        }
                        if save_clicked {
                            self.save_editor_code(id);
                        }
                        if execute_clicked {
                            // Actualizar TODOS los canales antes de ejecutar para tener valores frescos
                            let all_node_ids: Vec<_> = self.graph.nodes().iter().map(|n| n.id).collect();
                            for nid in all_node_ids {
                                self.update_node_channels(nid);
                            }
                            
                            if let Some(node) = self.graph.nodes().iter().find(|n| n.id == id) {
                                let effective_language = self.resolve_effective_language(id, &inheritance_chain);
                                
                                // Si es NodeLanguage::Text, mostrar mensaje informativo en lugar de compilar
                                if effective_language == NodeLanguage::Text {
                                    self.terminal.visible = true;
                                    self.terminal.c_output.clear();
                                    self.terminal.c_output.push_str(&format!(
                                        "╔═══════════════════════════════════════════════════════════════════╗\n\
                                         ║  📄 NODO DE DOCUMENTACIÓN / REFERENCIA                           ║\n\
                                         ╠═══════════════════════════════════════════════════════════════════╣\n\
                                         ║                                                                   ║\n\
                                         ║  El nodo \"{}\" es de tipo TEXTO/DOC.             \n\
                                         ║                                                                   ║\n\
                                         ║  Este tipo de nodo NO se puede ejecutar directamente.            ║\n\
                                         ║  Es código de referencia para un proyecto multi-archivo.         ║\n\
                                         ║                                                                   ║\n\
                                         ║  📋 OPCIONES:                                                     ║\n\
                                         ║  • Presiona Ctrl+I para ver el código heredado                   ║\n\
                                         ║  • Exporta los archivos y compila con make                       ║\n\
                                         ║  • Cambia el lenguaje del nodo si deseas ejecutar                ║\n\
                                         ║                                                                   ║\n\
                                         ╚═══════════════════════════════════════════════════════════════════╝\n",
                                        node.title
                                    ));
                                    self.terminal.active_tab = TerminalTab::C;
                                } else {
                                    let lang = Self::language_to_terminal(effective_language, &node.title);
                                    let workspace_path = self.workspace.root_path.as_ref();
                                    
                                    // Detectar si es un nodo parámetro (título empieza con "Parámetro" o "Parametro")
                                    let is_parameter_node = node.title.starts_with("Parámetro") || node.title.starts_with("Parametro");
                                    
                                    // Combinar código heredado + propio para ejecutar
                                    let inherited_raw = self.graph.get_inherited_code(id).unwrap_or_default();
                                    // Evaluar ch() en código heredado
                                    let inherited = self.evaluate_ch_expressions_in_code(&inherited_raw, id);
                                    
                                    let full_code = if is_parameter_node {
                                        // Para nodos parámetro: solo ejecutar código heredado
                                        // El código propio del parámetro es solo un valor, no código ejecutable
                                        inherited
                                    } else {
                                        // Para nodos normales: heredado + propio
                                        let own_code_evaluated = self.evaluate_ch_expressions_in_code(&node.code, id);
                                        
                                        if !inherited.is_empty() && !own_code_evaluated.is_empty() {
                                            format!("{}\n\n{}", inherited, own_code_evaluated)
                                        } else if !inherited.is_empty() {
                                            inherited
                                        } else {
                                            own_code_evaluated
                                        }
                                    };
                                    
                                    self.terminal.run_code(&full_code, lang, workspace_path);
                                }
                            }
                        }

                        // Obtener información de herencia y evaluar expresiones ch()
                        let (inherited_code, parent_node_id, parent_node_title) = if let Some(inherited_raw) = self.graph.get_inherited_code(id) {
                            // Evaluar expresiones ch() en el código heredado
                            let inherited_evaluated = self.evaluate_ch_expressions_in_code(&inherited_raw, id);
                            
                            if let Some(parent_id) = self.graph.get_parent_node(id) {
                                // Obtener información del nodo padre
                                if let Some(parent_node) = self.graph.nodes().iter().find(|n| n.id == parent_id) {
                                    (Some(inherited_evaluated), Some(parent_id), Some(parent_node.title.clone()))
                                } else {
                                    (Some(inherited_evaluated), Some(parent_id), None)
                                }
                            } else {
                                (Some(inherited_evaluated), None, None)
                            }
                        } else {
                            (None, None, None)
                        };
                        
                        // Mostrar información de herencia si existe
                        if let (Some(_inh_code), Some(parent_id), Some(parent_title)) = (inherited_code, parent_node_id, parent_node_title) {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📥 Código Heredado de:").strong().color(Color32::from_rgb(89, 185, 89)));
                                if ui.button(egui::RichText::new(format!("→ {}", parent_title)).color(Color32::from_rgb(100, 200, 255))).clicked() {
                                    // Ir al nodo padre
                                    self.interaction.editing_node = Some(parent_id);
                                    self.interaction.selected_nodes.clear();
                                    self.interaction.selected_nodes.insert(parent_id);
                                    // Cerrar este editor y abrir el del padre
                                    should_close = true;
                                    // El editor del padre se abrirá automáticamente
                                }
                            });
                            ui.add_space(4.0);
                        }

                        // Mostrar información de la cadena de herencia
                        if !inheritance_chain.is_empty() {
                            let chain_names: Vec<&str> = inheritance_chain.iter()
                                .map(|(_, title, _)| title.as_str())
                                .collect();
                            
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("🔗 Cadena de herencia:")
                                        .small()
                                        .color(Color32::from_rgb(150, 150, 150))
                                );
                                ui.label(
                                    egui::RichText::new(chain_names.join(" → "))
                                        .small()
                                        .strong()
                                        .color(Color32::from_rgb(89, 185, 89))
                                );
                            });
                            ui.add_space(8.0);
                        }
                        
                        // Construir lista de canales disponibles a través de la cadena de herencia
                        let mut available_channels: Vec<(String, String, String, String)> = Vec::new();
                        for (ancestor_id, ancestor_title, _) in &inheritance_chain {
                            if let Some(channels) = self.channel_manager.get_node_channels(*ancestor_id) {
                                for (name, value) in channels {
                                    let expr = if name == "code" {
                                        format!(r#"ch("{}")"#, ancestor_title)
                                    } else {
                                        format!(r#"ch("{}/{}")"#, ancestor_title, name)
                                    };
                                    available_channels.push((
                                        ancestor_title.clone(),
                                        name.clone(),
                                        value.as_string(),
                                        expr,
                                    ));
                                }
                            }
                        }
                        if let Some(node_channels) = self.channel_manager.get_node_channels(id) {
                            for (name, value) in node_channels {
                                let expr = if name == "code" {
                                    format!(r#"ch("{}")"#, node_title)
                                } else {
                                    format!(r#"ch("{}/{}")"#, node_title, name)
                                };
                                available_channels.push((
                                    format!("{} (este nodo)", node_title),
                                    name.clone(),
                                    value.as_string(),
                                    expr,
                                ));
                            }
                        }
                        
                        ui.collapsing("📡 Canales disponibles para ch()", |ui| {
                            if available_channels.is_empty() {
                                ui.label("Conecta este nodo a otro para heredar canales disponibles.");
                            } else {
                                for (node_label, channel_name, preview, expr) in &available_channels {
                                    ui.horizontal(|ui| {
                                        let label_text = if channel_name == "code" {
                                            format!("{} → código", node_label)
                                        } else {
                                            format!("{} → {}", node_label, channel_name)
                                        };
                                        ui.label(
                                            egui::RichText::new(label_text)
                                                .strong()
                                                .color(Color32::from_rgb(150, 200, 255))
                                        );
                                        ui.label(
                                            egui::RichText::new(expr)
                                                .monospace()
                                                .color(Color32::from_rgb(100, 200, 255))
                                        );
                                        if ui.small_button("Copiar ch").clicked() {
                                            let expr_clip = expr.clone();
                                            ctx.output_mut(|o| o.copied_text = expr_clip);
                                        }
                                    });
                                    
                                    let preview_line = preview.lines().next().unwrap_or("").trim();
                                    if !preview_line.is_empty() {
                                        let mut snippet = preview_line.to_string();
                                        if snippet.len() > 80 {
                                            snippet.truncate(80);
                                            snippet.push('…');
                                        }
                                        ui.label(
                                            egui::RichText::new(snippet)
                                                .small()
                                                .color(Color32::from_gray(140))
                                        );
                                    }
                                    
                                    ui.add_space(4.0);
                                    ui.separator();
                                }
                            }
                        });
                        ui.add_space(8.0);
                        
                        // Calcular total de líneas heredadas
                        let mut total_inherited_lines = 0;
                        
                        // Mostrar cada bloque heredado por separado con su etiqueta
                        egui::ScrollArea::vertical()
                            .max_height(300.0)
                            .id_source("inherited_blocks")
                            .show(ui, |ui| {
                                for (i, (node_id, title, code)) in inheritance_chain.iter().enumerate() {
                                    // Evaluar expresiones ch() en el código heredado
                                    let evaluated_code = self.evaluate_ch_expressions_in_code(code, id);
                                    let lines_in_block = evaluated_code.lines().count();
                                    
                                    // Colores alternados para distinguir bloques
                                    let bg_alpha = if i % 2 == 0 { 15 } else { 25 };
                                    let border_color = if i % 2 == 0 {
                                        Color32::from_rgb(89, 185, 89)
                                    } else {
                                        Color32::from_rgb(100, 200, 150)
                                    };
                                    
                                    egui::Frame::none()
                                        .fill(Color32::from_rgba_unmultiplied(89, 185, 89, bg_alpha))
                                        .stroke(egui::Stroke::new(1.0, border_color))
                                        .inner_margin(egui::Margin::same(6.0))
                                        .outer_margin(egui::Margin::symmetric(0.0, 2.0))
                                        .rounding(egui::Rounding::same(4.0))
                                        .show(ui, |ui| {
                                            // Encabezado del bloque
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new(format!("📦 Heredado de: {}", title))
                                                        .small()
                                                        .strong()
                                                        .color(border_color)
                                                );
                                                
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.label(
                                                        egui::RichText::new(format!("Líneas {}-{}", 
                                                            total_inherited_lines + 1,
                                                            total_inherited_lines + lines_in_block
                                                        ))
                                                            .small()
                                                            .color(Color32::from_gray(120))
                                                    );
                                                    
                                                    // Botón para ir al nodo padre
                                                    if ui.small_button("📝 Editar").clicked() {
                                                        self.interaction.editing_node = Some(*node_id);
                                                        self.interaction.selected_nodes.clear();
                                                        self.interaction.selected_nodes.insert(*node_id);
                                                        should_close = true;
                                                    }
                                                });
                                            });
                                            
                                            ui.add_space(4.0);
                                            
                                            // Código del bloque (solo lectura)
                                            ui.add(
                                                egui::TextEdit::multiline(&mut evaluated_code.as_str())
                                                    .font(egui::TextStyle::Monospace)
                                                    .code_editor()
                                                    .interactive(false)
                                                    .desired_width(f32::INFINITY)
                                            );
                                        });
                                    
                                    total_inherited_lines += lines_in_block;
                                }
                            });
                        
                        // Separador visual si hay herencia
                        if !inheritance_chain.is_empty() {
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                            
                            // Indicador de código propio
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("✏️ Tu código (editable):")
                                        .strong()
                                        .color(Color32::from_rgb(100, 200, 255))
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        egui::RichText::new("Tip: Ctrl+P extrae selección a parámetro")
                                            .small()
                                            .italics()
                                            .color(Color32::from_gray(150))
                                    );
                                });
                            });
                            ui.add_space(4.0);
                        }
                        
                        // Obtener código propio del nodo actual
                        let current_own_code = self.graph.get_own_code(id);
                        let mut own_code_editable = current_own_code.clone();
                        let own_lines_count = own_code_editable.lines().count().max(1);
                        
                        // Calcular ancho de columna de números
                        let total_lines = total_inherited_lines + own_lines_count;
                        let num_width = (total_lines.to_string().len() as f32 * 10.0) + 20.0;
                        let start_line = total_inherited_lines + 1;
                        
                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.horizontal_top(|ui| {
                                    // COLUMNA DE NÚMEROS DE LÍNEA
                                    ui.vertical(|ui| {
                                        ui.set_width(num_width);
                                        // Obtener altura de línea del texto monoespaciado
                                        let line_height = ui.text_style_height(&egui::TextStyle::Monospace);
                                        ui.style_mut().spacing.item_spacing.y = 0.0;
                                        
                                        // Mostrar números de línea
                                        for i in 0..own_lines_count {
                                            let line_num = start_line + i;
                                            ui.add_sized(
                                                [num_width, line_height],
                                                egui::Label::new(
                                                    egui::RichText::new(format!("{:>4}", line_num))
                                                        .monospace()
                                                        .color(Color32::from_gray(100))
                                                )
                                            );
                                        }
                                    });
                                    
                                    // SEPARADOR
                                    ui.separator();
                                    
                                    // EDITOR DE CÓDIGO
                        let text_output = egui::TextEdit::multiline(&mut own_code_editable)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(own_lines_count.max(5))
                            .show(ui);

                        let mut code_changed = text_output.response.changed();
                        let mut convert_selection_request: Option<(usize, usize, String)> = None;

                        // Manejo del atajo Ctrl+P para convertir selección en parámetro (único)
                        if text_output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl && !i.modifiers.shift) {
                             if let Some(range) = text_output.cursor_range {
                                if let Some((selected_text, start_byte, end_byte)) = Self::extract_selected_text(&own_code_editable, &range) {
                                    convert_selection_request = Some((start_byte, end_byte, selected_text));
                                }
                             }
                        }
                        
                        // Manejo del atajo Ctrl+Shift+P para modo múltiples parámetros
                        if text_output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl && i.modifiers.shift) {
                            self.interaction.multi_param_mode = true;
                            if let Some(range) = text_output.cursor_range {
                                if let Some((selected_text, start_byte, end_byte)) = Self::extract_selected_text(&own_code_editable, &range) {
                                    convert_selection_request = Some((start_byte, end_byte, selected_text));
                                }
                            }
                        }
                        
                        // Manejo del atajo Ctrl+R para menú rápido de exportación a nodos existentes
                        if text_output.response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl && !i.modifiers.shift) {
                            if let Some(range) = text_output.cursor_range {
                                if let Some((selected_text, start_byte, end_byte)) = Self::extract_selected_text(&own_code_editable, &range) {
                                    // Guardar la selección y posición antes de abrir el menú
                                    self.interaction.r_menu_selection = Some((start_byte, end_byte, selected_text));
                                    self.interaction.r_menu_pos = ctx.input(|i| i.pointer.hover_pos());
                                    self.interaction.show_r_menu = true;
                                }
                            }
                        }

                        text_output.response.context_menu(|ui| {
                            if let Some(range) = text_output.cursor_range {
                                if let Some((selected_text, start_byte, end_byte)) = Self::extract_selected_text(&own_code_editable, &range) {
                                    ui.label(
                                        egui::RichText::new(selected_text.clone())
                                            .color(Color32::from_rgb(180, 220, 255))
                                    );
                                    ui.separator();
                                    if ui.button("📤 Nuevo Parámetro (Ctrl+P)").clicked() {
                                        convert_selection_request = Some((start_byte, end_byte, selected_text.clone()));
                                        ui.close_menu();
                                    }
                                    if ui.button("📤 Múltiples Parámetros (Ctrl+Shift+P)").clicked() {
                                        self.interaction.multi_param_mode = true;
                                        convert_selection_request = Some((start_byte, end_byte, selected_text.clone()));
                                        ui.close_menu();
                                    }
                                    if ui.button("📂 Exportar a Nodo Existente (Ctrl+R)").clicked() {
                                        // Guardar la selección antes de abrir el menú
                                        self.interaction.r_menu_selection = Some((start_byte, end_byte, selected_text.clone()));
                                        self.interaction.r_menu_pos = ctx.input(|i| i.pointer.hover_pos());
                                        self.interaction.show_r_menu = true;
                                        ui.close_menu();
                                    }
                                    ui.separator();
                                    ui.label(
                                        egui::RichText::new("Tip: Ctrl+R muestra nodos guardados")
                                            .small()
                                            .italics()
                                            .color(Color32::from_gray(140))
                                    );
                                } else {
                                    ui.label("Selecciona un fragmento de código para convertir.");
                                }
                            } else {
                                ui.label("Selecciona un fragmento de código para convertir.");
                            }
                        });
                        
                        // Variable para exportar a nodo existente
                        let mut export_to_existing_node: Option<(NodeId, String, usize, usize, String)> = None;
                        
                        // Menú Ctrl+R flotante para exportación a nodos existentes - DISEÑO PROFESIONAL
                        if self.interaction.show_r_menu {
                            let menu_pos = self.interaction.r_menu_pos.unwrap_or(egui::Pos2::new(400.0, 300.0));
                            
                            // Recopilar nodos disponibles para exportar (excluyendo el actual)
                            let available_nodes: Vec<(NodeId, String, NodeLanguage, Color32, String)> = self.graph.nodes()
                                .iter()
                                .filter(|n| n.id != id)
                                .map(|n| {
                                    let code_preview = if n.code.len() > 30 {
                                        format!("{}...", n.code.chars().take(30).collect::<String>())
                                    } else if n.code.is_empty() {
                                        "(vacío)".to_string()
                                    } else {
                                        n.code.clone()
                                    };
                                    (n.id, n.title.clone(), n.language, n.color, code_preview)
                                })
                                .collect();
                            
                            let saved_selection = self.interaction.r_menu_selection.clone();
                            
                            egui::Area::new(egui::Id::new("r_export_menu"))
                                .fixed_pos(menu_pos)
                                .order(egui::Order::Foreground)
                                .show(ctx, |ui| {
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(28, 32, 40))
                                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 70, 90)))
                                        .rounding(12.0)
                                        .shadow(egui::epaint::Shadow {
                                            offset: egui::vec2(0.0, 8.0),
                                            blur: 24.0,
                                            spread: 4.0,
                                            color: Color32::from_black_alpha(80),
                                        })
                                        .inner_margin(egui::Margin::same(0.0))
                                        .show(ui, |ui| {
                                            ui.set_min_width(340.0);
                                            ui.set_max_height(500.0);
                                            
                                            // Header con gradiente
                                            egui::Frame::none()
                                                .fill(Color32::from_rgb(45, 55, 75))
                                                .rounding(egui::Rounding { nw: 12.0, ne: 12.0, sw: 0.0, se: 0.0 })
                                                .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new("⚡")
                                                                .size(18.0)
                                                        );
                                                        ui.label(
                                                            egui::RichText::new("Exportar Rápido")
                                                                .strong()
                                                                .size(16.0)
                                                                .color(Color32::WHITE)
                                                        );
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                            ui.label(
                                                                egui::RichText::new("Ctrl+R")
                                                                    .small()
                                                                    .color(Color32::from_rgb(140, 160, 200))
                                                            );
                                                        });
                                                    });
                                                });
                                            
                                            // Contenido principal
                                            egui::Frame::none()
                                                .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                                                .show(ui, |ui| {
                                                    if let Some((start_byte, end_byte, selected_text)) = saved_selection {
                                                        // Preview del valor seleccionado
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgb(35, 42, 55))
                                                            .rounding(6.0)
                                                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    ui.label(
                                                                        egui::RichText::new("📋")
                                                                            .size(14.0)
                                                                    );
                                                                    let preview = if selected_text.len() > 45 {
                                                                        format!("{}...", &selected_text.chars().take(45).collect::<String>())
                                                                    } else {
                                                                        selected_text.clone()
                                                                    };
                                                                    ui.label(
                                                                        egui::RichText::new(format!("\"{}\"", preview))
                                                                            .monospace()
                                                                            .size(12.0)
                                                                            .color(Color32::from_rgb(130, 220, 130))
                                                                    );
                                                                });
                                                            });
                                                        
                                                        ui.add_space(12.0);
                                                        
                                                        // Sección: Crear nuevo
                                                        ui.horizontal(|ui| {
                                                            let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 14.0), egui::Sense::hover());
                                                            ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(100, 200, 100));
                                                            ui.add_space(4.0);
                                                            ui.label(
                                                                egui::RichText::new("CREAR NUEVO")
                                                                    .small()
                                                                    .strong()
                                                                    .color(Color32::from_rgb(100, 200, 100))
                                                            );
                                                        });
                                                        ui.add_space(6.0);
                                                        
                                                        let new_btn = ui.add_sized(
                                                            [ui.available_width(), 32.0],
                                                            egui::Button::new(
                                                                egui::RichText::new("➕  Nuevo Parámetro")
                                                                    .size(13.0)
                                                                    .color(Color32::WHITE)
                                                            )
                                                            .fill(Color32::from_rgb(50, 120, 80))
                                                            .rounding(6.0)
                                                        );
                                                        if new_btn.clicked() {
                                                            convert_selection_request = Some((start_byte, end_byte, selected_text.clone()));
                                                            self.interaction.show_r_menu = false;
                                                            self.interaction.r_menu_selection = None;
                                                        }
                                                        
                                                        ui.add_space(16.0);
                                                        
                                                        // Sección: Nodos existentes
                                                        ui.horizontal(|ui| {
                                                            let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 14.0), egui::Sense::hover());
                                                            ui.painter().rect_filled(rect, 2.0, Color32::from_rgb(200, 160, 80));
                                                            ui.add_space(4.0);
                                                            ui.label(
                                                                egui::RichText::new("NODOS EXISTENTES")
                                                                    .small()
                                                                    .strong()
                                                                    .color(Color32::from_rgb(200, 160, 80))
                                                            );
                                                            ui.label(
                                                                egui::RichText::new(format!("({})", available_nodes.len()))
                                                                    .small()
                                                                    .color(Color32::from_gray(120))
                                                            );
                                                        });
                                                        ui.add_space(6.0);
                                                        
                                                        if available_nodes.is_empty() {
                                                            egui::Frame::none()
                                                                .fill(Color32::from_rgb(40, 35, 35))
                                                                .rounding(6.0)
                                                                .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                                                                .show(ui, |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        ui.label(egui::RichText::new("📭").size(16.0));
                                                                        ui.vertical(|ui| {
                                                                            ui.label(
                                                                                egui::RichText::new("No hay nodos disponibles")
                                                                                    .color(Color32::from_gray(160))
                                                                            );
                                                                            ui.label(
                                                                                egui::RichText::new("Usa Ctrl+P para crear uno primero")
                                                                                    .small()
                                                                                    .color(Color32::from_gray(100))
                                                                            );
                                                                        });
                                                                    });
                                                                });
                                                        } else {
                                                            egui::ScrollArea::vertical()
                                                                .max_height(280.0)
                                                                .show(ui, |ui| {
                                                                    for (node_id, title, language, color, code_preview) in &available_nodes {
                                                                        let (lang_str, lang_icon, lang_bg) = match language {
                                                                            NodeLanguage::Asm => ("ASM", "🔧", Color32::from_rgb(80, 50, 40)),
                                                                            NodeLanguage::C => ("C", "©", Color32::from_rgb(40, 60, 80)),
                                                                            NodeLanguage::Cpp => ("C++", "⊕", Color32::from_rgb(50, 40, 80)),
                                                                            NodeLanguage::Rust => ("Rust", "🦀", Color32::from_rgb(80, 40, 40)),
                                                                            NodeLanguage::Zig => ("Zig", "⚡", Color32::from_rgb(240, 170, 0)),
                                                                            NodeLanguage::Java => ("Java", "☕", Color32::from_rgb(237, 139, 0)),
                                                                            NodeLanguage::Text => ("Doc", "📄", Color32::from_rgb(60, 60, 40)),
                                                                            NodeLanguage::Mojo => ("Mojo", "🔥", Color32::from_rgb(200, 50, 50)),
                                                                            NodeLanguage::MojoAI => ("MojoAI", "🤖", Color32::from_rgb(200, 100, 50)),
                                                                            NodeLanguage::Auto => ("Auto", "⚙", Color32::from_rgb(50, 50, 50)),
                                                                        };
                                                                        let lang_color = match *language {
                                                                            NodeLanguage::Asm => Color32::from_rgb(255, 170, 120),
                                                                            NodeLanguage::C => Color32::from_rgb(120, 200, 255),
                                                                            NodeLanguage::Cpp => Color32::from_rgb(180, 140, 255),
                                                                            NodeLanguage::Rust => Color32::from_rgb(255, 130, 100),
                                                                            NodeLanguage::Zig => Color32::from_rgb(240, 170, 0), // Amarillo/naranja para Zig
                                                                            NodeLanguage::Java => Color32::from_rgb(237, 139, 0), // Naranja Java
                                                                            NodeLanguage::Text => Color32::from_rgb(200, 200, 150),
                                                                            NodeLanguage::Mojo => Color32::from_rgb(255, 100, 100), // Rojo para Mojo
                                                                            NodeLanguage::MojoAI => Color32::from_rgb(255, 150, 100), // Naranja para MojoAI
                                                                            NodeLanguage::Auto => Color32::from_gray(180),
                                                                        };
                                                                        
                                                                        let node_frame = egui::Frame::none()
                                                                            .fill(lang_bg)
                                                                            .rounding(8.0)
                                                                            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                                                            .stroke(egui::Stroke::new(1.0, Color32::from_rgb(60, 70, 85)));
                                                                        
                                                                        node_frame.show(ui, |ui| {
                                                                            ui.horizontal(|ui| {
                                                                                // Barra de color del nodo
                                                                                let (rect, _) = ui.allocate_exact_size(
                                                                                    egui::vec2(4.0, 36.0),
                                                                                    egui::Sense::hover()
                                                                                );
                                                                                ui.painter().rect_filled(rect, 2.0, *color);
                                                                                
                                                                                ui.add_space(8.0);
                                                                                
                                                                                ui.vertical(|ui| {
                                                                                    ui.horizontal(|ui| {
                                                                                        ui.label(
                                                                                            egui::RichText::new(lang_icon)
                                                                                                .size(12.0)
                                                                                        );
                                                                                        ui.label(
                                                                                            egui::RichText::new(title)
                                                                                                .strong()
                                                                                                .size(13.0)
                                                                                                .color(Color32::WHITE)
                                                                                        );
                                                                                        ui.label(
                                                                                            egui::RichText::new(format!("[{}]", lang_str))
                                                                                                .small()
                                                                                                .color(lang_color)
                                                                                        );
                                                                                    });
                                                                                    ui.label(
                                                                                        egui::RichText::new(code_preview)
                                                                                            .small()
                                                                                            .monospace()
                                                                                            .color(Color32::from_gray(130))
                                                                                    );
                                                                                });
                                                                                
                                                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                                    let export_btn = ui.add(
                                                                                        egui::Button::new(
                                                                                            egui::RichText::new("📤")
                                                                                                .size(16.0)
                                                                                        )
                                                                                        .fill(Color32::from_rgb(60, 100, 150))
                                                                                        .rounding(4.0)
                                                                                        .min_size(egui::vec2(32.0, 28.0))
                                                                                    );
                                                                                    
                                                                                    if export_btn.clicked() {
                                                                                        export_to_existing_node = Some((
                                                                                            *node_id,
                                                                                            title.clone(),
                                                                                            start_byte,
                                                                                            end_byte,
                                                                                            selected_text.clone()
                                                                                        ));
                                                                                        self.interaction.show_r_menu = false;
                                                                                        self.interaction.r_menu_selection = None;
                                                                                    }
                                                                                });
                                                                            });
                                                                        });
                                                                        ui.add_space(4.0);
                                                                    }
                                                                });
                                                        }
                                                    } else {
                                                        egui::Frame::none()
                                                            .fill(Color32::from_rgb(60, 50, 40))
                                                            .rounding(6.0)
                                                            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                                                            .show(ui, |ui| {
                                                                ui.horizontal(|ui| {
                                                                    ui.label(egui::RichText::new("⚠").size(18.0));
                                                                    ui.vertical(|ui| {
                                                                        ui.label(
                                                                            egui::RichText::new("No hay texto seleccionado")
                                                                                .color(Color32::from_rgb(255, 200, 120))
                                                                        );
                                                                        ui.label(
                                                                            egui::RichText::new("Selecciona texto y presiona Ctrl+R")
                                                                                .small()
                                                                                .color(Color32::from_gray(150))
                                                                        );
                                                                    });
                                                                });
                                                            });
                                                    }
                                                });
                                            
                                            // Footer
                                            egui::Frame::none()
                                                .fill(Color32::from_rgb(35, 40, 50))
                                                .rounding(egui::Rounding { nw: 0.0, ne: 0.0, sw: 12.0, se: 12.0 })
                                                .inner_margin(egui::Margin::symmetric(16.0, 10.0))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new("💡 Tip: Los nodos se conectan automáticamente")
                                                                .small()
                                                                .italics()
                                                                .color(Color32::from_gray(100))
                                                        );
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                            let cancel_btn = ui.add(
                                                                egui::Button::new(
                                                                    egui::RichText::new("Cancelar")
                                                                        .small()
                                                                        .color(Color32::from_gray(180))
                                                                )
                                                                .fill(Color32::from_rgb(55, 50, 50))
                                                                .rounding(4.0)
                                                            );
                                                            if cancel_btn.clicked() {
                                                                self.interaction.show_r_menu = false;
                                                                self.interaction.r_menu_selection = None;
                                                            }
                                                        });
                                                    });
                                                });
                                        });
                                });
                            
                            // Cerrar con Escape
                            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                                self.interaction.show_r_menu = false;
                                self.interaction.r_menu_selection = None;
                            }
                        }
                        
                        // Procesar exportación a nodo existente
                        if let Some((target_node_id, target_title, start_byte, end_byte, selected_text)) = export_to_existing_node {
                            // Actualizar el código del nodo destino con el nuevo valor
                            if let Some(target_node) = self.graph.node_mut(target_node_id) {
                                target_node.code = selected_text;
                            }
                            // Actualizar canales
                            self.update_node_channels(target_node_id);
                            
                            // Reemplazar en el código actual con ch("nombre_nodo")
                            let placeholder = format!(r#"ch("{}")"#, target_title);
                            own_code_editable.replace_range(start_byte..end_byte, &placeholder);
                            code_changed = true;
                            
                            // Conectar el nodo destino al nodo actual si no está conectado
                            if let Some(current_node) = self.graph.node(id) {
                                if let Some(input_pin) = current_node.inputs.first() {
                                    if let Some(target_node) = self.graph.node(target_node_id) {
                                        if let Some(output_pin) = target_node.outputs.first() {
                                            // Verificar si ya existe la conexión
                                            let link_exists = self.graph.links().iter().any(|l| 
                                                l.from == output_pin.id && l.to == input_pin.id
                                            );
                                            if !link_exists {
                                                self.graph.add_link(output_pin.id, input_pin.id, Color32::from_rgb(150, 200, 255));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Indicador de modo múltiples parámetros
                        if self.interaction.multi_param_mode {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("🔄 Modo Múltiples Parámetros ACTIVO")
                                        .strong()
                                        .color(Color32::from_rgb(255, 200, 100))
                                );
                                ui.label(
                                    egui::RichText::new("(Selecciona más texto y presiona Ctrl+P, o Esc para salir)")
                                        .small()
                                        .color(Color32::from_gray(160))
                                );
                                if ui.small_button("✓ Finalizar").clicked() {
                                    self.interaction.multi_param_mode = false;
                                }
                            });
                            
                            // Salir del modo con Escape
                            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                self.interaction.multi_param_mode = false;
                            }
                        }

                        if let Some((start_byte, end_byte, selected_text)) = convert_selection_request {
                            if let Some(param_title) = self.create_parameter_node_from_selection(id, &selected_text) {
                                let placeholder = format!(r#"ch("{}")"#, param_title);
                                own_code_editable.replace_range(start_byte..end_byte, &placeholder);
                                code_changed = true;
                                // En modo múltiple, mantener el modo activo
                                // En modo normal, ya está desactivado
                            }
                        }
                                    
                                    // Procesar cambios
                                    if code_changed {
                                        if let Some(node) = self.graph.node_mut(id) {
                                            node.code = own_code_editable.clone();
                                            
                                            if let Some(history) = &mut self.interaction.editor_history {
                                                history.add_version(node.code.clone());
                                                history.save_to_temp(&node.code);
                                            }
                                            self.update_node_channels(id);
                                            ctx.request_repaint_after(std::time::Duration::from_millis(1));
                                        }
                                    }
                                });
                            });
                    }
                });
            if should_close {
                open = false;
            }
        }

        // Auto-save cada 10 segundos
        if open {
            let should_auto_save = if let Some(history) = &self.interaction.editor_history {
                history.should_auto_save()
            } else {
                false
            };
            
            if should_auto_save {
                if let Some(id) = node_id {
                    self.save_editor_code(id);
                    if let Some(history) = &mut self.interaction.editor_history {
                        history.mark_saved();
                    }
                }
            }
        }

        if !open {
            // Guardar antes de cerrar
            if let Some(id) = node_id {
                self.save_editor_code(id);
            }
            // Limpiar historial al cerrar
            if let Some(history) = &self.interaction.editor_history {
                history.cleanup_temp();
            }
            self.interaction.editing_node = None;
            self.interaction.editor_history = None;
        }
        
        // Auto-save del grafo después de editar
        self.check_and_auto_save();
    }
    
    fn save_editor_code(&mut self, node_id: NodeId) {
        // Obtener código heredado para separarlo
        let inherited = self.graph.get_inherited_code(node_id).unwrap_or_default();
        
        if let Some(node) = self.graph.node_mut(node_id) {
            // Separar código propio del heredado antes de guardar
            let full_code = &node.code;
            let own_code = if !inherited.is_empty() && full_code.starts_with(&inherited) {
                full_code[inherited.len()..].trim_start_matches('\n').trim_start_matches('\r').to_string()
            } else {
                full_code.clone()
            };
            
            // Guardar solo el código propio en el nodo
            node.code = own_code.clone();
            
            // Guardar en el historial (con código completo para referencia)
            if let Some(history) = &mut self.interaction.editor_history {
                // Guardar código completo en temp para referencia
                let full_for_temp = if !inherited.is_empty() {
                    format!("{}\n\n{}", inherited, own_code)
                } else {
                    own_code.clone()
                };
                history.add_version(full_for_temp.clone());
                history.save_to_temp(&full_for_temp);
                history.mark_saved();
            }
            
            // Guardar en el workspace si está configurado
            if self.workspace.has_root() {
                if let Err(e) = self.workspace.save_graph(&mut self.graph) {
                    eprintln!("Error guardando código del editor: {}", e);
                } else {
                    self.last_save_hash = self.graph_hash();
                    self.last_save_time = Some(std::time::Instant::now());
                }
            }
        }
    }
    
    fn load_editor_temp(&self, node_id: NodeId) -> Option<String> {
        let temp_dir = std::env::temp_dir().join("Ultra-Omega");
        let filename = format!("node_{}_code.tmp", node_id.0);
        let temp_path = temp_dir.join(&filename);
        
        if temp_path.exists() {
            if let Ok(code) = std::fs::read_to_string(&temp_path) {
                return Some(code);
            }
        }
        None
    }

    fn inheritance_view_ui(&mut self, ctx: &egui::Context) {
        if let Some(node_id) = self.interaction.viewing_inheritance {
            let mut should_close = false;
            egui::Window::new("Herencia de Código")
                .open(&mut true)
                .resizable(true)
                .default_size([600.0, 400.0])
                .frame(egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(37, 37, 38))
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 4.0),
                        blur: 12.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(200),
                    }))
                .show(ctx, |ui| {
                    if let Some(node) = self.graph.nodes().iter().find(|n| n.id == node_id) {
                        ui.heading(egui::RichText::new(&node.title).color(egui::Color32::from_rgb(212, 212, 212)));
                        ui.separator();
                        
                        if let Some(parent_id) = self.graph.get_parent_node(node_id) {
                            if let Some(parent) = self.graph.nodes().iter().find(|n| n.id == parent_id) {
                                ui.label(egui::RichText::new("Hereda de:").strong().color(egui::Color32::from_rgb(89, 185, 89)));
                                ui.label(egui::RichText::new(&parent.title).color(egui::Color32::from_rgb(89, 185, 89)));
                                ui.add_space(8.0);
                                
                                ui.label(egui::RichText::new("Código heredado:").strong().color(egui::Color32::from_rgb(212, 212, 212)));
                                ui.separator();
                                
                                let mut code_display = parent.code.clone();
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut code_display)
                                            .font(egui::TextStyle::Monospace)
                                            .desired_width(f32::INFINITY)
                                            .interactive(false),
                                    );
                                });
                            }
                        } else {
                            ui.label(egui::RichText::new("Este nodo no hereda de ningún otro nodo.").color(egui::Color32::from_rgb(128, 128, 128)));
                        }
                        
                        ui.add_space(8.0);
                        if ui.button("Cerrar").clicked() {
                            should_close = true;
                        }
                    }
                });
            
            if should_close {
                self.interaction.viewing_inheritance = None;
            }
        }
    }

    // --- Painting Helpers (Moved from old main) ---

    fn paint_grid(&self, painter: &egui::Painter, rect: Rect, _visuals: &Visuals) {
        const GRID_SPACING: f32 = 32.0;
        let spacing = (GRID_SPACING * self.viewport.zoom).clamp(12.0, 256.0);

        let offset_x = self.viewport.pan.x.rem_euclid(spacing);
        let offset_y = self.viewport.pan.y.rem_euclid(spacing);

        // Fondo completamente negro
        painter.rect_filled(rect, 0.0, Color32::from_rgb(0, 0, 0));

        // Líneas de grilla en gris muy oscuro para visibilidad sutil
        let grid_color_minor = Color32::from_rgb(15, 15, 15); // Líneas menores muy sutiles
        let grid_color_major = Color32::from_rgb(25, 25, 25); // Líneas mayores ligeramente más visibles

        let mut count_x = 0;
        let mut x = rect.min.x + offset_x;
        while x < rect.max.x {
            let major = count_x % 4 == 0;
            let color = if major { grid_color_major } else { grid_color_minor };
            painter.line_segment(
                [pos2(x, rect.min.y), pos2(x, rect.max.y)],
                Stroke::new(1.0, color),
            );
            x += spacing;
            count_x += 1;
        }

        let mut count_y = 0;
        let mut y = rect.min.y + offset_y;
        while y < rect.max.y {
            let major = count_y % 4 == 0;
            let color = if major { grid_color_major } else { grid_color_minor };
            painter.line_segment(
                [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                Stroke::new(1.0, color),
            );
            y += spacing;
            count_y += 1;
        }
    }

    fn paint_links(&self, painter: &egui::Painter, rect: Rect, time: f64) {
        for link in self.graph.links() {
            let Some(start) = self.pin_screen_position(link.from, rect) else {
                continue;
            };
            let Some(end) = self.pin_screen_position(link.to, rect) else {
                continue;
            };

            // Usar el estilo de conector según el layout
            match self.layout_config.style {
                crate::ui::layout::LayoutStyle::SemanticMap => {
                    // Conectores verticales para mapa semántico
                    // Usar blanco para todas las conexiones (ignorar link.color)
                    crate::ui::nodes_semantic::draw_semantic_connector(
                        painter,
                        start,
                        end,
                        Color32::WHITE, // Siempre blanco sobre fondo negro
                        self.viewport.zoom,
                        false, // is_highlighted
                    );
                }
                crate::ui::layout::LayoutStyle::Horizontal => {
                    // Conectores horizontales originales
                    crate::ui::connectors::draw_connection(
                        painter,
                        start,
                        end,
                        link.color,
                        self.viewport.zoom,
                        time,
                    );
                }
            }
        }
    }

    fn paint_nodes(&self, painter: &egui::Painter, rect: Rect, visuals: &Visuals) {
        // Obtener nodos que están siendo heredados (para resaltarlos en verde)
        let inherited_nodes: std::collections::HashSet<NodeId> = if let Some(viewing_id) = self.interaction.viewing_inheritance {
            self.graph.get_parent_node(viewing_id).into_iter().collect()
        } else {
            std::collections::HashSet::new()
        };

        // Recolectar todos los pins conectados para el efecto de llenado
        let connected_pins: std::collections::HashSet<PinId> = self.graph.links()
            .iter()
            .flat_map(|link| vec![link.from, link.to])
            .collect();

        for node in self.graph.nodes() {
            let node_rect = self.node_rect(node, rect);
            let selected = self.interaction.selected_nodes.contains(&node.id);
            let is_inherited = inherited_nodes.contains(&node.id);
            
            // Usar el estilo de nodo según el layout
            match self.layout_config.style {
                crate::ui::layout::LayoutStyle::SemanticMap => {
                    crate::ui::nodes_semantic::draw_semantic_node(
                        painter, node, node_rect, self.viewport.zoom, 
                        selected, is_inherited, visuals, &connected_pins
                    );
                }
                crate::ui::layout::LayoutStyle::Horizontal => {
                    crate::ui::nodes::draw_node(
                        painter, node, node_rect, self.viewport.zoom, 
                        selected, is_inherited, visuals, &connected_pins
                    );
                }
            }
        }
    }

    fn handle_node_dragging(&mut self, input: &PointerSnapshot, rect: Rect) {
        // PRIORIDAD ABSOLUTA: Si el modo de corte está activo, no hacer NADA aquí
        // Esto previene cualquier interferencia con la selección
        if self.interaction.cut_tool.active {
            // Limpiar cualquier estado de drag/selección si el modo de corte está activo
            self.interaction.dragging_node = None;
            self.interaction.box_selection_start = None;
            self.interaction.box_selection_current = None;
            return;
        }

        if let Some(pointer_pos) = input.pos {
            // 0. PRIORIDAD: Verificar si se hace click en un pin ANTES de cualquier otra cosa
            if input.primary_pressed {
                // Primero verificar si hay un pin bajo el cursor
                if let Some((pin_id, _pin_kind)) = self.hit_test_pin(pointer_pos, rect) {
                    if let Some(addr) = self.graph.locate_pin(pin_id) {
                        if addr.kind == PinKind::Output {
                            // Iniciar conexión desde pin de salida
                            self.interaction.connecting_from = Some(pin_id);
                            // NO activar drag de nodo si estamos conectando
                            return;
                        }
                    }
                }
            }

            // 1. Node Interaction (solo si no estamos conectando)
            if input.primary_pressed && self.interaction.connecting_from.is_none() {
                if let Some(node_id) = self.hit_test(pointer_pos, rect) {
                    // Verificar que no sea un pin antes de activar drag
                    if self.hit_test_pin(pointer_pos, rect).is_none() {
                        // ═══════════════════════════════════════════════════════════════════
                        // 🆕 DETECTAR DOBLE CLIC PARA ENTRAR A SUBNETWORKS
                        // ═══════════════════════════════════════════════════════════════════
                        let now = std::time::Instant::now();
                        let is_double_click = if let Some((last_node, last_time)) = self.interaction.last_click_node {
                            last_node == node_id && now.duration_since(last_time).as_millis() < 500
                        } else {
                            false
                        };
                        
                        // Si es doble clic y el nodo es un subnetwork, entrar
                        if is_double_click {
                            if let Some(node) = self.graph.node(node_id) {
                                if node.subnetwork_graph.is_some() {
                                    if let Err(e) = self.enter_subnetwork(node_id) {
                                        eprintln!("Error entering subnetwork: {}", e);
                                    }
                                    // Limpiar el último click para evitar triple clic
                                    self.interaction.last_click_node = None;
                                    return; // No hacer selección normal en doble clic
                                }
                            }
                        }
                        
                        // Guardar este click para detectar doble clic
                        self.interaction.last_click_node = Some((node_id, now));
                        
                        // Node clicked (no pin)
                        if !input.modifiers.ctrl && !self.interaction.selected_nodes.contains(&node_id) {
                            self.interaction.selected_nodes.clear();
                        }
                        self.interaction.selected_nodes.insert(node_id);
                        self.interaction.dragging_node = Some(node_id);
                    }
                } else {
                    // Background clicked -> Start Box Select
                    if !input.modifiers.ctrl {
                        self.interaction.selected_nodes.clear();
                    }
                    self.interaction.box_selection_start = Some(pointer_pos);
                    self.interaction.box_selection_current = Some(pointer_pos);
                    // Limpiar último click si se hace click en el fondo
                    self.interaction.last_click_node = None;
                }
            }

            // 2. Node Dragging OR Box Selection
            if input.primary_down {
                if self.interaction.dragging_node.is_some() {
                    // Moved to step 3
                } else if self.interaction.box_selection_start.is_some() {
                    // Update box selection
                    self.interaction.box_selection_current = Some(pointer_pos);
                }
            } else {
                // Mouse released
                if let (Some(start), Some(current)) = (self.interaction.box_selection_start, self.interaction.box_selection_current) {
                    // Commit box selection
                    let selection_rect = Rect::from_two_pos(start, current);
                    // Convert selection rect to world to check intersection? 
                    // No, easier to check visual overlap.
                    for node in self.graph.nodes() {
                        let node_rect = self.node_rect(node, rect);
                        if selection_rect.intersects(node_rect) {
                            self.interaction.selected_nodes.insert(node.id);
                        }
                    }
                }
                let was_dragging = self.interaction.dragging_node.is_some();
                self.interaction.dragging_node = None;
                self.interaction.box_selection_start = None;
                self.interaction.box_selection_current = None;
                
                // Auto-save after releasing mouse if we were dragging
                if was_dragging {
                    self.check_and_auto_save();
                }
            }

            // 3. Apply Node Movement (Deferred to solve borrow checker)
            let was_dragging = self.interaction.dragging_node.is_some();
            if input.primary_down && was_dragging {
                 let delta_world = input.delta / self.viewport.zoom;
                 // Collect IDs to avoid borrow issues
                 let nodes_to_move: Vec<NodeId> = self.interaction.selected_nodes.iter().cloned().collect();
                 for id in nodes_to_move {
                     if let Some(node) = self.graph.node_mut(id) {
                         node.position += delta_world;
                     }
                 }
                 // Sincronizar cambios de posición al nivel actual
                 self.sync_current_level_to_graph();
            } else if !input.primary_down && was_dragging {
                // Mouse released after dragging - sincronizar y auto-save
                self.sync_current_level_to_graph();
                self.check_and_auto_save();
            }

            if input.secondary_pressed && rect.contains(pointer_pos) {
                if let Some(node_id) = self.hit_test(pointer_pos, rect) {
                    if input.modifiers.ctrl {
                        // Ctrl + Click derecho: Ver herencia
                        self.interaction.viewing_inheritance = Some(node_id);
                    } else {
                        // Click derecho normal: Editar
                        self.interaction.editing_node = Some(node_id);
                        self.interaction.selected_nodes.insert(node_id);
                    }
                }
            }

            // Sistema de conexión de nodos - Completar conexión al soltar
            if let Some(pointer_pos) = input.pos {
                // Soltar en pin de entrada para completar conexión (con snap)
                if !input.primary_down && self.interaction.connecting_from.is_some() {
                    let from_pin = self.interaction.connecting_from;
                    
                    // Primero intentar snap
                    if let Some((snap_pin_id, _snap_pos, snap_kind)) = self.find_nearest_snap_pin(pointer_pos, rect, from_pin) {
                        if snap_kind == PinKind::Input {
                            if let Some(from_pin_id) = from_pin {
                                // Crear conexión con snap
                                self.graph.add_link(from_pin_id, snap_pin_id, Color32::from_rgb(100, 200, 255));
                                // Sincronizar cambios al nivel actual
                                self.sync_current_level_to_graph();
                                // No aplicar herencia automáticamente - se combina en tiempo de ejecución/visualización
                                self.check_and_auto_save();
                            }
                        }
                    } else {
                        // Si no hay snap, intentar hit test normal
                        if let Some((pin_id, _kind)) = self.hit_test_pin(pointer_pos, rect) {
                            if let Some(addr) = self.graph.locate_pin(pin_id) {
                                if addr.kind == PinKind::Input {
                                    if let Some(from_pin_id) = from_pin {
                                        // Crear conexión
                                        self.graph.add_link(from_pin_id, pin_id, Color32::from_rgb(100, 200, 255));
                                        // Sincronizar cambios al nivel actual
                                        self.sync_current_level_to_graph();
                                        // No aplicar herencia automáticamente - se combina en tiempo de ejecución/visualización
                                        self.check_and_auto_save();
                                    }
                                }
                            }
                        }
                    }
                    self.interaction.connecting_from = None;
                }
            }
        }
    }
    
    fn draw_box_selection(&self, painter: &egui::Painter) {
        // NO dibujar selección de caja si el modo de corte está activo
        if self.interaction.cut_tool.active {
            return;
        }
        
        if let (Some(start), Some(current)) = (self.interaction.box_selection_start, self.interaction.box_selection_current) {
            let rect = Rect::from_two_pos(start, current);
            painter.rect(
                rect,
                0.0,
                Color32::from_rgba_unmultiplied(100, 200, 255, 30),
                Stroke::new(1.0, Color32::from_rgb(100, 200, 255)),
            );
        }
    }

    fn draw_connecting_line(&self, painter: &egui::Painter, canvas: Rect, pointer_pos: Option<Pos2>, time: f64) {
        if let (Some(from_pin), Some(pointer)) = (self.interaction.connecting_from, pointer_pos) {
            if let Some(start_pos) = self.pin_screen_position(from_pin, canvas) {
                // Verificar si hay snap disponible
                let (end_pos, snap_color) = if let Some((_snap_pin_id, snap_pos, snap_kind)) = self.find_nearest_snap_pin(pointer, canvas, Some(from_pin)) {
                    if snap_kind == PinKind::Input {
                        // Snap a pin de entrada - color verde brillante
                        (snap_pos, Color32::from_rgb(100, 255, 150))
                    } else {
                        (snap_pos, Color32::from_rgb(100, 200, 255))
                    }
                } else {
                    (pointer, Color32::from_rgb(100, 200, 255))
                };
                
                // Usar blanco para todas las conexiones (ignorar snap_color)
                crate::ui::connectors::draw_connection(
                    painter,
                    start_pos,
                    end_pos,
                    Color32::WHITE, // Siempre blanco sobre fondo negro
                    self.viewport.zoom,
                    time,
                );
                
                // Dibujar indicadores de snap mejorados
                self.draw_snap_indicators(painter, canvas, pointer_pos, Some(from_pin));
            }
        }
    }

    fn hit_test(&self, pointer: Pos2, rect: Rect) -> Option<NodeId> {
        self.graph.nodes().iter().rev().find_map(|node| {
            let r = self.node_rect(node, rect);
            if r.contains(pointer) {
                Some(node.id)
            } else {
                None
            }
        })
    }

    fn node_rect(&self, node: &Node, canvas: Rect) -> Rect {
        let size = self.node_size(node) * self.viewport.zoom;
        let min = self.viewport.world_to_screen(node.position, canvas);
        Rect::from_min_size(min, size)
    }

    fn node_size(&self, node: &Node) -> Vec2 {
        use crate::ui::layout::LayoutStyle;
        
        match self.layout_config.style {
            LayoutStyle::SemanticMap => {
                // Nodo semántico: más compacto y cuadrado
                let rows = node.inputs.len().max(node.outputs.len()).max(1) as f32;
                let height = crate::ui::nodes_semantic::HEADER_HEIGHT 
                    + crate::ui::nodes_semantic::BODY_HEIGHT.max(rows * 20.0);
                Vec2::new(crate::ui::nodes_semantic::NODE_WIDTH, height)
            }
            LayoutStyle::Horizontal => {
                // Nodo horizontal: estilo original
                let rows = node.inputs.len().max(node.outputs.len()).max(1) as f32;
                let height = crate::ui::nodes::HEADER_HEIGHT 
                    + rows * crate::ui::nodes::PIN_SPACING 
                    + crate::ui::nodes::CONTENT_PADDING * 2.0;
                Vec2::new(crate::ui::nodes::NODE_WIDTH, height)
            }
        }
    }

    fn pin_slot_position(
        &self,
        node: &Node,
        canvas: Rect,
        kind: PinKind,
        index: usize,
    ) -> Pos2 {
        use crate::ui::layout::LayoutStyle;
        
        let rect = self.node_rect(node, canvas);
        
        match self.layout_config.style {
            LayoutStyle::SemanticMap => {
                // Pins arriba (input) y abajo (output)
                match kind {
                    PinKind::Input => {
                        crate::ui::nodes_semantic::get_input_pin_position(
                            rect, index, node.inputs.len(), self.viewport.zoom
                        )
                    }
                    PinKind::Output => {
                        crate::ui::nodes_semantic::get_output_pin_position(
                            rect, index, node.outputs.len(), self.viewport.zoom
                        )
                    }
                }
            }
            LayoutStyle::Horizontal => {
                // Pins izquierda (input) y derecha (output) - original
                let y = rect.min.y
                    + crate::ui::nodes::HEADER_HEIGHT * self.viewport.zoom
                    + crate::ui::nodes::PIN_SPACING * self.viewport.zoom * (index as f32 + 0.5);

                match kind {
                    PinKind::Input => pos2(rect.min.x + crate::ui::nodes::CONTENT_PADDING * self.viewport.zoom, y),
                    PinKind::Output => pos2(rect.max.x - crate::ui::nodes::CONTENT_PADDING * self.viewport.zoom, y),
                }
            }
        }
    }

    fn pin_screen_position(&self, pin_id: PinId, canvas: Rect) -> Option<Pos2> {
        let address = self.graph.locate_pin(pin_id)?;
        let node = &self.graph.nodes()[address.node_index];
        Some(self.pin_slot_position(node, canvas, address.kind, address.slot))
    }

    fn hit_test_pin(&self, pointer: Pos2, canvas: Rect) -> Option<(PinId, PinKind)> {
        // Área de detección más grande para facilitar el agarre
        // Usar distancia en píxeles de pantalla (no escalada por zoom) para mejor UX
        const PIN_HIT_RADIUS: f32 = 20.0; // Píxeles de pantalla
        let mut closest: Option<(PinId, PinKind, f32)> = None;
        
        for node in self.graph.nodes() {
            // Check input pins
            for (idx, pin) in node.inputs.iter().enumerate() {
                let pin_pos = self.pin_slot_position(node, canvas, PinKind::Input, idx);
                let distance = (pin_pos - pointer).length();
                // Usar distancia en píxeles de pantalla directamente
                if distance < PIN_HIT_RADIUS {
                    if closest.is_none() || distance < closest.unwrap().2 {
                        closest = Some((pin.id, PinKind::Input, distance));
                    }
                }
            }
            
            // Check output pins
            for (idx, pin) in node.outputs.iter().enumerate() {
                let pin_pos = self.pin_slot_position(node, canvas, PinKind::Output, idx);
                let distance = (pin_pos - pointer).length();
                // Usar distancia en píxeles de pantalla directamente
                if distance < PIN_HIT_RADIUS {
                    if closest.is_none() || distance < closest.unwrap().2 {
                        closest = Some((pin.id, PinKind::Output, distance));
                    }
                }
            }
        }
        
        closest.map(|(id, kind, _)| (id, kind))
    }

    // Encontrar el pin más cercano para snap (mejorado para pins de entrada)
    fn find_nearest_snap_pin(&self, pointer: Pos2, canvas: Rect, from_pin: Option<PinId>) -> Option<(PinId, Pos2, PinKind)> {
        // Distancia de snap más grande para pins de entrada (más fácil de atrapar)
        const SNAP_DISTANCE_INPUT: f32 = 50.0; // Mayor distancia para pins de entrada
        const SNAP_DISTANCE_OUTPUT: f32 = 30.0; // Distancia normal para pins de salida
        let mut nearest: Option<(PinId, Pos2, PinKind, f32)> = None;

        let from_node_id = if let Some(from_pin_id) = from_pin {
            if let Some(addr) = self.graph.locate_pin(from_pin_id) {
                Some(self.graph.nodes()[addr.node_index].id)
            } else {
                None
            }
        } else {
            None
        };

        for node in self.graph.nodes() {
            // No hacer snap a pins del mismo nodo
            if let Some(from_id) = from_node_id {
                if node.id == from_id {
                    continue;
                }
            }
            
            // Check input pins (solo para conexiones) - SNAP MEJORADO
            if from_pin.is_some() {
                for (idx, pin) in node.inputs.iter().enumerate() {
                    let pin_pos = self.pin_slot_position(node, canvas, PinKind::Input, idx);
                    let distance = (pin_pos - pointer).length();
                    let snap_distance = SNAP_DISTANCE_INPUT * self.viewport.zoom;
                    
                    if distance < snap_distance {
                        if nearest.is_none() || distance < nearest.unwrap().3 {
                            nearest = Some((pin.id, pin_pos, PinKind::Input, distance));
                        }
                    }
                }
            }
            
            // Check output pins (solo si no estamos conectando)
            if from_pin.is_none() {
                for (idx, pin) in node.outputs.iter().enumerate() {
                    let pin_pos = self.pin_slot_position(node, canvas, PinKind::Output, idx);
                    let distance = (pin_pos - pointer).length();
                    let snap_distance = SNAP_DISTANCE_OUTPUT * self.viewport.zoom;
                    
                    if distance < snap_distance {
                        if nearest.is_none() || distance < nearest.unwrap().3 {
                            nearest = Some((pin.id, pin_pos, PinKind::Output, distance));
                        }
                    }
                }
            }
        }
        
        nearest.map(|(id, pos, kind, _)| (id, pos, kind))
    }
    
    // Dibujar indicador de snap en pins de entrada
    fn draw_snap_indicators(&self, painter: &egui::Painter, canvas: Rect, pointer_pos: Option<Pos2>, connecting_from: Option<PinId>) {
        if let (Some(pointer), Some(from_pin)) = (pointer_pos, connecting_from) {
            if let Some((_snap_pin_id, snap_pos, snap_kind)) = self.find_nearest_snap_pin(pointer, canvas, Some(from_pin)) {
                if snap_kind == PinKind::Input {
                    // Calcular intensidad del snap basado en la distancia
                    let distance = (snap_pos - pointer).length();
                    let max_distance = 50.0 * self.viewport.zoom;
                    let intensity = 1.0 - (distance / max_distance).min(1.0);
                    
                    // Glow pulsante alrededor del pin de entrada
                    let pulse_size = 12.0 + intensity * 4.0;
                    let pulse_alpha = (intensity * 200.0) as u8;
                    
                    // Círculos concéntricos para efecto de atracción
                    painter.circle_filled(
                        snap_pos,
                        pulse_size * self.viewport.zoom,
                        Color32::from_rgba_unmultiplied(100, 255, 150, pulse_alpha / 3),
                    );
                    painter.circle_filled(
                        snap_pos,
                        pulse_size * 0.7 * self.viewport.zoom,
                        Color32::from_rgba_unmultiplied(100, 255, 150, pulse_alpha / 2),
                    );
                    painter.circle_stroke(
                        snap_pos,
                        pulse_size * 0.5 * self.viewport.zoom,
                        Stroke::new(2.0 * self.viewport.zoom, Color32::from_rgba_unmultiplied(100, 255, 150, pulse_alpha)),
                    );
                    
                    // Línea guía desde el puntero al pin (sutil)
                    painter.line_segment(
                        [pointer, snap_pos],
                        Stroke::new(1.0 * self.viewport.zoom, Color32::from_rgba_unmultiplied(100, 255, 150, (intensity * 100.0) as u8)),
                    );
                }
            }
        }
    }

    fn apply_inheritance(&mut self) {
        // Aplicar herencia de código a todos los nodos que tienen padre
        // NOTA: Ahora solo combinamos al guardar/ejecutar, no modificamos node.code directamente
        // para permitir edición separada en el editor
        // La combinación se hace en tiempo de ejecución o cuando se necesita el código completo
    }
    
    /// Evaluar todas las expresiones ch() en un bloque de código
    fn evaluate_ch_expressions_in_code(&self, code: &str, current_node_id: NodeId) -> String {
        use crate::expressions::ExpressionEvaluator;
        use crate::expressions::ChannelValue;
        
        // Crear evaluador con el channel_manager actual
        let mut evaluator = ExpressionEvaluator::new(self.channel_manager.clone());
        evaluator.set_current_node(current_node_id);
        
        // Convertir a Vec<char> para manejar UTF-8 correctamente
        let chars: Vec<char> = code.chars().collect();
        let mut result = String::new();
        let mut last_char_idx = 0;
        let mut i = 0;
        
        // Helper para convertir índice de char a substring
        let chars_to_string = |chars: &[char], start: usize, end: usize| -> String {
            chars[start..end].iter().collect()
        };
        
        while i < chars.len() {
            // Buscar "ch(" en cualquier contexto
            if i + 2 < chars.len() 
                && chars[i] == 'c' 
                && chars[i + 1] == 'h' 
                && chars[i + 2] == '(' {
                
                // Verificar si está dentro de comillas simples de NASM (ej: 'ch("...")')
                let in_nasm_string = i > 0 && chars[i - 1] == '\'';
                
                // Agregar todo antes de ch( (o antes de la comilla si es NASM string)
                let prefix_end = if in_nasm_string { i - 1 } else { i };
                result.push_str(&chars_to_string(&chars, last_char_idx, prefix_end));
                
                // Buscar el cierre del paréntesis
                let mut depth = 1;
                let mut j = i + 3;
                let mut in_inner_string = false;
                let mut string_char = '\0';
                
                while j < chars.len() && depth > 0 {
                    let c = chars[j];
                    
                    if !in_inner_string {
                        if c == '"' {
                            in_inner_string = true;
                            string_char = c;
                        } else if c == '(' {
                            depth += 1;
                        } else if c == ')' {
                            depth -= 1;
                        }
                    } else if c == string_char && (j == 0 || chars[j - 1] != '\\') {
                        in_inner_string = false;
                    }
                    
                    if depth > 0 {
                        j += 1;
                    }
                }
                
                if depth == 0 {
                    // Extraer la expresión completa ch(...)
                    let expr_str = chars_to_string(&chars, i, j + 1);
                    
                    // Verificar si hay comilla de cierre después (para NASM strings)
                    let has_closing_quote = in_nasm_string && j + 1 < chars.len() && chars[j + 1] == '\'';
                    
                    // Intentar evaluar
                    match evaluator.evaluate_string(&expr_str) {
                        Ok(value) => {
                            let value_str = match value {
                                ChannelValue::String(s) => s,
                                ChannelValue::Number(n) => n.to_string(),
                                ChannelValue::Boolean(b) => b.to_string(),
                                ChannelValue::Code(c) => c,
                            };
                            
                            if in_nasm_string && has_closing_quote {
                                // Para NASM: 'ch("...")' -> 'valor'
                                result.push('\'');
                                result.push_str(&value_str);
                                result.push('\'');
                                last_char_idx = j + 2;
                                i = j + 2;
                            } else {
                                result.push_str(&value_str);
                                last_char_idx = j + 1;
                                i = j + 1;
                            }
                            continue;
                        }
                        Err(_) => {
                            // Si falla, dejar la expresión original
                            if in_nasm_string {
                                result.push('\'');
                            }
                            result.push_str(&expr_str);
                            if in_nasm_string && has_closing_quote {
                                result.push('\'');
                                last_char_idx = j + 2;
                                i = j + 2;
                            } else {
                                last_char_idx = j + 1;
                                i = j + 1;
                            }
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
        
        // Agregar el resto del código
        if last_char_idx < chars.len() {
            result.push_str(&chars_to_string(&chars, last_char_idx, chars.len()));
        }
        
        if result.is_empty() {
            code.to_string()
        } else {
            result
        }
    }

    pub fn save_current_graph(&mut self) -> Result<(), String> {
        // ═══════════════════════════════════════════════════════════════════
        // 🆕 GUARDAR GRAFO ACTUAL Y SINCRONIZAR CON NETWORK_LEVELS
        // ═══════════════════════════════════════════════════════════════════
        // Primero sincronizar el grafo actual al nivel activo
        self.sync_current_level_to_graph();
        
        // Guardar el grafo del nivel raíz (que contiene todos los subnetworks)
        if let Some(root_level) = self.network_levels.first() {
            let mut root_graph = root_level.graph.clone();
            self.workspace.save_graph(&mut root_graph)?;
            // Actualizar el grafo raíz en network_levels
            if let Some(level) = self.network_levels.first_mut() {
                level.graph = root_graph;
            }
            // Si estamos en root, también sincronizar self.graph
            if self.is_at_root() {
                self.sync_graph_to_current_level();
            }
        } else {
            // Fallback: guardar el grafo actual (no debería pasar, pero por seguridad)
            self.workspace.save_graph(&mut self.graph)?;
        }
        
        self.last_save_hash = self.graph_hash();
        self.last_save_time = Some(std::time::Instant::now());
        Ok(())
    }

    pub fn load_graph_from_workspace(&mut self) -> Result<(), String> {
        // ═══════════════════════════════════════════════════════════════════
        // 🆕 RESETEAR A ROOT AL CARGAR PROYECTO
        // ═══════════════════════════════════════════════════════════════════
        // Si estamos en un subnetwork, volver al root primero
        while self.network_levels.len() > 1 {
            let _ = self.exit_subnetwork();
        }
        
        // Detectar si necesita migración ANTES de cargar
        if needs_migration(&self.workspace) {
            // Preparar estado del diálogo de migración
            self.migration_dialog = Some(MigrationDialogState {
                show: true,
                needs_migration: true,
                backup_path: None,
                result: None,
                error: None,
            });
            // Cargar el grafo de todas formas (en formato antiguo) para poder migrarlo
            let graph = self.workspace.load_graph()?;
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 SINCRONIZAR GRAFO CON NETWORK_LEVELS Y SELF.GRAPH
            // ═══════════════════════════════════════════════════════════════════
            if let Some(level) = self.network_levels.first_mut() {
                level.graph = graph.clone();
            }
            self.graph = graph;
        } else {
            // Cargar normalmente si no necesita migración
            let graph = self.workspace.load_graph()?;
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 SINCRONIZAR GRAFO CON NETWORK_LEVELS Y SELF.GRAPH
            // ═══════════════════════════════════════════════════════════════════
            if let Some(level) = self.network_levels.first_mut() {
                level.graph = graph.clone();
            }
            self.graph = graph;
        }
        
        // Recalcular IDs en el grafo actual
        self.graph.recalculate_ids();
        
        // Recalcular IDs también para subnetworks recursivamente
        if let Some(level) = self.network_levels.first_mut() {
            level.graph.recalculate_ids();
            // Recalcular IDs en subnetworks anidados
            Self::recalculate_subnetwork_ids(&mut level.graph);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // 🆕 ASEGURAR SINCRONIZACIÓN FINAL
        // ═══════════════════════════════════════════════════════════════════
        self.sync_graph_to_current_level();
        
        self.interaction.selected_nodes.clear();
        self.last_save_hash = self.graph_hash();
        self.last_save_time = Some(std::time::Instant::now());
        Ok(())
    }
    
    /// Recalcular IDs recursivamente en subnetworks
    fn recalculate_subnetwork_ids(graph: &mut NodeGraph) {
        for node in graph.nodes_mut() {
            if let Some(ref mut subgraph) = node.subnetwork_graph {
                subgraph.recalculate_ids();
                Self::recalculate_subnetwork_ids(subgraph);
            }
        }
    }
    
    /// Sincronizar self.graph con el nivel actual de network_levels
    fn sync_graph_to_current_level(&mut self) {
        if let Some(level) = self.network_levels.last() {
            self.graph = level.graph.clone();
        }
    }
    
    /// Sincronizar el nivel actual de network_levels con self.graph
    fn sync_current_level_to_graph(&mut self) {
        if let Some(level) = self.network_levels.last_mut() {
            level.graph = self.graph.clone();
        }
    }

    pub fn graph_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        serde_json::to_string(&self.graph).unwrap_or_default().hash(&mut hasher);
        hasher.finish()
    }

    pub fn check_and_auto_save(&mut self) {
        if self.workspace.auto_save && self.workspace.has_root() {
            let current_hash = self.graph_hash();
            if current_hash != self.last_save_hash {
                if let Err(e) = self.save_current_graph() {
                    eprintln!("Auto-save error: {}", e);
                }
            }
        }
    }

    pub fn check_file_changes(&mut self) {
        // Solo verificar cambios si estamos en root (los subnetworks se guardan en el grafo raíz)
        if !self.is_at_root() {
            return;
        }
        
        if self.workspace.has_root() {
            if let Ok(node_map_path) = self.workspace.get_node_map_path().ok_or("") {
                if node_map_path.exists() {
                    if let Ok(metadata) = std::fs::metadata(&node_map_path) {
                        if let Ok(_modified) = metadata.modified() {
                            // Simple check: if file was modified externally, reload
                            // In a real implementation, you'd track the last known modification time
                            // For now, we'll just check if the hash differs
                            if let Ok(loaded_graph) = self.workspace.load_graph() {
                                let loaded_hash = {
                                    use std::collections::hash_map::DefaultHasher;
                                    use std::hash::{Hash, Hasher};
                                    let mut hasher = DefaultHasher::new();
                                    serde_json::to_string(&loaded_graph).unwrap_or_default().hash(&mut hasher);
                                    hasher.finish()
                                };
                                
                                if loaded_hash != self.last_save_hash && loaded_hash != self.graph_hash() {
                                    // File was modified externally, reload
                                    if let Err(e) = self.load_graph_from_workspace() {
                                        eprintln!("Error reloading graph: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_cut_tool(&mut self, painter: &egui::Painter, input: &PointerSnapshot, rect: Rect) {
        if !self.interaction.cut_tool.active {
            return;
        }

        // LIMPIEZA PREVENTIVA: Asegurar que no haya estado de selección activo
        // Esto previene cualquier interferencia visual o funcional
        self.interaction.dragging_node = None;
        self.interaction.box_selection_start = None;
        self.interaction.box_selection_current = None;

        // Dibujar línea de corte si ya existe (con animación)
        if self.interaction.cut_tool.has_points() {
            let time = painter.ctx().input(|i| i.time);
            self.interaction.cut_tool.draw_cut_line(painter, time);
        }

        if let Some(pointer_pos) = input.pos {
            // Iniciar línea de corte al hacer click
            if input.primary_pressed {
                // LIMPIAR INMEDIATAMENTE cualquier selección previa al iniciar corte
                // Esto previene que aparezca la selección de caja incluso con clicks rápidos
                self.interaction.selected_nodes.clear();
                self.interaction.dragging_node = None;
                self.interaction.box_selection_start = None;
                self.interaction.box_selection_current = None;
                
                // Si no hay puntos, iniciar una nueva línea de corte
                if self.interaction.cut_tool.is_empty() {
                    self.interaction.cut_tool.add_point(pointer_pos);
                }
            }

            // Agregar puntos mientras se arrastra (dibujo libre)
            if input.primary_down && self.interaction.cut_tool.has_points() {
                self.interaction.cut_tool.add_point(pointer_pos);
            }

            // Completar corte al soltar el mouse
            // Verificar que el mouse se soltó (primary_down pasó de true a false)
            if !input.primary_down && self.interaction.cut_tool.has_points() {
                // Solo procesar si hay suficientes puntos (línea válida)
                if self.interaction.cut_tool.points.len() >= 2 {
                    // Verificar que la línea tenga una longitud mínima
                    let total_length: f32 = self.interaction.cut_tool.points
                        .windows(2)
                        .map(|w| w[0].distance(w[1]))
                        .sum();
                    
                    if total_length > 5.0 {
                        // Encontrar y eliminar conexiones que intersectan
                        let links_to_remove: Vec<Link> = self.graph.links()
                            .iter()
                            .filter_map(|link| {
                                let Some(link_start) = self.pin_screen_position(link.from, rect) else {
                                    return None;
                                };
                                let Some(link_end) = self.pin_screen_position(link.to, rect) else {
                                    return None;
                                };

                                // Obtener puntos de la curva Bézier
                                let dist = link_start.distance(link_end);
                                let control_dist = dist.min(100.0 * self.viewport.zoom) * 0.5;
                                let bezier_points = [
                                    link_start,
                                    link_start + Vec2::new(control_dist, 0.0),
                                    link_end - Vec2::new(control_dist, 0.0),
                                    link_end,
                                ];

                                if self.interaction.cut_tool.check_intersection_with_bezier(bezier_points) {
                                    Some(link.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        // Eliminar conexiones que intersectan
                        if !links_to_remove.is_empty() {
                            for link in &links_to_remove {
                                self.graph.remove_link(link.from, link.to);
                            }
                            // Sincronizar cambios al nivel actual
                            self.sync_current_level_to_graph();
                            self.check_and_auto_save();
                        }
                    }
                }

                // Limpiar puntos de corte después de procesar
                self.interaction.cut_tool.clear();
            }
        }
    }

    fn draw_compiler_status(&mut self, ctx: &egui::Context) {
        if !self.show_compiler_status {
            return;
        }

        let mut update_requested = false;
        let status_clone = self.compiler_status.clone();
        
        egui::Window::new("Estado de Compiladores")
            .open(&mut self.show_compiler_status)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                if let Some(status) = &status_clone {
                    ui.vertical(|ui| {
                        // Botón para actualizar
                        if ui.button("🔄 Actualizar").clicked() {
                            update_requested = true;
                        }
                        ui.separator();
                        
                        // Mostrar resumen
                        ui.label(egui::RichText::new(status.summary())
                            .font(egui::FontId::monospace(12.0)));
                        
                        ui.separator();
                        
                        // Estado general
                        ui.horizontal(|ui| {
                            if status.is_ready() {
                                ui.colored_label(egui::Color32::from_rgb(76, 175, 80), 
                                    "✅ Todos los compiladores necesarios están disponibles");
                            } else {
                                ui.colored_label(egui::Color32::from_rgb(244, 67, 54), 
                                    "❌ Faltan algunos compiladores");
                            }
                        });
                        
                        ui.separator();
                        
                        // Información detallada
                        ui.collapsing("📋 Detalles Completos", |ui| {
                            // NASM
                            ui.horizontal(|ui| {
                                ui.label(if status.nasm.available { "✅" } else { "❌" });
                                ui.label("NASM (ASM):");
                                ui.label(&status.nasm.version);
                            });
                            if let Some(path) = &status.nasm.path {
                                ui.label(format!("  📍 {}", path.display()));
                            } else if !status.nasm.available {
                                ui.label("  ⚠️ No encontrado. Buscado en PATH y ubicaciones comunes.");
                            }
                            ui.add_space(8.0);
                            
                            // Rust
                            ui.horizontal(|ui| {
                                ui.label(if status.rust.available { "✅" } else { "❌" });
                                ui.label("Rust:");
                                ui.label(&status.rust.version);
                            });
                            if let Some(path) = &status.rust.path {
                                ui.label(format!("  📍 {}", path.display()));
                            } else if !status.rust.available {
                                ui.label("  ⚠️ No encontrado. Instala con: rustup");
                            }
                            ui.add_space(8.0);
                            
                            // Zig
                            ui.horizontal(|ui| {
                                ui.label(if status.zig.available { "✅" } else { "❌" });
                                ui.label("Zig:");
                                ui.label(&status.zig.version);
                            });
                            if let Some(path) = &status.zig.path {
                                ui.label(format!("  📍 {}", path.display()));
                            } else if !status.zig.available {
                                ui.label("  ⚠️ No encontrado. Descarga: https://ziglang.org/download/");
                            }
                            ui.add_space(8.0);
                            
                            // C++ Compilers
                            ui.label("Compiladores C++:");
                            ui.indent("cpp", |ui| {
                                // GCC
                                ui.horizontal(|ui| {
                                    ui.label(if status.cpp_gcc.available { "✅" } else { "❌" });
                                    ui.label("GCC/g++:");
                                    ui.label(&status.cpp_gcc.version);
                                });
                                if let Some(path) = &status.cpp_gcc.path {
                                    ui.label(format!("  📍 {}", path.display()));
                                }
                                ui.add_space(4.0);
                                
                                // Clang
                                ui.horizontal(|ui| {
                                    ui.label(if status.cpp_clang.available { "✅" } else { "❌" });
                                    ui.label("Clang++:");
                                    ui.label(&status.cpp_clang.version);
                                });
                                if let Some(path) = &status.cpp_clang.path {
                                    ui.label(format!("  📍 {}", path.display()));
                                }
                                ui.add_space(4.0);
                                
                                // MSVC
                                ui.horizontal(|ui| {
                                    ui.label(if status.cpp_msvc.available { "✅" } else { "❌" });
                                    ui.label("MSVC:");
                                    ui.label(&status.cpp_msvc.version);
                                });
                                if let Some(path) = &status.cpp_msvc.path {
                                    ui.label(format!("  📍 {}", path.display()));
                                }
                            });
                            
                            if let Some(cpp) = status.best_cpp_compiler() {
                                ui.add_space(8.0);
                                ui.label(format!("✅ Compilador C++ preferido: {} ({})", cpp.name, cpp.version));
                            } else {
                                ui.add_space(8.0);
                                ui.colored_label(egui::Color32::from_rgb(244, 67, 54), 
                                    "❌ No se encontró ningún compilador C++");
                            }
                        });
                    });
                } else {
                    ui.label("Detectando compiladores...");
                    if ui.button("Detectar ahora").clicked() {
                        update_requested = true;
                    }
                }
            });
            
        if update_requested {
            self.compiler_status = Some(detect_all_compilers());
        }
    }

    fn draw_migration_dialog(&mut self, ctx: &egui::Context) {
        if self.migration_dialog.is_none() {
            return;
        }

        let mut close_dialog = false;
        let mut should_migrate = false;
        
        // Clonar datos necesarios para mostrar el diálogo
        let backup_path_clone = self.migration_dialog.as_ref().and_then(|d| d.backup_path.clone());
        let result_clone = self.migration_dialog.as_ref().and_then(|d| d.result.clone());
        let error_clone = self.migration_dialog.as_ref().and_then(|d| d.error.clone());

        egui::Window::new("🔄 Migración de Proyecto")
            .collapsible(false)
            .resizable(false)
            .default_size([500.0, 350.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Migración Requerida");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.label(egui::RichText::new("Este proyecto usa el formato antiguo (código embebido en JSON).").color(Color32::from_rgb(255, 193, 7)));
                    ui.add_space(8.0);

                    ui.label("El nuevo formato separa el código en archivos individuales para:");
                    ui.label("  • Mejor compatibilidad con Git");
                    ui.label("  • Edición externa del código");
                    ui.label("  • Mejor rendimiento");
                    ui.add_space(8.0);

                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("⚠️ IMPORTANTE:").strong().color(Color32::from_rgb(244, 67, 54)));
                    ui.label("Se creará un backup automático antes de migrar.");
                    ui.add_space(8.0);

                    if let Some(ref backup_path) = backup_path_clone {
                        ui.label(egui::RichText::new(format!("📁 Backup: {}", backup_path.display())).small().color(Color32::from_rgb(76, 175, 80)));
                        ui.add_space(8.0);
                    }

                    if let Some(ref result) = result_clone {
                        ui.separator();
                        if result.migrated {
                            ui.colored_label(Color32::from_rgb(76, 175, 80), 
                                format!("✅ Migración exitosa: {} nodos migrados", result.nodes_migrated));
                            if !result.errors.is_empty() {
                                ui.label(egui::RichText::new(format!("⚠️ {} errores durante la migración", result.errors.len())).color(Color32::from_rgb(255, 152, 0)));
                            }
                            ui.add_space(8.0);
                            if ui.button("✅ Cerrar").clicked() {
                                close_dialog = true;
                            }
                        } else {
                            ui.colored_label(Color32::from_rgb(244, 67, 54), 
                                "❌ La migración falló");
                            if ui.button("❌ Cerrar").clicked() {
                                close_dialog = true;
                            }
                        }
                    } else if let Some(ref error) = error_clone {
                        ui.separator();
                        ui.colored_label(Color32::from_rgb(244, 67, 54), 
                            format!("❌ Error: {}", error));
                        ui.add_space(8.0);
                        if ui.button("❌ Cerrar").clicked() {
                            close_dialog = true;
                        }
                    } else {
                        ui.horizontal(|ui| {
                            ui.add_space(50.0);
                            if ui.add(egui::Button::new(egui::RichText::new("✅ Migrar").size(16.0))
                                .min_size(egui::Vec2::new(150.0, 40.0))).clicked() {
                                should_migrate = true;
                            }
                            ui.add_space(20.0);
                            if ui.add(egui::Button::new(egui::RichText::new("❌ Cancelar").size(16.0))
                                .min_size(egui::Vec2::new(150.0, 40.0))).clicked() {
                                close_dialog = true;
                            }
                        });
                    }
                });
            });

        // Procesar acciones DESPUÉS del bloque de UI
        if should_migrate {
            // Crear backup primero
            match create_backup(&self.workspace) {
                Ok(backup_path) => {
                    if let Some(ref mut dialog) = self.migration_dialog {
                        dialog.backup_path = Some(backup_path);
                    }
                    
                    // Ejecutar migración
                    match migrate_project(&self.workspace, &mut self.graph) {
                        Ok(result) => {
                            if let Some(ref mut dialog) = self.migration_dialog {
                                dialog.result = Some(result.clone());
                            }
                            
                            // Guardar el grafo migrado
                            if result.migrated {
                                if let Err(e) = self.save_current_graph() {
                                    if let Some(ref mut dialog) = self.migration_dialog {
                                        dialog.error = Some(format!("Error guardando proyecto migrado: {}", e));
                                    }
                                } else {
                                    // Recargar para asegurar que todo está sincronizado
                                    if let Err(e) = self.load_graph_from_workspace() {
                                        if let Some(ref mut dialog) = self.migration_dialog {
                                            dialog.error = Some(format!("Error recargando proyecto: {}", e));
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if let Some(ref mut dialog) = self.migration_dialog {
                                dialog.error = Some(format!("Error durante la migración: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    if let Some(ref mut dialog) = self.migration_dialog {
                        dialog.error = Some(format!("Error creando backup: {}", e));
                    }
                }
            }
        }

        if close_dialog {
            if let Some(ref mut dialog) = self.migration_dialog {
                dialog.show = false;
                if dialog.result.is_some() || dialog.error.is_some() {
                    // Cerrar completamente el diálogo después de mostrar resultado
                    self.migration_dialog = None;
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // 🆕 MÉTODOS PARA NAVEGACIÓN DE SUBNETWORKS
    // ═══════════════════════════════════════════════════════════════════

    /// Obtener el grafo actual (del nivel activo)
    /// Nota: self.graph siempre debe estar sincronizado con el nivel actual
    fn current_graph(&self) -> &NodeGraph {
        &self.graph
    }
    
    /// Obtener el grafo actual mutable (del nivel activo)
    /// Después de modificar, llamar sync_current_level_to_graph()
    fn current_graph_mut(&mut self) -> &mut NodeGraph {
        &mut self.graph
    }

    /// Entrar a un subnetwork (navegar hacia dentro)
    pub fn enter_subnetwork(&mut self, subnetwork_node_id: NodeId) -> Result<(), String> {
        // Obtener el grafo y título antes de hacer borrows mutables
        let (inner_graph, node_title) = {
            let current_graph = self.current_graph();
            let node = current_graph.node(subnetwork_node_id)
                .ok_or_else(|| format!("Node {} not found", subnetwork_node_id.0))?;
            
            let inner = node.subnetwork_graph.as_ref()
                .cloned()
                .ok_or_else(|| format!("Node {} is not a subnetwork", subnetwork_node_id.0))?;
            
            (inner, node.title.clone())
        };

        // Obtener breadcrumbs actuales (clonar antes de modificar)
        let mut breadcrumbs = self.network_levels.last()
            .map(|l| l.breadcrumbs.clone())
            .unwrap_or_else(|| vec!["Root".to_string()]);
        breadcrumbs.push(node_title);

        // Crear nuevo nivel
        let new_level = NetworkLevel {
            graph: inner_graph.clone(),
            parent_subnetwork_id: Some(subnetwork_node_id),
            breadcrumbs,
        };

        // Primero sincronizar el grafo actual antes de entrar (por si hay cambios pendientes)
        // Esto guarda cualquier cambio pendiente en el nivel actual antes de navegar
        self.sync_current_level_to_graph();
        
        // Ahora crear el nuevo nivel y actualizar el grafo visible
        self.network_levels.push(new_level);
        self.graph = inner_graph;
        
        Ok(())
    }

    /// Salir del subnetwork actual (volver al nivel padre)
    pub fn exit_subnetwork(&mut self) -> Result<(), String> {
        if self.network_levels.len() <= 1 {
            return Err("Already at root level".to_string());
        }

        // Primero sincronizar el grafo actual antes de salir (por si hay cambios pendientes)
        self.sync_current_level_to_graph();

        // Clonar el nivel actual y su índice antes de hacer borrows mutables
        let current_graph = self.network_levels.last().map(|l| l.graph.clone())
            .ok_or_else(|| "No current level".to_string())?;
        let parent_id = self.network_levels.last()
            .and_then(|l| l.parent_subnetwork_id);

        // Guardar el grafo del nivel actual de vuelta al nodo subnetwork
        if let Some(parent_id) = parent_id {
            // Buscar el nivel padre (el penúltimo en la pila)
            if self.network_levels.len() >= 2 {
                let parent_idx = self.network_levels.len() - 2;
                if let Some(parent_level) = self.network_levels.get_mut(parent_idx) {
                    if let Some(node) = parent_level.graph.node_mut(parent_id) {
                        node.subnetwork_graph = Some(current_graph);
                    }
                }
            }
        }

        // Volver al nivel anterior
        self.network_levels.pop();
        
        // Actualizar el grafo visible y asegurar sincronización
        if let Some(level) = self.network_levels.last() {
            self.graph = level.graph.clone();
        }
        
        Ok(())
    }

    /// Obtener los breadcrumbs actuales
    pub fn get_breadcrumbs(&self) -> Vec<String> {
        self.network_levels.last()
            .map(|l| l.breadcrumbs.clone())
            .unwrap_or_else(|| vec!["Root".to_string()])
    }

    /// Verificar si estamos en el nivel raíz
    pub fn is_at_root(&self) -> bool {
        self.network_levels.len() == 1
    }
    
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 DIÁLOGOS DE HDA (Houdini Digital Assets)
    // ═══════════════════════════════════════════════════════════════════
    
    /// Diálogo para exportar HDA
    fn hda_export_dialog_ui(&mut self, ctx: &egui::Context) {
        if !self.show_hda_export_dialog {
            return;
        }
        
        let mut open = self.show_hda_export_dialog;
        let mut should_close = false;
        egui::Window::new("📦 Exportar HDA (Houdini Digital Asset)")
            .open(&mut open)
            .resizable(true)
            .default_size([600.0, 500.0])
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Exportar Asset Reutilizable");
                    ui.separator();
                    
                    // Campos del formulario
                    ui.horizontal(|ui| {
                        ui.label("Nombre (ID único):");
                        ui.text_edit_singleline(&mut self.hda_export_name);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Etiqueta:");
                        ui.text_edit_singleline(&mut self.hda_export_label);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Descripción:");
                        ui.add(egui::TextEdit::multiline(&mut self.hda_export_description).desired_rows(3));
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Autor:");
                        ui.text_edit_singleline(&mut self.hda_export_author);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Categoría:");
                        ui.text_edit_singleline(&mut self.hda_export_category);
                    });
                    
                    ui.checkbox(&mut self.hda_export_to_global, "Exportar a assets globales (compartido entre proyectos)");
                    
                    ui.separator();
                    
                    // ═══════════════════════════════════════════════════════════════════
                    // PARÁMETROS CONFIGURABLES
                    // ═══════════════════════════════════════════════════════════════════
                    ui.heading("⚙️ Parámetros Configurables");
                    ui.label(egui::RichText::new("Define parámetros que pueden ser configurados al usar este asset").small().color(egui::Color32::GRAY));
                    
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            let mut to_remove = None;
                            for (idx, param) in self.hda_export_parameters.iter_mut().enumerate() {
                                egui::Frame::none()
                                    .fill(egui::Color32::from_rgba_unmultiplied(50, 55, 65, 200))
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label("Nombre:");
                                                    ui.text_edit_singleline(&mut param.name);
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.label("Etiqueta:");
                                                    ui.text_edit_singleline(&mut param.label);
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.label("Tipo:");
                                                    let type_names: Vec<&str> = crate::storage::ParameterType::all_types().iter().map(|t| t.as_str()).collect();
                                                    let current_type_str = param.param_type.as_str();
                                                    let mut selected_type_idx = type_names.iter().position(|&s| s == current_type_str).unwrap_or(0);
                                                    egui::ComboBox::from_id_source(format!("param_type_{}", idx))
                                                        .selected_text(type_names[selected_type_idx])
                                                        .show_ui(ui, |ui| {
                                                            for (i, type_name) in type_names.iter().enumerate() {
                                                                if ui.selectable_value(&mut selected_type_idx, i, *type_name).clicked() {
                                                                    if let Some(new_type) = crate::storage::ParameterType::from_str(type_name) {
                                                                        param.param_type = new_type;
                                                                    }
                                                                }
                                                            }
                                                        });
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.label("Valor por defecto:");
                                                    ui.text_edit_singleline(&mut param.default_value);
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.label("Descripción:");
                                                    ui.text_edit_singleline(&mut param.description);
                                                });
                                                // Min/Max solo para Float e Int
                                                if matches!(param.param_type, crate::storage::ParameterType::Float | crate::storage::ParameterType::Int) {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Min:");
                                                        let mut min_str = param.min_value.map(|v| v.to_string()).unwrap_or_default();
                                                        if ui.text_edit_singleline(&mut min_str).changed() {
                                                            param.min_value = min_str.parse().ok();
                                                        }
                                                        ui.label("Max:");
                                                        let mut max_str = param.max_value.map(|v| v.to_string()).unwrap_or_default();
                                                        if ui.text_edit_singleline(&mut max_str).changed() {
                                                            param.max_value = max_str.parse().ok();
                                                        }
                                                    });
                                                }
                                                // Choices solo para Enum
                                                if matches!(param.param_type, crate::storage::ParameterType::Enum) {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Opciones (separadas por coma):");
                                                        let mut choices_str = param.choices.as_ref()
                                                            .map(|v| v.join(", "))
                                                            .unwrap_or_default();
                                                        if ui.text_edit_singleline(&mut choices_str).changed() {
                                                            param.choices = if choices_str.is_empty() {
                                                                None
                                                            } else {
                                                                Some(choices_str.split(',').map(|s| s.trim().to_string()).collect())
                                                            };
                                                        }
                                                    });
                                                }
                                            });
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                                if ui.button("❌").clicked() {
                                                    to_remove = Some(idx);
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(4.0);
                            }
                            if let Some(idx) = to_remove {
                                self.hda_export_parameters.remove(idx);
                            }
                        });
                    
                    ui.horizontal(|ui| {
                        if ui.button("➕ Agregar Parámetro").clicked() {
                            self.hda_export_parameters.push(crate::storage::HDAParameter {
                                name: format!("param_{}", self.hda_export_parameters.len() + 1),
                                label: format!("Parámetro {}", self.hda_export_parameters.len() + 1),
                                description: String::new(),
                                param_type: crate::storage::ParameterType::String,
                                default_value: String::new(),
                                min_value: None,
                                max_value: None,
                                choices: None,
                            });
                        }
                    });
                    
                    ui.separator();
                    
                    // Información sobre qué se va a exportar
                    let node_count = self.interaction.selected_nodes.len();
                    let is_subnetwork = if let Some(&selected_id) = self.interaction.selected_nodes.iter().next() {
                        self.graph.node(selected_id).map(|n| n.subnetwork_graph.is_some()).unwrap_or(false)
                    } else {
                        false
                    };
                    
                    ui.horizontal(|ui| {
                        let msg = if is_subnetwork {
                            let node_count_internal = if let Some(&selected_id) = self.interaction.selected_nodes.iter().next() {
                                self.graph.node(selected_id)
                                    .and_then(|n| n.subnetwork_graph.as_ref())
                                    .map(|g| g.nodes().len())
                                    .unwrap_or(0)
                            } else {
                                0
                            };
                            format!("📁 Exportando subnetwork ({} nodos internos)", node_count_internal)
                        } else {
                            format!("📦 Exportando {} nodos seleccionados", node_count)
                        };
                        ui.label(egui::RichText::new(msg).color(egui::Color32::from_rgb(150, 200, 255)));
                    });
                    
                    ui.separator();
                    
                    // Botones
                    ui.horizontal(|ui| {
                        if ui.button("❌ Cancelar").clicked() {
                            should_close = true;
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let can_export = !self.hda_export_name.is_empty() && 
                                           (!self.interaction.selected_nodes.is_empty() || is_subnetwork);
                            
                            if ui.add_enabled(can_export, egui::Button::new("✅ Exportar")).clicked() {
                                if let Some(root_path) = &self.workspace.root_path {
                                    let manager = crate::storage::HDAManager::new(root_path);
                                    
                                    let hda_result = if is_subnetwork {
                                        // Exportar desde subnetwork
                                        if let Some(&selected_id) = self.interaction.selected_nodes.iter().next() {
                                            if let Some(node) = self.graph.node(selected_id) {
                                                match crate::storage::create_hda_from_subnetwork(node) {
                                                    Ok(mut hda) => {
                                                        hda.name = self.hda_export_name.clone();
                                                        hda.label = self.hda_export_label.clone();
                                                        hda.description = self.hda_export_description.clone();
                                                        hda.author = self.hda_export_author.clone();
                                                        hda.category = self.hda_export_category.clone();
                                                        hda.parameters = self.hda_export_parameters.clone();
                                                        Ok(hda)
                                                    }
                                                    Err(e) => Err(e),
                                                }
                                            } else {
                                                Err("Nodo no encontrado".to_string())
                                            }
                                        } else {
                                            Err("No hay nodo seleccionado".to_string())
                                        }
                                    } else {
                                        // Exportar desde nodos seleccionados
                                        let node_ids: Vec<_> = self.interaction.selected_nodes.iter().copied().collect();
                                        match crate::storage::create_hda_from_nodes(
                                            &self.graph,
                                            &node_ids,
                                            self.hda_export_name.clone(),
                                            self.hda_export_label.clone(),
                                            self.hda_export_description.clone(),
                                        ) {
                                            Ok(mut hda) => {
                                                hda.author = self.hda_export_author.clone();
                                                hda.category = self.hda_export_category.clone();
                                                hda.parameters = self.hda_export_parameters.clone();
                                                Ok(hda)
                                            }
                                            Err(e) => Err(e),
                                        }
                                    };
                                    
                                    match hda_result {
                                        Ok(hda) => {
                                            match manager.export_hda(&hda, self.hda_export_to_global) {
                                                Ok(asset_path) => {
                                                    self.terminal.visible = true;
                                                    self.terminal.rust_output = format!(
                                                        "✅ HDA exportado exitosamente!\n\n\
                                                        Nombre: {}\n\
                                                        Ubicación: {}\n\
                                                        Nodos: {}\n\
                                                        Parámetros: {}",
                                                        hda.name,
                                                        asset_path.display(),
                                                        hda.graph.nodes().len(),
                                                        hda.parameters.len()
                                                    );
                                                    self.terminal.active_tab = TerminalTab::Rust;
                                                    
                                                    // Limpiar campos
                                                    self.hda_export_name.clear();
                                                    self.hda_export_label.clear();
                                                    self.hda_export_description.clear();
                                                    self.hda_export_author.clear();
                                                    self.hda_export_category = "General".to_string();
                                                    self.hda_export_parameters.clear();
                                                    
                                                    should_close = true;
                                                }
                                                Err(e) => {
                                                    self.terminal.visible = true;
                                                    self.terminal.rust_output = format!("❌ Error al exportar HDA: {}", e);
                                                    self.terminal.active_tab = TerminalTab::Rust;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            self.terminal.visible = true;
                                            self.terminal.rust_output = format!("❌ Error al crear HDA: {}", e);
                                            self.terminal.active_tab = TerminalTab::Rust;
                                        }
                                    }
                                } else {
                                    self.terminal.visible = true;
                                    self.terminal.rust_output = "❌ No hay workspace abierto. Abre un workspace primero.".to_string();
                                    self.terminal.active_tab = TerminalTab::Rust;
                                }
                            }
                        });
                    });
                });
            });
        
        self.show_hda_export_dialog = open;
        if should_close {
            self.show_hda_export_dialog = false;
        }
    }
    
    /// Diálogo para importar HDA
    fn hda_import_dialog_ui(&mut self, ctx: &egui::Context) {
        if !self.show_hda_import_dialog {
            return;
        }
        
        let mut open = self.show_hda_import_dialog;
        let mut should_close = false;
        egui::Window::new("📥 Importar HDA (Houdini Digital Asset)")
            .open(&mut open)
            .resizable(true)
            .default_size([700.0, 600.0])
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Importar Asset Reutilizable");
                    ui.separator();
                    
                    // Listar HDAs disponibles
                    let hdas = if let Some(root_path) = &self.workspace.root_path {
                        let manager = crate::storage::HDAManager::new(root_path);
                        manager.list_available_hdas()
                    } else {
                        Vec::new()
                    };
                    
                    if hdas.is_empty() {
                        ui.label(egui::RichText::new("📭 No hay HDAs disponibles").color(egui::Color32::GRAY));
                        ui.label("Exporta algunos nodos como HDA primero.");
                    } else {
                        ui.label(format!("📦 {} HDAs disponibles:", hdas.len()));
                        ui.separator();
                        
                        egui::ScrollArea::vertical()
                            .max_height(400.0)
                            .show(ui, |ui| {
                                for (asset_path, info) in &hdas {
                                    egui::Frame::none()
                                        .fill(egui::Color32::from_rgba_unmultiplied(40, 45, 55, 150))
                                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255)))
                                        .rounding(egui::Rounding::same(6.0))
                                        .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(&info.icon).size(24.0));
                                                ui.add_space(10.0);
                                                
                                                ui.vertical(|ui| {
                                                    ui.label(egui::RichText::new(&info.label).strong().size(14.0));
                                                    if !info.description.is_empty() {
                                                        ui.label(egui::RichText::new(&info.description).small().color(egui::Color32::GRAY));
                                                    }
                                                    ui.horizontal(|ui| {
                                                        ui.label(egui::RichText::new(format!("v{}", info.version)).small().color(egui::Color32::from_rgb(150, 200, 255)));
                                                        ui.label(egui::RichText::new(" • ").small().color(egui::Color32::GRAY));
                                                        ui.label(egui::RichText::new(format!("{} nodos", info.node_count)).small().color(egui::Color32::GRAY));
                                                        if info.is_global {
                                                            ui.label(egui::RichText::new(" • 🌐 Global").small().color(egui::Color32::from_rgb(200, 150, 80)));
                                                        }
                                                    });
                                                });
                                                
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    if ui.button("📥 Importar").clicked() {
                                                        let manager = crate::storage::HDAManager::new(
                                                            self.workspace.root_path.as_ref().unwrap()
                                                        );
                                                        
                                                        match manager.import_hda(asset_path) {
                                                            Ok(hda) => {
                                                                // Si el HDA tiene parámetros, prepararlos para configuración
                                                                if !hda.parameters.is_empty() {
                                                                    // Inicializar valores de parámetros con defaults
                                                                    self.hda_import_parameter_values.clear();
                                                                    for param in &hda.parameters {
                                                                        self.hda_import_parameter_values.insert(
                                                                            param.name.clone(),
                                                                            param.default_value.clone()
                                                                        );
                                                                    }
                                                                    // Guardar el HDA seleccionado para configurar parámetros
                                                                    self.hda_import_selected_asset = Some((asset_path.clone(), info.clone()));
                                                                    // Mostrar diálogo de configuración de parámetros
                                                                    // (continuará en el siguiente frame)
                                                                } else {
                                                                    // Importar directamente si no hay parámetros
                                                                    self.import_hda_with_parameters(hda, ctx);
                                                                    should_close = true;
                                                                }
                                                            }
                                                            Err(e) => {
                                                                self.terminal.visible = true;
                                                                self.terminal.rust_output = format!("❌ Error al importar HDA: {}", e);
                                                                self.terminal.active_tab = TerminalTab::Rust;
                                                            }
                                                        }
                                                    }
                                                });
                                            });
                                        });
                                    ui.add_space(8.0);
                                }
                            });
                    }
                    
                    ui.separator();
                    
                    // Botón cancelar
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌ Cerrar").clicked() {
                                should_close = true;
                            }
                        });
                    });
                });
            });
        
        self.show_hda_import_dialog = open;
        if should_close {
            self.show_hda_import_dialog = false;
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // DIÁLOGO DE CONFIGURACIÓN DE PARÁMETROS (si hay un HDA seleccionado)
        // ═══════════════════════════════════════════════════════════════════
        if let Some((ref asset_path, ref info)) = self.hda_import_selected_asset {
            // Clonar datos necesarios antes del closure
            let asset_path_clone = asset_path.clone();
            let info_label = info.label.clone();
            let workspace_root = self.workspace.root_path.clone();
            let mut config_open = true;
            
            egui::Window::new("⚙️ Configurar Parámetros del HDA")
                .open(&mut config_open)
                .resizable(true)
                .default_size([500.0, 400.0])
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(format!("Configurar: {}", info_label));
                        ui.separator();
                        
                        // Cargar HDA para mostrar parámetros
                        if let Some(ref root_path) = workspace_root {
                            let manager = crate::storage::HDAManager::new(root_path);
                            if let Ok(hda) = manager.import_hda(&asset_path_clone) {
                                ui.label(egui::RichText::new("Ajusta los parámetros antes de importar:").small().color(egui::Color32::GRAY));
                                ui.separator();
                                
                                egui::ScrollArea::vertical()
                                    .max_height(250.0)
                                    .show(ui, |ui| {
                                        for param in &hda.parameters {
                                            egui::Frame::none()
                                                .fill(egui::Color32::from_rgba_unmultiplied(50, 55, 65, 200))
                                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 150, 255)))
                                                .rounding(egui::Rounding::same(6.0))
                                                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                                .show(ui, |ui| {
                                                    ui.vertical(|ui| {
                                                        ui.label(egui::RichText::new(&param.label).strong());
                                                        if !param.description.is_empty() {
                                                            ui.label(egui::RichText::new(&param.description).small().color(egui::Color32::GRAY));
                                                        }
                                                        
                                                        let value = self.hda_import_parameter_values
                                                            .entry(param.name.clone())
                                                            .or_insert_with(|| param.default_value.clone());
                                                        
                                                        match param.param_type {
                                                            crate::storage::ParameterType::Bool => {
                                                                let mut bool_val = value.parse().unwrap_or(false);
                                                                if ui.checkbox(&mut bool_val, "").changed() {
                                                                    *value = bool_val.to_string();
                                                                }
                                                            }
                                                            crate::storage::ParameterType::Int => {
                                                                let mut int_val = value.parse().unwrap_or(0);
                                                                ui.horizontal(|ui| {
                                                                    ui.label("Valor:");
                                                                    if let (Some(min), Some(max)) = (param.min_value, param.max_value) {
                                                                        ui.add(egui::DragValue::new(&mut int_val).clamp_range(min as i64..=max as i64));
                                                                    } else if let Some(min) = param.min_value {
                                                                        ui.add(egui::DragValue::new(&mut int_val).clamp_range(min as i64..=i64::MAX));
                                                                    } else if let Some(max) = param.max_value {
                                                                        ui.add(egui::DragValue::new(&mut int_val).clamp_range(i64::MIN..=max as i64));
                                                                    } else {
                                                                        ui.add(egui::DragValue::new(&mut int_val));
                                                                    }
                                                                    *value = int_val.to_string();
                                                                });
                                                            }
                                                            crate::storage::ParameterType::Float => {
                                                                let mut float_val = value.parse().unwrap_or(0.0);
                                                                ui.horizontal(|ui| {
                                                                    ui.label("Valor:");
                                                                    if let (Some(min), Some(max)) = (param.min_value, param.max_value) {
                                                                        ui.add(egui::DragValue::new(&mut float_val).clamp_range(min..=max));
                                                                    } else if let Some(min) = param.min_value {
                                                                        ui.add(egui::DragValue::new(&mut float_val).clamp_range(min..=f64::MAX));
                                                                    } else if let Some(max) = param.max_value {
                                                                        ui.add(egui::DragValue::new(&mut float_val).clamp_range(f64::MIN..=max));
                                                                    } else {
                                                                        ui.add(egui::DragValue::new(&mut float_val));
                                                                    }
                                                                    *value = float_val.to_string();
                                                                });
                                                            }
                                                            crate::storage::ParameterType::Enum => {
                                                                if let Some(ref choices) = param.choices {
                                                                    let current_idx = choices.iter().position(|c| c == value).unwrap_or(0);
                                                                    let mut selected_idx = current_idx;
                                                                    egui::ComboBox::from_id_source(&param.name)
                                                                        .selected_text(value.clone())
                                                                        .show_ui(ui, |ui| {
                                                                            for (i, choice) in choices.iter().enumerate() {
                                                                                if ui.selectable_value(&mut selected_idx, i, choice).clicked() {
                                                                                    *value = choice.clone();
                                                                                }
                                                                            }
                                                                        });
                                                                } else {
                                                                    ui.text_edit_singleline(value);
                                                                }
                                                            }
                                                            _ => {
                                                                ui.text_edit_singleline(value);
                                                            }
                                                        }
                                                    });
                                                });
                                            ui.add_space(4.0);
                                        }
                                    });
                                
                                ui.separator();
                                
                                ui.horizontal(|ui| {
                                    if ui.button("❌ Cancelar").clicked() {
                                        self.hda_import_selected_asset = None;
                                        self.hda_import_parameter_values.clear();
                                    }
                                    
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("✅ Importar con Parámetros").clicked() {
                                            // Usar una flag para indicar que se debe importar
                                            // (se procesará después del cierre del closure)
                                            ctx.data_mut(|data| {
                                                data.insert_temp(egui::Id::new("hda_import_flag"), true);
                                            });
                                        }
                                    });
                                });
                            }
                        }
                    });
                });
            
            if !config_open {
                self.hda_import_selected_asset = None;
                self.hda_import_parameter_values.clear();
            }
            
            // Verificar si se debe importar (después del cierre del closure)
            if ctx.data(|data| data.get_temp::<bool>(egui::Id::new("hda_import_flag")).unwrap_or(false)) {
                ctx.data_mut(|data| {
                    data.remove::<bool>(egui::Id::new("hda_import_flag"));
                });
                
                // Cargar HDA e importar con parámetros
                if let Some(ref root_path) = self.workspace.root_path {
                    let manager = crate::storage::HDAManager::new(root_path);
                    if let Ok(hda) = manager.import_hda(&asset_path_clone) {
                        self.import_hda_with_parameters(hda, ctx);
                        self.hda_import_selected_asset = None;
                        self.hda_import_parameter_values.clear();
                        self.show_hda_import_dialog = false;
                    }
                }
            }
        }
    }
    
    /// Importar HDA aplicando valores de parámetros
    fn import_hda_with_parameters(&mut self, mut hda: crate::storage::HDA, ctx: &egui::Context) {
        // Aplicar valores de parámetros al código del HDA
        // Buscar en el código del HDA referencias a parámetros y reemplazarlas
        for node in hda.graph.nodes_mut() {
            // Reemplazar referencias de parámetros en el código
            for (param_name, param_value) in &self.hda_import_parameter_values {
                // Reemplazar ${param_name} o {param_name} con el valor
                node.code = node.code.replace(&format!("${{{}}}", param_name), param_value);
                node.code = node.code.replace(&format!("{{{}}}", param_name), param_value);
                node.code = node.code.replace(&format!("${}", param_name), param_value);
            }
        }
        
        // Agregar nodos del HDA al grafo actual
        let world_pos = self.viewport.screen_to_world(
            ctx.pointer_hover_pos().unwrap_or(pos2(400.0, 300.0)),
            ctx.screen_rect()
        );
        
        // Crear un nodo contenedor para el HDA (como un subnetwork)
        let input_slices: Vec<&str> = hda.exposed_inputs.iter().map(|s| s.as_str()).collect();
        let output_slices: Vec<&str> = hda.exposed_outputs.iter().map(|s| s.as_str()).collect();
        let container_id = self.graph.add_node(
            &hda.label,
            world_pos,
            egui::Color32::from_rgb(150, 100, 200),
            &input_slices,
            &output_slices,
            NodeLanguage::Auto,
        );
        
        if let Some(node) = self.graph.node_mut(container_id) {
            node.subnetwork_graph = Some(hda.graph.clone());
        }
        
        self.sync_current_level_to_graph();
        
        self.terminal.visible = true;
        self.terminal.rust_output = format!(
            "✅ HDA importado exitosamente!\n\n\
            Nombre: {}\n\
            Nodos: {}\n\
            Parámetros configurados: {}",
            hda.label,
            hda.graph.nodes().len(),
            self.hda_import_parameter_values.len()
        );
        self.terminal.active_tab = TerminalTab::Rust;
        
        if self.workspace.has_root() {
            let _ = self.save_current_graph();
        }
    }
}


