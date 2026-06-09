mod core;
mod storage;
mod templates;
mod expressions;
mod inheritance;
mod utils;
mod vulkan;
mod config; // <-- AÑADIDO para resolver el error de importación

use crate::core::NodeGraphApp;
use vulkan::context::VulkanContext;
use vulkan::renderer::Viewport2D;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    vulkan_ctx: Option<VulkanContext>,
    graph_app: NodeGraphApp,
    viewport: Viewport2D,
    is_panning: bool,
    last_cursor_position: Option<(f32, f32)>,
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
                    ctx.draw_frame(self.graph_app.graph(), self.viewport);
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let current = (position.x as f32, position.y as f32);

                if self.is_panning {
                    if let Some(previous) = self.last_cursor_position {
                        self.viewport.pan_by(current.0 - previous.0, current.1 - previous.1);
                    }
                }

                self.last_cursor_position = Some(current);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Middle {
                    self.is_panning = state == ElementState::Pressed;
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let steps = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 120.0,
                };
                self.viewport.zoom_by(steps);
            }
            _ => {}
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
        graph_app: NodeGraphApp::default(),
        viewport: Viewport2D::default(),
        is_panning: false,
        last_cursor_position: None,
    };

    event_loop.run_app(&mut app).unwrap();
}
