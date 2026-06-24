use ash::vk;

use crate::core::node_graph::{Node, NodeGraph, NodeLanguage, PinKind};
use crate::core::NodeId;
use crate::ui::theme::THEME;
use crate::vulkan::pipeline::{GraphicsPipeline, Vertex};
use crate::vulkan::text::{FontAtlas, TextPipeline, TextVertex, ATLAS_FONT_SIZE};

#[derive(Clone, Debug, Default)]
pub struct RenderState {
    pub hovered_node: Option<NodeId>,
    pub selected_node: Option<NodeId>,
    pub link_source_node: Option<NodeId>,
    pub code_editor_node: Option<NodeId>,
    pub template_palette_open: bool,
    pub template_visible_start: usize,
    pub selected_template_index: usize,
    pub template_entries: Vec<TemplatePaletteEntry>,
    pub workspace_label: String,
    pub code_editor: Option<CodeEditorState>,
    pub output: OutputPanel,
    pub frame_counter: u64,
    pub open_menu: Option<crate::app::runtime::MenuKind>,
    pub toast_message: Option<String>,
    pub sidebar_entries: Vec<crate::app::workspace::SidebarEntry>,
    pub sidebar_open: bool,
}

#[derive(Clone, Debug)]
pub struct TemplatePaletteEntry {
    pub label: String,
    pub color: [f32; 3],
}

#[derive(Clone, Debug)]
pub struct CodeEditorState {
    pub node_id: NodeId,
    pub title: String,
    pub language: String,
    pub code_path: String,
    pub lines: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub is_active: bool,
}

#[derive(Clone, Debug, Default)]
pub struct OutputPanel {
    pub lines: Vec<String>,
    pub is_error: bool,
    pub has_run: bool,
    pub error_line: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport2D {
    pub pan: [f32; 2],
    pub zoom: f32,
}

impl Default for Viewport2D {
    fn default() -> Self {
        Self { pan: [0.0, 0.0], zoom: 1.0 }
    }
}

impl Viewport2D {
    pub fn pan_by(&mut self, dx: f32, dy: f32) { self.pan[0] += dx; self.pan[1] += dy; }
    pub fn zoom_by(&mut self, steps: f32) { let f = 1.0 + steps * 0.10; self.zoom = (self.zoom * f.max(0.10)).clamp(0.25, 4.0); }
    pub fn zoom_at(&mut self, steps: f32, sx: f32, sy: f32) {
        let b = self.screen_to_world(sx, sy);
        self.zoom_by(steps);
        let a = self.screen_to_world(sx, sy);
        self.pan[0] += (a.0 - b.0) * self.zoom;
        self.pan[1] += (a.1 - b.1) * self.zoom;
    }
    pub fn screen_to_world(&self, x: f32, y: f32) -> (f32, f32) { ((x - self.pan[0]) / self.zoom, (y - self.pan[1]) / self.zoom) }
    pub fn screen_delta_to_world(&self, dx: f32, dy: f32) -> (f32, f32) { (dx / self.zoom, dy / self.zoom) }
    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) { (x * self.zoom + self.pan[0], y * self.zoom + self.pan[1]) }
    fn scale(&self, v: f32) -> f32 { v * self.zoom }
}

pub struct Renderer {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    vertex_capacity: usize,
    vertex_count: u32,
    text_vertex_buffer: vk::Buffer,
    text_vertex_buffer_memory: vk::DeviceMemory,
    text_vertex_capacity: usize,
    text_vertex_count: u32,
}

const MAX_VERTICES: usize = 65_536;
pub const NODE_WIDTH: f32 = 300.0;
pub const NODE_HEIGHT: f32 = 160.0;
pub const HEADER_HEIGHT: f32 = 36.0;
pub const PIN_SIZE: f32 = 11.0;
pub const PIN_ROW_HEIGHT: f32 = 20.0;
pub const SECTION_HEIGHT: f32 = 14.0;
const GRID_SPACING: f32 = 64.0;
const NODE_CORNER: f32 = 6.0;

impl Renderer {
    pub fn new(device: &ash::Device, instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        let buffer_size = (std::mem::size_of::<Vertex>() * MAX_VERTICES) as vk::DeviceSize;
        let buffer_info = vk::BufferCreateInfo { size: buffer_size, usage: vk::BufferUsageFlags::VERTEX_BUFFER, sharing_mode: vk::SharingMode::EXCLUSIVE, ..Default::default() };
        let vertex_buffer = unsafe { device.create_buffer(&buffer_info, None).unwrap() };
        let mem_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };
        let alloc_info = vk::MemoryAllocateInfo { allocation_size: mem_requirements.size, memory_type_index: find_memory_type(instance, physical_device, mem_requirements.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT), ..Default::default() };
        let vertex_buffer_memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0).unwrap() };

        let text_buffer_size = (std::mem::size_of::<TextVertex>() * MAX_VERTICES) as vk::DeviceSize;
        let text_buffer_info = vk::BufferCreateInfo { size: text_buffer_size, usage: vk::BufferUsageFlags::VERTEX_BUFFER, sharing_mode: vk::SharingMode::EXCLUSIVE, ..Default::default() };
        let text_vertex_buffer = unsafe { device.create_buffer(&text_buffer_info, None).unwrap() };
        let text_mem_req = unsafe { device.get_buffer_memory_requirements(text_vertex_buffer) };
        let text_alloc = vk::MemoryAllocateInfo { allocation_size: text_mem_req.size, memory_type_index: find_memory_type(instance, physical_device, text_mem_req.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT), ..Default::default() };
        let text_vertex_buffer_memory = unsafe { device.allocate_memory(&text_alloc, None).unwrap() };
        unsafe { device.bind_buffer_memory(text_vertex_buffer, text_vertex_buffer_memory, 0).unwrap() };

        Self {
            vertex_buffer, vertex_buffer_memory, vertex_capacity: MAX_VERTICES, vertex_count: 0,
            text_vertex_buffer, text_vertex_buffer_memory, text_vertex_capacity: MAX_VERTICES, text_vertex_count: 0,
        }
    }

    pub fn update_from_graph(&mut self, device: &ash::Device, graph: &NodeGraph, extent: vk::Extent2D, viewport: Viewport2D, state: RenderState, atlas: Option<&FontAtlas>) {
        let mut verts = Vec::with_capacity(graph.nodes().len() * 40);
        let mut text_verts = Vec::with_capacity(graph.nodes().len() * 64);

        self.push_grid(&mut verts, extent, viewport);
        self.push_links(&mut verts, graph, extent, viewport);
        for node in graph.nodes() {
            self.push_node(&mut verts, &mut text_verts, node, extent, viewport, &state, atlas);
            if verts.len() >= self.vertex_capacity { verts.truncate(self.vertex_capacity); break; }
        }
        if state.template_palette_open { self.push_template_palette(&mut verts, &mut text_verts, extent, &state, atlas); }
        self.push_activity_bar(&mut verts, extent);
        self.push_sidebar(&mut verts, &mut text_verts, extent, &state.sidebar_entries, state.sidebar_open, atlas);
        if let Some(editor) = &state.code_editor { self.push_code_editor(&mut verts, &mut text_verts, extent, editor, &state.output, state.frame_counter, atlas); }
        self.push_workspace_badge(&mut verts, &mut text_verts, extent, &state.workspace_label, atlas);
        self.push_menu_bar(&mut verts, &mut text_verts, extent, state.open_menu, atlas);
        if let Some(msg) = &state.toast_message {
            self.push_toast(&mut text_verts, extent, msg, state.frame_counter, atlas);
        }

        self.vertex_count = verts.len() as u32;
        if !verts.is_empty() {
            let copy_size = (std::mem::size_of::<Vertex>() * verts.len()) as vk::DeviceSize;
            unsafe {
                let ptr = device.map_memory(self.vertex_buffer_memory, 0, copy_size, vk::MemoryMapFlags::empty()).unwrap() as *mut Vertex;
                ptr.copy_from_nonoverlapping(verts.as_ptr(), verts.len());
                device.unmap_memory(self.vertex_buffer_memory);
            }
        }

        self.text_vertex_count = text_verts.len() as u32;
        if !text_verts.is_empty() {
            let copy_size = (std::mem::size_of::<TextVertex>() * text_verts.len()) as vk::DeviceSize;
            unsafe {
                let ptr = device.map_memory(self.text_vertex_buffer_memory, 0, copy_size, vk::MemoryMapFlags::empty()).unwrap() as *mut TextVertex;
                ptr.copy_from_nonoverlapping(text_verts.as_ptr(), text_verts.len());
                device.unmap_memory(self.text_vertex_buffer_memory);
            }
        }
    }

    // ─── Nodo estilo Houdini ───
    fn push_node(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, node: &Node, extent: vk::Extent2D, vp: Viewport2D, state: &RenderState, atlas: Option<&FontAtlas>) {
        let (x, y) = vp.world_to_screen(node.position.x, node.position.y);
        let w = vp.scale(NODE_WIDTH);
        let h = vp.scale(NODE_HEIGHT);
        let hdr = vp.scale(HEADER_HEIGHT);
        let is_sel = state.selected_node == Some(node.id);
        let is_hov = state.hovered_node == Some(node.id);
        let is_src = state.link_source_node == Some(node.id);
        let is_editing = state.code_editor_node == Some(node.id);

        let (hdr_color, body_color, accent) = match node.language {
            NodeLanguage::Rust => (THEME.vermillion, THEME.node_rust_body, THEME.vermillion),
            NodeLanguage::Text => (THEME.copper, THEME.node_text_body, THEME.copper),
            NodeLanguage::Auto => (THEME.jade_green, THEME.node_auto_body, THEME.jade_green),
        };

        // Sombra exterior
        let shadow = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
        push_rounded_rect(verts, extent, x - vp.scale(4.0), y - vp.scale(4.0), w + vp.scale(8.0), h + vp.scale(8.0), vp.scale(NODE_CORNER + 2.0), shadow);

        // Borde de selección/hover
        if is_src || is_editing || is_sel || is_hov {
            let bc = if is_src { THEME.jade_green } else if is_editing { THEME.plum } else if is_sel { THEME.imperial_gold } else { accent };
            let bc3 = [bc.r, bc.g, bc.b];
            push_rounded_rect(verts, extent, x - vp.scale(2.0), y - vp.scale(2.0), w + vp.scale(4.0), h + vp.scale(4.0), vp.scale(NODE_CORNER + 1.0), bc3);
        }

        // Cuerpo del nodo
        let bc = [body_color.r, body_color.g, body_color.b];
        push_rounded_rect(verts, extent, x, y, w, h, vp.scale(NODE_CORNER), bc);

        // Header (estilo Houdini: sin brillo, solo color sólido elegante)
        let hc = [hdr_color.r, hdr_color.g, hdr_color.b];
        push_rounded_rect_top(verts, extent, x, y, w, hdr, vp.scale(NODE_CORNER), hc);

        // Franja divisoria header/cuerpo (más oscura para profundidad)
        let hc_dim = [hdr_color.r * 0.4, hdr_color.g * 0.4, hdr_color.b * 0.4];
        push_rect(verts, extent, x, y + hdr - vp.scale(1.5), w, vp.scale(1.5), hc_dim);

        // ── TIPO a la IZQUIERDA del header (estilo Houdini: "RUST", "TEXT", "AUTO") ──
        let type_label = match node.language {
            NodeLanguage::Rust => "RUST",
            NodeLanguage::Text => "TEXT",
            NodeLanguage::Auto => "AUTO",
        };
        let type_color = [1.0, 0.95, 0.85];
        push_text_gpu(text_verts, extent, x + vp.scale(8.0), y + vp.scale(7.0), vp.scale(2.0), type_color, type_label, atlas);

        // ── Status dot a la DERECHA del header (elegante, sin highlight blanco) ──
        let status_color = if node.code.is_empty() {
            [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b]
        } else if node.code.contains("fn main") {
            [THEME.jade_green.r, THEME.jade_green.g, THEME.jade_green.b]
        } else {
            [THEME.copper.r, THEME.copper.g, THEME.copper.b]
        };
        let dot_r = vp.scale(3.0);
        let dot_x = x + w - vp.scale(12.0);
        let dot_y = y + hdr * 0.5;
        // Solo el dot, sin anillo brillante
        push_circle(verts, extent, dot_x, dot_y, dot_r, status_color);

        // ── Sombra interior header → body (sutil) ──
        let inner_shadow = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
        push_rect(verts, extent, x + vp.scale(2.0), y + hdr, w - vp.scale(4.0), vp.scale(2.0), inner_shadow);

        // ── TÍTULO del nodo (debajo del header) ──
        let title_y = y + hdr + vp.scale(8.0);
        let title_color = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
        push_text_gpu(text_verts, extent, x + vp.scale(8.0), title_y, vp.scale(2.5), title_color, &node.title, atlas);

        // ── SECCIÓN DE PINS con label Houdini-style ──
        let mut current_y = title_y + vp.scale(20.0);
        for (i, pin) in node.inputs.iter().enumerate() {
            let py = current_y + vp.scale(PIN_ROW_HEIGHT) * i as f32 + vp.scale(PIN_ROW_HEIGHT) * 0.5;
            let cx = x;
            let cy = py;

            // Pin circular con sombra
            push_circle(verts, extent, cx + 1.0, cy + 1.0, vp.scale(PIN_SIZE) * 0.5, [0.0, 0.0, 0.0]);
            push_circle(verts, extent, cx, cy, vp.scale(PIN_SIZE) * 0.5, [THEME.pin_input.r, THEME.pin_input.g, THEME.pin_input.b]);
            push_circle(verts, extent, cx - vp.scale(PIN_SIZE) * 0.1, cy - vp.scale(PIN_SIZE) * 0.1, vp.scale(PIN_SIZE) * 0.22, [THEME.pin_input.r * 1.4, THEME.pin_input.g * 1.4, THEME.pin_input.b * 1.4]);

            // Label
            let label_color = [THEME.text_secondary.r, THEME.text_secondary.g, THEME.text_secondary.b];
            push_text_gpu(text_verts, extent, x + vp.scale(14.0), cy - vp.scale(3.0), vp.scale(1.5), label_color, &pin.label, atlas);
        }

        // Outputs alineados a la derecha
        for (i, pin) in node.outputs.iter().enumerate() {
            let py = current_y + vp.scale(PIN_ROW_HEIGHT) * i as f32 + vp.scale(PIN_ROW_HEIGHT) * 0.5;
            let cx = x + w;
            let cy = py;

            push_circle(verts, extent, cx + 1.0, cy + 1.0, vp.scale(PIN_SIZE) * 0.5, [0.0, 0.0, 0.0]);
            push_circle(verts, extent, cx, cy, vp.scale(PIN_SIZE) * 0.5, [THEME.pin_output.r, THEME.pin_output.g, THEME.pin_output.b]);
            push_circle(verts, extent, cx - vp.scale(PIN_SIZE) * 0.1, cy - vp.scale(PIN_SIZE) * 0.1, vp.scale(PIN_SIZE) * 0.22, [THEME.pin_output.r * 1.4, THEME.pin_output.g * 1.4, THEME.pin_output.b * 1.4]);

            let label_color = [THEME.text_secondary.r, THEME.text_secondary.g, THEME.text_secondary.b];
            let label_w = pin.label.len() as f32 * vp.scale(6.5);
            push_text_gpu(text_verts, extent, x + w - vp.scale(14.0) - label_w, cy - vp.scale(3.0), vp.scale(1.5), label_color, &pin.label, atlas);
        }

        let pin_count = node.inputs.len() + node.outputs.len();
        current_y = current_y + vp.scale(PIN_ROW_HEIGHT) * pin_count as f32;

        // ── DIVISOR entre pins y código ──
        if pin_count > 0 {
            current_y += vp.scale(4.0);
            let divider_color = [THEME.border_secondary.r, THEME.border_secondary.g, THEME.border_secondary.b];
            push_rect(verts, extent, x + vp.scale(8.0), current_y, w - vp.scale(16.0), vp.scale(1.0), divider_color);
            current_y += vp.scale(6.0);
        }

        // ── CODE PREVIEW estilo parámetro Houdini (key: value) ──
        let key_color = [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b];
        let key_label = if node.code.is_empty() { "code" } else { "source" };
        push_text_gpu(text_verts, extent, x + vp.scale(8.0), current_y, vp.scale(1.3), key_color, key_label, atlas);
        current_y += vp.scale(16.0);

        let code_preview = node.code.lines()
            .find(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
            .or_else(|| node.code.lines().find(|l| !l.trim().is_empty()))
            .unwrap_or("");
        if !code_preview.is_empty() {
            let value_color = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
            push_text_gpu(text_verts, extent, x + vp.scale(8.0), current_y, vp.scale(1.8), value_color, &clip_text(code_preview.trim(), 36), atlas);
        } else {
            let value_color = [THEME.text_muted.r * 0.8, THEME.text_muted.g * 0.8, THEME.text_muted.b * 0.8];
            push_text_gpu(text_verts, extent, x + vp.scale(8.0), current_y, vp.scale(1.5), value_color, "  (vacio)", atlas);
        }

        // Puntos decorativos en esquinas del header
        let dot = vp.scale(2.0);
        let dc = [THEME.border_gold.r, THEME.border_gold.g, THEME.border_gold.b];
        push_rect(verts, extent, x + w - vp.scale(5.0), y + h - vp.scale(5.0), dot, dot, dc);
    }

    // ─── Grid estilo cuaderno de caligrafía ───
    fn push_grid(&self, verts: &mut Vec<Vertex>, extent: vk::Extent2D, vp: Viewport2D) {
        let tl = vp.screen_to_world(0.0, 0.0);
        let br = vp.screen_to_world(extent.width as f32, extent.height as f32);
        let min_x = tl.0.min(br.0); let max_x = tl.0.max(br.0);
        let min_y = tl.1.min(br.1); let max_y = tl.1.max(br.1);

        let sx = (min_x / GRID_SPACING).floor() as i32 - 1;
        let ex = (max_x / GRID_SPACING).ceil() as i32 + 1;
        let sy = (min_y / GRID_SPACING).floor() as i32 - 1;
        let ey = (max_y / GRID_SPACING).ceil() as i32 + 1;

        // Líneas principales (ejes)
        let axis_c = [THEME.grid_axis.r, THEME.grid_axis.g, THEME.grid_axis.b];
        let line_c = [THEME.grid_line.r, THEME.grid_line.g, THEME.grid_line.b];

        for gx in sx..=ex {
            let wx = gx as f32 * GRID_SPACING;
            let from = vp.world_to_screen(wx, min_y - GRID_SPACING);
            let to = vp.world_to_screen(wx, max_y + GRID_SPACING);
            let (c, t) = if gx == 0 { (axis_c, 2.0) } else { (line_c, 0.8) };
            push_line(verts, extent, from, to, t, c);
        }
        for gy in sy..=ey {
            let wy = gy as f32 * GRID_SPACING;
            let from = vp.world_to_screen(min_x - GRID_SPACING, wy);
            let to = vp.world_to_screen(max_x + GRID_SPACING, wy);
            let (c, t) = if gy == 0 { (axis_c, 2.0) } else { (line_c, 0.8) };
            push_line(verts, extent, from, to, t, c);
        }

        // Puntos en intersecciones (estilo puntos de tinta)
        if vp.zoom > 0.6 {
            let dot_c = [THEME.grid_dot.r, THEME.grid_dot.g, THEME.grid_dot.b];
            let dot_sz = vp.scale(2.0).max(1.0);
            for gx in sx..=ex {
                for gy in sy..=ey {
                    let (sx2, sy2) = vp.world_to_screen(gx as f32 * GRID_SPACING, gy as f32 * GRID_SPACING);
                    if sx2 > -20.0 && sx2 < extent.width as f32 + 20.0 && sy2 > -20.0 && sy2 < extent.height as f32 + 20.0 {
                        push_rect(verts, extent, sx2 - dot_sz * 0.5, sy2 - dot_sz * 0.5, dot_sz, dot_sz, dot_c);
                    }
                }
            }
        }
    }

    // ─── Conexiones estilo tinta con curva Bezier y glow sutil ───
    fn push_links(&self, verts: &mut Vec<Vertex>, graph: &NodeGraph, extent: vk::Extent2D, vp: Viewport2D) {
        for link in graph.links() {
            let Some(fa) = graph.locate_pin(link.from) else { continue; };
            let Some(ta) = graph.locate_pin(link.to) else { continue; };
            let fn_ = &graph.nodes()[fa.node_index];
            let tn = &graph.nodes()[ta.node_index];
            let from = pin_screen_center(fn_, fa.kind, fa.slot, vp);
            let to = pin_screen_center(tn, ta.kind, ta.slot, vp);

            let link_c = THEME.link_default;
            let color = [link_c.r, link_c.g, link_c.b];

            // Glow sutil (más estrecho y oscuro)
            let glow = [link_c.r * 0.4, link_c.g * 0.4, link_c.b * 0.3];
            push_bezier(verts, extent, from, to, vp.scale(8.0).max(2.5), glow);
            // Sombra oscura (profundidad)
            let shadow_c = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
            push_bezier(verts, extent, from, to, vp.scale(4.5).max(1.5), shadow_c);
            // Línea principal (más delgada, más elegante)
            push_bezier(verts, extent, from, to, vp.scale(2.2).max(0.8), color);
            // Highlight sutil
            let highlight = [link_c.r * 1.25, link_c.g * 1.25, link_c.b * 1.25];
            push_bezier(verts, extent, from, to, vp.scale(0.8).max(0.3), highlight);
        }
    }

    // ─── Paleta de templates estilo menú chino ───
    fn push_template_palette(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, state: &RenderState, atlas: Option<&FontAtlas>) {
        let px = 40.0;
        let py = 40.0;
        let pw = 580.0;
        let ih = 36.0;
        let vis = state.template_entries.len().min(12);
        let ph = 94.0 + ih * vis as f32;

        // Sombra
        let shadow = [THEME.ink_black.r * 0.8, THEME.ink_black.g * 0.8, THEME.ink_black.b * 0.8];
        push_rounded_rect(verts, extent, px - 8.0, py - 8.0, pw + 16.0, ph + 16.0, 12.0, shadow);

        // Fondo principal
        let bg = [THEME.slate.r, THEME.slate.g, THEME.slate.b];
        push_rounded_rect(verts, extent, px, py, pw, ph, 8.0, bg);

        // Header decorativo (rojo vermillón imperial)
        let hdr = [THEME.vermillion.r, THEME.vermillion.g, THEME.vermillion.b];
        push_rounded_rect_top(verts, extent, px, py, pw, 54.0, 8.0, hdr);

        // Línea dorada decorativa bajo header
        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        push_rect(verts, extent, px + 12.0, py + 50.0, pw - 24.0, 3.0, gold);

        // Texto del header
        push_text_gpu(text_verts, extent, px + 20.0, py + 18.0, 2.0, [1.0, 0.92, 0.7], "RUST TEMPLATES", atlas);
        push_text_gpu(text_verts, extent, px + pw - 175.0, py + 20.0, 1.4, [1.0, 0.92, 0.7], "ENTER CREATE", atlas);

        // Items
        for i in 0..vis {
            let y = py + 62.0 + i as f32 * ih;
            let gi = state.template_visible_start + i;
            let sel = gi == state.selected_template_index;
            let entry = &state.template_entries[i];

            let item_bg = if sel {
                [THEME.jade_dark.r + 0.08, THEME.jade_dark.g + 0.08, THEME.jade_dark.b + 0.08]
            } else if i % 2 == 0 {
                [THEME.ink_medium.r, THEME.ink_medium.g, THEME.ink_medium.b]
            } else {
                [THEME.obsidian.r, THEME.obsidian.g, THEME.obsidian.b]
            };
            push_rect(verts, extent, px + 14.0, y, pw - 28.0, ih - 4.0, item_bg);

            // Indicador de selección (barra lateral dorada)
            if sel {
                push_rect(verts, extent, px + 14.0, y, 4.0, ih - 4.0, gold);
            }

            // Color del template
            push_rect(verts, extent, px + 26.0, y + 8.0, 8.0, ih - 20.0, entry.color);

            // Texto
            let tc = if sel { [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b] } else { [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b] };
            push_text_gpu(text_verts, extent, px + 44.0, y + 10.0, 1.4, tc, &entry.label, atlas);
        }
    }

    // ─── Editor de código con cursor y coloring de sintaxis ───
    fn push_code_editor(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, editor: &CodeEditorState, output: &OutputPanel, frame_counter: u64, atlas: Option<&FontAtlas>) {
        let margin = 32.0;
        let pw = (extent.width as f32 * 0.42).clamp(420.0, 720.0);
        let total_h = (extent.height as f32 - margin * 2.0).clamp(400.0, 820.0);
        // Altura dividida: 60% editor, 40% output
        let out_h = if output.has_run { 220.0 } else { 0.0 };
        let ph = total_h - out_h - 10.0;
        let px = (extent.width as f32 - pw - margin).max(margin);
        let py = margin;

        let shade = [THEME.ink_black.r * 0.78, THEME.ink_black.g * 0.78, THEME.ink_black.b * 0.78];
        push_rounded_rect(verts, extent, px - 12.0, py - 12.0, pw + 24.0, ph + 24.0, 14.0, shade);

        let bg = [THEME.ink_deep.r, THEME.ink_deep.g, THEME.ink_deep.b];
        push_rounded_rect(verts, extent, px, py, pw, ph, 10.0, bg);

        let hdr = [THEME.indigo.r, THEME.indigo.g, THEME.indigo.b];
        push_rounded_rect_top(verts, extent, px, py, pw, 72.0, 10.0, hdr);

        let accent = [THEME.plum.r, THEME.plum.g, THEME.plum.b];
        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        push_rect(verts, extent, px + 16.0, py + 68.0, pw - 32.0, 3.0, gold);
        push_rect(verts, extent, px, py + 82.0, 5.0, ph - 98.0, accent);

        let primary = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
        push_text_gpu(text_verts, extent, px + 20.0, py + 24.0, 2.2, primary, "NODE CODE EDITOR", atlas);
        // Indicador F5
        push_text_gpu(text_verts, extent, px + pw - 230.0, py + 28.0, 1.2, [THEME.text_secondary.r, THEME.text_secondary.g, THEME.text_secondary.b], "F5 = play", atlas);
        // Indicador LIVE o estado
        let status_label = if output.has_run { if output.is_error { "ERROR" } else { "OK" } } else { "LIVE" };
        let status_color = if output.has_run { if output.is_error { gold } else { [THEME.jade_green.r, THEME.jade_green.g, THEME.jade_green.b] } } else { gold };
        push_text_gpu(text_verts, extent, px + pw - 140.0, py + 28.0, 1.4, status_color, status_label, atlas);

        let title = format!("{}  [{}]", editor.title, editor.language);
        push_text_gpu(text_verts, extent, px + 22.0, py + 92.0, 1.5, gold, &clip_text(&title, 54), atlas);

        let meta = format!("node {} | {}", editor.node_id.0, editor.code_path);
        push_text_gpu(text_verts, extent, px + 22.0, py + 116.0, 1.15, [THEME.text_secondary.r, THEME.text_secondary.g, THEME.text_secondary.b], &clip_text(&meta, 66), atlas);

        let code_x = px + 24.0;
        let code_y = py + 152.0;
        let line_h = 22.0;
        let max_lines = ((ph - 190.0) / line_h).max(1.0) as usize;
        let total_lines = editor.lines.len();
        let scroll_offset = if total_lines > max_lines { total_lines.saturating_sub(max_lines) } else { 0 };

        let keyword_color = [THEME.copper.r, THEME.copper.g, THEME.copper.b];
        let string_color = [THEME.jade_green.r, THEME.jade_green.g, THEME.jade_green.b];
        let comment_color = [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b];
        let error_highlight = [0.45, 0.18, 0.15]; // rojo oscuro translucido

        for screen_line in 0..max_lines.min(total_lines) {
            let line_idx = scroll_offset + screen_line;
            let line = editor.lines.get(line_idx).map(|s| s.as_str()).unwrap_or("");
            let y = code_y + screen_line as f32 * line_h;

            if screen_line % 2 == 0 {
                push_rect(verts, extent, px + 16.0, y - 4.0, pw - 32.0, line_h, [THEME.ink_medium.r, THEME.ink_medium.g, THEME.ink_medium.b]);
            }

            // Highlight de línea actual
            if editor.is_active && line_idx == editor.cursor_line {
                push_rect(verts, extent, px + 16.0, y - 4.0, pw - 32.0, line_h, [THEME.jade_dark.r + 0.04, THEME.jade_dark.g + 0.04, THEME.jade_dark.b + 0.04]);
            }

            // Highlight de línea de error (del output panel)
            if let Some(err_line) = output.error_line {
                if err_line == line_idx + 1 {
                    push_rect(verts, extent, px + 16.0, y - 4.0, pw - 32.0, line_h, error_highlight);
                }
            }

            // Separador visual
            push_rect(verts, extent, code_x + 40.0, y - 4.0, 1.0, line_h, [THEME.border_secondary.r, THEME.border_secondary.g, THEME.border_secondary.b]);

            let line_no = format!("{:>3}", line_idx + 1);
            push_text_gpu(text_verts, extent, code_x, y + 10.0, 1.1, [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b], &line_no, atlas);

            let trimmed = line.trim();
            let (text_color, text_str) = if trimmed.starts_with("//") {
                (comment_color, line)
            } else if trimmed.starts_with("pub") || trimmed.starts_with("fn ") || trimmed.starts_with("let ") || trimmed.starts_with("mut ") || trimmed.starts_with("if ") || trimmed.starts_with("else") || trimmed.starts_with("match ") || trimmed.starts_with("return") || trimmed.starts_with("where") || trimmed.starts_with("struct ") || trimmed.starts_with("enum ") || trimmed.starts_with("impl ") || trimmed.starts_with("use ") || trimmed.starts_with("mod ") || trimmed.starts_with("for ") || trimmed.starts_with("while ") || trimmed.starts_with("loop ") || trimmed.starts_with("async ") || trimmed.starts_with("await ") || trimmed.starts_with("self") || trimmed.starts_with("Self") {
                (keyword_color, line)
            } else {
                let has_string = line.contains('"');
                if has_string { (string_color, line) } else { (primary, line) }
            };
            push_text_gpu(text_verts, extent, code_x + 48.0, y + 10.0, 1.2, text_color, &clip_text(text_str, 68), atlas);
        }

        // Cursor parpadeante (parpadea cada 30 frames)
        if editor.is_active && (frame_counter / 30) % 2 == 0 {
            let cursor_line_idx = editor.cursor_line.saturating_sub(scroll_offset);
            if cursor_line_idx < max_lines {
                let cursor_y = code_y + cursor_line_idx as f32 * line_h;
                let cursor_line_text = editor.lines.get(editor.cursor_line).map_or("", |l| l.as_str());
                let col = editor.cursor_col.min(cursor_line_text.len());
                let display_text: String = cursor_line_text.chars().take(col).collect();
                let char_count = display_text.chars().count() as f32;
                let cursor_x = code_x + 48.0 + char_count * 7.5;
                push_rect(verts, extent, cursor_x, cursor_y - 2.0, 2.0, line_h - 2.0, gold);
            }
        }

        let hint = "F5 = compilar+ejecutar | flechas = navegar | Esc = cerrar";
        push_text_gpu(text_verts, extent, px + 20.0, py + ph - 24.0, 1.05, [THEME.text_jade.r, THEME.text_jade.g, THEME.text_jade.b], hint, atlas);

        // ── PANEL DE OUTPUT (debajo del editor) ──
        if output.has_run {
            let opy = py + ph + 10.0;
            let op_h = out_h;

            let op_shade = [THEME.ink_black.r * 0.78, THEME.ink_black.g * 0.78, THEME.ink_black.b * 0.78];
            push_rounded_rect(verts, extent, px - 8.0, opy - 8.0, pw + 16.0, op_h + 16.0, 12.0, op_shade);

            let op_bg = [THEME.obsidian.r, THEME.obsidian.g, THEME.obsidian.b];
            push_rounded_rect(verts, extent, px, opy, pw, op_h, 8.0, op_bg);

            // Header del output
            let op_hdr_color = if output.is_error { [0.35, 0.18, 0.16] } else { [0.18, 0.28, 0.22] };
            push_rect(verts, extent, px, opy, pw, 28.0, op_hdr_color);

            let op_title = if output.is_error { "OUTPUT (ERROR)" } else { "OUTPUT (OK)" };
            let op_title_color = if output.is_error { [0.95, 0.65, 0.45] } else { [0.55, 0.85, 0.65] };
            push_text_gpu(text_verts, extent, px + 14.0, opy + 8.0, 1.5, op_title_color, op_title, atlas);

            // Líneas del output
            let op_y_start = opy + 36.0;
            let op_line_h = 18.0;
            let op_max_lines = ((op_h - 50.0) / op_line_h).max(1.0) as usize;
            let op_total = output.lines.len();
            let op_scroll = if op_total > op_max_lines { op_total - op_max_lines } else { 0 };

            let op_text_color = if output.is_error { [0.95, 0.75, 0.55] } else { [0.75, 0.85, 0.72] };

            for sline in 0..op_max_lines.min(op_total) {
                let line_idx = op_scroll + sline;
                let line = output.lines.get(line_idx).map(|s| s.as_str()).unwrap_or("");
                let ly = op_y_start + sline as f32 * op_line_h;
                if sline % 2 == 0 {
                    push_rect(verts, extent, px + 8.0, ly - 2.0, pw - 16.0, op_line_h, [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b]);
                }
                push_text_gpu(text_verts, extent, px + 16.0, ly + 4.0, 1.1, op_text_color, &clip_text(line, 82), atlas);
            }
        }
    }

    // ─── Top Menu Bar estilo VSCode ───
    fn push_menu_bar(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, open_menu: Option<crate::app::runtime::MenuKind>, atlas: Option<&FontAtlas>) {
        // Barra superior (32px de alto) con fondo más visible
        let bar_bg = [0.055, 0.045, 0.038]; // #0E0B0A
        push_rect(verts, extent, 0.0, 0.0, extent.width as f32, 32.0, bar_bg);
        // Borde inferior dorado sutil
        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        push_rect(verts, extent, 0.0, 31.0, extent.width as f32, 1.0, [gold[0]*0.4, gold[1]*0.4, gold[2]*0.4]);

        // Logo a la izquierda
        let logo_color = [gold[0], gold[1], gold[2]];
        push_text_gpu(text_verts, extent, 16.0, 10.0, 1.7, logo_color, "Ultra-Omega", atlas);

        // Separador vertical despues del logo
        push_rect(verts, extent, 140.0, 6.0, 1.0, 20.0, [0.25, 0.22, 0.18]);

        // Items del menu - empiezan DESPUES del separador
        let items = [("File", crate::app::runtime::MenuKind::File),
                     ("Edit", crate::app::runtime::MenuKind::Edit),
                     ("View", crate::app::runtime::MenuKind::View),
                     ("Run", crate::app::runtime::MenuKind::Run)];
        let mut x = 152.0;
        for (label, kind) in items.iter() {
            let w = (label.len() as f32) * 9.0 + 24.0;
            let is_active = open_menu == Some(*kind);
            if is_active {
                let active_bg = [0.18, 0.14, 0.10];
                push_rect(verts, extent, x, 0.0, w, 32.0, active_bg);
                // Indicador de bottom
                push_rect(verts, extent, x, 30.0, w, 2.0, [gold[0]*0.7, gold[1]*0.7, gold[2]*0.7]);
            }
            let txt_color = if is_active { [1.0, 0.95, 0.85] } else { [0.78, 0.74, 0.66] };
            push_text_gpu(text_verts, extent, x + 12.0, 10.0, 1.5, txt_color, label, atlas);
            x += w;
        }

        // Indicador derecho
        let right_txt = "Vulkan Puro | Rust";
        let rw = (right_txt.len() as f32) * 7.5;
        push_text_gpu(text_verts, extent, extent.width as f32 - rw - 16.0, 10.0, 1.3, [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b], right_txt, atlas);

        // Dropdown menu si esta abierto
        if let Some(menu) = open_menu {
            // Calcular posicion X segun el menu activo
            let menu_x = match menu {
                crate::app::runtime::MenuKind::File => 152.0,
                crate::app::runtime::MenuKind::Edit => 200.0,
                crate::app::runtime::MenuKind::View => 248.0,
                crate::app::runtime::MenuKind::Run => 300.0,
            };
            let my = 32.0;
            let mw = 240.0;
            let items_text: Vec<(&str, &str)> = match menu {
                crate::app::runtime::MenuKind::File => vec![
                    ("New Project", "Ctrl+N"),
                    ("Open Folder...", "Ctrl+O"),
                    ("Save", "Ctrl+S"),
                    ("Export Graph", ""),
                ],
                crate::app::runtime::MenuKind::Edit => vec![
                    ("Delete Selected", "Del"),
                    ("Duplicate Node", "Ctrl+D"),
                    ("Select All", "Ctrl+A"),
                ],
                crate::app::runtime::MenuKind::View => vec![
                    ("Reset Zoom", "R"),
                    ("Zoom In", "Ctrl++"),
                    ("Zoom Out", "Ctrl+-"),
                    ("Toggle Grid", "G"),
                ],
                crate::app::runtime::MenuKind::Run => vec![
                    ("Run Active Node", "F5"),
                    ("Build Project", "Ctrl+B"),
                    ("Clean Build", ""),
                ],
            };
            let mh = items_text.len() as f32 * 32.0 + 12.0;

            // Sombra
            push_rect(verts, extent, menu_x + 4.0, my + 4.0, mw, mh, [0.0, 0.0, 0.0]);
            // Fondo del dropdown
            let dd_bg = [0.082, 0.072, 0.062]; // #151210
            push_rounded_rect(verts, extent, menu_x, my, mw, mh, 4.0, dd_bg);
            // Borde dorado
            push_rect(verts, extent, menu_x, my, mw, 1.0, [gold[0]*0.6, gold[1]*0.6, gold[2]*0.6]);
            push_rect(verts, extent, menu_x, my + mh - 1.0, mw, 1.0, [gold[0]*0.4, gold[1]*0.4, gold[2]*0.4]);

            // Items
            for (i, (label, shortcut)) in items_text.iter().enumerate() {
                let iy = my + 6.0 + i as f32 * 32.0;
                // Hover background sutil
                let item_bg = [0.13, 0.10, 0.08];
                push_rect(verts, extent, menu_x + 4.0, iy, mw - 8.0, 28.0, item_bg);
                // Label
                let lbl_color = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
                push_text_gpu(text_verts, extent, menu_x + 16.0, iy + 7.0, 1.4, lbl_color, label, atlas);
                // Shortcut
                if !shortcut.is_empty() {
                    let sc_color = [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b];
                    let sc_w = (shortcut.len() as f32) * 7.0;
                    push_text_gpu(text_verts, extent, menu_x + mw - sc_w - 16.0, iy + 7.0, 1.2, sc_color, shortcut, atlas);
                }
                // Separador sutil entre items
                if i < items_text.len() - 1 {
                    push_rect(verts, extent, menu_x + 12.0, iy + 28.0, mw - 24.0, 0.5, [0.18, 0.15, 0.12]);
                }
            }
        }
    }

    // ─── Toast notification con fade animation ───
    fn push_toast(&self, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, msg: &str, frame: u64, atlas: Option<&FontAtlas>) {
        let tw = (msg.len() as f32) * 8.0 + 40.0;
        let th = 36.0;
        let tx = (extent.width as f32 - tw) * 0.5;
        let ty = 60.0;

        // Sutil fondo con alpha (solo outline)
        // Skipping background for now - use only text shadow effect

        // Texto
        let color = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        push_text_gpu(text_verts, extent, tx + 20.0, ty + 10.0, 1.5, color, msg, atlas);

        // Sombra de texto (offset)
        let shadow = [0.0, 0.0, 0.0];
        push_text_gpu(text_verts, extent, tx + 21.0, ty + 11.0, 1.5, shadow, msg, atlas);
    }

    // ─── Activity Bar VSCode (iconos verticales izquierda) ───
    fn push_activity_bar(&self, verts: &mut Vec<Vertex>, extent: vk::Extent2D) {
        const ACT_W: f32 = 44.0;
        let ab_y = 32.0;
        let ab_h = extent.height as f32 - 32.0 - 24.0;

        // Fondo
        let ab_bg = [0.040, 0.034, 0.028]; // #0A0907
        push_rect(verts, extent, 0.0, ab_y, ACT_W, ab_h, ab_bg);
        // Borde derecho
        push_rect(verts, extent, ACT_W, ab_y, 1.0, ab_h, [0.18, 0.15, 0.12]);

        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        let active = [0.20, 0.16, 0.12];

        // Icon 1: Explorer (activo por defecto)
        push_rect(verts, extent, 0.0, ab_y + 6.0, ACT_W, 40.0, active);
        push_rect(verts, extent, 0.0, ab_y + 6.0, 2.0, 40.0, [gold[0], gold[1], gold[2]]);

        // Dibujar icono Explorer (2 carpetas stacked)
        let ix = 14.0;
        let iy = ab_y + 14.0;
        push_rounded_rect(verts, extent, ix, iy, 16.0, 11.0, 2.0, [0.55, 0.42, 0.20]);
        push_rounded_rect(verts, extent, ix + 3.0, iy + 13.0, 16.0, 11.0, 2.0, [0.65, 0.52, 0.30]);

        // Icon 2: Search (lupa)
        let iy2 = ab_y + 56.0;
        push_circle(verts, extent, 18.0, iy2 + 8.0, 6.0, [0.40, 0.36, 0.30]);
        push_circle(verts, extent, 18.0, iy2 + 8.0, 3.5, [0.040, 0.034, 0.028]);
        // Mango de la lupa
        push_line(verts, extent, (23.0, iy2 + 13.0), (27.0, iy2 + 17.0), 2.0, [0.40, 0.36, 0.30]);

        // Icon 3: Source Control (rama)
        let iy3 = ab_y + 96.0;
        push_circle(verts, extent, 22.0, iy3 + 4.0, 3.0, [0.40, 0.36, 0.30]);
        push_circle(verts, extent, 22.0, iy3 + 14.0, 3.0, [0.40, 0.36, 0.30]);
        push_rect(verts, extent, 21.0, iy3 + 4.0, 2.0, 10.0, [0.40, 0.36, 0.30]);

        // Icon 4: Extensions (cuadrado con 4 puntos)
        let iy4 = ab_y + 136.0;
        push_rect(verts, extent, 14.0, iy4 + 2.0, 16.0, 16.0, [0.40, 0.36, 0.30]);
        // 4 puntos esquinas
        for &(dx, dy) in &[(0.0, 0.0), (13.0, 0.0), (0.0, 13.0), (13.0, 13.0)] {
            push_circle(verts, extent, 14.0 + dx + 2.0, iy4 + 2.0 + dy + 2.0, 1.5, [0.040, 0.034, 0.028]);
        }

        // Icon 5: Settings (engranaje simplificado) al final
        let iy5 = ab_h - 40.0;
        push_circle(verts, extent, 22.0, iy5 + 8.0, 7.0, [0.40, 0.36, 0.30]);
        push_circle(verts, extent, 22.0, iy5 + 8.0, 3.0, [0.040, 0.034, 0.028]);
        // Dientes del engranaje
        for i in 0..6 {
            let a = (i as f32) * std::f32::consts::TAU / 6.0;
            let dx = 22.0 + (a.cos() * 8.0);
            let dy = iy5 + 8.0 + (a.sin() * 8.0);
            push_rect(verts, extent, dx - 1.5, dy - 1.5, 3.0, 3.0, [0.40, 0.36, 0.30]);
        }
    }

    // ─── Sidebar VSCode (explorador de archivos + secciones) ───
    fn push_sidebar(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, entries: &[crate::app::workspace::SidebarEntry], has_workspace: bool, atlas: Option<&FontAtlas>) {
        const ACT_W: f32 = 44.0;
        const SB_W: f32 = 240.0;
        let sb_x = ACT_W;
        let sb_y = 32.0;
        let sb_h = extent.height as f32 - 32.0 - 24.0;

        // Fondo del sidebar
        let sb_bg = [0.058, 0.048, 0.040]; // #0F0C0A
        push_rect(verts, extent, sb_x, sb_y, SB_W, sb_h, sb_bg);
        // Borde derecho
        let border = [0.20, 0.18, 0.15];
        push_rect(verts, extent, sb_x + SB_W, sb_y, 1.0, sb_h, border);

        // Header del sidebar
        let header_bg = [0.075, 0.062, 0.052];
        push_rect(verts, extent, sb_x, sb_y, SB_W, 26.0, header_bg);
        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        let gold_dim = [gold[0]*0.5, gold[1]*0.5, gold[2]*0.5];
        let title = if has_workspace { "EXPLORER" } else { "ULTRA-OMEGA" };
        push_text_gpu(text_verts, extent, sb_x + 12.0, sb_y + 8.0, 1.3, gold, title, atlas);
        // Botones de accion (decorativos)
        push_text_gpu(text_verts, extent, sb_x + SB_W - 50.0, sb_y + 8.0, 1.5, [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b], "+", atlas);
        push_text_gpu(text_verts, extent, sb_x + SB_W - 24.0, sb_y + 8.0, 1.5, [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b], "...", atlas);

        // Separador bajo el header
        push_rect(verts, extent, sb_x, sb_y + 26.0, SB_W, 1.0, gold_dim);

        // Nombre del workspace si existe
        let mut y = sb_y + 30.0;
        let row_h = 20.0;
        if has_workspace {
            // Mostrar el path del workspace
            let ws_color = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
            push_text_gpu(text_verts, extent, sb_x + 8.0, y + 4.0, 1.2, ws_color, "Workspace", atlas);
            y += row_h;
        }

        // Tree recursivo
        let max_y = sb_y + sb_h - 20.0;
        for entry in entries.iter() {
            if y + row_h > max_y { break; }
            let indent = entry.depth as f32 * 12.0 + sb_x + 8.0;
            if entry.is_dir {
                let dir_bg = [0.082, 0.068, 0.058];
                push_rect(verts, extent, sb_x, y, SB_W, row_h, dir_bg);
            }
            // Chevron para carpetas
            if entry.is_dir {
                let chev = if entry.is_expanded { "v" } else { ">" };
                push_text_gpu(text_verts, extent, indent, y + 4.0, 1.0, [gold[0]*0.7, gold[1]*0.7, gold[2]*0.7], chev, atlas);
            }
            // Icono
            let icon = if entry.is_dir { "[D]" } else { "[F]" };
            let icon_color = if entry.is_dir { gold } else { [0.55, 0.50, 0.42] };
            push_text_gpu(text_verts, extent, indent + 14.0, y + 4.0, 1.0, icon_color, icon, atlas);
            // Nombre
            let name_color = if entry.is_dir { [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b] } else { [THEME.text_secondary.r, THEME.text_secondary.g, THEME.text_secondary.b] };
            push_text_gpu(text_verts, extent, indent + 38.0, y + 4.0, 1.2, name_color, &clip_text(&entry.name, 20), atlas);
            y += row_h;
        }

        // Si no hay workspace, mostrar mensaje + sugerencias
        if !has_workspace {
            let empty_color = [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b];
            push_text_gpu(text_verts, extent, sb_x + 12.0, y + 8.0, 1.2, empty_color, "Sin workspace", atlas);
            y += row_h + 4.0;
            push_text_gpu(text_verts, extent, sb_x + 12.0, y + 4.0, 1.0, empty_color, "Para empezar:", atlas);
            y += row_h;
            push_text_gpu(text_verts, extent, sb_x + 12.0, y + 4.0, 1.1, [gold[0], gold[1], gold[2]], "File > Open Folder", atlas);
            y += row_h;
            push_text_gpu(text_verts, extent, sb_x + 12.0, y + 4.0, 1.1, [gold[0], gold[1], gold[2]], "(o presiona O)", atlas);
            y += row_h + 8.0;

            // Seccion: Quick Start
            y += 4.0;
            push_text_gpu(text_verts, extent, sb_x + 12.0, y + 4.0, 1.2, gold, "QUICK START", atlas);
            y += row_h + 4.0;
            let shortcuts = [
                ("Tab", "Templates"),
                ("N", "New Rust node"),
                ("F5", "Run code"),
                ("O", "Open folder"),
                ("R", "Reset view"),
            ];
            for (key, desc) in shortcuts.iter() {
                let kc = [gold[0]*0.85, gold[1]*0.85, gold[2]*0.85];
                push_text_gpu(text_verts, extent, sb_x + 12.0, y + 3.0, 1.1, kc, key, atlas);
                push_text_gpu(text_verts, extent, sb_x + 52.0, y + 3.0, 1.0, empty_color, desc, atlas);
                y += row_h - 4.0;
            }
        }
    }

    // ─── Status Bar inferior mejorada (VSCode-style) ───
    fn push_workspace_badge(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, label: &str, atlas: Option<&FontAtlas>) {
        // Barra de status inferior (24px de alto)
        let status_y = extent.height.saturating_sub(24) as f32;
        let status_bg = [0.090, 0.078, 0.067];
        push_rect(verts, extent, 0.0, status_y, extent.width as f32, 24.0, status_bg);
        // Borde superior
        push_rect(verts, extent, 0.0, status_y, extent.width as f32, 1.0, [0.20, 0.18, 0.15]);

        // Lado izquierdo: workspace
        let _w = 240.0;
        let gold = [THEME.imperial_gold.r, THEME.imperial_gold.g, THEME.imperial_gold.b];
        let gold_dark = [gold[0]*0.6, gold[1]*0.6, gold[2]*0.6];
        push_rect(verts, extent, 0.0, status_y + 1.0, 3.0, 22.0, gold_dark);
        let lbl_color = [THEME.text_gold.r, THEME.text_gold.g, THEME.text_gold.b];
        push_text_gpu(text_verts, extent, 12.0, status_y + 6.0, 1.2, lbl_color, label, atlas);

        // Lado derecho: hints
        let hint = "F5: Run  |  Del: Delete  |  Tab: Templates  |  O: Open folder";
        let hw = (hint.len() as f32) * 6.5;
        let hint_color = [THEME.text_muted.r, THEME.text_muted.g, THEME.text_muted.b];
        push_text_gpu(text_verts, extent, extent.width as f32 - hw - 12.0, status_y + 6.0, 1.2, hint_color, hint, atlas);

        // Centro: indicador vacio (por ahora)
        // push_text_gpu(text_verts, extent, w + 20.0, status_y + 6.0, 1.2, lbl_color, "Listo", atlas);
    }

    // ─── Pins estilo perla circular ───
    fn push_pins(&self, verts: &mut Vec<Vertex>, node: &Node, extent: vk::Extent2D, vp: Viewport2D, _hdr: f32) {
        let (nx, ny) = vp.world_to_screen(node.position.x, node.position.y);
        let ps = vp.scale(PIN_SIZE).max(3.0);
        let nw = vp.scale(NODE_WIDTH);
        let nh = vp.scale(NODE_HEIGHT);
        let hdr_h = vp.scale(HEADER_HEIGHT);

        let in_step = if node.inputs.is_empty() { 0.0 } else { (nh - hdr_h) / (node.inputs.len() + 1) as f32 };
        let out_step = if node.outputs.is_empty() { 0.0 } else { (nh - hdr_h) / (node.outputs.len() + 1) as f32 };

        for (i, _) in node.inputs.iter().enumerate() {
            let cx = nx;
            let cy = ny + hdr_h + in_step * (i + 1) as f32;
            // Sombra circular
            push_circle(verts, extent, cx + 1.0, cy + 1.0, ps * 0.5, [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b]);
            // Pin base
            push_circle(verts, extent, cx, cy, ps * 0.5, [THEME.pin_input.r, THEME.pin_input.g, THEME.pin_input.b]);
            // Highlight interior
            push_circle(verts, extent, cx - ps * 0.08, cy - ps * 0.08, ps * 0.22, [THEME.pin_input.r * 1.5, THEME.pin_input.g * 1.5, THEME.pin_input.b * 1.5]);
        }

        for (i, _) in node.outputs.iter().enumerate() {
            let cx = nx + nw;
            let cy = ny + hdr_h + out_step * (i + 1) as f32;
            push_circle(verts, extent, cx + 1.0, cy + 1.0, ps * 0.5, [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b]);
            push_circle(verts, extent, cx, cy, ps * 0.5, [THEME.pin_output.r, THEME.pin_output.g, THEME.pin_output.b]);
            push_circle(verts, extent, cx - ps * 0.08, cy - ps * 0.08, ps * 0.22, [THEME.pin_output.r * 1.5, THEME.pin_output.g * 1.5, THEME.pin_output.b * 1.5]);
        }
    }

    pub fn record_command_buffer(&self, device: &ash::Device, command_buffer: vk::CommandBuffer, pipeline: &GraphicsPipeline, text_pipeline: Option<&TextPipeline>, atlas: Option<&FontAtlas>) {
        unsafe {
            // Pass 1: geometry (opaque)
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);
            let vb = [self.vertex_buffer];
            let off = [0];
            device.cmd_bind_vertex_buffers(command_buffer, 0, &vb, &off);
            if self.vertex_count > 0 { device.cmd_draw(command_buffer, self.vertex_count, 1, 0, 0); }

            // Pass 2: text (alpha blended)
            if self.text_vertex_count > 0 {
                if let (Some(tp), Some(atlas)) = (text_pipeline, atlas) {
                    device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, tp.pipeline);
                    let tvb = [self.text_vertex_buffer];
                    device.cmd_bind_vertex_buffers(command_buffer, 0, &tvb, &off);
                    device.cmd_bind_descriptor_sets(
                        command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        tp.pipeline_layout,
                        0,
                        &[atlas.descriptor_set],
                        &[],
                    );
                    device.cmd_draw(command_buffer, self.text_vertex_count, 1, 0, 0);
                }
            }
        }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
            device.destroy_buffer(self.text_vertex_buffer, None);
            device.free_memory(self.text_vertex_buffer_memory, None);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Funciones de dibujo
// ═══════════════════════════════════════════════════════════════════════════════

fn push_rect(v: &mut Vec<Vertex>, ext: vk::Extent2D, x: f32, y: f32, w: f32, h: f32, c: [f32; 3]) {
    let x0 = ndc_x(x, ext.width); let y0 = ndc_y(y, ext.height);
    let x1 = ndc_x(x + w, ext.width); let y1 = ndc_y(y + h, ext.height);
    v.extend_from_slice(&[
        Vertex { pos: [x0, y0], color: c }, Vertex { pos: [x1, y0], color: c }, Vertex { pos: [x1, y1], color: c },
        Vertex { pos: [x0, y0], color: c }, Vertex { pos: [x1, y1], color: c }, Vertex { pos: [x0, y1], color: c },
    ]);
}

fn push_rounded_rect(v: &mut Vec<Vertex>, ext: vk::Extent2D, x: f32, y: f32, w: f32, h: f32, r: f32, c: [f32; 3]) {
    let r = r.min(w * 0.5).min(h * 0.5);
    // Centro
    push_rect(v, ext, x + r, y, w - r * 2.0, h, c);
    // laterales
    push_rect(v, ext, x, y + r, r, h - r * 2.0, c);
    push_rect(v, ext, x + w - r, y + r, r, h - r * 2.0, c);
    // Esquinas (aproximación con triángulos)
    let segs = 6;
    for i in 0..segs {
        let a0 = std::f32::consts::PI + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let a1 = std::f32::consts::PI + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let cx = x + r; let cy = y + r;
        let p0 = (cx + r * a0.cos(), cy + r * a0.sin());
        let p1 = (cx + r * a1.cos(), cy + r * a1.sin());
        push_tri(v, ext, (cx, cy), p0, p1, c);
        // Esquina superior derecha
        let cx2 = x + w - r; let cy2 = y + r;
        let pa0 = -std::f32::consts::FRAC_PI_2 + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let pa1 = -std::f32::consts::FRAC_PI_2 + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let q0 = (cx2 + r * pa0.cos(), cy2 + r * pa0.sin());
        let q1 = (cx2 + r * pa1.cos(), cy2 + r * pa1.sin());
        push_tri(v, ext, (cx2, cy2), q0, q1, c);
        // Esquina inferior izquierda
        let cx3 = x + r; let cy3 = y + h - r;
        let rb0 = std::f32::consts::PI * 0.5 + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let rb1 = std::f32::consts::PI * 0.5 + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let r0 = (cx3 + r * rb0.cos(), cy3 + r * rb0.sin());
        let r1 = (cx3 + r * rb1.cos(), cy3 + r * rb1.sin());
        push_tri(v, ext, (cx3, cy3), r0, r1, c);
        // Esquina inferior derecha
        let cx4 = x + w - r; let cy4 = y + h - r;
        let s0_angle = 0.0 + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let s1_angle = 0.0 + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let s0 = (cx4 + r * s0_angle.cos(), cy4 + r * s0_angle.sin());
        let s1 = (cx4 + r * s1_angle.cos(), cy4 + r * s1_angle.sin());
        push_tri(v, ext, (cx4, cy4), s0, s1, c);
    }
}

fn push_rounded_rect_top(v: &mut Vec<Vertex>, ext: vk::Extent2D, x: f32, y: f32, w: f32, h: f32, r: f32, c: [f32; 3]) {
    let r = r.min(w * 0.5).min(h);
    push_rect(v, ext, x, y + r, w, h - r, c);
    push_rect(v, ext, x + r, y, w - r * 2.0, r, c);
    let segs = 6;
    for i in 0..segs {
        let a0 = std::f32::consts::PI + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let a1 = std::f32::consts::PI + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        push_tri(v, ext, (x + r, y + r), (x + r + r * a0.cos(), y + r + r * a0.sin()), (x + r + r * a1.cos(), y + r + r * a1.sin()), c);
        let a0r = -std::f32::consts::FRAC_PI_2 + (i as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        let a1r = -std::f32::consts::FRAC_PI_2 + ((i + 1) as f32) * (std::f32::consts::PI * 0.5 / segs as f32);
        push_tri(v, ext, (x + w - r, y + r), (x + w - r + r * a0r.cos(), y + r + r * a0r.sin()), (x + w - r + r * a1r.cos(), y + r + r * a1r.sin()), c);
    }
}

fn push_tri(v: &mut Vec<Vertex>, ext: vk::Extent2D, a: (f32, f32), b: (f32, f32), c: (f32, f32), col: [f32; 3]) {
    v.extend_from_slice(&[
        Vertex { pos: [ndc_x(a.0, ext.width), ndc_y(a.1, ext.height)], color: col },
        Vertex { pos: [ndc_x(b.0, ext.width), ndc_y(b.1, ext.height)], color: col },
        Vertex { pos: [ndc_x(c.0, ext.width), ndc_y(c.1, ext.height)], color: col },
    ]);
}

fn push_line(v: &mut Vec<Vertex>, ext: vk::Extent2D, from: (f32, f32), to: (f32, f32), thick: f32, c: [f32; 3]) {
    let dx = to.0 - from.0; let dy = to.1 - from.1;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len * thick * 0.5; let ny = dx / len * thick * 0.5;
    let p0 = (from.0 + nx, from.1 + ny); let p1 = (to.0 + nx, to.1 + ny);
    let p2 = (to.0 - nx, to.1 - ny); let p3 = (from.0 - nx, from.1 - ny);
    v.extend_from_slice(&[
        Vertex { pos: [ndc_x(p0.0, ext.width), ndc_y(p0.1, ext.height)], color: c },
        Vertex { pos: [ndc_x(p1.0, ext.width), ndc_y(p1.1, ext.height)], color: c },
        Vertex { pos: [ndc_x(p2.0, ext.width), ndc_y(p2.1, ext.height)], color: c },
        Vertex { pos: [ndc_x(p0.0, ext.width), ndc_y(p0.1, ext.height)], color: c },
        Vertex { pos: [ndc_x(p2.0, ext.width), ndc_y(p2.1, ext.height)], color: c },
        Vertex { pos: [ndc_x(p3.0, ext.width), ndc_y(p3.1, ext.height)], color: c },
    ]);
}

fn push_circle(v: &mut Vec<Vertex>, ext: vk::Extent2D, cx: f32, cy: f32, radius: f32, c: [f32; 3]) {
    let segments = 12;
    let center = (ndc_x(cx, ext.width), ndc_y(cy, ext.height));
    for i in 0..segments {
        let a0 = (i as f32) * (std::f32::consts::TAU / segments as f32);
        let a1 = ((i + 1) as f32) * (std::f32::consts::TAU / segments as f32);
        let p0 = (ndc_x(cx + radius * a0.cos(), ext.width), ndc_y(cy + radius * a0.sin(), ext.height));
        let p1 = (ndc_x(cx + radius * a1.cos(), ext.width), ndc_y(cy + radius * a1.sin(), ext.height));
        v.extend_from_slice(&[
            Vertex { pos: [center.0, center.1], color: c },
            Vertex { pos: [p0.0, p0.1], color: c },
            Vertex { pos: [p1.0, p1.1], color: c },
        ]);
    }
}

fn push_bezier(v: &mut Vec<Vertex>, ext: vk::Extent2D, from: (f32, f32), to: (f32, f32), thick: f32, c: [f32; 3]) {
    let h = (to.0 - from.0).abs().max(120.0);
    let co = h * 0.42;
    let c1 = (from.0 + co, from.1);
    let c2 = (to.0 - co, to.1);
    let segs = 36;
    let mut prev = from;
    for i in 1..=segs {
        let t = i as f32 / segs as f32;
        let pt = cubic_bezier(from, c1, c2, to, t);
        push_line(v, ext, prev, pt, thick, c);
        prev = pt;
    }
}

fn cubic_bezier(p0: (f32, f32), p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), t: f32) -> (f32, f32) {
    let u = 1.0 - t;
    let b0 = u * u * u; let b1 = 3.0 * u * u * t; let b2 = 3.0 * u * t * t; let b3 = t * t * t;
    (p0.0 * b0 + p1.0 * b1 + p2.0 * b2 + p3.0 * b3, p0.1 * b0 + p1.1 * b1 + p2.1 * b2 + p3.1 * b3)
}

pub fn pin_screen_center(node: &Node, kind: PinKind, slot: usize, vp: Viewport2D) -> (f32, f32) {
    let (nx, ny) = vp.world_to_screen(node.position.x, node.position.y);
    let pc = match kind { PinKind::Input => node.inputs.len(), PinKind::Output => node.outputs.len() };
    let step = if pc == 0 { 0.0 } else { vp.scale(NODE_HEIGHT - HEADER_HEIGHT) / (pc + 1) as f32 };
    let y = ny + vp.scale(HEADER_HEIGHT) + step * (slot + 1) as f32 + vp.scale(PIN_SIZE) * 0.5;
    let x = match kind { PinKind::Input => nx, PinKind::Output => nx + vp.scale(NODE_WIDTH) };
    (x, y)
}

fn ndc_x(x: f32, w: u32) -> f32 { (x / w.max(1) as f32) * 2.0 - 1.0 }
fn ndc_y(y: f32, h: u32) -> f32 { (y / h.max(1) as f32) * 2.0 - 1.0 }

fn clip_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }

    let keep = max_chars.saturating_sub(3);
    let mut clipped: String = text.chars().take(keep).collect();
    clipped.push_str("...");
    clipped
}

fn push_text_gpu(tv: &mut Vec<TextVertex>, ext: vk::Extent2D, x: f32, y: f32, scale: f32, c: [f32; 3], text: &str, atlas: Option<&FontAtlas>) {
    let Some(atlas) = atlas else {
        return;
    };
    let font_scale = 12.0 * scale / ATLAS_FONT_SIZE;
    let mut cx = x;
    for ch in text.chars().take(128) {
        if ch == ' ' {
            cx += atlas.space_advance() * font_scale;
            continue;
        }
        if let Some(glyph) = atlas.glyph_cache.get(&ch) {
            let gw = glyph.px_width * font_scale;
            let gh = glyph.px_height * font_scale;
            let bx = glyph.bearing_x * font_scale;
            let by = glyph.bearing_y * font_scale;

            let left = cx + bx;
            let top = y - by;
            let right = left + gw;
            let bottom = top + gh;

            let (u0, v0, u1, v1) = (glyph.u0, glyph.v0, glyph.u1, glyph.v1);

            let x0 = ndc_x(left, ext.width);
            let y0 = ndc_y(top, ext.height);
            let x1 = ndc_x(right, ext.width);
            let y1 = ndc_y(bottom, ext.height);

            tv.extend_from_slice(&[
                TextVertex { pos: [x0, y0], tex_coord: [u0, v0], color: c },
                TextVertex { pos: [x1, y0], tex_coord: [u1, v0], color: c },
                TextVertex { pos: [x1, y1], tex_coord: [u1, v1], color: c },
                TextVertex { pos: [x0, y0], tex_coord: [u0, v0], color: c },
                TextVertex { pos: [x1, y1], tex_coord: [u1, v1], color: c },
                TextVertex { pos: [x0, y1], tex_coord: [u0, v1], color: c },
            ]);

            cx += glyph.advance * font_scale;
        }
    }
}

fn find_memory_type(instance: &ash::Instance, physical_device: vk::PhysicalDevice, type_filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {
    let mem = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    for i in 0..mem.memory_type_count {
        if (type_filter & (1 << i)) != 0 && mem.memory_types[i as usize].property_flags & properties == properties { return i; }
    }
    log::error!("No suitable memory type found - falling back to type 0");
    0
}
