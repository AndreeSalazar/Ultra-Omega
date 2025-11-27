use eframe::egui;
use crate::app::NodeGraphApp;
use crate::terminal::TerminalTab;

pub fn draw_menu_bar(app: &mut NodeGraphApp, ctx: &egui::Context, open_factor: f32) {
    // We can't easily animate height of a TopBottomPanel top without it pushing content
    // But that's what we want.
    // However, TopBottomPanel doesn't have exact_height, it fits content.
    // We can wrap content in a Frame or set min_height/max_height on the panel?
    // Actually standard menu bar is small. Let's just use exact_height on the content row if possible?
    // Or better: animate the Translation of the inner response?
    // Simpler: TopBottomPanel with a Frame that clips?
    
    // Egui TopPanel fits content. If we want to slide it, we should probably use a fixed height approach
    // or just `show_animated` if it existed.
    
    // Trick: We can use `multiply(Color32, opacity)` to fade it out? No user asked for slide/hide.
    // Let's try to limit the height of the panel.
    
    if open_factor < 0.05 { return; }

    egui::TopBottomPanel::top("menu_bar")
        .show_separator_line(open_factor > 0.9)
        .show(ctx, |ui| {
            // We can scale the ui?
            // ui.set_height(30.0 * open_factor); // doesn't really clip content nicely
            
            // Instead, we'll just Clip the content area based on factor?
            // Or just accept that "hiding" means "not calling .show()".
            // But we want smooth animation.
            
            // Let's vertically scroll/offset the content?
            // `ui.allocate_ui_at_rect` with a moving rect?
            
            let height = 24.0;
            let visible_height = height * open_factor;
            
            // Reserve space
            let (rect, _) = ui.allocate_exact_size(eframe::egui::vec2(ui.available_width(), visible_height), eframe::egui::Sense::hover());
            
            // Clip to visible rect
            let mut child_ui = ui.child_ui(rect, *ui.layout());
            child_ui.set_clip_rect(rect);
            
            // Draw content shifted up if we want "slide up" effect
            // Offset: (1.0 - open_factor) * height -> move UP
            let _offset = eframe::egui::vec2(0.0, -height * (1.0 - open_factor));
            
            // Actually, simpler is just drawing it. If rect is small, it clips.
            // But menu bars are tricky because they spawn popups.
            
            // Let's just draw standard menu bar but only if factor is near 1.0?
            // No, user wants animation.
            
            // Let's just use the `visible_height` to push down the rest of UI, 
            // and draw the menu bar fully visible *inside* that rect (clipped)?
            
            if open_factor > 0.9 {
                 egui::menu::bar(&mut child_ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
        
                    ui.menu_button("Edit", |ui| {
                        ui.label("Undo (Ctrl+Z) - WIP");
                        ui.label("Redo (Ctrl+Y) - WIP");
                    });
        
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
        
                    ui.menu_button("View", |ui| {
                        if ui.button("Reset View").clicked() {
                            app.viewport = crate::ui::viewport::Viewport2D::default();
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.label(format!("Zoom: {:.0}%", app.viewport.zoom * 100.0));
                    });
        
            ui.menu_button("Run", |ui| {
                if ui.button("Run Selected Node").clicked() {
                    if let Some(id) = app.interaction.selected_nodes.iter().next() {
                        if let Some(node) = app.graph.nodes().iter().find(|n| n.id == *id) {
                            let lang = if node.title.contains("ASM") {
                                crate::terminal::Language::Nasm
                            } else if node.title.contains("C++") {
                                crate::terminal::Language::Cpp
                            } else if node.title.contains("Rust") {
                                crate::terminal::Language::Rust
                            } else {
                                crate::terminal::Language::C
                            };
                            app.terminal.run_code(&node.code, lang);
                        }
                    }
                    ui.close_menu();
                }
            });

            ui.menu_button("Terminal", |ui| {
                ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Nasm, "NASM Output");
                ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::C, "C Output");
                ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Cpp, "C++ Output");
                ui.selectable_value(&mut app.terminal.active_tab, TerminalTab::Rust, "Rust Output");
            });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Nodos: {}", app.graph.nodes().len()));
                        ui.separator();
                        ui.label("Ultra Omega");
                    });
                });
            }
        });
}

