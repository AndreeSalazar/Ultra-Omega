use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2, Visuals, pos2, PointerButton};
use crate::node_graph::{self, NodeGraph, NodeId, PinId, PinKind};
use crate::terminal::{TerminalManager, TerminalTab};
use crate::ui::viewport::Viewport2D;
use crate::workspace::Workspace;
use crate::config::AppConfig;

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
    pub editor_history: Option<crate::editor_history::EditorHistory>, // Historial del editor
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
        
        let mut app = Self {
            graph: NodeGraph::default(),
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
                }
            } else {
                app.graph = NodeGraph::demo();
                app.graph.recalculate_ids();
            }
        } else {
            app.graph = NodeGraph::demo();
            app.graph.recalculate_ids();
        }
        
        // Registrar todos los nodos existentes en el sistema de canales
        let nodes_to_register: Vec<_> = app.graph.nodes().iter().map(|n| (n.id, n.title.clone(), n.code.clone())).collect();
        for (node_id, title, code) in nodes_to_register {
            use crate::expressions::ChannelValue;
            app.channel_manager.set_channel(title.clone(), ChannelValue::Code(code.clone()));
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

        // Handle Delete key to remove selected nodes
        if ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
            if !self.interaction.selected_nodes.is_empty() {
                // Don't delete if editing a node
                if self.interaction.editing_node.is_none() {
                    // Limpiar canales de los nodos que se van a eliminar
                    for node_id in &self.interaction.selected_nodes {
                        self.channel_manager.clear_node_channels(*node_id);
                        // También limpiar por nombre si existe
                        if let Some(node) = self.graph.nodes().iter().find(|n| n.id == *node_id) {
                            // Nota: No podemos eliminar del HashMap directamente aquí, pero el canal quedará obsoleto
                            // Se actualizará cuando se registre un nuevo nodo con el mismo nombre
                        }
                    }
                    self.graph.remove_nodes(&self.interaction.selected_nodes);
                    self.interaction.selected_nodes.clear();
                    // Auto-save after deletion
                    self.check_and_auto_save();
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

    fn node_rect_at_scale(&self, node: &node_graph::Node, scale: f32) -> Rect {
        let size = self.node_size(node) * scale;
        // Position is already in world coordinates
        Rect::from_min_size(node.position, size)
    }

    fn add_template_node(&mut self, ctx: &egui::Context, title: &str, code: &str, color: Color32) {
        let world_pos = self.viewport.screen_to_world(self.node_menu_pos, Rect::from_min_size(Pos2::ZERO, Vec2::new(10000.0, 10000.0)));
        
        // Todos los nodos tienen una entrada "Entrada" y una salida "Código"
        let id = self.graph.add_node(title, world_pos, color, &["Entrada"], &["Código"]);
        let (node_title, node_code) = {
            if let Some(node) = self.graph.node_mut(id) {
                node.code = code.to_string();
                (node.title.clone(), node.code.clone())
            } else {
                return;
            }
        };
        // Registrar nodo en el sistema de canales (después de soltar el borrow)
        use crate::expressions::ChannelValue;
        self.channel_manager.set_channel(node_title.clone(), ChannelValue::Code(node_code.clone()));
        self.channel_manager.set_node_channel(id, "code".to_string(), ChannelValue::Code(node_code));
        
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
    
    /// Registrar un nodo en el sistema de canales para acceso mediante ch()
    fn register_node_in_channels(&mut self, node_id: crate::node_graph::NodeId, node: &crate::node_graph::Node) {
        use crate::expressions::ChannelValue;
        // Registrar por nombre del nodo
        self.channel_manager.set_channel(
            node.title.clone(),
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
    pub fn update_node_channels(&mut self, node_id: crate::node_graph::NodeId) {
        // Clonar datos para evitar problemas de borrow
        if let Some(node) = self.graph.nodes().iter().find(|n| n.id == node_id) {
            let title = node.title.clone();
            let code = node.code.clone();
            use crate::expressions::ChannelValue;
            self.channel_manager.set_channel(title.clone(), ChannelValue::Code(code.clone()));
            self.channel_manager.set_node_channel(node_id, "code".to_string(), ChannelValue::Code(code));
        }
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
                                ui.set_width(140.0);
                                
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("➕").color(Color32::from_rgb(100, 200, 255)));
                                    ui.label(egui::RichText::new("Add").strong());
                                    ui.label(egui::RichText::new("Shift A").small().color(Color32::GRAY));
                                });
                                ui.separator();
                                
                                // Nodo nuevo
                                if ui.selectable_label(self.selected_category.as_deref() == Some("new"), "✨ Nuevo Nodo").clicked() {
                                    self.selected_category = Some("new".to_string());
                                }
                                
                                ui.separator();
                                
                                // Categorías de templates
                                let categories = ["Assembler", "C", "C++", "Rust"];
                                let category_icons = ["🔧", "📘", "📗", "🦀"];
                                let category_colors = [
                                    Color32::from_rgb(0xff, 0x47, 0x00),
                                    Color32::from_rgb(0x00, 0x59, 0x9C),
                                    Color32::from_rgb(0x00, 0x44, 0x82),
                                    Color32::from_rgb(0xde, 0x39, 0x00),
                                ];
                                
                                for (i, cat) in categories.iter().enumerate() {
                                    let selected = self.selected_category.as_deref() == Some(*cat);
                                    let response = ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(category_icons[i]).color(category_colors[i]));
                                        ui.selectable_label(selected, *cat)
                                    }).inner;
                                    
                                    if response.clicked() {
                                        self.selected_category = Some(cat.to_string());
                                    }
                                }
                                
                                ui.add_space(8.0);
                                ui.separator();
                                ui.add_space(4.0);
                                
                                // Shortcuts
                                ui.label(egui::RichText::new("Buscar").small().color(Color32::GRAY));
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("🔍 Find...").small());
                                    ui.label(egui::RichText::new("F3").small().color(Color32::from_rgb(100, 150, 200)));
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
                                            let id = self.graph.add_node(&title, world_pos, Color32::from_rgb(100, 150, 200), &["Entrada"], &["Código"]);
                                            
                                            if let Some(node) = self.graph.nodes().iter().find(|n| n.id == id) {
                                                let title_clone = node.title.clone();
                                                let code_clone = node.code.clone();
                                                use crate::expressions::ChannelValue;
                                                self.channel_manager.set_channel(title_clone.clone(), ChannelValue::Code(code_clone.clone()));
                                                self.channel_manager.set_node_channel(id, "code".to_string(), ChannelValue::Code(code_clone));
                                            }
                                            
                                            self.new_node_title.clear();
                                            
                                            if self.workspace.has_root() {
                                                let _ = self.save_current_graph();
                                            }
                                            
                                            close_menu = true;
                                        }
                                    } else {
                                        // Mostrar templates de la categoría seleccionada
                                        let templates = crate::templates::all_templates();
                                        let filtered: Vec<_> = templates.iter()
                                            .filter(|t| t.category == cat.as_str())
                                            .collect();
                                        
                                        // Agrupar por subcategoría
                                        let mut subcats: Vec<&str> = filtered.iter()
                                            .map(|t| t.subcategory)
                                            .collect();
                                        subcats.sort();
                                        subcats.dedup();
                                        
                                        egui::ScrollArea::vertical()
                                            .max_height(300.0)
                                            .show(ui, |ui| {
                                                for subcat in subcats {
                                                    ui.label(egui::RichText::new(subcat).strong().color(Color32::from_rgb(150, 150, 150)));
                                                    ui.add_space(4.0);
                                                    
                                                    for template in filtered.iter().filter(|t| t.subcategory == subcat) {
                                                        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
                                                        let btn_text = format!("{} {}", template.icon, template.name);
                                                        
                                                        if ui.add(egui::Button::new(
                                                            egui::RichText::new(&btn_text).color(color)
                                                        ).min_size(Vec2::new(180.0, 24.0))).clicked() {
                                                            template_to_add = Some((*template).clone());
                                                            close_menu = true;
                                                        }
                                                    }
                                                    
                                                    ui.add_space(8.0);
                                                }
                                            });
                                    }
                                } else {
                                    // Sin categoría seleccionada
                                    ui.centered_and_justified(|ui| {
                                        ui.label(egui::RichText::new("← Selecciona una categoría").color(Color32::GRAY));
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
                self.add_template_node(ctx, &title, template.code, color);
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
                self.add_template_node(ctx, &title, template.code, color);
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
                    self.interaction.editor_history = Some(crate::editor_history::EditorHistory::new(id, initial_code));
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
                        let node_code_len = self.graph.nodes().iter()
                            .find(|n| n.id == id)
                            .map(|n| n.code.lines().count())
                            .unwrap_or(1);
                        
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
                            if let Some(node) = self.graph.nodes().iter().find(|n| n.id == id) {
                                let lang = if node.title.contains("ASM") {
                                    crate::terminal::Language::Nasm
                                } else if node.title.contains("C++") {
                                    crate::terminal::Language::Cpp
                                } else if node.title.contains("Rust") {
                                    crate::terminal::Language::Rust
                                } else {
                                    crate::terminal::Language::C
                                };
                                let workspace_path = self.workspace.root_path.as_ref();
                                // Combinar código heredado + propio para ejecutar
                                let inherited_raw = self.graph.get_inherited_code(id).unwrap_or_default();
                                // Evaluar ch() en código heredado
                                let inherited = self.evaluate_ch_expressions_in_code(&inherited_raw, id);
                                // Evaluar ch() en código propio también
                                let own_code_evaluated = self.evaluate_ch_expressions_in_code(&node.code, id);
                                
                                let full_code = if !inherited.is_empty() && !own_code_evaluated.is_empty() {
                                    format!("{}\n\n{}", inherited, own_code_evaluated)
                                } else if !inherited.is_empty() {
                                    inherited
                                } else {
                                    own_code_evaluated
                                };
                                self.terminal.run_code(&full_code, lang, workspace_path);
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

                        // Obtener la cadena completa de herencia: A → B → C → ...
                        let inheritance_chain = self.graph.get_inheritance_chain(id);
                        
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
                                    let code_changed = ui.add(
                                        egui::TextEdit::multiline(&mut own_code_editable)
                                            .font(egui::TextStyle::Monospace)
                                            .code_editor()
                                            .desired_width(f32::INFINITY)
                                            .desired_rows(own_lines_count.max(5)),
                                    ).changed();
                                    
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
                if let Err(e) = self.workspace.save_graph(&self.graph) {
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

    fn paint_grid(&self, painter: &egui::Painter, rect: Rect, visuals: &Visuals) {
        const GRID_SPACING: f32 = 32.0;
        let spacing = (GRID_SPACING * self.viewport.zoom).clamp(12.0, 256.0);

        let offset_x = self.viewport.pan.x.rem_euclid(spacing);
        let offset_y = self.viewport.pan.y.rem_euclid(spacing);

        painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);

        let mut count_x = 0;
        let mut x = rect.min.x + offset_x;
        while x < rect.max.x {
            let major = count_x % 4 == 0;
            let color = if major {
                visuals.extreme_bg_color.gamma_multiply(1.4)
            } else {
                visuals.extreme_bg_color.gamma_multiply(1.15)
            };
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
            let color = if major {
                visuals.extreme_bg_color.gamma_multiply(1.4)
            } else {
                visuals.extreme_bg_color.gamma_multiply(1.15)
            };
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
            crate::ui::nodes::draw_node(painter, node, node_rect, self.viewport.zoom, selected, is_inherited, visuals, &connected_pins);
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
            } else if !input.primary_down && was_dragging {
                // Mouse released after dragging - auto-save
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
                
                crate::ui::connectors::draw_connection(
                    painter,
                    start_pos,
                    end_pos,
                    snap_color,
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

    fn node_rect(&self, node: &node_graph::Node, canvas: Rect) -> Rect {
        let size = self.node_size(node) * self.viewport.zoom;
        let min = self.viewport.world_to_screen(node.position, canvas);
        Rect::from_min_size(min, size)
    }

    fn node_size(&self, node: &node_graph::Node) -> Vec2 {
        let rows = node.inputs.len().max(node.outputs.len()).max(1) as f32;
        let height = crate::ui::nodes::HEADER_HEIGHT + rows * crate::ui::nodes::PIN_SPACING + crate::ui::nodes::CONTENT_PADDING * 2.0;
        Vec2::new(crate::ui::nodes::NODE_WIDTH, height)
    }

    fn pin_slot_position(
        &self,
        node: &node_graph::Node,
        canvas: Rect,
        kind: PinKind,
        index: usize,
    ) -> Pos2 {
        let rect = self.node_rect(node, canvas);
        let y = rect.min.y
            + crate::ui::nodes::HEADER_HEIGHT * self.viewport.zoom
            + crate::ui::nodes::PIN_SPACING * self.viewport.zoom * (index as f32 + 0.5);

        match kind {
            PinKind::Input => pos2(rect.min.x + crate::ui::nodes::CONTENT_PADDING * self.viewport.zoom, y),
            PinKind::Output => pos2(rect.max.x - crate::ui::nodes::CONTENT_PADDING * self.viewport.zoom, y),
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
        use crate::expressions::{ExpressionParser, ExpressionEvaluator};
        use crate::expressions::ChannelValue;
        
        // Crear evaluador con el channel_manager actual
        let mut evaluator = ExpressionEvaluator::new(self.channel_manager.clone());
        evaluator.set_current_node(current_node_id);
        
        // Buscar todas las expresiones ch() en el código
        let mut result = String::new();
        let mut last_end = 0;
        
        // Buscar patrones ch("...") o ch('...')
        let code_chars: Vec<char> = code.chars().collect();
        let mut i = 0;
        
        while i < code_chars.len() {
            // Buscar "ch("
            if i + 2 < code_chars.len() 
                && code_chars[i] == 'c' 
                && code_chars[i + 1] == 'h' 
                && code_chars[i + 2] == '(' {
                
                // Agregar todo antes de ch(
                result.push_str(&code[last_end..i]);
                
                // Buscar el cierre del paréntesis
                let mut depth = 1;
                let mut j = i + 3;
                let mut in_string = false;
                let mut string_char = '\0';
                
                while j < code_chars.len() && depth > 0 {
                    let c = code_chars[j];
                    
                    if !in_string {
                        if c == '"' || c == '\'' {
                            in_string = true;
                            string_char = c;
                        } else if c == '(' {
                            depth += 1;
                        } else if c == ')' {
                            depth -= 1;
                        }
                    } else {
                        if c == string_char && (j == 0 || code_chars[j - 1] != '\\') {
                            in_string = false;
                        }
                    }
                    
                    if depth > 0 {
                        j += 1;
                    }
                }
                
                if depth == 0 {
                    // Extraer la expresión completa ch(...)
                    let expr_str = &code[i..=j];
                    
                    // Intentar evaluar
                    match evaluator.evaluate_string(expr_str) {
                        Ok(value) => {
                            // Reemplazar con el valor evaluado
                            let value_str = match value {
                                ChannelValue::String(s) => s,
                                ChannelValue::Number(n) => n.to_string(),
                                ChannelValue::Boolean(b) => b.to_string(),
                                ChannelValue::Code(c) => c,
                            };
                            result.push_str(&value_str);
                            last_end = j + 1;
                            i = j + 1;
                            continue;
                        }
                        Err(_) => {
                            // Si falla, dejar la expresión original
                            result.push_str(expr_str);
                            last_end = j + 1;
                            i = j + 1;
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
        
        // Agregar el resto del código
        if last_end < code.len() {
            result.push_str(&code[last_end..]);
        }
        
        if result.is_empty() {
            code.to_string()
        } else {
            result
        }
    }

    pub fn save_current_graph(&mut self) -> Result<(), String> {
        self.workspace.save_graph(&self.graph)?;
        self.last_save_hash = self.graph_hash();
        self.last_save_time = Some(std::time::Instant::now());
        Ok(())
    }

    pub fn load_graph_from_workspace(&mut self) -> Result<(), String> {
        let graph = self.workspace.load_graph()?;
        self.graph = graph;
        self.graph.recalculate_ids();
        self.interaction.selected_nodes.clear();
        self.last_save_hash = self.graph_hash();
        self.last_save_time = Some(std::time::Instant::now());
        Ok(())
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
                        let links_to_remove: Vec<crate::node_graph::Link> = self.graph.links()
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
                            self.check_and_auto_save();
                        }
                    }
                }

                // Limpiar puntos de corte después de procesar
                self.interaction.cut_tool.clear();
            }
        }
    }
}

