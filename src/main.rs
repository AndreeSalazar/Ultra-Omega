mod core;
mod vulkan;

use crate::core::{NodeGraph, NodeId};
use vulkan::context::VulkanContext;
use vulkan::renderer::{pin_screen_center, RenderState, Viewport2D, NODE_HEIGHT, NODE_WIDTH, PIN_SIZE};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::core::node_graph::{NodeLanguage, PinKind};
use crate::core::types::{pos2, Color32};

#[derive(Clone, Copy, Debug)]
struct HitPin {
    node_id: NodeId,
    kind: PinKind,
    slot: usize,
}

struct App {
    window: Option<Window>,
    vulkan_ctx: Option<VulkanContext>,
    graph: NodeGraph,
    viewport: Viewport2D,
    is_panning: bool,
    last_cursor_position: Option<(f32, f32)>,
    hovered_node: Option<NodeId>,
    selected_node: Option<NodeId>,
    dragging_node: Option<NodeId>,
    link_source_pin: Option<HitPin>,
    created_nodes: u32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Ultra-Omega | Node Editor (Vulkan Puro)")
                    .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            ).unwrap();
            
            // FIX: Añadimos los paréntesis para resolver la ambigüedad del trait
            self.vulkan_ctx = Some(VulkanContext::new(&window));
            window.request_redraw();
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(ctx) = &mut self.vulkan_ctx {
                    ctx.draw_frame(
                        &self.graph,
                        self.viewport,
                        RenderState {
                            hovered_node: self.hovered_node,
                            selected_node: self.selected_node,
                            link_source_node: self.link_source_pin.map(|pin| pin.node_id),
                        },
                    );
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
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
                } else if button == MouseButton::Left && state == ElementState::Pressed {
                    if self.try_finish_link_from_hover() {
                        self.dragging_node = None;
                    } else if self.try_start_link_from_hovered_pin() {
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
                if event.state == ElementState::Pressed {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyN) => self.create_rust_node_at_view_center(),
                        PhysicalKey::Code(KeyCode::Delete) => self.delete_selected_node(),
                        PhysicalKey::Code(KeyCode::Escape) => {
                            self.selected_node = None;
                            self.dragging_node = None;
                            self.link_source_pin = None;
                        }
                        PhysicalKey::Code(KeyCode::KeyR) => self.viewport = Viewport2D::default(),
                        PhysicalKey::Code(KeyCode::KeyC) => self.start_link_from_selected_node(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

impl App {
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
        let Some(window) = &self.window else {
            return;
        };
        let size = window.inner_size();
        let world = self
            .viewport
            .screen_to_world(size.width as f32 * 0.5, size.height as f32 * 0.5);

        self.created_nodes += 1;
        let node_id = self.graph.add_node(
            format!("Rust Node {}", self.created_nodes),
            pos2(world.0 - NODE_WIDTH * 0.5, world.1 - NODE_HEIGHT * 0.5),
            Color32::from_rgb(0xde, 0x39, 0x00),
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
    }

    fn try_finish_link_from_hover(&mut self) -> bool {
        let Some(source_pin) = self.link_source_pin else {
            return false;
        };
        let Some(cursor) = self.last_cursor_position else {
            return false;
        };
        let Some(target_pin) = self.pin_at_screen_position(cursor) else {
            return false;
        };

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

        self.graph.add_link(from_pin, to_pin, Color32::from_rgb(0xde, 0x39, 0x00));
        self.selected_node = Some(target_pin.node_id);
        self.link_source_pin = None;
        true
    }

    fn try_start_link_from_hovered_pin(&mut self) -> bool {
        let Some(screen) = self.last_cursor_position else {
            return false;
        };
        let Some(pin) = self.pin_at_screen_position(screen) else {
            return false;
        };

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
            if self.link_source_pin.map(|pin| pin.node_id) == Some(node_id) {
                self.link_source_pin = None;
            }
        }
    }
}

fn distance_sq(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

fn main() {
    env_logger::init();
    println!("🚀 Iniciando Ultra-Omega v0.2.0 (100% Rust + Vulkan Puro)");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        vulkan_ctx: None,
        graph: NodeGraph::demo(),
        viewport: Viewport2D::default(),
        is_panning: false,
        last_cursor_position: None,
        hovered_node: None,
        selected_node: None,
        dragging_node: None,
        link_source_pin: None,
        created_nodes: 0,
    };

    event_loop.run_app(&mut app).unwrap();
}
