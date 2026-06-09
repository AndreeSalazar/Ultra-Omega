use ash::vk;

use crate::core::node_graph::{Node, NodeGraph, NodeLanguage, PinKind};
use crate::vulkan::pipeline::{GraphicsPipeline, Vertex};

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

    fn world_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
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
const NODE_WIDTH: f32 = 260.0;
const NODE_HEIGHT: f32 = 120.0;
const HEADER_HEIGHT: f32 = 34.0;
const PIN_SIZE: f32 = 10.0;

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
    ) {
        let mut vertices = Vec::with_capacity(graph.nodes().len() * 30);

        self.push_links(&mut vertices, graph, extent, viewport);

        for node in graph.nodes() {
            self.push_node(&mut vertices, node, extent, viewport);

            if vertices.len() >= self.vertex_capacity {
                vertices.truncate(self.vertex_capacity);
                break;
            }
        }

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
        let pin_color = match node.language {
            NodeLanguage::Rust => [0.95, 0.38, 0.12],
            NodeLanguage::Text => [0.35, 0.55, 0.90],
            NodeLanguage::Auto => [0.65, 0.65, 0.70],
        };

        // Borde/sombra exterior.
        push_rect(
            vertices,
            extent,
            x - 2.0,
            y - 2.0,
            node_width + 4.0,
            node_height + 4.0,
            border_color,
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
            let from = pin_center(from_node, from_addr.kind, from_addr.slot, viewport);
            let to = pin_center(to_node, to_addr.kind, to_addr.slot, viewport);
            push_line(vertices, extent, from, to, viewport.scale(4.0).max(1.0), color_to_rgb(link.color));
        }
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

fn pin_center(node: &Node, kind: PinKind, slot: usize, viewport: Viewport2D) -> (f32, f32) {
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
    1.0 - (y / height.max(1) as f32) * 2.0
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
