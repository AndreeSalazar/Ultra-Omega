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
        
        let id = self.graph.add_node(title, world_pos, color, &[], &["Código"]);
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
                self.draw_box_selection(&painter);

                self.handle_node_dragging(&input, rect);

                // Request repaint for animations (connectors pulse)
                ctx.request_repaint();
            });
    }

    fn editor_ui(&mut self, ctx: &egui::Context) {
        let mut open = self.interaction.editing_node.is_some();
        let node_id = self.interaction.editing_node;

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
                                        self.terminal.run_code(&node.code, lang);
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
        for node in self.graph.nodes() {
            let node_rect = self.node_rect(node, rect);
            let selected = self.interaction.selected_nodes.contains(&node.id);
            crate::ui::nodes::draw_node(painter, node, node_rect, self.viewport.zoom, selected, visuals);
        }
    }

    fn handle_node_dragging(&mut self, input: &PointerSnapshot, rect: Rect) {
        if let Some(pointer_pos) = input.pos {
            // 1. Node Interaction
            if input.primary_pressed {
                if let Some(node_id) = self.hit_test(pointer_pos, rect) {
                    // Node clicked
                    if !input.modifiers.ctrl && !self.interaction.selected_nodes.contains(&node_id) {
                        self.interaction.selected_nodes.clear();
                    }
                    self.interaction.selected_nodes.insert(node_id);
                    self.interaction.dragging_node = Some(node_id);
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
                    self.interaction.editing_node = Some(node_id);
                    self.interaction.selected_nodes.insert(node_id); // Select on right click too
                }
            }
        }
    }
    
    fn draw_box_selection(&self, painter: &egui::Painter) {
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
}

