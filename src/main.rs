mod core;
mod compilation;
mod utils;
mod ui;
mod storage;
mod templates;
mod config;
mod expressions;
mod inheritance;
mod vulkan;

use crate::config::AppConfig;
use crate::vulkan::VulkanContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let config = AppConfig::load();

    // 1. Inicializar Event Loop y Ventana con Winit
    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new()
        .with_title("Ultra-Omega Node Lab [Vulkan]")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720));

    if let Some((w, h)) = config.window_size {
        window_builder = window_builder.with_inner_size(winit::dpi::LogicalSize::new(w, h));
    }

    let window = window_builder.build(&event_loop).expect("Failed to create window");

    // 2. Inicializar Contexto de Vulkan (Instance, Device, Surface)
    let vulkan_ctx = VulkanContext::new(&window);
    println!("✅ Vulkan Context initialized successfully!");
    println!("🚀 Ultra-Omega v2.0 - 100% Rust + Vulkan (ash)");

    // 3. Loop Principal de Eventos
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Aquí irá el renderizado con Vulkan
                // Por ahora, solo limpiamos la pantalla con un color
            }
            _ => {}
        }
    });
}
