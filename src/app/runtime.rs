use crate::core::{NodeGraph, NodeId};
use crate::core::node_graph::{NodeLanguage, PinKind};
use crate::core::types::{pos2, Color32};
use super::template_palette::{PaletteAction, TemplatePalette};
use super::workspace::WorkspaceState;
use crate::vulkan::context::VulkanContext;
use crate::vulkan::renderer::{pin_screen_center, CodeEditorState, OutputPanel, RenderState, TemplatePaletteEntry, Viewport2D, NODE_HEIGHT, NODE_WIDTH, PIN_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuKind {
    File,
    Edit,
    View,
    Run,
}
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Clone, Copy, Debug)]
struct HitPin {
    node_id: NodeId,
    kind: PinKind,
    slot: usize,
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = AppRuntime::new();
    event_loop.run_app(&mut app).unwrap();
}

struct AppRuntime {
    window: Option<Window>,
    vulkan_ctx: Option<VulkanContext>,
    graph: NodeGraph,
    viewport: Viewport2D,
    is_panning: bool,
    last_cursor_position: Option<(f32, f32)>,
    hovered_node: Option<NodeId>,
    selected_node: Option<NodeId>,
    active_editor_node: Option<NodeId>,
    dragging_node: Option<NodeId>,
    link_source_pin: Option<HitPin>,
    created_nodes: u32,
    template_palette: TemplatePalette,
    workspace: WorkspaceState,
    editor_cursor_line: usize,
    editor_cursor_col: usize,
    output: OutputPanel,
    frame_counter: u64,
    open_menu: Option<MenuKind>,
    toast_message: Option<String>,
    toast_until: u64,
}

impl AppRuntime {
    fn new() -> Self {
        Self {
            window: None,
            vulkan_ctx: None,
            graph: NodeGraph::demo(),
            viewport: Viewport2D::default(),
            is_panning: false,
            last_cursor_position: None,
            hovered_node: None,
            selected_node: None,
            active_editor_node: None,
            dragging_node: None,
            link_source_pin: None,
            created_nodes: 0,
            template_palette: TemplatePalette::new(),
            workspace: WorkspaceState::default(),
            editor_cursor_line: 0,
            editor_cursor_col: 0,
            output: OutputPanel::default(),
            frame_counter: 0,
            open_menu: None,
            toast_message: None,
            toast_until: 0,
        }
    }

    fn render_state(&self) -> RenderState {
        let visible_limit = 12;
        let template_visible_start = self.template_palette.visible_start(visible_limit);
        let template_entries = self.template_palette.templates()
            .iter()
            .enumerate()
            .skip(template_visible_start)
            .take(visible_limit)
            .map(|(index, template)| TemplatePaletteEntry {
                label: format!("{} {} / {} / {}", quick_slot_label(index), template.category, template.subcategory, template.name),
                color: [
                    template.color.0 as f32 / 255.0,
                    template.color.1 as f32 / 255.0,
                    template.color.2 as f32 / 255.0,
                ],
            })
            .collect();

        let code_editor = self.active_editor_node.and_then(|node_id| {
            self.graph.node(node_id).map(|node| {
                let lines: Vec<String> = if node.code.is_empty() {
                    vec!["// Escribe aqui el codigo del nodo...".to_string()]
                } else {
                    node.code.lines().map(str::to_string).collect()
                };
                CodeEditorState {
                    node_id,
                    title: node.title.clone(),
                    language: NodeGraph::language_display_name(node.language).to_string(),
                    code_path: node.code_path.clone().unwrap_or_else(|| "memoria".to_string()),
                    lines,
                    cursor_line: self.editor_cursor_line,
                    cursor_col: self.editor_cursor_col,
                    is_active: true,
                }
            })
        });

        RenderState {
            hovered_node: self.hovered_node,
            selected_node: self.selected_node,
            link_source_node: self.link_source_pin.map(|pin| pin.node_id),
            code_editor_node: self.active_editor_node,
            template_palette_open: self.template_palette.is_open(),
            template_visible_start,
            selected_template_index: self.template_palette.selected_index(),
            template_entries,
            workspace_label: self.workspace.label(),
            code_editor,
            output: self.output.clone(),
            frame_counter: self.frame_counter,
            open_menu: self.open_menu,
            toast_message: self.toast_message.clone(),
            sidebar_entries: self.workspace.list_files_for_sidebar(),
            sidebar_open: true,
        }
    }

    fn node_at_screen_position(&self, screen: (f32, f32)) -> Option<NodeId> {
        let world = self.viewport.screen_to_world(screen.0, screen.1);
        self.graph
            .nodes()
            .iter()
            .rev()
            .find(|node| {
                world.0 >= node.position.x
                    && world.0 <= node.position.x + NODE_WIDTH
                    && world.1 >= node.position.y
                    && world.1 <= node.position.y + NODE_HEIGHT
            })
            .map(|node| node.id)
    }

    fn pin_at_screen_position(&self, screen: (f32, f32)) -> Option<HitPin> {
        let radius = (PIN_SIZE * self.viewport.zoom).max(8.0);
        let radius_sq = radius * radius;

        for node in self.graph.nodes().iter().rev() {
            for (slot, _) in node.outputs.iter().enumerate() {
                let center = pin_screen_center(node, PinKind::Output, slot, self.viewport);
                if distance_sq(screen, center) <= radius_sq {
                    return Some(HitPin { node_id: node.id, kind: PinKind::Output, slot });
                }
            }

            for (slot, _) in node.inputs.iter().enumerate() {
                let center = pin_screen_center(node, PinKind::Input, slot, self.viewport);
                if distance_sq(screen, center) <= radius_sq {
                    return Some(HitPin { node_id: node.id, kind: PinKind::Input, slot });
                }
            }
        }

        None
    }

    fn create_rust_node_at_view_center(&mut self) {
        let Some(window) = &self.window else { return; };
        let size = window.inner_size();
        let world = self.viewport.screen_to_world(size.width as f32 * 0.5, size.height as f32 * 0.5);

        self.created_nodes += 1;
        let node_id = self.graph.add_node(
            format!("Rust Node {}", self.created_nodes),
            pos2(world.0 - NODE_WIDTH * 0.5, world.1 - NODE_HEIGHT * 0.5),
            Color32::from_rgb(194, 59, 34), // Vermillion
            &["in"],
            &["out"],
            NodeLanguage::Rust,
        );

        if let Some(node) = self.graph.node_mut(node_id) {
            node.code = format!(
                "pub fn node_{}() {{\n    println!(\"Ultra-Omega Rust node {}\");\n}}",
                self.created_nodes, self.created_nodes
            );
        }

        self.selected_node = Some(node_id);
        self.hovered_node = Some(node_id);
        self.active_editor_node = Some(node_id);
        self.auto_save();
    }

    fn create_template_node_at_view_center(&mut self, template_index: usize) {
        let Some(template) = self.template_palette.template(template_index).cloned() else { return; };
        let Some(window) = &self.window else { return; };

        let size = window.inner_size();
        let world = self.viewport.screen_to_world(size.width as f32 * 0.5, size.height as f32 * 0.5);
        let color = Color32::from_rgb(template.color.0, template.color.1, template.color.2);
        let node_id = self.graph.add_node(
            format!("{} {}", template.icon, template.name),
            pos2(world.0 - NODE_WIDTH * 0.5, world.1 - NODE_HEIGHT * 0.5),
            color,
            &["in"],
            &["out"],
            template.language,
        );

        if let Some(node) = self.graph.node_mut(node_id) {
            node.code = template.code.to_string();
        }

        self.selected_node = Some(node_id);
        self.hovered_node = Some(node_id);
        self.active_editor_node = Some(node_id);
        self.template_palette.close();
        self.auto_save();
    }

    fn try_finish_link_from_hover(&mut self) -> bool {
        let Some(source_pin) = self.link_source_pin else { return false; };
        let Some(cursor) = self.last_cursor_position else { return false; };
        let Some(target_pin) = self.pin_at_screen_position(cursor) else { return false; };

        if source_pin.node_id == target_pin.node_id || target_pin.kind != PinKind::Input {
            return false;
        }

        let Some(from_pin) = self.graph.pin_id(source_pin.node_id, PinKind::Output, source_pin.slot) else {
            self.link_source_pin = None;
            return false;
        };
        let Some(to_pin) = self.graph.pin_id(target_pin.node_id, PinKind::Input, target_pin.slot) else {
            self.link_source_pin = None;
            return false;
        };

        self.graph.add_link(from_pin, to_pin, Color32::from_rgb(168, 112, 62)); // Copper
        self.selected_node = Some(target_pin.node_id);
        self.link_source_pin = None;
        self.auto_save();
        true
    }

    fn try_start_link_from_hovered_pin(&mut self) -> bool {
        let Some(screen) = self.last_cursor_position else { return false; };
        let Some(pin) = self.pin_at_screen_position(screen) else { return false; };

        if pin.kind != PinKind::Output {
            return false;
        }

        self.selected_node = Some(pin.node_id);
        self.link_source_pin = Some(pin);
        true
    }

    fn start_link_from_selected_node(&mut self) {
        self.link_source_pin = self.selected_node.map(|node_id| HitPin {
            node_id,
            kind: PinKind::Output,
            slot: 0,
        });
    }

    fn delete_selected_node(&mut self) {
        if let Some(node_id) = self.selected_node.take() {
            self.graph.remove_node(node_id);
            self.hovered_node = None;
            self.dragging_node = None;
            if self.active_editor_node == Some(node_id) {
                self.active_editor_node = None;
            }
            if self.link_source_pin.map(|pin| pin.node_id) == Some(node_id) {
                self.link_source_pin = None;
            }
            self.auto_save();
        }
    }

    fn auto_save(&mut self) {
        if self.workspace.root().is_some() {
            if let Err(e) = self.workspace.save_graph(&mut self.graph) {
                log::warn!("Auto-save falló: {}", e);
            }
        }
    }

    fn toggle_template_palette(&mut self) {
        self.template_palette.toggle();
    }

    fn open_code_editor_at_cursor(&mut self) -> bool {
        let Some(node_id) = self.hovered_node else {
            self.active_editor_node = None;
            return false;
        };

        self.selected_node = Some(node_id);
        self.active_editor_node = Some(node_id);
        self.dragging_node = None;
        self.link_source_pin = None;
        self.template_palette.close();

        // Posicionar cursor al final del código
        if let Some(node) = self.graph.node(node_id) {
            let lines: Vec<&str> = node.code.lines().collect();
            self.editor_cursor_line = lines.len().saturating_sub(1);
            self.editor_cursor_col = lines.last().map_or(0, |l| l.len());
        }
        true
    }

    fn insert_editor_text(&mut self, text: &str) {
        let Some(node_id) = self.active_editor_node else { return; };
        let mut changed = false;

        if let Some(node) = self.graph.node_mut(node_id) {
            for ch in text.chars() {
                if ch == '\r' { continue; }
                if ch == '\n' {
                    // Insertar nueva línea en la posición del cursor
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    let line = self.editor_cursor_line.min(lines.len().saturating_sub(1).max(0));
                    let col = self.editor_cursor_col.min(lines.get(line).map_or(0, |l| l.len()));
                    let before: String = lines.get(line).map_or(String::new(), |l| l[..col].to_string());
                    let after: String = lines.get(line).map_or(String::new(), |l| l[col..].to_string());
                    let mut new_lines: Vec<String> = lines[..line].to_vec();
                    new_lines.push(before);
                    new_lines.push(after);
                    node.code = new_lines.join("\n");
                    self.editor_cursor_line = (line + 1).min(new_lines.len().saturating_sub(1));
                    self.editor_cursor_col = 0;
                    changed = true;
                } else if !ch.is_control() {
                    // Insertar carácter en la posición del cursor
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    let line = self.editor_cursor_line.min(lines.len().saturating_sub(1).max(0));
                    let col = self.editor_cursor_col.min(lines.get(line).map_or(0, |l| l.len()));
                    let mut new_lines = lines;
                    if let Some(l) = new_lines.get_mut(line) {
                        l.insert(col, ch);
                    }
                    node.code = new_lines.join("\n");
                    self.editor_cursor_col += ch.len_utf8();
                    changed = true;
                }
            }
        }

        if changed {
            self.auto_save();
        }
    }

    fn handle_code_editor_key(&mut self, key: KeyCode) -> bool {
        let Some(node_id) = self.active_editor_node else { return false; };

        match key {
            KeyCode::Escape => {
                self.active_editor_node = None;
                true
            }
            KeyCode::Backspace => {
                let mut changed = false;
                if let Some(node) = self.graph.node_mut(node_id) {
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    if self.editor_cursor_col > 0 {
                        let line = self.editor_cursor_line.min(lines.len().saturating_sub(1).max(0));
                        let col = self.editor_cursor_col;
                        let mut new_lines = lines;
                        if let Some(l) = new_lines.get_mut(line) {
                            if col <= l.len() {
                                let byte_idx = l.char_indices().nth(col.saturating_sub(1)).map_or(l.len(), |(i, _)| i);
                                l.remove(byte_idx);
                                changed = true;
                                self.editor_cursor_col -= 1;
                            }
                        }
                        if changed { node.code = new_lines.join("\n"); }
                    } else if self.editor_cursor_line > 0 {
                        // Unir con la línea anterior
                        let prev_line_len = lines.get(self.editor_cursor_line.saturating_sub(1)).map_or(0, |l| l.len());
                        let mut new_lines = lines;
                        if self.editor_cursor_line < new_lines.len() {
                            let current = new_lines.remove(self.editor_cursor_line);
                            if let Some(prev) = new_lines.last_mut() {
                                prev.push_str(&current);
                            }
                            node.code = new_lines.join("\n");
                            self.editor_cursor_line -= 1;
                            self.editor_cursor_col = prev_line_len;
                            changed = true;
                        }
                    }
                }
                if changed { self.auto_save(); }
                true
            }
            KeyCode::Enter => {
                self.insert_editor_text("\n");
                true
            }
            KeyCode::Tab => {
                self.insert_editor_text("    ");
                true
            }
            KeyCode::ArrowLeft => {
                if self.editor_cursor_col > 0 {
                    self.editor_cursor_col -= 1;
                } else if self.editor_cursor_line > 0 {
                    self.editor_cursor_line -= 1;
                    if let Some(node) = self.graph.node(node_id) {
                        let lines: Vec<&str> = node.code.lines().collect();
                        self.editor_cursor_col = lines.get(self.editor_cursor_line).map_or(0, |l| l.len());
                    }
                }
                true
            }
            KeyCode::ArrowRight => {
                if let Some(node) = self.graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    let max_col = lines.get(self.editor_cursor_line).map_or(0, |l| l.len());
                    if self.editor_cursor_col < max_col {
                        self.editor_cursor_col += 1;
                    } else if self.editor_cursor_line + 1 < lines.len() {
                        self.editor_cursor_line += 1;
                        self.editor_cursor_col = 0;
                    }
                }
                true
            }
            KeyCode::ArrowUp => {
                if self.editor_cursor_line > 0 {
                    self.editor_cursor_line -= 1;
                    if let Some(node) = self.graph.node(node_id) {
                        let lines: Vec<&str> = node.code.lines().collect();
                        let max_col = lines.get(self.editor_cursor_line).map_or(0, |l| l.len());
                        self.editor_cursor_col = self.editor_cursor_col.min(max_col);
                    }
                }
                true
            }
            KeyCode::ArrowDown => {
                if let Some(node) = self.graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    if self.editor_cursor_line + 1 < lines.len() {
                        self.editor_cursor_line += 1;
                        let max_col = lines.get(self.editor_cursor_line).map_or(0, |l| l.len());
                        self.editor_cursor_col = self.editor_cursor_col.min(max_col);
                    }
                }
                true
            }
            KeyCode::Home => {
                self.editor_cursor_col = 0;
                true
            }
            KeyCode::End => {
                if let Some(node) = self.graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    self.editor_cursor_col = lines.get(self.editor_cursor_line).map_or(0, |l| l.len());
                }
                true
            }
            _ => true,
        }
    }

    fn select_workspace_folder(&mut self) {
        if self.workspace.select_folder().is_some() {
            // Reset completo de estado
            self.hovered_node = None;
            self.selected_node = None;
            self.active_editor_node = None;
            self.dragging_node = None;
            self.link_source_pin = None;
            self.editor_cursor_line = 0;
            self.editor_cursor_col = 0;
            self.open_menu = None;
            self.output = OutputPanel::default();

            // Cargar grafo guardado si existe, sino crear demo
            if let Some(loaded) = self.workspace.load_graph() {
                self.graph = loaded;
                self.graph.recalculate_ids();
                log::info!("Grafo cargado desde workspace");
            } else {
                self.graph = NodeGraph::demo();
                log::info!("Nuevo workspace - cargando demo");
            }
            self.show_toast(">> Workspace cargado");
        }
    }

    fn compile_and_run_active_node(&mut self) {
        let Some(node_id) = self.active_editor_node else { return; };
        let Some(node) = self.graph.node(node_id) else { return; };

        let node_title = node.title.clone();
        let code = if node.code.trim().is_empty() {
            "fn main() { println!(\"vacio\"); }".to_string()
        } else {
            node.code.clone()
        };

        // Escribir a archivo temporal
        let tmp_dir = std::env::temp_dir();
        let tmp_rs = tmp_dir.join(format!("ultra_omega_play_{}.rs", node_id.0));
        let tmp_exe = tmp_dir.join(format!("ultra_omega_play_{}.exe", node_id.0));

        if let Err(e) = std::fs::write(&tmp_rs, &code) {
            self.output = OutputPanel {
                lines: vec![format!("Error escribiendo archivo: {}", e)],
                is_error: true,
                has_run: true,
                error_line: None,
            };
            return;
        }

        // Compilar con rustc
        let compile_result = std::process::Command::new("rustc")
            .arg(&tmp_rs)
            .arg("-o")
            .arg(&tmp_exe)
            .arg("--edition")
            .arg("2021")
            .arg("-O") // Optimización para velocidad
            .output();

        match compile_result {
            Ok(out) => {
                if out.status.success() {
                    // Ejecutar el binario
                    let run_result = std::process::Command::new(&tmp_exe).output();
                    match run_result {
                        Ok(r) => {
                            let stdout = String::from_utf8_lossy(&r.stdout);
                            let stderr = String::from_utf8_lossy(&r.stderr);
                            let mut lines: Vec<String> = Vec::new();
                            lines.push(format!(">>> Ejecutando: {}", node_title));
                            if !stdout.is_empty() {
                                for l in stdout.lines() { lines.push(l.to_string()); }
                            }
                            if !stderr.is_empty() {
                                lines.push("[stderr]".to_string());
                                for l in stderr.lines() { lines.push(l.to_string()); }
                            }
                            if lines.len() == 1 {
                                lines.push("(sin salida)".to_string());
                            }
                            lines.push(format!("<<< exit code: {}", r.status.code().unwrap_or(-1)));
                            self.output = OutputPanel {
                                lines,
                                is_error: !r.status.success(),
                                has_run: true,
                                error_line: None,
                            };
                        }
                        Err(e) => {
                            self.output = OutputPanel {
                                lines: vec![format!("Error ejecutando: {}", e)],
                                is_error: true,
                                has_run: true,
                                error_line: None,
                            };
                        }
                    }
                } else {
                    // Error de compilación
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let mut lines: Vec<String> = vec![">>> Error de compilación:".to_string()];
                    let mut error_line: Option<usize> = None;
                    for l in stderr.lines() {
                        lines.push(l.to_string());
                        // Parsear "linea X" del error de rustc
                        if error_line.is_none() {
                            if let Some(pos) = l.find("-->") {
                                let after = &l[pos+3..];
                                if let Some(colon_pos) = after.find(':') {
                                    if let Ok(n) = after[..colon_pos].trim().parse::<usize>() {
                                        error_line = Some(n.saturating_sub(1));
                                    }
                                }
                            }
                            // Formato alternativo: "tmp.rs:5:"
                            if let Some(colon_pos) = l.find(':') {
                                let after = &l[colon_pos+1..];
                                if let Some(colon2) = after.find(':') {
                                    if let Ok(n) = after[..colon2].trim().parse::<usize>() {
                                        if n < 10000 { error_line = Some(n.saturating_sub(1)); }
                                    }
                                }
                            }
                        }
                    }
                    self.output = OutputPanel {
                        lines,
                        is_error: true,
                        has_run: true,
                        error_line,
                    };
                }
            }
            Err(e) => {
                self.output = OutputPanel {
                    lines: vec![
                        format!("Error: rustc no encontrado ({})", e),
                        "Asegurate de tener Rust instalado (https://rustup.rs)".to_string(),
                    ],
                    is_error: true,
                    has_run: true,
                    error_line: None,
                };
            }
        }
    }

    fn handle_template_palette_key(&mut self, key: KeyCode) -> bool {
        match self.template_palette.handle_key(key) {
            PaletteAction::None => true,
            PaletteAction::Create(index) => {
                self.create_template_node_at_view_center(index);
                true
            }
        }
    }

    fn click_menu_bar(&mut self, pos: (f32, f32)) -> Option<MenuKind> {
        // Solo si el click esta en la barra superior
        if pos.1 > 32.0 { return None; }
        let items = [("File", MenuKind::File, 152.0), ("Edit", MenuKind::Edit, 200.0), ("View", MenuKind::View, 248.0), ("Run", MenuKind::Run, 300.0)];
        for (_label, kind, base_x) in items.iter() {
            let label_w = match *kind {
                MenuKind::File => 4.0 * 9.0 + 24.0,
                MenuKind::Edit => 4.0 * 9.0 + 24.0,
                MenuKind::View => 4.0 * 9.0 + 24.0,
                MenuKind::Run => 3.0 * 9.0 + 24.0,
            };
            if pos.0 >= *base_x && pos.0 <= *base_x + label_w {
                return Some(*kind);
            }
        }
        None
    }

    fn menu_items(&self) -> Vec<(&'static str, &'static str)> {
        match self.open_menu {
            Some(MenuKind::File) => vec![
                ("New Project", "Ctrl+N"),
                ("Open Folder...", "Ctrl+O"),
                ("Save", "Ctrl+S"),
                ("Export Graph", ""),
            ],
            Some(MenuKind::Edit) => vec![
                ("Delete Selected", "Del"),
                ("Duplicate Node", "Ctrl+D"),
                ("Select All", "Ctrl+A"),
            ],
            Some(MenuKind::View) => vec![
                ("Reset Zoom", "R"),
                ("Zoom In", "Ctrl++"),
                ("Zoom Out", "Ctrl+-"),
                ("Toggle Grid", "G"),
            ],
            Some(MenuKind::Run) => vec![
                ("Run Active Node", "F5"),
                ("Build Project", "Ctrl+B"),
                ("Clean Build", ""),
            ],
            None => vec![],
        }
    }

    fn try_click_menu_item(&self, pos: (f32, f32)) -> Option<usize> {
        let menu = self.open_menu?;
        // Calcular menu_x segun el menu activo
        let menu_x = match menu {
            MenuKind::File => 152.0,
            MenuKind::Edit => 200.0,
            MenuKind::View => 248.0,
            MenuKind::Run => 300.0,
        };
        let menu_y = 32.0;
        let mw = 240.0;
        let items = self.menu_items();
        for (i, _item) in items.iter().enumerate() {
            let item_y = menu_y + 6.0 + i as f32 * 32.0;
            if pos.0 >= menu_x && pos.0 <= menu_x + mw && pos.1 >= item_y && pos.1 <= item_y + 28.0 {
                return Some(i);
            }
        }
        None
    }

    fn execute_menu_action(&mut self, idx: usize) {
        // Ejecutar la accion
        match self.open_menu {
            Some(MenuKind::File) => {
                match idx {
                    0 => { self.create_demo_graph(); self.show_toast(">> New Project"); }
                    1 => { self.select_workspace_folder(); self.show_toast(">> Open Folder..."); }
                    2 => { self.auto_save(); self.show_toast(">> Save"); }
                    3 => { self.show_toast(">> Export Graph"); }
                    _ => {}
                }
            }
            Some(MenuKind::Edit) => {
                match idx {
                    0 => { self.delete_selected_node(); self.show_toast(">> Delete Selected"); }
                    1 => { self.show_toast(">> Duplicate Node (TODO)"); }
                    2 => { self.show_toast(">> Select All (TODO)"); }
                    _ => {}
                }
            }
            Some(MenuKind::View) => {
                match idx {
                    0 => { self.viewport = Viewport2D::default(); self.show_toast(">> Reset Zoom"); }
                    1 => { self.viewport.zoom_by(2.0); self.show_toast(">> Zoom In"); }
                    2 => { self.viewport.zoom_by(-2.0); self.show_toast(">> Zoom Out"); }
                    3 => { self.show_toast(">> Toggle Grid (TODO)"); }
                    _ => {}
                }
            }
            Some(MenuKind::Run) => {
                match idx {
                    0 => { self.compile_and_run_active_node(); self.show_toast(">> Run Active Node"); }
                    1 => { self.show_toast(">> Build Project (TODO)"); }
                    2 => { self.show_toast(">> Clean Build (TODO)"); }
                    _ => {}
                }
            }
            None => {}
        }
    }

    fn create_demo_graph(&mut self) {
        self.graph = NodeGraph::demo();
        self.hovered_node = None;
        self.selected_node = None;
        self.active_editor_node = None;
    }

    fn show_toast(&mut self, msg: &str) {
        self.toast_message = Some(msg.to_string());
        self.toast_until = self.frame_counter + 120; // 2 seconds at 60fps
    }
}

impl ApplicationHandler for AppRuntime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Ultra-Omega | Node Editor (Vulkan Puro)")
                    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            ).unwrap();

            self.vulkan_ctx = Some(VulkanContext::new(&window));
            window.request_redraw();
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.frame_counter = self.frame_counter.wrapping_add(1);
                let state = self.render_state();
                if let (Some(window), Some(ctx)) = (&self.window, &mut self.vulkan_ctx) {
                    ctx.draw_frame(window, &self.graph, self.viewport, state);
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(ctx) = &mut self.vulkan_ctx {
                        ctx.mark_swapchain_dirty();
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(ctx) = &mut self.vulkan_ctx {
                    ctx.mark_swapchain_dirty();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let current = (position.x as f32, position.y as f32);

                if let Some(node_id) = self.dragging_node {
                    if let Some(previous) = self.last_cursor_position {
                        let (dx, dy) = self
                            .viewport
                            .screen_delta_to_world(current.0 - previous.0, current.1 - previous.1);

                        if let Some(node) = self.graph.node_mut(node_id) {
                            node.position.x += dx;
                            node.position.y += dy;
                        }
                    }
                } else if self.is_panning {
                    if let Some(previous) = self.last_cursor_position {
                        self.viewport.pan_by(current.0 - previous.0, current.1 - previous.1);
                    }
                }

                self.last_cursor_position = Some(current);
                self.hovered_node = self.node_at_screen_position(current);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Middle {
                    self.is_panning = state == ElementState::Pressed;
                } else if button == MouseButton::Right && state == ElementState::Pressed {
                    self.open_code_editor_at_cursor();
                } else if button == MouseButton::Left && state == ElementState::Pressed {
                    // Si hay un menu abierto, intentar clickear un item
                    if self.open_menu.is_some() {
                        if let Some(cursor) = self.last_cursor_position {
                            if let Some(action) = self.try_click_menu_item(cursor) {
                                self.execute_menu_action(action);
                            }
                            self.open_menu = None;
                            return;
                        }
                    }
                    // Si el click esta en la top bar, abrir menu
                    if let Some(cursor) = self.last_cursor_position {
                        if cursor.1 < 32.0 {
                            if let Some(menu) = self.click_menu_bar(cursor) {
                                self.open_menu = Some(menu);
                                return;
                            }
                        }
                    }
                    // Logica normal de nodo/pin
                    if self.try_finish_link_from_hover() || self.try_start_link_from_hovered_pin() {
                        self.dragging_node = None;
                    } else {
                        self.selected_node = self.hovered_node;
                        self.dragging_node = self.hovered_node;
                    }
                } else if button == MouseButton::Left && state == ElementState::Released {
                    self.dragging_node = None;
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let steps = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 120.0,
                };

                if let Some(cursor) = self.last_cursor_position {
                    self.viewport.zoom_at(steps, cursor.0, cursor.1);
                } else {
                    self.viewport.zoom_by(steps);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }

                let PhysicalKey::Code(key) = event.physical_key else {
                    return;
                };

                if self.active_editor_node.is_some() {
                    match key {
                        KeyCode::Escape | KeyCode::Backspace | KeyCode::Enter | KeyCode::Tab
                        | KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown
                        | KeyCode::Home | KeyCode::End => {
                            self.handle_code_editor_key(key);
                        }
                        KeyCode::F5 => {
                            self.compile_and_run_active_node();
                        }
                        _ => {
                            if let Some(text) = event.text {
                                self.insert_editor_text(&text);
                            }
                        }
                    }
                    return;
                }

                if self.template_palette.is_open() && self.handle_template_palette_key(key) {
                    return;
                }

                match key {
                    KeyCode::Tab => self.toggle_template_palette(),
                    KeyCode::KeyN => self.create_rust_node_at_view_center(),
                    KeyCode::Delete => self.delete_selected_node(),
                    KeyCode::Escape => {
                        self.open_menu = None;
                        self.selected_node = None;
                        self.active_editor_node = None;
                        self.dragging_node = None;
                        self.link_source_pin = None;
                        self.template_palette.close();
                    }
                    KeyCode::KeyO => self.select_workspace_folder(),
                    KeyCode::KeyR => self.viewport = Viewport2D::default(),
                    KeyCode::KeyC => self.start_link_from_selected_node(),
                    KeyCode::F5 => {
                        // F5 funciona tambien con la seleccion si no hay editor
                        if self.active_editor_node.is_none() {
                            if let Some(sel) = self.selected_node {
                                self.active_editor_node = Some(sel);
                            }
                        }
                        self.compile_and_run_active_node();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn distance_sq(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

fn quick_slot_label(index: usize) -> String {
    match index {
        0..=8 => (index + 1).to_string(),
        9 => "0".to_string(),
        _ => "-".to_string(),
    }
}
