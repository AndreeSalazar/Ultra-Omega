use eframe::egui;
use crate::app::NodeGraphApp;

pub fn draw_view_menu(ui: &mut egui::Ui, _ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("View", |ui| {
        if ui.button("Reset View").clicked() {
            app.viewport = crate::ui::viewport::Viewport2D::default();
            ui.close_menu();
        }
        ui.separator();
        ui.label(format!("Zoom: {:.0}%", app.viewport.zoom * 100.0));
    });
}

