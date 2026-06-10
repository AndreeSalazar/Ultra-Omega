mod app;
mod core;
mod templates;
mod vulkan;

fn main() {
    env_logger::init();
    println!("🚀 Iniciando Ultra-Omega v0.2.0 (100% Rust + Vulkan Puro)");

    app::run();
}
