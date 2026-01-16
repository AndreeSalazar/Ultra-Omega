mod core;
mod compilation;
mod utils;
mod ui;
mod storage;
mod templates;
mod config;
mod expressions;
mod inheritance;

use crate::core::NodeGraphApp;
use crate::config::AppConfig;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let config = AppConfig::load();
    
    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_app_id("ultra-omega-node-lab")
        .with_min_inner_size([960.0, 540.0]);
    
    if let Some((w, h)) = config.window_size {
        viewport_builder = viewport_builder.with_inner_size([w, h]);
    } else {
        viewport_builder = viewport_builder.with_inner_size([1280.0, 720.0]);
    }
    
    if let Some((x, y)) = config.window_pos {
        viewport_builder = viewport_builder.with_position(egui::pos2(x, y));
    }

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    eframe::run_native(
        "Ultra Omega Node Lab",
        options,
        Box::new(move |cc| {
            // Apply VS Code-like dark theme
            apply_vscode_theme(&cc.egui_ctx);
            Box::new(NodeGraphApp::from_config(config))
        }),
    )
}

fn apply_vscode_theme(ctx: &egui::Context) {
    use egui::{Color32, Rounding, Stroke, Visuals};
    
    let mut visuals = Visuals::dark();
    
    // VS Code Dark+ color scheme - Tema Negro Mejorado
    visuals.dark_mode = true;
    visuals.override_text_color = Some(Color32::from_rgb(212, 212, 212));
    visuals.extreme_bg_color = Color32::from_rgb(0, 0, 0); // Fondo completamente negro
    visuals.panel_fill = Color32::from_rgb(20, 20, 20); // Sidebar background (negro con poco contraste)
    visuals.window_fill = Color32::from_rgb(37, 37, 38);
    visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 60));
    visuals.faint_bg_color = Color32::from_rgb(45, 45, 45);
    visuals.selection.bg_fill = Color32::from_rgb(38, 79, 120); // Selection blue
    visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(0, 122, 204));
    visuals.hyperlink_color = Color32::from_rgb(78, 148, 206);
    visuals.warn_fg_color = Color32::from_rgb(204, 204, 0);
    visuals.error_fg_color = Color32::from_rgb(244, 135, 113);
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(45, 45, 45);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 60));
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(60, 60, 60);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(80, 80, 80));
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(62, 62, 66);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(90, 90, 90));
    visuals.widgets.active.bg_fill = Color32::from_rgb(0, 122, 204);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, Color32::from_rgb(0, 122, 204));
    visuals.widgets.open.bg_fill = Color32::from_rgb(45, 45, 45);
    
    // Rounded corners
    visuals.window_rounding = Rounding::same(4.0);
    visuals.menu_rounding = Rounding::same(4.0);
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 2.0),
        blur: 8.0,
        spread: 0.0,
        color: Color32::from_black_alpha(200),
    };
    
    ctx.set_visuals(visuals);
    
    // Set better fonts
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(18.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(11.0, egui::FontFamily::Proportional),
    );
    // Configurar fuente monoespaciada mejorada para terminal y código
    // Tamaño más grande para mejor legibilidad y diferenciación de caracteres
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(14.5, egui::FontFamily::Monospace), // Tamaño aumentado para mejor legibilidad
    );
    ctx.set_style(style);
}

