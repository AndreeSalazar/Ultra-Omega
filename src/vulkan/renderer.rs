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
    pub template_palette_open: bool,
    pub template_visible_start: usize,
    pub selected_template_index: usize,
    pub template_entries: Vec<TemplatePaletteEntry>,
    pub workspace_label: String,
}

#[derive(Clone, Debug)]
pub struct TemplatePaletteEntry {
    pub label: String,
    pub color: [f32; 3],
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
pub const NODE_WIDTH: f32 = 280.0;
pub const NODE_HEIGHT: f32 = 130.0;
pub const HEADER_HEIGHT: f32 = 38.0;
pub const PIN_SIZE: f32 = 10.0;
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
        self.push_workspace_badge(&mut verts, &mut text_verts, extent, &state.workspace_label, atlas);

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

    // ─── Nodo estilo sello chino ───
    fn push_node(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, node: &Node, extent: vk::Extent2D, vp: Viewport2D, state: &RenderState, atlas: Option<&FontAtlas>) {
        let (x, y) = vp.world_to_screen(node.position.x, node.position.y);
        let w = vp.scale(NODE_WIDTH);
        let h = vp.scale(NODE_HEIGHT);
        let hdr = vp.scale(HEADER_HEIGHT);
        let is_sel = state.selected_node == Some(node.id);
        let is_hov = state.hovered_node == Some(node.id);
        let is_src = state.link_source_node == Some(node.id);

        // Colores del nodo según lenguaje
        let (hdr_color, body_color, accent) = match node.language {
            NodeLanguage::Rust => (THEME.vermillion, THEME.node_rust_body, THEME.vermillion),
            NodeLanguage::Text => (THEME.copper, THEME.node_text_body, THEME.copper),
            NodeLanguage::Auto => (THEME.jade_green, THEME.node_auto_body, THEME.jade_green),
        };

        // Sombra exterior (estilo tinta difuminada)
        let shadow = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
        push_rounded_rect(verts, extent, x - vp.scale(4.0), y - vp.scale(4.0), w + vp.scale(8.0), h + vp.scale(8.0), vp.scale(NODE_CORNER + 2.0), shadow);

        // Borde de selección/hover
        if is_src || is_sel || is_hov {
            let bc = if is_src { THEME.jade_green } else if is_sel { THEME.imperial_gold } else { accent };
            let bc3 = [bc.r, bc.g, bc.b];
            push_rounded_rect(verts, extent, x - vp.scale(2.0), y - vp.scale(2.0), w + vp.scale(4.0), h + vp.scale(4.0), vp.scale(NODE_CORNER + 1.0), bc3);
        }

        // Cuerpo del nodo
        let bc = [body_color.r, body_color.g, body_color.b];
        push_rounded_rect(verts, extent, x, y, w, h, vp.scale(NODE_CORNER), bc);

        // Header con gradiente sutil (simulado con dos rectángulos)
        let hc = [hdr_color.r, hdr_color.g, hdr_color.b];
        push_rounded_rect_top(verts, extent, x, y, w, hdr, vp.scale(NODE_CORNER), hc);
        // Franja sutil en header
        let hc_dim = [hdr_color.r * 0.75, hdr_color.g * 0.75, hdr_color.b * 0.75];
        push_rect(verts, extent, x, y + hdr - vp.scale(2.0), w, vp.scale(2.0), hc_dim);

        // Línea decorativa inferior (estilo sello)
        let footer_h = vp.scale(5.0);
        let fc = [hdr_color.r * 0.6, hdr_color.g * 0.6, hdr_color.b * 0.6];
        push_rect(verts, extent, x + vp.scale(8.0), y + h - footer_h - vp.scale(3.0), w - vp.scale(16.0), footer_h, fc);

        // Puntos decorativos en esquinas (estilo tachuela)
        let dot = vp.scale(3.0);
        let dc = [THEME.border_gold.r, THEME.border_gold.g, THEME.border_gold.b];
        push_rect(verts, extent, x + vp.scale(3.0), y + vp.scale(3.0), dot, dot, dc);
        push_rect(verts, extent, x + w - vp.scale(6.0), y + vp.scale(3.0), dot, dot, dc);

        // Pins (perlas)
        self.push_pins(verts, node, extent, vp, hdr);

        // Título del nodo (GPU text)
        let title_color = [THEME.text_primary.r, THEME.text_primary.g, THEME.text_primary.b];
        push_text_gpu(text_verts, extent, x + vp.scale(10.0), y + vp.scale(5.0), vp.scale(2.0), title_color, &node.title, atlas);
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

    // ─── Conexiones estilo tinta con curva Bezier ───
    fn push_links(&self, verts: &mut Vec<Vertex>, graph: &NodeGraph, extent: vk::Extent2D, vp: Viewport2D) {
        for link in graph.links() {
            let Some(fa) = graph.locate_pin(link.from) else { continue; };
            let Some(ta) = graph.locate_pin(link.to) else { continue; };
            let fn_ = &graph.nodes()[fa.node_index];
            let tn = &graph.nodes()[ta.node_index];
            let from = pin_screen_center(fn_, fa.kind, fa.slot, vp);
            let to = pin_screen_center(tn, ta.kind, ta.slot, vp);

            // Color de la conexión: cobre por defecto, vermillón si activa
            let link_c = THEME.link_default;
            let color = [link_c.r, link_c.g, link_c.b];

            // Sombra de la conexión (más ancha, más oscura)
            let shadow_c = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
            push_bezier(verts, extent, from, to, vp.scale(6.0).max(2.0), shadow_c);
            // Línea principal
            push_bezier(verts, extent, from, to, vp.scale(3.0).max(1.0), color);
            // Highlight sutil
            let highlight = [link_c.r * 1.3, link_c.g * 1.3, link_c.b * 1.3];
            push_bezier(verts, extent, from, to, vp.scale(1.0).max(0.5), highlight);
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

    // ─── Badge de workspace estilo sello chino ───
    fn push_workspace_badge(&self, verts: &mut Vec<Vertex>, text_verts: &mut Vec<TextVertex>, extent: vk::Extent2D, label: &str, atlas: Option<&FontAtlas>) {
        let w = (label.chars().count() as f32 * 8.0 * 1.2 + 40.0).clamp(300.0, extent.width.saturating_sub(48) as f32);
        let x = 24.0;
        let y = extent.height.saturating_sub(48) as f32;

        // Sombra
        let shadow = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];
        push_rounded_rect(verts, extent, x - 6.0, y - 6.0, w + 12.0, 34.0, 6.0, shadow);

        // Fondo
        let bg = [THEME.obsidian.r, THEME.obsidian.g, THEME.obsidian.b];
        push_rounded_rect(verts, extent, x, y, w, 26.0, 4.0, bg);

        // Borde dorado
        let gold = [THEME.border_gold.r, THEME.border_gold.g, THEME.border_gold.b];
        push_rect(verts, extent, x, y, 3.0, 26.0, gold);

        // Texto
        let tc = [THEME.text_gold.r, THEME.text_gold.g, THEME.text_gold.b];
        push_text_gpu(text_verts, extent, x + 14.0, y + 8.0, 1.2, tc, label, atlas);
    }

    // ─── Pins estilo perla ───
    fn push_pins(&self, verts: &mut Vec<Vertex>, node: &Node, extent: vk::Extent2D, vp: Viewport2D, _hdr: f32) {
        let (nx, ny) = vp.world_to_screen(node.position.x, node.position.y);
        let ps = vp.scale(PIN_SIZE).max(3.0);
        let nw = vp.scale(NODE_WIDTH);
        let nh = vp.scale(NODE_HEIGHT);
        let hdr_h = vp.scale(HEADER_HEIGHT);

        let in_step = if node.inputs.is_empty() { 0.0 } else { (nh - hdr_h) / (node.inputs.len() + 1) as f32 };
        let out_step = if node.outputs.is_empty() { 0.0 } else { (nh - hdr_h) / (node.outputs.len() + 1) as f32 };

        // Sombra de pin
        let shadow = [THEME.ink_black.r, THEME.ink_black.g, THEME.ink_black.b];

        for (i, _) in node.inputs.iter().enumerate() {
            let px = nx - ps * 0.5;
            let py = ny + hdr_h + in_step * (i + 1) as f32;
            push_rect(verts, extent, px + 1.0, py + 1.0, ps, ps, shadow);
            let pc = [THEME.pin_input.r, THEME.pin_input.g, THEME.pin_input.b];
            push_rect(verts, extent, px, py, ps, ps, pc);
            // Highlight
            let ph = [THEME.pin_input.r * 1.4, THEME.pin_input.g * 1.4, THEME.pin_input.b * 1.4];
            push_rect(verts, extent, px + ps * 0.2, py + ps * 0.2, ps * 0.4, ps * 0.4, ph);
        }

        for (i, _) in node.outputs.iter().enumerate() {
            let px = nx + nw - ps * 0.5;
            let py = ny + hdr_h + out_step * (i + 1) as f32;
            push_rect(verts, extent, px + 1.0, py + 1.0, ps, ps, shadow);
            let pc = [THEME.pin_output.r, THEME.pin_output.g, THEME.pin_output.b];
            push_rect(verts, extent, px, py, ps, ps, pc);
            let ph = [THEME.pin_output.r * 1.4, THEME.pin_output.g * 1.4, THEME.pin_output.b * 1.4];
            push_rect(verts, extent, px + ps * 0.2, py + ps * 0.2, ps * 0.4, ps * 0.4, ph);
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

fn push_bezier(v: &mut Vec<Vertex>, ext: vk::Extent2D, from: (f32, f32), to: (f32, f32), thick: f32, c: [f32; 3]) {
    let h = (to.0 - from.0).abs().max(120.0);
    let co = h * 0.42;
    let c1 = (from.0 + co, from.1);
    let c2 = (to.0 - co, to.1);
    let segs = 22;
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

fn push_text_gpu(tv: &mut Vec<TextVertex>, ext: vk::Extent2D, x: f32, y: f32, scale: f32, c: [f32; 3], text: &str, atlas: Option<&FontAtlas>) {
    let Some(atlas) = atlas else {
        return;
    };
    let font_scale = 7.0 * scale / ATLAS_FONT_SIZE;
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
