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
                    
                    // Botón de búsqueda discreto
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Si hay texto, siempre mostrar el campo
                        if !app.sidebar_search_query.is_empty() {
                            app.sidebar_search_visible = true;
                        }
                        
                        // Mostrar campo de búsqueda si está visible
                        if app.sidebar_search_visible {
                            // Botón para cerrar
                            if ui.add(egui::Button::new(egui::RichText::new("✕")
                                    .size(11.0)
                                    .color(Color32::from_rgb(180, 180, 200)))
                                .frame(false)
                                .min_size(egui::vec2(20.0, 20.0)))
                                .clicked() {
                                app.sidebar_search_query.clear();
                                app.sidebar_search_visible = false;
                            }
                            
                            ui.add_space(4.0);
                            
                            // Campo de búsqueda
                            let available_width = (ui.available_width() - 30.0).max(150.0);
                            let response = ui.add_sized(
                                egui::vec2(available_width, 22.0),
                                egui::TextEdit::singleline(&mut app.sidebar_search_query)
                                    .hint_text("Buscar archivos y nodos...")
                            );
                            
                            // Auto-enfocar cuando se muestra
                            if !app.sidebar_search_query.is_empty() || ui.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
                                response.request_focus();
                            }
                        } else {
                            // Botón para seleccionar carpeta del explorador de archivos
                            let search_btn = ui.add(egui::Button::new(egui::RichText::new("🔍")
                                    .size(13.0)
                                    .color(Color32::from_rgb(150, 180, 200)))
                                .frame(false)
                                .min_size(egui::vec2(22.0, 22.0)));
                            
                            if search_btn.clicked() {
                                // Siempre abrir diálogo del explorador para seleccionar carpeta
                                let mut dialog = rfd::FileDialog::new();
                                
                                // Si ya hay una carpeta seleccionada, empezar desde ahí
                                if let Some(ref current_root) = app.workspace.root_path {
                                    if let Some(parent) = current_root.parent() {
                                        dialog = dialog.set_directory(parent);
                                    }
                                }
                                
                                if let Some(path) = dialog.pick_folder() {
                                    app.workspace.set_root(path.clone());
                                    // Cargar el mapa de nodos si existe
                                    if let Err(e) = app.load_graph_from_workspace() {
                                        eprintln!("Error loading graph: {}", e);
                                    }
                                    // Guardar la configuración
                                    app.save_config();
                                }
                            }
                            
                            // Tooltip para explicar la funcionalidad
                            search_btn.on_hover_ui(|ui| {
                                ui.label("Seleccionar carpeta del explorador");
                                ui.label(egui::RichText::new("(Buscar y elegir carpeta para guardar archivos y mapas de nodos)")
                                    .small()
                                    .color(Color32::from_rgb(150, 150, 170)));
                            });
                            
                            // Atajo Ctrl+F para abrir diálogo de carpeta
                            if ui.input(|i| i.key_pressed(egui::Key::F) && i.modifiers.ctrl) {
                                let mut dialog = rfd::FileDialog::new();
                                if let Some(ref current_root) = app.workspace.root_path {
                                    if let Some(parent) = current_root.parent() {
                                        dialog = dialog.set_directory(parent);
                                    }
                                }
                                if let Some(path) = dialog.pick_folder() {
                                    app.workspace.set_root(path.clone());
                                    if let Err(e) = app.load_graph_from_workspace() {
                                        eprintln!("Error loading graph: {}", e);
                                    }
                                    app.save_config();
                                }
                            }
                        }
                    });
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
                            
                            // Información simplificada del proyecto
                            ui.horizontal(|ui| {
                                let node_count = app.graph.nodes().len();
                                let count_text = if node_count == 1 {
                                    "📄 1 nodo".to_string()
                                } else {
                                    format!("📄 {} nodos", node_count)
                                };
                                ui.label(egui::RichText::new(count_text)
                                    .size(11.0)
                                    .color(Color32::from_rgb(150, 220, 255)));
                                
                                if let Some(node_map_path) = app.workspace.get_node_map_path() {
                                    if node_map_path.exists() {
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                                                .size(12.0)
                                                .color(status_text.1));
                                        });
                                    }
                                }
                            });
                            ui.add_space(6.0);
                            
                            if let Ok(mut items_vec) = app.workspace.list_files() {
                                // Filtrar por búsqueda si hay texto
                                let search_query = app.sidebar_search_query.to_lowercase();
                                if !search_query.is_empty() {
                                    items_vec.retain(|(name, _)| {
                                        name.to_lowercase().contains(&search_query)
                                    });
                                }
                                
                                // Ordenar: carpetas primero, luego archivos alfabéticamente
                                items_vec.sort_by(|(a, a_dir), (b, b_dir)| {
                                    match (a_dir, b_dir) {
                                        (true, false) => std::cmp::Ordering::Less,
                                        (false, true) => std::cmp::Ordering::Greater,
                                        _ => a.cmp(b),
                                    }
                                });
                                
                                // Mostrar mensaje si no hay resultados
                                if !search_query.is_empty() && items_vec.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.label(egui::RichText::new("🔍")
                                            .size(32.0)
                                            .color(Color32::from_rgb(100, 100, 130)));
                                        ui.add_space(4.0);
                                        ui.label(egui::RichText::new("No se encontraron archivos")
                                            .size(11.0)
                                            .color(Color32::from_rgb(150, 150, 170)));
                                        ui.label(egui::RichText::new(format!("Buscando: \"{}\"", search_query))
                                            .size(10.0)
                                            .color(Color32::from_rgb(120, 120, 140)));
                                    });
                                } else {
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
                                    } else if name.ends_with(".py") {
                                        ("🐍", Color32::from_rgb(55, 118, 171), Color32::from_rgba_unmultiplied(30, 50, 70, 80))
                                    } else if name.ends_with(".java") {
                                        ("☕", Color32::from_rgb(237, 139, 0), Color32::from_rgba_unmultiplied(70, 50, 30, 80))
                                    } else if name.ends_with(".cpp") || name.ends_with(".c") {
                                        ("©", Color32::from_rgb(120, 180, 255), Color32::from_rgba_unmultiplied(30, 50, 70, 80))
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
                                    
                                    // Verificar si hay un nodo asociado a este archivo
                                    let associated_node_id = if name.starts_with("nodes/") {
                                        app.get_node_for_file(&name)
                                    } else {
                                        None
                                    };
                                    
                                    let is_node_file = associated_node_id.is_some();
                                    let is_selected = associated_node_id.map_or(false, |id| app.interaction.selected_nodes.contains(&id));
                                    
                                    // Color de fondo mejorado si está asociado a un nodo
                                    let final_bg_color = if is_selected {
                                        Color32::from_rgba_unmultiplied(100, 150, 255, 180)
                                    } else if is_node_file {
                                        Color32::from_rgba_unmultiplied(bg_color.r(), bg_color.g(), bg_color.b(), 120)
                                    } else {
                                        bg_color
                                    };
                                    
                                    let response = egui::Frame::none()
                                        .fill(final_bg_color)
                                        .stroke(Stroke::new(if is_selected { 2.0 } else { 0.5 }, 
                                            if is_selected { Color32::from_rgb(150, 200, 255) } else { Color32::from_rgba_unmultiplied(100, 100, 120, 50) }))
                                        .rounding(egui::Rounding::same(4.0))
                                        .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(icon)
                                                    .size(14.0)
                                                    .color(icon_color));
                                                ui.label(egui::RichText::new(&name)
                                                    .size(11.5)
                                                    .color(if is_selected { Color32::WHITE } else { Color32::from_rgb(220, 220, 230) }));
                                                
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    // Botón de eliminar (visible al hover)
                                                    let delete_btn = ui.add(egui::Button::new(egui::RichText::new("✕")
                                                            .size(10.0)
                                                            .color(Color32::from_rgb(255, 100, 100)))
                                                        .frame(false)
                                                        .min_size(egui::vec2(18.0, 18.0)));
                                                    
                                                    if delete_btn.clicked() {
                                                        // Guardar acción de eliminación
                                                        let file_name_clone = name.clone();
                                                        // Usar un flag temporal para ejecutar después
                                                        if let Some(ref mut menu_state) = app.explorer_context_menu {
                                                            menu_state.pending_delete = Some(file_name_clone);
                                                            menu_state.show = false;
                                                        } else {
                                                            // Crear menú temporal solo para la acción
                                                            app.explorer_context_menu = Some(crate::core::ExplorerContextMenuState {
                                                                show: false,
                                                                position: egui::Pos2::ZERO,
                                                                file_name: name.clone(),
                                                                is_directory: is_dir,
                                                                rename_mode: false,
                                                                rename_text: String::new(),
                                                                delete_confirm: false,
                                                                pending_rename: None,
                                                                pending_delete: Some(name.clone()),
                                                                pending_goto_node: None,
                                                            });
                                                        }
                                                    }
                                                    
                                                    // Indicador si está asociado a un nodo
                                                    if is_node_file {
                                                        ui.add_space(4.0);
                                                        ui.label(egui::RichText::new("🔗")
                                                            .size(10.0)
                                                            .color(Color32::from_rgb(100, 200, 255)));
                                                    }
                                                });
                                            })
                                        }).response;
                                    
                                    // Click izquierdo: seleccionar nodo asociado o cargar archivo
                                    if response.clicked() {
                                        if let Some(node_id) = associated_node_id {
                                            // Seleccionar nodo asociado
                                            if !ui.input(|i| i.modifiers.ctrl) {
                                                app.interaction.selected_nodes.clear();
                                            }
                                            if is_selected && ui.input(|i| i.modifiers.ctrl) {
                                                app.interaction.selected_nodes.remove(&node_id);
                                            } else {
                                                app.interaction.selected_nodes.insert(node_id);
                                                // Enfocar el nodo en el viewport
                                                if let Some(node) = app.graph.node(node_id) {
                                                    use eframe::egui::Rect;
                                                    let node_rect = Rect::from_center_size(node.position, eframe::egui::vec2(100.0, 50.0));
                                                    app.viewport.focus_on(node_rect, ctx.screen_rect());
                                                }
                                            }
                                        } else if !is_dir && name.ends_with(".json") {
                                            // Si es un archivo JSON, intentar cargarlo
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
                                    
                                    // Click derecho: mostrar menú contextual
                                    if response.secondary_clicked() {
                                        app.explorer_context_menu = Some(crate::core::ExplorerContextMenuState {
                                            show: true,
                                            position: response.rect.left_bottom(),
                                            file_name: name.clone(),
                                            is_directory: is_dir,
                                            rename_mode: false,
                                            rename_text: name.clone(),
                                            delete_confirm: false,
                                            pending_rename: None,
                                            pending_delete: None,
                                            pending_goto_node: None,
                                        });
                                    }
                                    
                                    ui.add_space(2.0);
                                    }
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
                        // Lista simplificada de nodos
                        let search_query = app.sidebar_search_query.to_lowercase();
                        let mut nodes_info: Vec<_> = app.graph.nodes().iter()
                            .map(|n| (n.id, n.title.clone(), n.language, n.color))
                            .collect();
                        
                        // Filtrar por búsqueda si hay texto
                        if !search_query.is_empty() {
                            nodes_info.retain(|(_, title, _, _)| {
                                title.to_lowercase().contains(&search_query)
                            });
                        }
                        
                        // Mostrar mensaje si no hay resultados
                        if !search_query.is_empty() && nodes_info.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("🔍")
                                    .size(32.0)
                                    .color(Color32::from_rgb(100, 100, 130)));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("No se encontraron nodos")
                                    .size(11.0)
                                    .color(Color32::from_rgb(150, 150, 170)));
                                ui.label(egui::RichText::new(format!("Buscando: \"{}\"", search_query))
                                    .size(10.0)
                                    .color(Color32::from_rgb(120, 120, 140)));
                            });
                        } else {
                            for (id, title, language, color) in nodes_info {
                            let is_selected = app.interaction.selected_nodes.contains(&id);
                            
                            let (lang_icon, lang_color) = match language {
                                crate::core::node_graph::NodeLanguage::Rust => ("🦀", Color32::from_rgb(255, 140, 100)),
                                crate::core::node_graph::NodeLanguage::C => ("©", Color32::from_rgb(120, 180, 255)),
                                crate::core::node_graph::NodeLanguage::Cpp => ("⊕", Color32::from_rgb(180, 140, 255)),
                                crate::core::node_graph::NodeLanguage::Asm => ("⚡", Color32::from_rgb(255, 220, 100)),
                                crate::core::node_graph::NodeLanguage::Zig => ("⚡", Color32::from_rgb(240, 170, 0)),
                                crate::core::node_graph::NodeLanguage::Java => ("☕", Color32::from_rgb(237, 139, 0)),
                                crate::core::node_graph::NodeLanguage::Python => ("🐍", Color32::from_rgb(55, 118, 171)),
                                crate::core::node_graph::NodeLanguage::Mojo => ("🔥", Color32::from_rgb(255, 100, 100)),
                                crate::core::node_graph::NodeLanguage::MojoAI => ("🤖", Color32::from_rgb(255, 150, 100)),
                                crate::core::node_graph::NodeLanguage::Text => ("📄", Color32::from_rgb(200, 200, 150)),
                                crate::core::node_graph::NodeLanguage::Auto => ("⚙", Color32::from_rgb(180, 180, 180)),
                            };
                            
                            let bg_color = if is_selected {
                                Color32::from_rgba_unmultiplied(100, 150, 255, 150)
                            } else {
                                Color32::from_rgba_unmultiplied(40, 45, 55, 100)
                            };
                            
                            let response = egui::Frame::none()
                                .fill(bg_color)
                                .stroke(Stroke::new(if is_selected { 1.5 } else { 0.5 }, 
                                    if is_selected { Color32::from_rgb(150, 200, 255) } else { color }))
                                .rounding(egui::Rounding::same(4.0))
                                .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Barra de color
                                        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
                                        ui.painter().rect_filled(rect, 1.0, color);
                                        ui.add_space(6.0);
                                        
                                        ui.label(egui::RichText::new(lang_icon).size(13.0).color(lang_color));
                                        ui.label(egui::RichText::new(&title)
                                            .size(11.5)
                                            .color(if is_selected { Color32::WHITE } else { Color32::from_rgb(220, 220, 230) }));
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            // Botón eliminar
                                            if ui.add(egui::Button::new(egui::RichText::new("✕")
                                                    .size(10.0)
                                                    .color(Color32::from_rgb(255, 120, 120)))
                                                .frame(false)
                                                .min_size(egui::vec2(18.0, 18.0)))
                                                .clicked() {
                                                app.graph.remove_node(id);
                                                app.interaction.selected_nodes.remove(&id);
                                                app.check_and_auto_save();
                                            }
                                        });
                                    });
                                }).response;
                            
                            // Click: seleccionar nodo
                            if response.clicked() {
                                if !ui.input(|i| i.modifiers.ctrl) {
                                    app.interaction.selected_nodes.clear();
                                }
                                if is_selected && ui.input(|i| i.modifiers.ctrl) {
                                    app.interaction.selected_nodes.remove(&id);
                                } else {
                                    app.interaction.selected_nodes.insert(id);
                                }
                            }
                            ui.add_space(2.0);
                            }
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
                                
                                // Información simplificada
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("📍").size(11.0).color(Color32::from_rgb(150, 200, 255)));
                                    ui.label(egui::RichText::new(format!("{:.0}, {:.0}", node.position.x, node.position.y))
                                        .monospace()
                                        .size(10.0)
                                        .color(Color32::from_rgb(200, 220, 255)));
                                    
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new("🎨").size(11.0).color(Color32::from_rgb(150, 200, 255)));
                                    let (rect, _) = ui.allocate_exact_size(egui::vec2(30.0, 14.0), egui::Sense::hover());
                                    ui.painter().rect_filled(rect, 2.0, node.color);
                                    ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgb(100, 100, 120)));
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
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 MENÚ CONTEXTUAL DEL EXPLORADOR (Eliminar, Renombrar)
            // ═══════════════════════════════════════════════════════════════════
            // Extraer información necesaria antes de entrar al bloque mutable
            let menu_file_name = app.explorer_context_menu.as_ref().map(|m| m.file_name.clone());
            let menu_is_node_file = menu_file_name.as_ref().map(|n| n.starts_with("nodes/")).unwrap_or(false);
            let menu_node_id_opt = if menu_is_node_file {
                menu_file_name.as_ref().and_then(|name| app.get_node_for_file(name))
            } else {
                None
            };
            
            if let Some(ref mut menu_state) = app.explorer_context_menu {
                if menu_state.show {
                    let mut open = menu_state.show;
                    
                    egui::Window::new("📁 Opciones de archivo")
                        .open(&mut open)
                        .fixed_pos(menu_state.position)
                        .resizable(false)
                        .collapsible(false)
                        .title_bar(false)
                        .frame(egui::Frame::popup(&ctx.style())
                            .fill(Color32::from_rgb(40, 40, 50))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(100, 150, 255))))
                        .show(ctx, |ui| {
                            ui.set_min_width(200.0);
                            
                            // Nombre del archivo
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(if menu_state.is_directory { "📁" } else { "📄" })
                                    .size(14.0));
                                ui.label(egui::RichText::new(&menu_state.file_name)
                                    .size(12.0)
                                    .color(Color32::from_rgb(220, 220, 230)));
                            });
                            ui.separator();
                            
                            // Modo renombrar
                            if menu_state.rename_mode {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Nuevo nombre:")
                                        .size(11.0)
                                        .color(Color32::from_rgb(180, 190, 210)));
                                    let response = ui.text_edit_singleline(&mut menu_state.rename_text);
                                    
                                    ui.add_space(4.0);
                                    ui.horizontal(|ui| {
                                        let accept_clicked = ui.button(egui::RichText::new("✓ Aceptar")
                                                .size(11.0)
                                                .color(Color32::WHITE))
                                            .clicked();
                                        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                                        
                                        if accept_clicked || enter_pressed {
                                            if !menu_state.rename_text.is_empty() && menu_state.rename_text != menu_state.file_name {
                                                // Guardar acción pendiente
                                                menu_state.pending_rename = Some((menu_state.file_name.clone(), menu_state.rename_text.clone()));
                                            }
                                            menu_state.show = false;
                                        }
                                        
                                        let cancel_clicked = ui.button(egui::RichText::new("✗ Cancelar")
                                                .size(11.0)
                                                .color(Color32::from_rgb(200, 150, 150)))
                                            .clicked();
                                        let escape_pressed = ui.input(|i| i.key_pressed(egui::Key::Escape));
                                        
                                        if cancel_clicked || escape_pressed {
                                            menu_state.show = false;
                                        }
                                    });
                                });
                            } else {
                                // Modo normal: mostrar opciones
                                
                                // Opción: Renombrar
                                if ui.button(egui::RichText::new("✏️ Renombrar")
                                        .size(12.0)
                                        .color(Color32::from_rgb(150, 200, 255)))
                                    .clicked() {
                                    menu_state.rename_mode = true;
                                    menu_state.rename_text = menu_state.file_name.clone();
                                }
                                
                                // Opción: Eliminar
                                if menu_state.delete_confirm {
                                    ui.separator();
                                    ui.label(egui::RichText::new("¿Eliminar este archivo?")
                                        .size(11.0)
                                        .color(Color32::from_rgb(255, 150, 150)));
                                    
                                    ui.horizontal(|ui| {
                                        if ui.add(egui::Button::new(egui::RichText::new("✓ Sí, eliminar")
                                                .size(11.0)
                                                .color(Color32::WHITE))
                                            .fill(Color32::from_rgb(200, 50, 50)))
                                            .clicked() {
                                            // Guardar acción pendiente
                                            menu_state.pending_delete = Some(menu_state.file_name.clone());
                                            menu_state.show = false;
                                        }
                                        if ui.button(egui::RichText::new("✗ Cancelar")
                                                .size(11.0)
                                                .color(Color32::from_rgb(200, 150, 150)))
                                            .clicked() {
                                            menu_state.delete_confirm = false;
                                        }
                                    });
                                } else {
                                    if ui.button(egui::RichText::new("🗑️ Eliminar")
                                            .size(12.0)
                                            .color(Color32::from_rgb(255, 150, 150)))
                                        .clicked() {
                                        menu_state.delete_confirm = true;
                                    }
                                }
                                
                                // Si es un archivo de nodo, mostrar opción para ir al nodo
                                if menu_is_node_file {
                                    if let Some(node_id) = menu_node_id_opt {
                                        ui.separator();
                                        if ui.button(egui::RichText::new("🔗 Ir al nodo")
                                                .size(12.0)
                                                .color(Color32::from_rgb(100, 200, 255)))
                                            .clicked() {
                                            // Guardar acción pendiente
                                            menu_state.pending_goto_node = Some(node_id);
                                            menu_state.show = false;
                                        }
                                    }
                                }
                            }
                        });
                    
                    // Actualizar estado y ejecutar acciones pendientes
                    if !menu_state.show {
                        let menu_state_clone = menu_state.clone();
                        drop(menu_state); // Liberar el préstamo
                        
                        // Ejecutar acciones pendientes
                        if let Some((old_name, new_name)) = menu_state_clone.pending_rename {
                            match app.rename_file_in_explorer(&old_name, &new_name) {
                                Ok(_) => {
                                    app.sync_nodes_from_file_organization();
                                }
                                Err(e) => {
                                    app.terminal.visible = true;
                                    app.terminal.rust_output = format!("❌ Error al renombrar: {}", e);
                                    app.terminal.active_tab = crate::compilation::terminal::TerminalTab::Rust;
                                }
                            }
                        }
                        
                        if let Some(file_name) = menu_state_clone.pending_delete {
                            match app.delete_file_from_explorer(&file_name) {
                                Ok(_) => {
                                    app.sync_nodes_from_file_organization();
                                }
                                Err(e) => {
                                    app.terminal.visible = true;
                                    app.terminal.rust_output = format!("❌ Error al eliminar: {}", e);
                                    app.terminal.active_tab = crate::compilation::terminal::TerminalTab::Rust;
                                }
                            }
                        }
                        
                        if let Some(node_id) = menu_state_clone.pending_goto_node {
                            // Seleccionar y enfocar el nodo
                            app.interaction.selected_nodes.clear();
                            app.interaction.selected_nodes.insert(node_id);
                            if let Some(node) = app.graph.node(node_id) {
                                use eframe::egui::Rect;
                                let node_rect = Rect::from_center_size(node.position, eframe::egui::vec2(100.0, 50.0));
                                app.viewport.focus_on(node_rect, ctx.screen_rect());
                            }
                        }
                        
                        app.explorer_context_menu = None;
                    }
                }
            }
            
            // Ejecutar eliminaciones pendientes desde botones directos
            if let Some(ref menu_state) = app.explorer_context_menu {
                if !menu_state.show && menu_state.pending_delete.is_some() {
                    let file_name = menu_state.pending_delete.clone();
                    app.explorer_context_menu = None; // Limpiar primero
                    
                    if let Some(file_name) = file_name {
                        match app.delete_file_from_explorer(&file_name) {
                            Ok(_) => {
                                app.sync_nodes_from_file_organization();
                            }
                            Err(e) => {
                                app.terminal.visible = true;
                                app.terminal.rust_output = format!("❌ Error al eliminar: {}", e);
                                app.terminal.active_tab = crate::compilation::terminal::TerminalTab::Rust;
                            }
                        }
                    }
                }
            }
        });
}

