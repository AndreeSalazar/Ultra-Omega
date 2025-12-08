pub mod file;
pub mod edit;
pub mod selection;
pub mod view;
pub mod run;
pub mod terminal;

use eframe::egui;
use crate::core::NodeGraphApp;

pub fn draw_menu_bar(app: &mut NodeGraphApp, ctx: &egui::Context, _open_factor: f32) {
    egui::TopBottomPanel::top("menu_bar")
        .frame(egui::Frame::side_top_panel(&ctx.style())
            .fill(egui::Color32::from_rgb(37, 37, 38))
            .inner_margin(egui::Margin::same(4.0)))
        .show_separator_line(true)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file::draw_file_menu(ui, ctx, app);
                edit::draw_edit_menu(ui, ctx, app);
                selection::draw_selection_menu(ui, ctx, app);
                view::draw_view_menu(ui, ctx, app);
                run::draw_run_menu(ui, ctx, app);
                terminal::draw_terminal_menu(ui, ctx, app);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("Nodos: {}", app.graph.nodes().len()))
                        .color(egui::Color32::from_rgb(212, 212, 212))
                        .size(12.0));
                    ui.separator();
                    ui.label(egui::RichText::new("Ultra Omega")
                        .color(egui::Color32::from_rgb(78, 148, 206))
                        .strong());
                });
            });
        });
}

