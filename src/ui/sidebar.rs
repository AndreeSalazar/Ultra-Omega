use eframe::egui::{self, ScrollArea, Color32, Stroke};
use crate::core::NodeGraphApp;

pub fn draw_sidebar(app: &mut NodeGraphApp, ctx: &egui::Context, _open_factor: f32) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(300.0)
        .min_width(240.0)
        .max_width(480.0)
        .frame(egui::Frame::side_top_panel(&ctx.style())
            .fill(Color32::from_rgb(25, 25, 30)) // Fondo gris muy oscuro con tinte azulado
            .inner_margin(egui::Margin::same(12.0)))
        .show(ctx, |ui| {
            // Header mejorado con estilo profesional
            ui.vertical(|ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📂")
                        .size(20.0)
                        .color(Color32::from_rgb(100, 150, 255)));
                    ui.heading(egui::RichText::new("Explorador")
                        .strong()
                        .size(18.0)
                        .color(Color32::from_rgb(230, 230, 235)));
                });
                ui.add_space(4.0);
                
                // Línea separadora con color
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 2.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(100, 150, 255, 100)
                );
            });
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                // Carpeta seleccionada - Header mejorado
                egui::CollapsingHeader::new(
                    egui::RichText::new("📁 Carpeta seleccionada")
                        .strong()
                        .size(14.0)
                        .color(Color32::from_rgb(150, 180, 255))
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(6.0);
                    if app.workspace.has_root() {
                        if let Some(folder_name) = app.workspace.get_folder_name() {
                            // Nombre de carpeta destacado
                            egui::Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(40, 50, 70, 150))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(100, 150, 255)))
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("📁")
                                            .size(18.0)
                                            .color(Color32::from_rgb(255, 200, 100)));
                                        ui.label(egui::RichText::new(&folder_name)
                                            .strong()
                                            .size(13.0)
                                            .color(Color32::from_rgb(255, 255, 255)));
                                    });
                                });
                            ui.add_space(8.0);
                            
                            // Información del mapa de nodos con mejor diseño
                            if let Some(node_map_path) = app.workspace.get_node_map_path() {
                                let exists = node_map_path.exists();
                                let bg_color = if exists {
                                    Color32::from_rgba_unmultiplied(40, 70, 50, 120)
                                } else {
                                    Color32::from_rgba_unmultiplied(70, 50, 40, 120)
                                };
                                let border_color = if exists {
                                    Color32::from_rgb(100, 220, 150)
                                } else {
                                    Color32::from_rgb(220, 150, 100)
                                };
                                
                                egui::Frame::none()
                                    .fill(bg_color)
                                    .stroke(Stroke::new(1.5, border_color))
                                    .rounding(egui::Rounding::same(5.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("🗺️")
                                                .size(16.0)
                                                .color(Color32::from_rgb(150, 200, 255)));
                                            ui.label(egui::RichText::new("node_map.json")
                                                .strong()
                                                .monospace()
                                                .size(12.0)
                                                .color(Color32::from_rgb(180, 220, 255)));
                                            
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if exists {
                                                    let status_text = if let Some(last_save) = app.last_save_time {
                                                        let elapsed = last_save.elapsed().as_secs();
                                                        if elapsed < 2 {
                                                            ("●", Color32::from_rgb(100, 255, 150))
                                                        } else {
                                                            ("✓", Color32::from_rgb(150, 255, 150))
                                                        }
                                                    } else {
                                                        ("✓", Color32::from_rgb(150, 255, 150))
                                                    };
                                                    ui.label(egui::RichText::new(status_text.0)
                                                        .size(14.0)
                                                        .color(status_text.1));
                                                } else {
                                                    ui.label(egui::RichText::new("○")
                                                        .size(12.0)
                                                        .color(Color32::from_rgb(200, 150, 100)));
                                                }
                                            });
                                        });
                                        
                                        // Contador de nodos con mejor estilo
                                        ui.add_space(4.0);
                                        ui.horizontal(|ui| {
                                            let node_count = app.graph.nodes().len();
                                            let count_color = if node_count > 0 {
                                                Color32::from_rgb(150, 220, 255)
                                            } else {
                                                Color32::from_rgb(150, 150, 150)
                                            };
                                            let count_text = if node_count == 1 {
                                                "📄 1 nodo".to_string()
                                            } else {
                                                format!("📄 {} nodos", node_count)
                                            };
                                            ui.label(egui::RichText::new(count_text)
                                                .size(11.0)
                                                .color(count_color));
                                        });
                                    });
                            }
                            ui.add_space(8.0);
                            
                            // Listar archivos y carpetas con mejor visualización
                            ui.label(egui::RichText::new("Archivos del proyecto:")
                                .size(11.0)
                                .color(Color32::from_rgb(150, 150, 180)));
                            ui.add_space(4.0);
                            
                            if let Ok(mut items_vec) = app.workspace.list_files() {
                                // Ordenar: carpetas primero, luego archivos alfabéticamente
                                items_vec.sort_by(|(a, a_dir), (b, b_dir)| {
                                    match (a_dir, b_dir) {
                                        (true, false) => std::cmp::Ordering::Less,
                                        (false, true) => std::cmp::Ordering::Greater,
                                        _ => a.cmp(b),
                                    }
                                });
                                
                                for (name, is_dir) in items_vec {
                                    // Skip node_map.json as we show it separately
                                    if name == "node_map.json" {
                                        continue;
                                    }
                                    
                                    // Icono y color según tipo de archivo
                                    let (icon, icon_color, bg_color) = if is_dir {
                                        ("📁", Color32::from_rgb(255, 200, 100), Color32::from_rgba_unmultiplied(70, 60, 40, 80))
                                    } else if name.ends_with(".rs") {
                                        ("🦀", Color32::from_rgb(255, 140, 100), Color32::from_rgba_unmultiplied(70, 40, 30, 80))
                                    } else if name.ends_with(".exe") {
                                        ("⚙️", Color32::from_rgb(150, 220, 255), Color32::from_rgba_unmultiplied(30, 50, 70, 80))
                                    } else if name.ends_with(".pdb") {
                                        ("🔧", Color32::from_rgb(180, 180, 200), Color32::from_rgba_unmultiplied(50, 50, 60, 80))
                                    } else if name.ends_with(".json") {
                                        ("📋", Color32::from_rgb(150, 200, 255), Color32::from_rgba_unmultiplied(40, 50, 70, 80))
                                    } else if name.ends_with(".asm") || name.ends_with(".s") {
                                        ("⚡", Color32::from_rgb(255, 220, 100), Color32::from_rgba_unmultiplied(70, 60, 30, 80))
                                    } else {
                                        ("📄", Color32::from_rgb(180, 180, 180), Color32::from_rgba_unmultiplied(50, 50, 50, 80))
                                    };
                                    
                                    let response = egui::Frame::none()
                                        .fill(bg_color)
                                        .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(100, 100, 120, 50)))
                                        .rounding(egui::Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(icon)
                                                    .size(14.0)
                                                    .color(icon_color));
                                                ui.label(egui::RichText::new(&name)
                                                    .size(11.5)
                                                    .color(Color32::from_rgb(220, 220, 230)));
                                            })
                                        }).response;
                                    
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
                                    ui.add_space(2.0);
                                }
                            } else {
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(70, 40, 40, 100))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(220, 100, 100)))
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("❌ Error al leer carpeta")
                                            .size(11.0)
                                            .color(Color32::from_rgb(255, 150, 150)));
                                    });
                            }
                        }
                    } else {
                        // Estado sin carpeta - mejorado
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(50, 50, 60, 100))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(100, 100, 130)))
                            .rounding(egui::Rounding::same(6.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(egui::RichText::new("📂")
                                        .size(24.0)
                                        .color(Color32::from_rgb(150, 150, 180)));
                                    ui.add_space(4.0);
                                    ui.label(egui::RichText::new("Ninguna carpeta seleccionada")
                                        .size(12.0)
                                        .color(Color32::from_rgb(180, 180, 200)));
                                    ui.add_space(8.0);
                                    
                                    if ui.add(egui::Button::new(egui::RichText::new("📂 Abrir carpeta...")
                                            .size(13.0)
                                            .color(Color32::WHITE))
                                        .fill(Color32::from_rgb(100, 150, 255))
                                        .min_size(egui::vec2(ui.available_width(), 32.0)))
                                        .clicked() {
                                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                            app.workspace.set_root(path);
                                            if let Err(e) = app.load_graph_from_workspace() {
                                                eprintln!("Error loading graph: {}", e);
                                            }
                                        }
                                    }
                                });
                            });
                    }
                });
                
                ui.add_space(6.0);
                
                // Separador visual mejorado
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(100, 150, 255, 60)
                );
                ui.add_space(6.0);
                
                // Sección de Nodos mejorada
                egui::CollapsingHeader::new(
                    egui::RichText::new(format!("📊 Nodos ({})", app.graph.nodes().len()))
                        .strong()
                        .size(14.0)
                        .color(Color32::from_rgb(150, 180, 255))
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(6.0);
                    
                    if app.graph.nodes().is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("📭")
                                .size(32.0)
                                .color(Color32::from_rgb(100, 100, 130)));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("No hay nodos")
                                .size(11.0)
                                .color(Color32::from_rgb(150, 150, 170)));
                            ui.label(egui::RichText::new("Presiona Tab para crear uno")
                                .size(10.0)
                                .color(Color32::from_rgb(120, 120, 140)));
                        });
                    } else {
                        // Create a safe copy of node info to avoid borrow checker issues
                        let nodes_info: Vec<_> = app.graph.nodes().iter()
                            .map(|n| (n.id, n.title.clone(), n.language, n.color))
                            .collect();
                        
                        for (id, title, language, color) in nodes_info {
                            let is_selected = app.interaction.selected_nodes.contains(&id);
                            
                            // Color de fondo según selección
                            let bg_color = if is_selected {
                                Color32::from_rgba_unmultiplied(100, 150, 255, 150)
                            } else {
                                Color32::from_rgba_unmultiplied(40, 45, 55, 100)
                            };
                            
                            // Icono según lenguaje
                            let (lang_icon, lang_color) = match language {
                                crate::core::node_graph::NodeLanguage::Rust => ("🦀", Color32::from_rgb(255, 140, 100)),
                                crate::core::node_graph::NodeLanguage::C => ("©", Color32::from_rgb(120, 180, 255)),
                                crate::core::node_graph::NodeLanguage::Cpp => ("⊕", Color32::from_rgb(180, 140, 255)),
                                crate::core::node_graph::NodeLanguage::Asm => ("⚡", Color32::from_rgb(255, 220, 100)),
                                crate::core::node_graph::NodeLanguage::Zig => ("⚡", Color32::from_rgb(240, 170, 0)),
                                crate::core::node_graph::NodeLanguage::Mojo => ("🔥", Color32::from_rgb(255, 100, 100)),
                                crate::core::node_graph::NodeLanguage::MojoAI => ("🤖", Color32::from_rgb(255, 150, 100)),
                                crate::core::node_graph::NodeLanguage::Text => ("📄", Color32::from_rgb(200, 200, 150)),
                                crate::core::node_graph::NodeLanguage::Auto => ("⚙", Color32::from_rgb(180, 180, 180)),
                            };
                            
                            let border_color = if is_selected {
                                Color32::from_rgb(150, 200, 255)
                            } else {
                                color
                            };
                            
                            let response = egui::Frame::none()
                                .fill(bg_color)
                                .stroke(Stroke::new(if is_selected { 2.0 } else { 1.0 }, border_color))
                                .rounding(egui::Rounding::same(5.0))
                                .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Barra de color del nodo
                                        let (rect, _) = ui.allocate_exact_size(
                                            egui::vec2(3.0, 20.0),
                                            egui::Sense::hover()
                                        );
                                        ui.painter().rect_filled(rect, 1.0, color);
                                        ui.add_space(6.0);
                                        
                                        // Icono del lenguaje
                                        ui.label(egui::RichText::new(lang_icon)
                                            .size(14.0)
                                            .color(lang_color));
                                        
                                        // Título del nodo
                                        ui.label(egui::RichText::new(&title)
                                            .size(12.0)
                                            .color(if is_selected {
                                                Color32::WHITE
                                            } else {
                                                Color32::from_rgb(220, 220, 230)
                                            }));
                                    });
                                }).response;
                            
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
                            ui.add_space(3.0);
                        }
                    }
                });
                
                ui.add_space(6.0);
                
                // Separador visual
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(100, 150, 255, 60)
                );
                ui.add_space(6.0);
                
                // Sección de Propiedades mejorada
                egui::CollapsingHeader::new(
                    egui::RichText::new("⚙️ Propiedades")
                        .strong()
                        .size(14.0)
                        .color(Color32::from_rgb(150, 180, 255))
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(6.0);
                    
                    if let Some(id) = app.interaction.selected_nodes.iter().next() {
                        let title_changed = {
                            if let Some(node) = app.graph.node_mut(*id) {
                                // Título editable con mejor diseño
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("📝")
                                        .size(14.0)
                                        .color(Color32::from_rgb(150, 200, 255)));
                                    ui.label(egui::RichText::new("Título:")
                                        .size(11.0)
                                        .color(Color32::from_rgb(180, 190, 210)));
                                });
                                ui.add_space(2.0);
                                
                                let changed = egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(40, 45, 55, 150))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(100, 150, 255)))
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.text_edit_singleline(&mut node.title).changed()
                                    }).inner;
                                
                                ui.add_space(8.0);
                                
                                // Información adicional del nodo
                                egui::Frame::none()
                                    .fill(Color32::from_rgba_unmultiplied(35, 40, 50, 100))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(80, 100, 130)))
                                    .rounding(egui::Rounding::same(4.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                    .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("📍")
                                                    .size(12.0)
                                                    .color(Color32::from_rgb(150, 200, 255)));
                                                ui.label(egui::RichText::new("Posición:")
                                                    .size(11.0)
                                                    .color(Color32::from_rgb(180, 190, 210)));
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.label(egui::RichText::new(format!("{:.1}, {:.1}", node.position.x, node.position.y))
                                                        .monospace()
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(200, 220, 255)));
                                                });
                                            });
                                            ui.add_space(4.0);
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("🎨")
                                                    .size(12.0)
                                                    .color(Color32::from_rgb(150, 200, 255)));
                                                ui.label(egui::RichText::new("Color:")
                                                    .size(11.0)
                                                    .color(Color32::from_rgb(180, 190, 210)));
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    // Muestra un cuadro de color
                                                    let (rect, _) = ui.allocate_exact_size(
                                                        egui::vec2(40.0, 16.0),
                                                        egui::Sense::hover()
                                                    );
                                                    ui.painter().rect_filled(rect, 2.0, node.color);
                                                    ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgb(100, 100, 120)));
                                                    ui.add_space(4.0);
                                                    ui.label(egui::RichText::new(format!("R:{}, G:{}, B:{}", 
                                                        node.color.r(), node.color.g(), node.color.b()))
                                                        .monospace()
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(200, 220, 255)));
                                                });
                                            });
                                        });
                                    });
                                
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
                        // Estado sin selección mejorado
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("👆")
                                .size(32.0)
                                .color(Color32::from_rgb(100, 100, 130)));
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("Ninguna selección")
                                .size(11.0)
                                .color(Color32::from_rgb(150, 150, 170)));
                            ui.label(egui::RichText::new("Selecciona un nodo para ver")
                                .size(10.0)
                                .color(Color32::from_rgb(120, 120, 140)));
                            ui.label(egui::RichText::new("sus propiedades")
                                .size(10.0)
                                .color(Color32::from_rgb(120, 120, 140)));
                        });
                    }
                });
            });
        });
}

