use eframe::egui::{self, ScrollArea};
use crate::core::NodeGraphApp;

pub fn draw_sidebar(app: &mut NodeGraphApp, ctx: &egui::Context, _open_factor: f32) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(280.0)
        .min_width(220.0)
        .max_width(450.0)
        .frame(egui::Frame::side_top_panel(&ctx.style())
            .fill(egui::Color32::from_rgb(37, 37, 38))
            .inner_margin(egui::Margin::same(8.0)))
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading(egui::RichText::new("Explorador").color(egui::Color32::from_rgb(212, 212, 212)));
            ui.add_space(4.0);
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                // Carpeta seleccionada
                ui.collapsing("Carpeta seleccionada", |ui| {
                    if app.workspace.has_root() {
                        if let Some(folder_name) = app.workspace.get_folder_name() {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📁").size(16.0));
                                ui.label(egui::RichText::new(&folder_name)
                                    .strong()
                                    .color(egui::Color32::from_rgb(212, 212, 212)));
                            });
                            ui.add_space(4.0);
                            
                            // Mostrar información del mapa de nodos
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("🗺️").size(14.0));
                                ui.label(egui::RichText::new("node_map.json")
                                    .color(egui::Color32::from_rgb(78, 148, 206))
                                    .size(13.0));
                                if let Some(node_map_path) = app.workspace.get_node_map_path() {
                                    if node_map_path.exists() {
                                        ui.label(egui::RichText::new("✓")
                                            .color(egui::Color32::from_rgb(89, 185, 89))
                                            .size(12.0));
                                        
                                        // Mostrar tiempo desde último guardado
                                        if let Some(last_save) = app.last_save_time {
                                            let elapsed = last_save.elapsed().as_secs();
                                            if elapsed < 2 {
                                                ui.label(egui::RichText::new("●")
                                                    .color(egui::Color32::from_rgb(89, 185, 89))
                                                    .size(10.0));
                                            }
                                        }
                                    } else {
                                        ui.label(egui::RichText::new("○")
                                            .color(egui::Color32::from_rgb(128, 128, 128))
                                            .size(10.0));
                                    }
                                }
                            });
                            ui.add_space(2.0);
                            ui.label(egui::RichText::new(format!("{} nodos", app.graph.nodes().len()))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(128, 128, 128)));
                            ui.add_space(4.0);
                            
                            // Listar archivos y carpetas
                            if let Ok(items) = app.workspace.list_files() {
                                for (name, is_dir) in items {
                                    // Skip node_map.json as we show it separately
                                    if name == "node_map.json" {
                                        continue;
                                    }
                                    
                                    let icon = if is_dir { "📁" } else { "📄" };
                                    let response = ui.selectable_label(false, 
                                        egui::RichText::new(format!("{} {}", icon, name))
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(212, 212, 212)));
                                    if response.clicked() {
                                        // Si es un archivo JSON, intentar cargarlo
                                        if !is_dir && name.ends_with(".json") {
                                            if let Some(root) = &app.workspace.root_path {
                                                let file_path = root.join(&name);
                                                if let Ok(json) = std::fs::read_to_string(&file_path) {
                                                    if let Ok(graph) = serde_json::from_str::<crate::core::node_graph::NodeGraph>(&json) {
                                                        app.graph = graph;
                                                        app.graph.recalculate_ids();
                                                        app.interaction.selected_nodes.clear();
                                                        app.last_save_hash = app.graph_hash();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                ui.label(egui::RichText::new("Error al leer carpeta").color(egui::Color32::RED));
                            }
                        }
                    } else {
                        ui.label(egui::RichText::new("Ninguna carpeta seleccionada")
                            .color(egui::Color32::from_rgb(128, 128, 128))
                            .size(12.0));
                        ui.add_space(6.0);
                        if ui.button(egui::RichText::new("Abrir carpeta...")
                            .color(egui::Color32::from_rgb(212, 212, 212))).clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                app.workspace.set_root(path);
                                if let Err(e) = app.load_graph_from_workspace() {
                                    eprintln!("Error loading graph: {}", e);
                                }
                            }
                        }
                    }
                });
                
                ui.separator();
                
                ui.collapsing("Nodos", |ui| {
                    // Create a safe copy of node info to avoid borrow checker issues
                    let nodes_info: Vec<_> = app.graph.nodes().iter().map(|n| (n.id, n.title.clone())).collect();
                    
                    for (id, title) in nodes_info {
                        let is_selected = app.interaction.selected_nodes.contains(&id);
                        
                        let response = ui.selectable_label(is_selected, 
                            egui::RichText::new(format!("📄 {}", title))
                                .size(12.0)
                                .color(if is_selected {
                                    egui::Color32::from_rgb(255, 255, 255)
                                } else {
                                    egui::Color32::from_rgb(212, 212, 212)
                                }));
                        if response.clicked() {
                            // Handle Selection
                            if !ui.input(|i| i.modifiers.ctrl) {
                                app.interaction.selected_nodes.clear();
                            }
                            if is_selected && ui.input(|i| i.modifiers.ctrl) {
                                app.interaction.selected_nodes.remove(&id);
                            } else {
                                app.interaction.selected_nodes.insert(id);
                            }
                        }
                    }
                });
                
                ui.collapsing("Propiedades", |ui| {
                    if let Some(id) = app.interaction.selected_nodes.iter().next() {
                        let title_changed = {
                            if let Some(node) = app.graph.node_mut(*id) {
                                ui.label("Título:");
                                let changed = ui.text_edit_singleline(&mut node.title).changed();
                                ui.add_space(4.0);
                                ui.label(format!("Pos: {:.1}, {:.1}", node.position.x, node.position.y));
                                ui.add_space(4.0);
                                ui.label(format!("Color: R:{}, G:{}, B:{}", 
                                    node.color.r(), node.color.g(), node.color.b()));
                                changed
                            } else {
                                false
                            }
                        };
                        if title_changed {
                            // Actualizar canales cuando cambia el título
                            if let Some(node_id) = app.interaction.selected_nodes.iter().next() {
                                app.update_node_channels(*node_id);
                            }
                            app.check_and_auto_save();
                        }
                    } else {
                        ui.label("Ninguna selección");
                    }
                });
            });
        });
}

