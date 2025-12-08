use eframe::egui;
use crate::core::NodeGraphApp;

pub fn draw_edit_menu(ui: &mut egui::Ui, _ctx: &egui::Context, _app: &mut NodeGraphApp) {
    ui.menu_button("Edit", |ui| {
        ui.label("Undo (Ctrl+Z) - WIP");
        ui.label("Redo (Ctrl+Y) - WIP");
        ui.separator();
        ui.label("Cut (Ctrl+X) - WIP");
        ui.label("Copy (Ctrl+C) - WIP");
        ui.label("Paste (Ctrl+V) - WIP");
    });
}

