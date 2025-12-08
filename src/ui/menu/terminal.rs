use eframe::egui;
use crate::core::NodeGraphApp;
use crate::compilation::terminal::TerminalTab;

pub fn draw_terminal_menu(ui: &mut egui::Ui, _ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("Terminal", |ui| {
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Nasm, "NASM Output");
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::C, "C Output");
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Cpp, "C++ Output");
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Rust, "Rust Output");
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Zig, "Zig Output");
        ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Java, "Java Output");
    });
}

