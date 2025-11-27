mod node_graph;
mod app;
mod terminal;
mod ui;
mod templates;

use crate::app::NodeGraphApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("ultra-omega-node-lab")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([960.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ultra Omega Node Lab",
        options,
        Box::new(|_cc| Box::<NodeGraphApp>::default()),
    )
}
