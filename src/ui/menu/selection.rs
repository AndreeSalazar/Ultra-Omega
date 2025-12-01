use eframe::egui;
use crate::app::NodeGraphApp;

pub fn draw_selection_menu(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("Selection", |ui| {
        if ui.button("Select All").clicked() {
            app.interaction.selected_nodes = app.graph.nodes().iter().map(|n| n.id).collect();
            ui.close_menu();
        }
        if ui.button("Clear Selection").clicked() {
            app.interaction.selected_nodes.clear();
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Focus Selected (F)").clicked() {
            app.focus_view(ctx.screen_rect());
            ui.close_menu();
        }
    });
}

