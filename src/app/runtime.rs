use crate::core::{NodeGraph, NodeId};
use crate::core::node_graph::NodeLanguage;
use crate::core::types::{pos2, Color32};
use super::template_palette::{PaletteAction, TemplatePalette};
use super::workspace::WorkspaceState;
use super::editor::EditorState;
use super::menu::{MenuKind, MenuBar};
use super::interaction::{self, HitPin};
use super::command_palette::{CommandPalette, CommandAction};
use crate::config::AppConfig;
use crate::vulkan::context::VulkanContext;
use crate::vulkan::renderer::{CodeEditorState, OutputPanel, RenderState, TemplatePaletteEntry, Viewport2D, NODE_HEIGHT, NODE_WIDTH};

use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::ModifiersState;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

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
    config: AppConfig,
    editor: EditorState,
    output: OutputPanel,
    frame_counter: u64,
    open_menu: Option<MenuKind>,
    toast_message: Option<String>,
    toast_until: u64,
    command_palette: CommandPalette,
    modifiers: ModifiersState,
}

impl AppRuntime {
    fn new() -> Self {
        let config = AppConfig::load();

        let mut workspace = WorkspaceState::default();
        let mut graph = NodeGraph::demo();
        if let Some(ref wp) = config.workspace_path {
            let path = std::path::PathBuf::from(wp);
            if path.is_dir() {
                workspace.open_path(path);
                if let Some(loaded) = workspace.load_graph() {
                    graph = loaded;
                    graph.recalculate_ids();
                    log::info!("Grafo restaurado desde workspace");
                }
            }
        }

        Self {
            window: None,
            vulkan_ctx: None,
            graph,
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
            workspace,
            config,
            editor: EditorState::default(),
            output: OutputPanel::default(),
            frame_counter: 0,
            open_menu: None,
            toast_message: None,
            toast_until: 0,
            command_palette: CommandPalette::new(),
            modifiers: ModifiersState::empty(),
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
                    cursor_line: self.editor.cursor_line,
                    cursor_col: self.editor.cursor_col,
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
            sidebar_open: self.workspace.root().is_some(),
            workspace_path: self.workspace.root().map(|p| p.display().to_string()).unwrap_or_default(),
            node_count: self.graph.nodes().len(),
            link_count: self.graph.links().len(),
            zoom_percent: (self.viewport.zoom * 100.0) as u32,
            command_palette: if self.command_palette.open {
                let entries = self.command_palette.filtered().map(|(_, c)| {
                    format!("{} [{}]", c.name, c.shortcut)
                }).collect();
                Some(crate::vulkan::renderer::CommandPaletteState {
                    query: self.command_palette.query.clone(),
                    selected: self.command_palette.selected,
                    entries,
                })
            } else {
                None
            },
        }
    }

    fn create_rust_node_at_view_center(&mut self) {
        let Some(window) = &self.window else { return; };
        let size = window.inner_size();
        let work_center_x = (size.width as f32 * 0.5).max(294.0 + 100.0);
        let work_center_y = size.height as f32 * 0.5;
        let world = self.viewport.screen_to_world(work_center_x, work_center_y);

        self.created_nodes += 1;
        let node_id = self.graph.add_node(
            format!("Rust Node {}", self.created_nodes),
            pos2(world.0 - NODE_WIDTH * 0.5, world.1 - NODE_HEIGHT * 0.5),
            Color32::from_rgb(194, 59, 34),
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
        let work_center_x = (size.width as f32 * 0.5).max(294.0 + 100.0);
        let work_center_y = size.height as f32 * 0.5;
        let world = self.viewport.screen_to_world(work_center_x, work_center_y);
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

    fn select_workspace_folder(&mut self) {
        if self.workspace.select_folder().is_some() {
            self.reset_workspace_state();

            if let Some(path) = self.workspace.root() {
                self.config.workspace_path = Some(path.display().to_string());
                let _ = self.config.save();
            }

            self.show_toast(">> Workspace cargado");
        }
    }

    fn reset_workspace_state(&mut self) {
        self.hovered_node = None;
        self.selected_node = None;
        self.active_editor_node = None;
        self.dragging_node = None;
        self.link_source_pin = None;
        self.editor = EditorState::default();
        self.open_menu = None;
        self.output = OutputPanel::default();

        if let Some(loaded) = self.workspace.load_graph() {
            self.graph = loaded;
            self.graph.recalculate_ids();
            log::info!("Grafo cargado desde workspace");
        } else {
            self.graph = NodeGraph::demo();
            log::info!("Nuevo workspace - cargando demo");
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

        let compile_result = std::process::Command::new("rustc")
            .arg(&tmp_rs)
            .arg("-o")
            .arg(&tmp_exe)
            .arg("--edition")
            .arg("2021")
            .arg("-O")
            .output();

        match compile_result {
            Ok(out) => {
                if out.status.success() {
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
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let mut lines: Vec<String> = vec![">>> Error de compilación:".to_string()];
                    let mut error_line: Option<usize> = None;
                    for l in stderr.lines() {
                        lines.push(l.to_string());
                        if error_line.is_none() {
                            if let Some(pos) = l.find("-->") {
                                let after = &l[pos+3..];
                                if let Some(colon_pos) = after.find(':') {
                                    if let Ok(n) = after[..colon_pos].trim().parse::<usize>() {
                                        error_line = Some(n.saturating_sub(1));
                                    }
                                }
                            }
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

    fn execute_menu_action(&mut self, idx: usize) {
        match self.open_menu {
            Some(MenuKind::File) => {
                match idx {
                    0 => { self.graph = NodeGraph::demo(); self.hovered_node = None; self.selected_node = None; self.active_editor_node = None; self.show_toast(">> New Project"); }
                    1 => { self.select_workspace_folder(); }
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

    fn execute_command_action(&mut self, action: CommandAction) {
        match action {
            CommandAction::NewNode => { self.create_rust_node_at_view_center(); self.show_toast(">> New Rust Node"); }
            CommandAction::OpenFolder => { self.select_workspace_folder(); }
            CommandAction::Save => { self.auto_save(); self.show_toast(">> Save"); }
            CommandAction::RunNode => { self.compile_and_run_active_node(); self.show_toast(">> Run Active Node"); }
            CommandAction::DeleteNode => { self.delete_selected_node(); self.show_toast(">> Delete Selected"); }
            CommandAction::ZoomIn => { self.viewport.zoom_by(2.0); self.show_toast(">> Zoom In"); }
            CommandAction::ZoomOut => { self.viewport.zoom_by(-2.0); self.show_toast(">> Zoom Out"); }
            CommandAction::ResetZoom => { self.viewport = Viewport2D::default(); self.show_toast(">> Reset Zoom"); }
            CommandAction::ToggleTemplates => { self.template_palette.toggle(); }
            CommandAction::ExportGraph => { self.show_toast(">> Export Graph (TODO)"); }
            CommandAction::BuildProject => { self.show_toast(">> Build Project (TODO)"); }
            CommandAction::CleanBuild => { self.show_toast(">> Clean Build (TODO)"); }
        }
    }

    fn show_toast(&mut self, msg: &str) {
        self.toast_message = Some(msg.to_string());
        self.toast_until = self.frame_counter + 120;
    }
}

impl ApplicationHandler for AppRuntime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Ultra-Omega | Node Editor (Vulkan Puro)")
                    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
                    .with_decorations(false)
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
                self.hovered_node = interaction::node_at_screen(&self.graph, self.viewport, current);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Middle {
                    self.is_panning = state == ElementState::Pressed;
                } else if button == MouseButton::Right && state == ElementState::Pressed {
                    if let Some(node_id) = self.hovered_node {
                        self.editor.open_at_cursor(
                            node_id,
                            &self.graph,
                            &mut self.active_editor_node,
                            &mut self.selected_node,
                            &mut self.dragging_node,
                            &mut self.link_source_pin,
                            &mut self.template_palette,
                        );
                    } else {
                        self.active_editor_node = None;
                    }
                } else if button == MouseButton::Left && state == ElementState::Pressed {
                    if self.open_menu.is_some() {
                        if let Some(cursor) = self.last_cursor_position {
                            if let Some(action) = MenuBar::hit_test(cursor, self.open_menu) {
                                self.execute_menu_action(action);
                            }
                            self.open_menu = None;
                            return;
                        }
                    }
                    if let Some(cursor) = self.last_cursor_position {
                        if cursor.1 < 32.0 {
                            if let Some(menu) = MenuBar::click(cursor) {
                                self.open_menu = Some(menu);
                                return;
                            }
                            if let Some(window) = &self.window {
                                let _ = window.drag_window();
                            }
                            return;
                        }
                    }

                    let (linked, new_source) = interaction::try_finish_link(
                        &mut self.graph,
                        self.link_source_pin,
                        self.last_cursor_position,
                        self.viewport,
                        &mut self.selected_node,
                    );
                    self.link_source_pin = new_source;

                    if !linked {
                        if let Some(pin) = interaction::try_start_link(
                            &self.graph,
                            self.viewport,
                            self.last_cursor_position,
                            &mut self.selected_node,
                        ) {
                            self.link_source_pin = Some(pin);
                            self.dragging_node = None;
                        } else {
                            self.selected_node = self.hovered_node;
                            self.dragging_node = self.hovered_node;
                        }
                    } else {
                        self.dragging_node = None;
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
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }

                let PhysicalKey::Code(key) = event.physical_key else {
                    return;
                };

                // Ctrl+Shift+P: Command Palette
                if self.modifiers.control_key() && self.modifiers.shift_key() && key == KeyCode::KeyP {
                    self.command_palette.toggle();
                    return;
                }

                // Command palette mode
                if self.command_palette.open {
                    match key {
                        KeyCode::Escape => { self.command_palette.close(); }
                        KeyCode::ArrowUp => { self.command_palette.move_selection(-1); }
                        KeyCode::ArrowDown => { self.command_palette.move_selection(1); }
                        KeyCode::Enter => {
                            if let Some(action) = self.command_palette.execute_selected() {
                                self.execute_command_action(action);
                            }
                            self.command_palette.close();
                        }
                        _ => {
                            if let Some(text) = event.text {
                                for ch in text.chars() {
                                    self.command_palette.append_char(ch);
                                }
                            }
                        }
                    }
                    return;
                }

                if self.active_editor_node.is_some() {
                    match key {
                        KeyCode::Escape | KeyCode::Backspace | KeyCode::Enter | KeyCode::Tab
                        | KeyCode::ArrowLeft | KeyCode::ArrowRight | KeyCode::ArrowUp | KeyCode::ArrowDown
                        | KeyCode::Home | KeyCode::End => {
                            let node_id = self.active_editor_node.unwrap();
                            let changed = self.editor.handle_key(key, node_id, &mut self.graph, &mut self.active_editor_node);
                            if changed { self.auto_save(); }
                        }
                        KeyCode::F5 => {
                            self.compile_and_run_active_node();
                        }
                        _ => {
                            if let Some(text) = event.text {
                                let node_id = self.active_editor_node.unwrap();
                                let changed = self.editor.insert_text(&text, node_id, &mut self.graph);
                                if changed { self.auto_save(); }
                            }
                        }
                    }
                    return;
                }

                if self.template_palette.is_open() && self.handle_template_palette_key(key) {
                    return;
                }

                match key {
                    KeyCode::Tab => self.template_palette.toggle(),
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
                    KeyCode::KeyC => {
                        self.link_source_pin = interaction::start_link_from_selected(self.selected_node);
                    }
                    KeyCode::F5 => {
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

fn quick_slot_label(index: usize) -> String {
    match index {
        0..=8 => (index + 1).to_string(),
        9 => "0".to_string(),
        _ => "-".to_string(),
    }
}
