use eframe::egui::{self, ScrollArea, Color32, Stroke};
use crate::core::NodeGraphApp;
use crate::ui::theme::THEME;

pub fn draw_sidebar(app: &mut NodeGraphApp, ctx: &egui::Context, _open_factor: f32) {
    egui::SidePanel::left("sidebar")
        .resizable(true)
        .default_width(300.0)
        .min_width(240.0)
        .max_width(480.0)
        .frame(egui::Frame::side_top_panel(&ctx.style())
            .fill(THEME.background_secondary) // Fondo usando tema
            .inner_margin(egui::Margin::same(16.0)) // Más padding
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(2.0, 0.0),
                blur: 12.0, // Sombra más suave
                spread: 0.0,
                color: THEME.shadow_medium,
            }))
        .show(ctx, |ui| {
            // Header mejorado con estilo profesional
            ui.vertical(|ui| {
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📂")
                        .size(24.0) // Icono más grande
                        .color(THEME.accent_primary));
                    ui.heading(egui::RichText::new("Explorador")
                        .strong()
                        .size(20.0) // Título más grande
                        .color(THEME.text_primary));
                    
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
                                    .size(12.0) // Botón más grande
                                    .color(THEME.text_muted))
                                .frame(false)
                                .min_size(egui::vec2(24.0, 24.0))) // Área de clic más grande
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
                
                // Línea separadora profesional con gradiente sutil
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 1.5),
                    egui::Sense::hover()
                );
                // Gradiente sutil de izquierda a derecha
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(80, 120, 200, 120)
                );
                // Línea superior más clara para efecto de profundidad
                let top_line = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 0.5));
                ui.painter().rect_filled(
                    top_line,
                    0.0,
                    Color32::from_rgba_unmultiplied(120, 160, 220, 80)
                );
            });
            ui.add_space(8.0);

            ScrollArea::vertical().show(ui, |ui| {
                // ═══════════════════════════════════════════════════════════════════
                // SECCIÓN: CARPETA SELECCIONADA
                // ═══════════════════════════════════════════════════════════════════
                egui::CollapsingHeader::new(
                    egui::RichText::new("📁 Carpeta seleccionada")
                        .strong()
                        .size(13.5)
                        .color(Color32::from_rgb(150, 180, 255))
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(8.0);
                    if app.workspace.has_root() {
                        if let Some(folder_name) = app.workspace.get_folder_name() {
                            // Nombre de carpeta destacado con diseño profesional
                            egui::Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(48, 58, 78, 200))
                                .stroke(Stroke::new(1.5, Color32::from_rgb(110, 160, 255)))
                                .rounding(egui::Rounding::same(10.0))
                                .inner_margin(egui::Margin::symmetric(14.0, 12.0))
                                .shadow(egui::epaint::Shadow {
                                    offset: egui::vec2(0.0, 2.0),
                                    blur: 6.0,
                                    spread: 0.0,
                                    color: Color32::from_black_alpha(60),
                                })
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("📁")
                                            .size(22.0)
                                            .color(Color32::from_rgb(255, 210, 110)));
                                        ui.add_space(10.0);
                                        ui.label(egui::RichText::new(&folder_name)
                                            .strong()
                                            .size(14.0)
                                            .color(Color32::from_rgb(255, 255, 255)));
                                    });
                                });
                            ui.add_space(10.0);
                            
                            // Información del proyecto mejorada con estilo profesional
                            egui::Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(38, 45, 58, 140))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                                .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(80, 100, 140, 60)))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let node_count = app.graph.nodes().len();
                                        let count_text = if node_count == 1 {
                                            "📄 1 nodo".to_string()
                                        } else {
                                            format!("📄 {} nodos", node_count)
                                        };
                                        ui.label(egui::RichText::new(count_text)
                                            .size(11.5)
                                            .color(Color32::from_rgb(150, 220, 255)));
                                        
                                        if let Some(node_map_path) = app.workspace.get_node_map_path() {
                                            if node_map_path.exists() {
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    let status_text = if let Some(last_save) = app.last_save_time {
                                                        let elapsed = last_save.elapsed().as_secs();
                                                        if elapsed < 2 {
                                                            ("● Guardado", Color32::from_rgb(100, 255, 150))
                                                        } else {
                                                            ("✓ Guardado", Color32::from_rgb(150, 255, 150))
                                                        }
                                                    } else {
                                                        ("✓ Guardado", Color32::from_rgb(150, 255, 150))
                                                    };
                                                    ui.label(egui::RichText::new(status_text.0)
                                                        .size(10.5)
                                                        .color(status_text.1));
                                                });
                                            }
                                        }
                                    });
                                });
                            ui.add_space(8.0);
                            
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
                
                ui.add_space(10.0);
                
                // ═══════════════════════════════════════════════════════════════════
                // SEPARADOR VISUAL MEJORADO
                // ═══════════════════════════════════════════════════════════════════
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 2.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(100, 150, 255, 80)
                );
                ui.add_space(10.0);
                
                // ═══════════════════════════════════════════════════════════════════
                // SECCIÓN: NODOS - Dividida en 2 columnas: Nodos y Carpetas
                // ═══════════════════════════════════════════════════════════════════
                let total_nodes = app.graph.nodes().len();
                let folder_nodes: Vec<_> = app.graph.nodes().iter()
                    .filter(|n| n.title.starts_with("📁 ") && n.subnetwork_graph.is_some())
                    .map(|n| n.id)
                    .collect();
                let normal_nodes_count = total_nodes - folder_nodes.len();
                
                egui::CollapsingHeader::new(
                    egui::RichText::new(format!("📊 Nodos ({})", total_nodes))
                        .strong()
                        .size(13.5)
                        .color(Color32::from_rgb(150, 180, 255))
                )
                .default_open(true)
                .show(ui, |ui| {
                    ui.add_space(8.0);
                    
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
                        // Separar nodos en dos grupos: normales y carpetas
                        let search_query = app.sidebar_search_query.to_lowercase();
                        let mut normal_nodes: Vec<_> = app.graph.nodes().iter()
                            .filter(|n| !(n.title.starts_with("📁 ") && n.subnetwork_graph.is_some()))
                            .map(|n| (n.id, n.title.clone(), n.language, n.color))
                            .collect();
                        
                        let mut folder_nodes_info: Vec<_> = app.graph.nodes().iter()
                            .filter(|n| n.title.starts_with("📁 ") && n.subnetwork_graph.is_some())
                            .map(|n| {
                                let node_count = n.subnetwork_graph.as_ref()
                                    .map(|g| g.nodes().len())
                                    .unwrap_or(0);
                                (n.id, n.title.clone(), n.color, node_count)
                            })
                            .collect();
                        
                        // Filtrar por búsqueda si hay texto
                        if !search_query.is_empty() {
                            normal_nodes.retain(|(_, title, _, _)| {
                                title.to_lowercase().contains(&search_query)
                            });
                            folder_nodes_info.retain(|(_, title, _, _)| {
                                title.to_lowercase().contains(&search_query)
                            });
                        }
                        
                        // Mostrar mensaje si no hay resultados
                        if !search_query.is_empty() && normal_nodes.is_empty() && folder_nodes_info.is_empty() {
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
                            // ═══════════════════════════════════════════════════════════════════
                            // 🆕 DISEÑO DE 2 COLUMNAS: Nodos normales | Carpetas
                            // ═══════════════════════════════════════════════════════════════════
                            // Calcular ancho disponible antes de crear las columnas
                            let total_width = ui.available_width();
                            let separator_width = 8.0; // Espacio para el separador
                            let column_width = (total_width - separator_width) / 2.0;
                            
                            ui.horizontal(|ui| {
                                // ═══════════════════════════════════════════════════════════════════
                                // COLUMNA IZQUIERDA: NODOS NORMALES
                                // ═══════════════════════════════════════════════════════════════════
                                ui.vertical(|ui| {
                                    ui.set_width(column_width);
                                    
                                    // Header de columna profesional
                                    egui::Frame::none()
                                        .fill(Color32::from_rgba_unmultiplied(42, 52, 72, 140))
                                        .rounding(egui::Rounding::same(8.0))
                                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                        .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(100, 140, 200, 40)))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("📄")
                                                    .size(15.0)
                                                    .color(Color32::from_rgb(160, 210, 255)));
                                                ui.add_space(8.0);
                                                ui.label(egui::RichText::new("Nodos")
                                                    .size(12.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(160, 210, 255)));
                                                ui.add_space(8.0);
                                                ui.label(egui::RichText::new(format!("({})", normal_nodes.len()))
                                                    .size(11.0)
                                                    .color(Color32::from_rgb(130, 160, 190)));
                                            });
                                        });
                                    ui.add_space(6.0);
                                    
                                    if normal_nodes.is_empty() {
                                        ui.vertical_centered(|ui| {
                                            ui.label(egui::RichText::new("📭")
                                                .size(24.0)
                                                .color(Color32::from_rgb(100, 100, 130)));
                                            ui.label(egui::RichText::new("Sin nodos")
                                                .size(9.0)
                                                .color(Color32::from_rgb(120, 120, 140)));
                                        });
                                    } else {
                                        for (id, title, language, color) in normal_nodes {
                                            let is_selected = app.interaction.selected_nodes.contains(&id);
                                            
                                            let (lang_icon, lang_color) = match language {
                                                crate::core::node_graph::NodeLanguage::Rust => ("🦀", Color32::from_rgb(255, 140, 100)),
                                                crate::core::node_graph::NodeLanguage::Asm => ("⚡", Color32::from_rgb(255, 220, 100)),
                                                crate::core::node_graph::NodeLanguage::Java => ("☕", Color32::from_rgb(237, 139, 0)),
                                                crate::core::node_graph::NodeLanguage::Python => ("🐍", Color32::from_rgb(55, 118, 171)),
                                                crate::core::node_graph::NodeLanguage::Cpp => ("🔷", Color32::from_rgb(0, 89, 153)),
                                                crate::core::node_graph::NodeLanguage::Text => ("📄", Color32::from_rgb(200, 200, 150)),
                                                crate::core::node_graph::NodeLanguage::Auto => ("⚙", Color32::from_rgb(180, 180, 180)),
                                            };
                                            
                                            let bg_color = if is_selected {
                                                Color32::from_rgba_unmultiplied(110, 160, 255, 200)
                                            } else {
                                                Color32::from_rgba_unmultiplied(42, 47, 57, 140)
                                            };
                                            
                                            let mut frame = egui::Frame::none()
                                                .fill(bg_color)
                                                .stroke(Stroke::new(if is_selected { 2.0 } else { 0.5 }, 
                                                    if is_selected { Color32::from_rgb(160, 210, 255) } else { Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 60) }))
                                                .rounding(egui::Rounding::same(8.0))
                                                .inner_margin(egui::Margin::symmetric(10.0, 8.0));
                                            
                                            if is_selected {
                                                frame = frame.shadow(egui::epaint::Shadow {
                                                    offset: egui::vec2(0.0, 2.0),
                                                    blur: 8.0,
                                                    spread: 0.0,
                                                    color: Color32::from_rgba_unmultiplied(110, 160, 255, 80),
                                                });
                                            }
                                            
                                            let response = frame.show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        // Barra de color
                                                        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
                                                        ui.painter().rect_filled(rect, 1.0, color);
                                                        ui.add_space(6.0);
                                                        
                                                        ui.label(egui::RichText::new(lang_icon).size(13.0).color(lang_color));
                                                        ui.add_space(4.0);
                                                        
                                                        // Título truncado si es muy largo para evitar desbordamiento
                                                        let display_title = if title.len() > 20 {
                                                            format!("{}...", &title.chars().take(17).collect::<String>())
                                                        } else {
                                                            title.clone()
                                                        };
                                                        ui.label(egui::RichText::new(&display_title)
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
                                });
                                
                                // Separador vertical entre columnas mejorado
                                ui.add_space(4.0);
                                ui.separator();
                                ui.add_space(4.0);
                                
                                // ═══════════════════════════════════════════════════════════════════
                                // COLUMNA DERECHA: CARPETAS
                                // ═══════════════════════════════════════════════════════════════════
                                ui.vertical(|ui| {
                                    ui.set_width(column_width);
                                    
                                    // Header de columna profesional
                                    egui::Frame::none()
                                        .fill(Color32::from_rgba_unmultiplied(62, 52, 42, 140))
                                        .rounding(egui::Rounding::same(8.0))
                                        .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                        .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(200, 160, 100, 40)))
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("📁")
                                                    .size(15.0)
                                                    .color(Color32::from_rgb(255, 210, 110)));
                                                ui.add_space(8.0);
                                                ui.label(egui::RichText::new("Carpetas")
                                                    .size(12.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(255, 210, 110)));
                                                ui.add_space(8.0);
                                                ui.label(egui::RichText::new(format!("({})", folder_nodes_info.len()))
                                                    .size(11.0)
                                                    .color(Color32::from_rgb(210, 190, 130)));
                                            });
                                        });
                                    ui.add_space(6.0);
                                    
                                    if folder_nodes_info.is_empty() {
                                        ui.vertical_centered(|ui| {
                                            ui.label(egui::RichText::new("📂")
                                                .size(24.0)
                                                .color(Color32::from_rgb(100, 100, 130)));
                                            ui.label(egui::RichText::new("Sin carpetas")
                                                .size(9.0)
                                                .color(Color32::from_rgb(120, 120, 140)));
                                        });
                                    } else {
                                        for (id, title, color, node_count) in folder_nodes_info {
                                            let is_selected = app.interaction.selected_nodes.contains(&id);
                                            
                                            // ═══════════════════════════════════════════════════════════════════
                                            // 🆕 DETECTAR SI ES CARPETA HEREDABLE Y APLICAR COLORES ÚNICOS
                                            // ═══════════════════════════════════════════════════════════════════
                                            let is_inheritable = title.contains("(Heredable)");
                                            
                                            // Colores únicos según tipo de carpeta
                                            let (folder_color, bg_color_base, bg_color_selected) = if is_inheritable {
                                                // Esquema púrpura/magenta único para carpetas heredables
                                                (
                                                    Color32::from_rgb(200, 150, 255), // Púrpura brillante
                                                    Color32::from_rgba_unmultiplied(55, 45, 65, 140), // Fondo púrpura oscuro
                                                    Color32::from_rgba_unmultiplied(220, 170, 255, 220), // Fondo púrpura cuando seleccionado
                                                )
                                            } else {
                                                // Esquema dorado para carpetas normales
                                                (
                                                    Color32::from_rgb(255, 200, 50), // Amarillo dorado
                                                    Color32::from_rgba_unmultiplied(52, 47, 42, 140), // Fondo dorado oscuro
                                                    Color32::from_rgba_unmultiplied(255, 210, 110, 220), // Fondo dorado cuando seleccionado
                                                )
                                            };
                                            
                                            // Limpiar el título: remover el prefijo "📁 " si existe (ya tenemos el icono)
                                            let clean_title = if title.starts_with("📁 ") {
                                                title.strip_prefix("📁 ").unwrap_or(&title).to_string()
                                            } else {
                                                title
                                            };
                                            
                                            let bg_color = if is_selected {
                                                bg_color_selected
                                            } else {
                                                bg_color_base
                                            };
                                            
                                            let mut frame = egui::Frame::none()
                                                .fill(bg_color)
                                                .stroke(Stroke::new(if is_selected { 2.0 } else { 0.5 }, 
                                                    if is_selected { Color32::from_rgb(255, 230, 130) } else { Color32::from_rgba_unmultiplied(folder_color.r(), folder_color.g(), folder_color.b(), 70) }))
                                                .rounding(egui::Rounding::same(8.0))
                                                .inner_margin(egui::Margin::symmetric(10.0, 8.0));
                                            
                                            if is_selected {
                                                frame = frame.shadow(egui::epaint::Shadow {
                                                    offset: egui::vec2(0.0, 2.0),
                                                    blur: 8.0,
                                                    spread: 0.0,
                                                    color: Color32::from_rgba_unmultiplied(255, 210, 110, 80),
                                                });
                                            }
                                            
                                            let response = frame.show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        // Barra de color única según tipo de carpeta
                                                        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 18.0), egui::Sense::hover());
                                                        ui.painter().rect_filled(rect, 1.0, folder_color);
                                                        ui.add_space(6.0);
                                                        
                                                        // Icono de carpeta con color único
                                                        let icon_color = if is_inheritable {
                                                            Color32::from_rgb(220, 170, 255) // Púrpura para heredables
                                                        } else {
                                                            Color32::from_rgb(255, 200, 100) // Dorado para normales
                                                        };
                                                        ui.label(egui::RichText::new("📁").size(13.0).color(icon_color));
                                                        ui.add_space(4.0);
                                                        
                                                        // Título de la carpeta truncado si es muy largo
                                                        let display_folder_title = if clean_title.len() > 18 {
                                                            format!("{}...", &clean_title.chars().take(15).collect::<String>())
                                                        } else {
                                                            clean_title.clone()
                                                        };
                                                        ui.label(egui::RichText::new(&display_folder_title)
                                                            .size(11.5)
                                                            .color(if is_selected { Color32::WHITE } else { Color32::from_rgb(255, 240, 200) }));
                                                        
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
                                                            ui.add_space(4.0);
                                                            
                                                            // Contador de nodos dentro de la carpeta (pequeño badge)
                                                            ui.label(egui::RichText::new(format!("{}", node_count))
                                                                .size(9.0)
                                                                .color(Color32::from_rgb(200, 200, 150)));
                                                        });
                                                    });
                                                }).response;
                                            
                                            // Click: seleccionar carpeta
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
                                });
                            });
                        }
                    }
                });
                
                ui.add_space(10.0);
                
                // ═══════════════════════════════════════════════════════════════════
                // SEPARADOR VISUAL MEJORADO
                // ═══════════════════════════════════════════════════════════════════
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 2.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgba_unmultiplied(100, 150, 255, 80)
                );
                ui.add_space(10.0);
                
                // ═══════════════════════════════════════════════════════════════════
                // SECCIÓN: PROPIEDADES (siempre visible, no colapsable)
                // ═══════════════════════════════════════════════════════════════════
                egui::Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(38, 43, 53, 180))
                    .stroke(Stroke::new(1.5, Color32::from_rgb(90, 110, 150)))
                    .rounding(egui::Rounding::same(10.0))
                    .inner_margin(egui::Margin::symmetric(14.0, 12.0))
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 2.0),
                        blur: 8.0,
                        spread: 0.0,
                        color: Color32::from_black_alpha(80),
                    })
                    .show(ui, |ui| {
                        // Header de propiedades profesional
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(48, 53, 63, 160))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                            .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(100, 130, 180, 50)))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("⚙️")
                                        .size(17.0)
                                        .color(Color32::from_rgb(160, 190, 255)));
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new("Propiedades")
                                        .strong()
                                        .size(14.0)
                                        .color(Color32::from_rgb(160, 190, 255)));
                                });
                            });
                        ui.add_space(10.0);
                    
                    if let Some(id) = app.interaction.selected_nodes.iter().next() {
                        // Obtener valores actuales antes de entrar al bloque mutable
                        let node_id = *id;
                        let (current_title, current_color, current_lang) = {
                            if let Some(node) = app.graph.node(node_id) {
                                (node.title.clone(), node.color, node.language)
                            } else {
                                return;
                            }
                        };
                        
                        let mut title_text = current_title.clone();
                        let mut rgb = [
                            current_color.r() as f32,
                            current_color.g() as f32,
                            current_color.b() as f32,
                        ];
                        let mut selected_lang = current_lang;
                        
                        let mut title_changed = false;
                        let mut color_changed = false;
                        let mut language_changed = false;
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // TÍTULO EDITABLE
                        // ═══════════════════════════════════════════════════════════════════
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("📝")
                                .size(14.0)
                                .color(Color32::from_rgb(150, 200, 255)));
                            ui.label(egui::RichText::new("Título:")
                                .size(11.0)
                                .color(Color32::from_rgb(180, 190, 210)));
                        });
                        ui.add_space(2.0);
                        
                        title_changed = egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(40, 45, 55, 150))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(100, 150, 255)))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                            .show(ui, |ui| {
                                ui.text_edit_singleline(&mut title_text).changed()
                            }).inner;
                        
                        if title_changed {
                            if let Some(node) = app.graph.node_mut(node_id) {
                                node.title = title_text.clone();
                            }
                        }
                        
                        ui.add_space(12.0);
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // SELECTOR DE COLOR INTERACTIVO
                        // ═══════════════════════════════════════════════════════════════════
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🎨")
                                .size(14.0)
                                .color(Color32::from_rgb(150, 200, 255)));
                            ui.label(egui::RichText::new("Color:")
                                .size(11.0)
                                .color(Color32::from_rgb(180, 190, 210)));
                        });
                        ui.add_space(2.0);
                        
                        // Área interactiva para el selector de color
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(25, 30, 40, 80))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Muestra el color actual (actualizado desde current_color)
                                    let display_color = if let Some(node) = app.graph.node(node_id) {
                                        node.color
                                    } else {
                                        current_color
                                    };
                                    let (rect, _) = ui.allocate_exact_size(egui::vec2(60.0, 28.0), egui::Sense::hover());
                                    ui.painter().rect_filled(rect, 4.0, display_color);
                                    ui.painter().rect_stroke(rect, 4.0, Stroke::new(2.0, Color32::from_rgb(100, 100, 120)));
                                    
                                    ui.add_space(8.0);
                                    
                                    // Sliders RGB con mejor manejo de eventos
                                    ui.vertical(|ui| {
                                        // Slider R
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("R:").size(10.0).color(Color32::from_rgb(255, 100, 100)));
                                            let slider_response = ui.add_sized(egui::vec2(120.0, 20.0), egui::Slider::new(&mut rgb[0], 0.0..=255.0));
                                            if slider_response.changed() || slider_response.dragged() {
                                                let new_color = Color32::from_rgb(rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
                                                if let Some(node) = app.graph.node_mut(node_id) {
                                                    node.color = new_color;
                                                    color_changed = true;
                                                }
                                            }
                                        });
                                        
                                        // Slider G
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("G:").size(10.0).color(Color32::from_rgb(100, 255, 100)));
                                            let slider_response = ui.add_sized(egui::vec2(120.0, 20.0), egui::Slider::new(&mut rgb[1], 0.0..=255.0));
                                            if slider_response.changed() || slider_response.dragged() {
                                                let new_color = Color32::from_rgb(rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
                                                if let Some(node) = app.graph.node_mut(node_id) {
                                                    node.color = new_color;
                                                    color_changed = true;
                                                }
                                            }
                                        });
                                        
                                        // Slider B
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("B:").size(10.0).color(Color32::from_rgb(100, 100, 255)));
                                            let slider_response = ui.add_sized(egui::vec2(120.0, 20.0), egui::Slider::new(&mut rgb[2], 0.0..=255.0));
                                            if slider_response.changed() || slider_response.dragged() {
                                                let new_color = Color32::from_rgb(rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
                                                if let Some(node) = app.graph.node_mut(node_id) {
                                                    node.color = new_color;
                                                    color_changed = true;
                                                }
                                            }
                                        });
                                    });
                                });
                            });
                        
                        ui.add_space(6.0);
                        
                        // Colores predefinidos rápidos con mejor manejo de eventos
                        ui.label(egui::RichText::new("Colores rápidos:").size(10.0).color(Color32::from_rgb(150, 150, 170)));
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(25, 30, 40, 80))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                            .show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    let preset_colors = [
                                        Color32::from_rgb(100, 150, 255), // Azul
                                        Color32::from_rgb(100, 255, 150), // Verde
                                        Color32::from_rgb(255, 150, 100), // Naranja
                                        Color32::from_rgb(255, 100, 150), // Rosa
                                        Color32::from_rgb(200, 100, 255), // Púrpura
                                        Color32::from_rgb(255, 200, 100), // Amarillo
                                        Color32::from_rgb(100, 255, 255), // Cyan
                                        Color32::from_rgb(255, 255, 100), // Amarillo claro
                                    ];
                                    
                                    for preset_color in preset_colors.iter() {
                                        // Usar Button para mejor interacción
                                        let button = egui::Button::new("")
                                            .fill(*preset_color)
                                            .min_size(egui::vec2(28.0, 28.0))
                                            .stroke(Stroke::new(1.5, Color32::from_rgb(100, 100, 120)));
                                        
                                        if ui.add(button).clicked() {
                                            if let Some(node) = app.graph.node_mut(node_id) {
                                                if node.color != *preset_color {
                                                    node.color = *preset_color;
                                                    color_changed = true;
                                                    // Actualizar valores RGB para que los sliders se actualicen
                                                    rgb[0] = preset_color.r() as f32;
                                                    rgb[1] = preset_color.g() as f32;
                                                    rgb[2] = preset_color.b() as f32;
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        
                        ui.add_space(12.0);
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // LENGUAJE DEL NODO
                        // ═══════════════════════════════════════════════════════════════════
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("💻")
                                .size(14.0)
                                .color(Color32::from_rgb(150, 200, 255)));
                            ui.label(egui::RichText::new("Lenguaje:")
                                .size(11.0)
                                .color(Color32::from_rgb(180, 190, 210)));
                        });
                        ui.add_space(2.0);
                        
                        // Selector de lenguaje con mejor manejo
                        egui::Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(25, 30, 40, 80))
                            .rounding(egui::Rounding::same(4.0))
                            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                            .show(ui, |ui| {
                                let current_lang_display = if let Some(node) = app.graph.node(node_id) {
                                    node.language
                                } else {
                                    current_lang
                                };
                                
                                egui::ComboBox::from_id_source(format!("node_language_{}", node_id.0))
                                    .selected_text(format!("{:?}", current_lang_display))
                                    .show_ui(ui, |ui| {
                                        use crate::core::node_graph::NodeLanguage;
                                        for lang in [
                                            NodeLanguage::Auto,
                                            NodeLanguage::Rust,
                                            NodeLanguage::Python,
                                            NodeLanguage::Java,
                                            NodeLanguage::Asm,
                                            NodeLanguage::Text,
                                        ].iter() {
                                            let is_selected = current_lang_display == *lang;
                                            let response = ui.selectable_label(is_selected, format!("{:?}", lang));
                                            if response.clicked() && !is_selected {
                                                if let Some(node) = app.graph.node_mut(node_id) {
                                                    node.language = *lang;
                                                    language_changed = true;
                                                }
                                            }
                                        }
                                    });
                            });
                        
                        ui.add_space(12.0);
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // INFORMACIÓN ADICIONAL
                        // ═══════════════════════════════════════════════════════════════════
                        ui.separator();
                        ui.add_space(6.0);
                        
                        if let Some(node) = app.graph.node(node_id) {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("📍")
                                    .size(11.0)
                                    .color(Color32::from_rgb(150, 200, 255)));
                                ui.label(egui::RichText::new("Posición:")
                                    .size(10.0)
                                    .color(Color32::from_rgb(150, 150, 170)));
                                ui.label(egui::RichText::new(format!("{:.0}, {:.0}", node.position.x, node.position.y))
                                    .monospace()
                                    .size(10.0)
                                    .color(Color32::from_rgb(200, 220, 255)));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("🔌")
                                    .size(11.0)
                                    .color(Color32::from_rgb(150, 200, 255)));
                                ui.label(egui::RichText::new("Pines:")
                                    .size(10.0)
                                    .color(Color32::from_rgb(150, 150, 170)));
                                ui.label(egui::RichText::new(format!("{} entrada(s), {} salida(s)", node.inputs.len(), node.outputs.len()))
                                    .size(10.0)
                                    .color(Color32::from_rgb(200, 220, 255)));
                            });
                            
                            // Información especial para nodos carpeta
                            if node.title.starts_with("📁 ") && node.subnetwork_graph.is_some() {
                                ui.add_space(4.0);
                                if let Some(folder_graph) = node.subnetwork_graph.as_ref() {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("📂")
                                            .size(11.0)
                                            .color(Color32::from_rgb(255, 200, 100)));
                                        ui.label(egui::RichText::new(format!("Contiene {} nodo(s)", folder_graph.nodes().len()))
                                            .size(10.0)
                                            .color(Color32::from_rgb(255, 220, 150)));
                                    });
                                }
                            }
                        }
                        
                        // Aplicar cambios después de renderizar todos los controles
                        if title_changed {
                            // Actualizar canales cuando cambia el título
                            app.update_node_channels(node_id);
                            app.check_and_auto_save();
                        }
                        
                        if color_changed || language_changed {
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

