use eframe::egui;
use crate::core::NodeGraphApp;

pub fn draw_run_menu(ui: &mut egui::Ui, _ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("Run", |ui| {
        if ui.button("Run Selected Node").clicked() {
            if let Some(id) = app.interaction.selected_nodes.iter().next() {
                if let Some(node) = app.graph.nodes().iter().find(|n| n.id == *id) {
                    let lang = if node.title.contains("ASM") {
                        crate::compilation::terminal::Language::Nasm
                    } else if node.title.contains("C++") {
                        crate::compilation::terminal::Language::Cpp
                    } else if node.title.contains("Rust") {
                        crate::compilation::terminal::Language::Rust
                    } else {
                        crate::compilation::terminal::Language::C
                    };
                    let workspace_path = app.workspace.root_path.as_ref();
                    app.terminal.run_code(&node.code, lang, workspace_path);
                }
            }
            ui.close_menu();
        }
        
        ui.separator();
        
        if ui.button("🔧 Estado de Compiladores (Ctrl+Shift+C)").clicked() {
            app.show_compiler_status = true;
            // Actualizar estado
            app.compiler_status = Some(crate::compilation::compiler_detector::detect_all_compilers());
            ui.close_menu();
        }
    });
}

