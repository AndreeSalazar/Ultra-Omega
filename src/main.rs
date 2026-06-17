mod app;
mod compilation;
mod config;
mod core;
mod storage;
mod templates;
mod ui;
mod utils;
mod vulkan;

fn main() {
    env_logger::init();
    log::info!("Ultra-Omega v0.2.0 - 100% Rust + Vulkan Puro");
    app::run();
}
