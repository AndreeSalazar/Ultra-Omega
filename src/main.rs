mod core;
mod vulkan;

use crate::core::{NodeGraph, NodeId};
use vulkan::context::VulkanContext;
use vulkan::renderer::{RenderState, Viewport2D, NODE_HEIGHT, NODE_WIDTH};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::core::node_graph::NodeLanguage;
use crate::core::types::{pos2, Color32};

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
    link_source_node: Option<NodeId>,
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
                            link_source_node: self.link_source_node,
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
                        let dx = (current.0 - previous.0) / self.viewport.zoom;
                        let dy = (current.1 - previous.1) / self.viewport.zoom;

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
                            self.link_source_node = None;
                        }
                        PhysicalKey::Code(KeyCode::KeyR) => self.viewport = Viewport2D::default(),
                        PhysicalKey::Code(KeyCode::KeyC) => self.link_source_node = self.selected_node,
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
        let Some(source_id) = self.link_source_node else {
            return false;
        };
        let Some(target_id) = self.hovered_node else {
            return false;
        };

        if source_id == target_id {
            return false;
        }

        let Some(from_pin) = self.graph.pin_id(source_id, crate::core::node_graph::PinKind::Output, 0) else {
            self.link_source_node = None;
            return false;
        };
        let Some(to_pin) = self.graph.pin_id(target_id, crate::core::node_graph::PinKind::Input, 0) else {
            self.link_source_node = None;
            return false;
        };

        self.graph.add_link(from_pin, to_pin, Color32::from_rgb(0xde, 0x39, 0x00));
        self.selected_node = Some(target_id);
        self.link_source_node = None;
        true
    }

    fn delete_selected_node(&mut self) {
        if let Some(node_id) = self.selected_node.take() {
            self.graph.remove_node(node_id);
            self.hovered_node = None;
            self.dragging_node = None;
            if self.link_source_node == Some(node_id) {
                self.link_source_node = None;
            }
        }
    }
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
        link_source_node: None,
        created_nodes: 0,
    };

    event_loop.run_app(&mut app).unwrap();
}
