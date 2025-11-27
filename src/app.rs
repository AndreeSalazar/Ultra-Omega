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
    pub workspace: Workspace,
    pub last_save_hash: u64,
    pub last_save_time: Option<std::time::Instant>,
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
            workspace,
            last_save_hash: 0,
            last_save_time: None,
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
            if self.show_node_menu {
                self.node_menu_pos = ctx.pointer_hover_pos().unwrap_or(pos2(200.0, 200.0));
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
                // Desactivar: limpiar línea y cancelar cualquier operación en curso
                self.interaction.cut_tool.line_start = None;
                self.interaction.cut_tool.line_end = None;
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
        if let Some(node) = self.graph.node_mut(id) {
             node.code = code.to_string();
        }
        
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
        if self.show_node_menu {
            let mut close_menu = false;
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
                        .inner_margin(egui::Margin::same(12.0));
                        
                    frame.show(ui, |ui| {
                        ui.set_width(150.0);
                        
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("➕").color(Color32::from_rgb(100, 200, 255)));
                            ui.heading("Agregar Nodo");
                        });
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                        
                        ui.label(egui::RichText::new("Assembler (NASM)").strong().color(Color32::LIGHT_GRAY));
                        if ui.button("⏵ Hola Mundo").clicked() {
                            self.add_template_node(ctx, "ASM: Hola Mundo", crate::templates::ASM_HELLO, Color32::from_rgb(0xff, 0x47, 0x00));
                            close_menu = true;
                        }
                        if ui.button("➕ Suma").clicked() {
                            self.add_template_node(ctx, "ASM: Suma", crate::templates::ASM_SUM, Color32::from_rgb(0xff, 0x47, 0x00));
                            close_menu = true;
                        }
                        if ui.button("↻ Bucle").clicked() {
                            self.add_template_node(ctx, "ASM: Bucle", crate::templates::ASM_LOOP, Color32::from_rgb(0xff, 0x47, 0x00));
                            close_menu = true;
                        }
                        if ui.button("🔀 Condicional").clicked() {
                            self.add_template_node(ctx, "ASM: Condicional", crate::templates::ASM_CONDITIONAL, Color32::from_rgb(0xff, 0x47, 0x00));
                            close_menu = true;
                        }
                        
                        ui.separator();
                        ui.label(egui::RichText::new("C").strong().color(Color32::LIGHT_GRAY));
                        if ui.button("Hola Mundo").clicked() {
                            self.add_template_node(ctx, "Base C", crate::templates::C_HELLO, Color32::from_rgb(0x00, 0x59, 0x9C));
                            close_menu = true;
                        }
                        
                        ui.separator();
                        ui.label(egui::RichText::new("C++").strong().color(Color32::LIGHT_GRAY));
                        if ui.button("Hola Mundo").clicked() {
                            self.add_template_node(ctx, "Base C++", crate::templates::CPP_HELLO, Color32::from_rgb(0x00, 0x44, 0x82));
                            close_menu = true;
                        }
                        
                        ui.separator();
                        ui.label(egui::RichText::new("Rust").strong().color(Color32::LIGHT_GRAY));
                        if ui.button("Hola Mundo").clicked() {
                            self.add_template_node(ctx, "Base Rust", crate::templates::RUST_HELLO, Color32::from_rgb(0xde, 0x39, 0x00));
                            close_menu = true;
                        }
                    });
                });
                
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_menu = true;
            }
            
            if close_menu {
                self.show_node_menu = false;
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
                    // Solo desactivar si no hay una línea de corte en progreso
                    if self.interaction.cut_tool.line_start.is_none() {
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

        // Check for Esc key to close editor
        if open && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            open = false;
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
                        if let Some(node) = self.graph.node_mut(id) {
                            ui.horizontal(|ui| {
                                ui.heading(&node.title);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button("Cerrar").clicked() {
                                        should_close = true;
                                    }
                                    if ui.button("▶ Ejecutar").clicked() {
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
                                        self.terminal.run_code(&node.code, lang, workspace_path);
                                    }
                                });
                            });
                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let font_id = egui::FontId::monospace(14.0);
                                let _row_height = ui.fonts(|f| f.row_height(&font_id));
                                let num_lines = node.code.lines().count().max(1);

                                ui.horizontal_top(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width(40.0);
                                        for i in 1..=num_lines {
                                            ui.label(
                                                egui::RichText::new(format!("{}", i))
                                                    .font(font_id.clone())
                                                    .color(Color32::GRAY),
                                            );
                                        }
                                    });

                                    let code_changed = ui.add_sized(
                                        ui.available_size(),
                                        egui::TextEdit::multiline(&mut node.code)
                                            .font(egui::TextStyle::Monospace)
                                            .code_editor()
                                            .lock_focus(true)
                                            .desired_width(f32::INFINITY),
                                    ).changed();
                                    
                                    // Store flag for auto-save after borrow ends
                                    if code_changed {
                                        ctx.request_repaint();
                                    }
                                });
                            });
                        }
                    }
                });
            if should_close {
                open = false;
            }
        }

        if !open {
            self.interaction.editing_node = None;
        }
        
        // Auto-save after editing (check if graph changed)
        self.check_and_auto_save();
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

        for node in self.graph.nodes() {
            let node_rect = self.node_rect(node, rect);
            let selected = self.interaction.selected_nodes.contains(&node.id);
            let is_inherited = inherited_nodes.contains(&node.id);
            crate::ui::nodes::draw_node(painter, node, node_rect, self.viewport.zoom, selected, is_inherited, visuals);
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
                                // Aplicar herencia de código
                                self.apply_inheritance();
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
                                        // Aplicar herencia de código
                                        self.apply_inheritance();
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
                crate::ui::connectors::draw_connection(
                    painter,
                    start_pos,
                    pointer,
                    Color32::from_rgb(100, 200, 255),
                    self.viewport.zoom,
                    time,
                );
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

    // Encontrar el pin más cercano para snap
    fn find_nearest_snap_pin(&self, pointer: Pos2, canvas: Rect, from_pin: Option<PinId>) -> Option<(PinId, Pos2, PinKind)> {
        const SNAP_DISTANCE: f32 = 30.0; // Distancia de snap
        let mut nearest: Option<(PinId, Pos2, PinKind, f32)> = None;
        
        for node in self.graph.nodes() {
            // Check input pins (solo si estamos conectando desde una salida)
            if from_pin.is_some() {
                for (idx, pin) in node.inputs.iter().enumerate() {
                    let pin_pos = self.pin_slot_position(node, canvas, PinKind::Input, idx);
                    let distance = (pin_pos - pointer).length();
                    
                    if distance < SNAP_DISTANCE * self.viewport.zoom {
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
                    
                    if distance < SNAP_DISTANCE * self.viewport.zoom {
                        if nearest.is_none() || distance < nearest.unwrap().3 {
                            nearest = Some((pin.id, pin_pos, PinKind::Output, distance));
                        }
                    }
                }
            }
        }
        
        nearest.map(|(id, pos, kind, _)| (id, pos, kind))
    }

    fn apply_inheritance(&mut self) {
        // Aplicar herencia de código a todos los nodos que tienen padre
        // Primero recolectar los IDs y códigos heredados
        let inheritance_map: Vec<(NodeId, String)> = self.graph.nodes()
            .iter()
            .filter_map(|node| {
                self.graph.get_inherited_code(node.id)
                    .map(|code| (node.id, code))
            })
            .collect();
        
        // Luego aplicar los cambios
        for (node_id, inherited_code) in inheritance_map {
            if let Some(node_mut) = self.graph.node_mut(node_id) {
                // Combinar código heredado con código propio
                if !node_mut.code.is_empty() && !inherited_code.is_empty() {
                    // Si ya tiene código, agregar el heredado al inicio
                    node_mut.code = format!("{}\n\n{}", inherited_code, node_mut.code);
                } else if node_mut.code.is_empty() {
                    // Si no tiene código, usar solo el heredado
                    node_mut.code = inherited_code;
                }
            }
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

        // Dibujar línea de corte si ya existe
        if self.interaction.cut_tool.line_start.is_some() {
            self.interaction.cut_tool.draw_cut_line(painter);
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
                
                // Si no hay línea activa, iniciar una nueva
                if self.interaction.cut_tool.line_start.is_none() {
                    self.interaction.cut_tool.line_start = Some(pointer_pos);
                    self.interaction.cut_tool.line_end = Some(pointer_pos);
                }
            }

            // Actualizar línea mientras se arrastra
            if input.primary_down && self.interaction.cut_tool.line_start.is_some() {
                self.interaction.cut_tool.line_end = Some(pointer_pos);
            }

            // Completar corte al soltar el mouse
            // Verificar que el mouse se soltó (primary_down pasó de true a false)
            if !input.primary_down && self.interaction.cut_tool.line_start.is_some() {
                // Solo procesar si hay una línea válida
                if let (Some(start), Some(end)) = (self.interaction.cut_tool.line_start, self.interaction.cut_tool.line_end) {
                    // Verificar que la línea tenga una longitud mínima
                    if start.distance(end) > 5.0 {
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

                // Limpiar línea de corte después de procesar
                self.interaction.cut_tool.line_start = None;
                self.interaction.cut_tool.line_end = None;
            }
        }
    }
}

