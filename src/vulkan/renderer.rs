use ash::vk;

use crate::core::node_graph::{Node, NodeGraph, NodeLanguage, PinKind};
use crate::core::NodeId;
use crate::vulkan::pipeline::{GraphicsPipeline, Vertex};

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
        Self {
            pan: [0.0, 0.0],
            zoom: 1.0,
        }
    }
}

impl Viewport2D {
    pub fn pan_by(&mut self, dx: f32, dy: f32) {
        self.pan[0] += dx;
        self.pan[1] += dy;
    }

    pub fn zoom_by(&mut self, steps: f32) {
        let factor = 1.0 + steps * 0.10;
        self.zoom = (self.zoom * factor.max(0.10)).clamp(0.25, 4.0);
    }

    pub fn zoom_at(&mut self, steps: f32, screen_x: f32, screen_y: f32) {
        let before = self.screen_to_world(screen_x, screen_y);
        self.zoom_by(steps);
        let after = self.screen_to_world(screen_x, screen_y);

        self.pan[0] += (after.0 - before.0) * self.zoom;
        self.pan[1] += (after.1 - before.1) * self.zoom;
    }

    pub fn screen_to_world(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.pan[0]) / self.zoom, (y - self.pan[1]) / self.zoom)
    }

    pub fn screen_delta_to_world(&self, dx: f32, dy: f32) -> (f32, f32) {
        (dx / self.zoom, dy / self.zoom)
    }

    pub fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        (x * self.zoom + self.pan[0], y * self.zoom + self.pan[1])
    }

    fn scale(&self, value: f32) -> f32 {
        value * self.zoom
    }
}

pub struct Renderer {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    vertex_capacity: usize,
    vertex_count: u32,
}

const MAX_VERTICES: usize = 65_536;
pub const NODE_WIDTH: f32 = 260.0;
pub const NODE_HEIGHT: f32 = 120.0;
pub const HEADER_HEIGHT: f32 = 34.0;
pub const PIN_SIZE: f32 = 10.0;
const GRID_SPACING: f32 = 64.0;

impl Renderer {
    pub fn new(
        device: &ash::Device,
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        let buffer_size = (std::mem::size_of::<Vertex>() * MAX_VERTICES) as vk::DeviceSize;

        // Primera fase: buffer dinámico host-visible para pintar el canvas desde NodeGraph.
        // Fase futura: staging buffer + device-local + instancing para miles de nodos.
        let buffer_info = vk::BufferCreateInfo {
            size: buffer_size,
            usage: vk::BufferUsageFlags::VERTEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let vertex_buffer = unsafe { device.create_buffer(&buffer_info, None).unwrap() };
        let mem_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index: find_memory_type(
                instance,
                physical_device,
                mem_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            ..Default::default()
        };

        let vertex_buffer_memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0).unwrap() };

        Self {
            vertex_buffer,
            vertex_buffer_memory,
            vertex_capacity: MAX_VERTICES,
            vertex_count: 0,
        }
    }

    pub fn update_from_graph(
        &mut self,
        device: &ash::Device,
        graph: &NodeGraph,
        extent: vk::Extent2D,
        viewport: Viewport2D,
        state: RenderState,
    ) {
        let mut vertices = Vec::with_capacity(graph.nodes().len() * 30);

        self.push_grid(&mut vertices, extent, viewport);
        self.push_links(&mut vertices, graph, extent, viewport);

        for node in graph.nodes() {
            self.push_node(&mut vertices, node, extent, viewport, &state);

            if vertices.len() >= self.vertex_capacity {
                vertices.truncate(self.vertex_capacity);
                break;
            }
        }

        if state.template_palette_open {
            self.push_template_palette(&mut vertices, extent, &state);
        }

        self.push_workspace_badge(&mut vertices, extent, &state.workspace_label);

        self.vertex_count = vertices.len() as u32;

        if vertices.is_empty() {
            return;
        }

        let copy_size = (std::mem::size_of::<Vertex>() * vertices.len()) as vk::DeviceSize;
        unsafe {
            let data_ptr = device
                .map_memory(
                    self.vertex_buffer_memory,
                    0,
                    copy_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap() as *mut Vertex;
            data_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
            device.unmap_memory(self.vertex_buffer_memory);
        }
    }

    fn push_node(
        &self,
        vertices: &mut Vec<Vertex>,
        node: &Node,
        extent: vk::Extent2D,
        viewport: Viewport2D,
        state: &RenderState,
    ) {
        let (x, y) = viewport.world_to_screen(node.position.x, node.position.y);
        let node_width = viewport.scale(NODE_WIDTH);
        let node_height = viewport.scale(NODE_HEIGHT);
        let header_height = viewport.scale(HEADER_HEIGHT);
        let node_color = color_to_rgb(node.color);
        let body_color = match node.language {
            NodeLanguage::Rust => [0.16, 0.16, 0.18],
            NodeLanguage::Text => [0.12, 0.15, 0.19],
            NodeLanguage::Auto => [0.14, 0.14, 0.15],
        };
        let border_color = [0.07, 0.07, 0.08];
        let is_selected = state.selected_node == Some(node.id);
        let is_hovered = state.hovered_node == Some(node.id);
        let is_link_source = state.link_source_node == Some(node.id);
        let outer_color = if is_link_source {
            [0.20, 1.0, 0.55]
        } else if is_selected {
            [1.0, 0.76, 0.25]
        } else if is_hovered {
            [0.35, 0.65, 1.0]
        } else {
            border_color
        };
        let outer_padding = if is_link_source || is_selected || is_hovered { 5.0 } else { 2.0 };
        let pin_color = match node.language {
            NodeLanguage::Rust => [0.95, 0.38, 0.12],
            NodeLanguage::Text => [0.35, 0.55, 0.90],
            NodeLanguage::Auto => [0.65, 0.65, 0.70],
        };

        // Borde/sombra exterior.
        push_rect(
            vertices,
            extent,
            x - outer_padding,
            y - outer_padding,
            node_width + outer_padding * 2.0,
            node_height + outer_padding * 2.0,
            outer_color,
        );

        // Cuerpo del nodo.
        push_rect(vertices, extent, x, y, node_width, node_height, body_color);

        // Header coloreado por tipo/lenguaje.
        push_rect(vertices, extent, x, y, node_width, header_height, node_color);

        // Franja inferior: señal visual de nodo reutilizable Rust.
        push_rect(
            vertices,
            extent,
            x,
            y + node_height - viewport.scale(8.0),
            node_width,
            viewport.scale(8.0),
            [node_color[0] * 0.55, node_color[1] * 0.55, node_color[2] * 0.55],
        );

        self.push_pins(vertices, node, extent, viewport, pin_color);
    }

    fn push_grid(&self, vertices: &mut Vec<Vertex>, extent: vk::Extent2D, viewport: Viewport2D) {
        let world_top_left = viewport.screen_to_world(0.0, 0.0);
        let world_bottom_right = viewport.screen_to_world(extent.width as f32, extent.height as f32);
        let min_x = world_top_left.0.min(world_bottom_right.0);
        let max_x = world_top_left.0.max(world_bottom_right.0);
        let min_y = world_top_left.1.min(world_bottom_right.1);
        let max_y = world_top_left.1.max(world_bottom_right.1);

        let start_x = (min_x / GRID_SPACING).floor() as i32 - 1;
        let end_x = (max_x / GRID_SPACING).ceil() as i32 + 1;
        let start_y = (min_y / GRID_SPACING).floor() as i32 - 1;
        let end_y = (max_y / GRID_SPACING).ceil() as i32 + 1;

        for gx in start_x..=end_x {
            let world_x = gx as f32 * GRID_SPACING;
            let from = viewport.world_to_screen(world_x, min_y - GRID_SPACING);
            let to = viewport.world_to_screen(world_x, max_y + GRID_SPACING);
            let is_axis = gx == 0;
            let color = if is_axis { [0.22, 0.22, 0.24] } else { [0.15, 0.15, 0.16] };
            push_line(vertices, extent, from, to, if is_axis { 2.0 } else { 1.0 }, color);
        }

        for gy in start_y..=end_y {
            let world_y = gy as f32 * GRID_SPACING;
            let from = viewport.world_to_screen(min_x - GRID_SPACING, world_y);
            let to = viewport.world_to_screen(max_x + GRID_SPACING, world_y);
            let is_axis = gy == 0;
            let color = if is_axis { [0.22, 0.22, 0.24] } else { [0.15, 0.15, 0.16] };
            push_line(vertices, extent, from, to, if is_axis { 2.0 } else { 1.0 }, color);
        }
    }

    fn push_links(
        &self,
        vertices: &mut Vec<Vertex>,
        graph: &NodeGraph,
        extent: vk::Extent2D,
        viewport: Viewport2D,
    ) {
        for link in graph.links() {
            let Some(from_addr) = graph.locate_pin(link.from) else {
                continue;
            };
            let Some(to_addr) = graph.locate_pin(link.to) else {
                continue;
            };

            let from_node = &graph.nodes()[from_addr.node_index];
            let to_node = &graph.nodes()[to_addr.node_index];
            let from = pin_screen_center(from_node, from_addr.kind, from_addr.slot, viewport);
            let to = pin_screen_center(to_node, to_addr.kind, to_addr.slot, viewport);
            push_bezier(
                vertices,
                extent,
                from,
                to,
                viewport.scale(4.0).max(1.0),
                color_to_rgb(link.color),
            );
        }
    }

    fn push_template_palette(
        &self,
        vertices: &mut Vec<Vertex>,
        extent: vk::Extent2D,
        state: &RenderState,
    ) {
        let panel_x = 32.0;
        let panel_y = 32.0;
        let panel_width = 560.0;
        let item_height = 34.0;
        let visible_items = state.template_entries.len().min(12);
        let panel_height = 86.0 + item_height * visible_items as f32;

        push_rect(vertices, extent, panel_x - 6.0, panel_y - 6.0, panel_width + 12.0, panel_height + 12.0, [0.02, 0.02, 0.025]);
        push_rect(vertices, extent, panel_x, panel_y, panel_width, panel_height, [0.075, 0.078, 0.095]);
        push_rect(vertices, extent, panel_x, panel_y, panel_width, 50.0, [0.25, 0.10, 0.025]);
        push_text(vertices, extent, panel_x + 18.0, panel_y + 16.0, 2.0, [1.0, 0.68, 0.30], "TAB RUST TEMPLATES");
        push_text(vertices, extent, panel_x + panel_width - 168.0, panel_y + 18.0, 1.4, [0.85, 0.85, 0.88], "ENTER CREA");

        for index in 0..visible_items {
            let y = panel_y + 64.0 + index as f32 * item_height;
            let global_index = state.template_visible_start + index;
            let selected = global_index == state.selected_template_index;
            let entry = &state.template_entries[index];
            let color = if selected {
                [0.92, 0.35, 0.10]
            } else if index % 2 == 0 {
                [0.135, 0.138, 0.16]
            } else {
                [0.105, 0.108, 0.13]
            };
            push_rect(vertices, extent, panel_x + 12.0, y, panel_width - 24.0, item_height - 5.0, color);
            push_rect(vertices, extent, panel_x + 18.0, y + 6.0, 8.0, item_height - 17.0, entry.color);
            push_text(vertices, extent, panel_x + 36.0, y + 9.0, 1.45, [0.90, 0.90, 0.93], &entry.label);
        }
    }

    fn push_workspace_badge(&self, vertices: &mut Vec<Vertex>, extent: vk::Extent2D, label: &str) {
        let width = (label.chars().count() as f32 * 8.0 * 1.2 + 30.0).clamp(280.0, extent.width.saturating_sub(48) as f32);
        let x = 24.0;
        let y = extent.height.saturating_sub(44) as f32;

        push_rect(vertices, extent, x - 4.0, y - 4.0, width + 8.0, 30.0, [0.035, 0.035, 0.045]);
        push_rect(vertices, extent, x, y, width, 22.0, [0.10, 0.105, 0.13]);
        push_text(vertices, extent, x + 12.0, y + 7.0, 1.2, [0.78, 0.84, 0.92], label);
    }

    fn push_pins(
        &self,
        vertices: &mut Vec<Vertex>,
        node: &Node,
        extent: vk::Extent2D,
        viewport: Viewport2D,
        pin_color: [f32; 3],
    ) {
        let input_step = if node.inputs.is_empty() {
            0.0
        } else {
            viewport.scale(NODE_HEIGHT - HEADER_HEIGHT) / (node.inputs.len() + 1) as f32
        };

        let (node_x, node_y) = viewport.world_to_screen(node.position.x, node.position.y);
        let pin_size = viewport.scale(PIN_SIZE).max(2.0);
        let header_height = viewport.scale(HEADER_HEIGHT);
        let node_width = viewport.scale(NODE_WIDTH);

        for (index, _) in node.inputs.iter().enumerate() {
            let x = node_x - pin_size * 0.5;
            let y = node_y + header_height + input_step * (index + 1) as f32;
            push_rect(vertices, extent, x, y, pin_size, pin_size, pin_color);
        }

        let output_step = if node.outputs.is_empty() {
            0.0
        } else {
            viewport.scale(NODE_HEIGHT - HEADER_HEIGHT) / (node.outputs.len() + 1) as f32
        };

        for (index, _) in node.outputs.iter().enumerate() {
            let x = node_x + node_width - pin_size * 0.5;
            let y = node_y + header_height + output_step * (index + 1) as f32;
            push_rect(vertices, extent, x, y, pin_size, pin_size, pin_color);
        }
    }

    pub fn record_command_buffer(
        &self,
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
        pipeline: &GraphicsPipeline,
    ) {
        unsafe {
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);

            let vertex_buffers = [self.vertex_buffer];
            let offsets = [0];
            device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);

            if self.vertex_count > 0 {
                device.cmd_draw(command_buffer, self.vertex_count, 1, 0, 0);
            }
        }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
        }
    }
}

fn push_rect(
    vertices: &mut Vec<Vertex>,
    extent: vk::Extent2D,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: [f32; 3],
) {
    let x0 = screen_to_ndc_x(x, extent.width);
    let y0 = screen_to_ndc_y(y, extent.height);
    let x1 = screen_to_ndc_x(x + width, extent.width);
    let y1 = screen_to_ndc_y(y + height, extent.height);

    vertices.extend_from_slice(&[
        Vertex { pos: [x0, y0], color },
        Vertex { pos: [x1, y0], color },
        Vertex { pos: [x1, y1], color },
        Vertex { pos: [x0, y0], color },
        Vertex { pos: [x1, y1], color },
        Vertex { pos: [x0, y1], color },
    ]);
}

fn push_line(
    vertices: &mut Vec<Vertex>,
    extent: vk::Extent2D,
    from: (f32, f32),
    to: (f32, f32),
    thickness: f32,
    color: [f32; 3],
) {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len * thickness * 0.5;
    let ny = dx / len * thickness * 0.5;

    let p0 = (from.0 + nx, from.1 + ny);
    let p1 = (to.0 + nx, to.1 + ny);
    let p2 = (to.0 - nx, to.1 - ny);
    let p3 = (from.0 - nx, from.1 - ny);

    vertices.extend_from_slice(&[
        Vertex { pos: [screen_to_ndc_x(p0.0, extent.width), screen_to_ndc_y(p0.1, extent.height)], color },
        Vertex { pos: [screen_to_ndc_x(p1.0, extent.width), screen_to_ndc_y(p1.1, extent.height)], color },
        Vertex { pos: [screen_to_ndc_x(p2.0, extent.width), screen_to_ndc_y(p2.1, extent.height)], color },
        Vertex { pos: [screen_to_ndc_x(p0.0, extent.width), screen_to_ndc_y(p0.1, extent.height)], color },
        Vertex { pos: [screen_to_ndc_x(p2.0, extent.width), screen_to_ndc_y(p2.1, extent.height)], color },
        Vertex { pos: [screen_to_ndc_x(p3.0, extent.width), screen_to_ndc_y(p3.1, extent.height)], color },
    ]);
}

fn push_bezier(
    vertices: &mut Vec<Vertex>,
    extent: vk::Extent2D,
    from: (f32, f32),
    to: (f32, f32),
    thickness: f32,
    color: [f32; 3],
) {
    let horizontal = (to.0 - from.0).abs().max(120.0);
    let control_offset = horizontal * 0.45;
    let c1 = (from.0 + control_offset, from.1);
    let c2 = (to.0 - control_offset, to.1);
    let segments = 18;
    let mut previous = from;

    for index in 1..=segments {
        let t = index as f32 / segments as f32;
        let point = cubic_bezier(from, c1, c2, to, t);
        push_line(vertices, extent, previous, point, thickness, color);
        previous = point;
    }
}

fn cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let inv = 1.0 - t;
    let b0 = inv * inv * inv;
    let b1 = 3.0 * inv * inv * t;
    let b2 = 3.0 * inv * t * t;
    let b3 = t * t * t;

    (
        p0.0 * b0 + p1.0 * b1 + p2.0 * b2 + p3.0 * b3,
        p0.1 * b0 + p1.1 * b1 + p2.1 * b2 + p3.1 * b3,
    )
}

pub fn pin_screen_center(node: &Node, kind: PinKind, slot: usize, viewport: Viewport2D) -> (f32, f32) {
    let (node_x, node_y) = viewport.world_to_screen(node.position.x, node.position.y);
    let pin_count = match kind {
        PinKind::Input => node.inputs.len(),
        PinKind::Output => node.outputs.len(),
    };
    let step = if pin_count == 0 {
        0.0
    } else {
        viewport.scale(NODE_HEIGHT - HEADER_HEIGHT) / (pin_count + 1) as f32
    };
    let y = node_y + viewport.scale(HEADER_HEIGHT) + step * (slot + 1) as f32 + viewport.scale(PIN_SIZE) * 0.5;
    let x = match kind {
        PinKind::Input => node_x,
        PinKind::Output => node_x + viewport.scale(NODE_WIDTH),
    };

    (x, y)
}

fn screen_to_ndc_x(x: f32, width: u32) -> f32 {
    (x / width.max(1) as f32) * 2.0 - 1.0
}

fn screen_to_ndc_y(y: f32, height: u32) -> f32 {
    // Vulkan con viewport de altura positiva mapea NDC -1 arriba y +1 abajo.
    // Mantener la misma orientación que winit/screen coords evita que input y render
    // queden desincronizados al seleccionar o arrastrar nodos.
    (y / height.max(1) as f32) * 2.0 - 1.0
}

fn push_text(
    vertices: &mut Vec<Vertex>,
    extent: vk::Extent2D,
    x: f32,
    y: f32,
    scale: f32,
    color: [f32; 3],
    text: &str,
) {
    let mut cursor_x = x;
    for ch in text.chars().take(64) {
        if ch == ' ' {
            cursor_x += 4.0 * scale;
            continue;
        }

        let glyph = glyph_5x7(ch);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    push_rect(
                        vertices,
                        extent,
                        cursor_x + col as f32 * scale,
                        y + row as f32 * scale,
                        scale,
                        scale,
                        color,
                    );
                }
            }
        }
        cursor_x += 6.0 * scale;
    }
}

fn glyph_5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
        'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110],
        '6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        '/' => [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
        ':' => [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100],
        '\\' => [0b10000, 0b01000, 0b01000, 0b00100, 0b00010, 0b00010, 0b00001],
        _ => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
    }
}

fn color_to_rgb(color: crate::core::types::Color32) -> [f32; 3] {
    [
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
    ]
}

fn find_memory_type(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && mem_properties.memory_types[i as usize].property_flags & properties == properties
        {
            return i;
        }
    }
    panic!("Failed to find suitable memory type!");
}
