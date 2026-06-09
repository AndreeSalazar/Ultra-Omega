mod core;
mod storage;
mod templates;
mod expressions;
mod inheritance;
mod utils;
mod vulkan;
mod config; // <-- AÑADIDO para resolver el error de importación

use vulkan::context::VulkanContext;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    vulkan_ctx: Option<VulkanContext>,
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
                    ctx.draw_frame();
                }
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
    };

    event_loop.run_app(&mut app).unwrap();
}
