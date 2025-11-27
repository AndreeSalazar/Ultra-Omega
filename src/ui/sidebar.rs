use eframe::egui::{self, ScrollArea};
use crate::app::NodeGraphApp;

pub fn draw_sidebar(app: &mut NodeGraphApp, ctx: &egui::Context, open_factor: f32) {
    if open_factor < 0.01 { return; }

    let width = 200.0 * open_factor;
    
    egui::SidePanel::left("sidebar")
        .resizable(false) // Manually handling width via animation
        .exact_width(width)
        .show(ctx, |ui| {
            // Clip content if closing
            ui.set_clip_rect(ui.max_rect());
            
            // Only draw content if wide enough to be useful
            if open_factor > 0.2 {
                // Fade in?
                ui.set_enabled(open_factor > 0.8);
                
                ui.add_space(4.0);
                ui.heading("Explorador");
                ui.separator();
    
                ScrollArea::vertical().show(ui, |ui| {
                    ui.collapsing("Nodos", |ui| {
                        // Create a safe copy of node info to avoid borrow checker issues
                        // We only need ID and Title to draw the buttons
                        let nodes_info: Vec<_> = app.graph.nodes().iter().map(|n| (n.id, n.title.clone())).collect();
                        
                        for (id, title) in nodes_info {
                            let is_selected = app.interaction.selected_nodes.contains(&id);
                            
                            if ui.selectable_label(is_selected, format!("📄 {}", title)).clicked() {
                                // Handle Selection
                                if !ui.input(|i| i.modifiers.ctrl) {
                                    app.interaction.selected_nodes.clear();
                                }
                                if is_selected && ui.input(|i| i.modifiers.ctrl) {
                                    app.interaction.selected_nodes.remove(&id);
                                } else {
                                    app.interaction.selected_nodes.insert(id);
                                    // Also focus? Maybe on double click?
                                    // Let's keep it simple: just select.
                                }
                            }
                        }
                    });
                    
                    ui.collapsing("Propiedades", |ui| {
                        if let Some(id) = app.interaction.selected_nodes.iter().next() {
                            if let Some(node) = app.graph.node_mut(*id) {
                                ui.label("Título:");
                                ui.text_edit_singleline(&mut node.title);
                                ui.add_space(4.0);
                                ui.label(format!("Pos: {:.1}, {:.1}", node.position.x, node.position.y));
                            }
                        } else {
                            ui.label("Ninguna selección");
                        }
                    });
                });
            }
        });
}

