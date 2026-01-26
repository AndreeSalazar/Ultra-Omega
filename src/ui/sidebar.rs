use eframe::egui::{self, ScrollArea, Color32, Stroke};
use crate::core::NodeGraphApp;
use crate::core::node_graph::NodeId;
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
                            
                            // ═══════════════════════════════════════════════════════════════════
                            // 🆕 SECCIÓN DE ARCHIVOS DETECTADOS - UI MEJORADA
                            // ═══════════════════════════════════════════════════════════════════
                            if app.file_watcher.detected_structure.is_some() {
                                let stats = app.file_watcher.get_stats();
                                let total_code_files = stats.cpp_files + stats.java_files + stats.asm_files + stats.python_files + stats.rust_files;
                                
                                if total_code_files > 0 {
                                    // Panel principal de archivos detectados
                                    egui::Frame::none()
                                        .fill(Color32::from_rgba_unmultiplied(35, 55, 45, 220))
                                        .stroke(Stroke::new(1.5, Color32::from_rgb(80, 180, 120)))
                                        .rounding(egui::Rounding::same(10.0))
                                        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                                        .shadow(egui::epaint::Shadow {
                                            offset: egui::vec2(0.0, 2.0),
                                            blur: 8.0,
                                            spread: 0.0,
                                            color: Color32::from_black_alpha(40),
                                        })
                                        .show(ui, |ui| {
                                            // Header con icono y título
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new("🔍")
                                                    .size(16.0)
                                                    .color(Color32::from_rgb(100, 255, 150)));
                                                ui.label(egui::RichText::new(format!("{} archivos detectados", total_code_files))
                                                    .size(12.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(200, 255, 220)));
                                            });
                                            
                                            ui.add_space(6.0);
                                            
                                            // Estadísticas por lenguaje en grid compacto
                                            ui.horizontal_wrapped(|ui| {
                                                if stats.cpp_files > 0 {
                                                    ui.label(egui::RichText::new(format!("© {} C++", stats.cpp_files))
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(100, 150, 255)));
                                                    ui.add_space(4.0);
                                                }
                                                if stats.java_files > 0 {
                                                    ui.label(egui::RichText::new(format!("☕ {} Java", stats.java_files))
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(237, 139, 0)));
                                                    ui.add_space(4.0);
                                                }
                                                if stats.asm_files > 0 {
                                                    ui.label(egui::RichText::new(format!("⚡ {} ASM", stats.asm_files))
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(255, 220, 100)));
                                                    ui.add_space(4.0);
                                                }
                                                if stats.python_files > 0 {
                                                    ui.label(egui::RichText::new(format!("🐍 {} Python", stats.python_files))
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(55, 118, 171)));
                                                    ui.add_space(4.0);
                                                }
                                                if stats.rust_files > 0 {
                                                    ui.label(egui::RichText::new(format!("🦀 {} Rust", stats.rust_files))
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(255, 140, 100)));
                                                }
                                            });
                                            
                                            ui.add_space(8.0);
                                            
                                            // Botones de acción
                                            ui.horizontal(|ui| {
                                                // Botón Importar
                                                if ui.add(egui::Button::new(egui::RichText::new("📥 Importar")
                                                        .size(11.0)
                                                        .color(Color32::WHITE))
                                                    .fill(Color32::from_rgb(60, 140, 100))
                                                    .rounding(egui::Rounding::same(6.0))
                                                    .min_size(egui::vec2(80.0, 26.0)))
                                                    .on_hover_text("Importar archivos como nodos en el grafo")
                                                    .clicked() {
                                                    app.import_detected_files_as_nodes();
                                                }
                                                
                                                ui.add_space(4.0);
                                                
                                                // Botón Auto-Layout
                                                if ui.add(egui::Button::new(egui::RichText::new("📐 Layout")
                                                        .size(11.0)
                                                        .color(Color32::WHITE))
                                                    .fill(Color32::from_rgb(80, 120, 180))
                                                    .rounding(egui::Rounding::same(6.0))
                                                    .min_size(egui::vec2(70.0, 26.0)))
                                                    .on_hover_text("Organizar nodos automáticamente (Tecla L)")
                                                    .clicked() {
                                                    app.apply_auto_layout();
                                                }
                                            });
                                            
                                            // Tip de teclas
                                            ui.add_space(4.0);
                                            ui.label(egui::RichText::new("💡 L = Layout | Ctrl+L = Horizontal | Ctrl+Shift+L = Árbol")
                                                .size(9.0)
                                                .color(Color32::from_rgb(140, 160, 140)));
                                        });
                                    ui.add_space(8.0);
                                }
                            }
                            
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
                        // ═══════════════════════════════════════════════════════════════════
                        // 🆕 LISTA JERÁRQUICA: Carpetas con sus archivos hijos
                        // ═══════════════════════════════════════════════════════════════════
                        let search_query = app.sidebar_search_query.to_lowercase();
                        let nodes = app.graph.nodes();
                        let links = app.graph.links();
                        
                        // Identificar carpetas (📁) y construir mapa de hijos
                        let mut folder_with_children: Vec<(NodeId, String, Color32, Vec<(NodeId, String, crate::core::node_graph::NodeLanguage, Color32)>)> = Vec::new();
                        let mut assigned_children: std::collections::HashSet<NodeId> = std::collections::HashSet::new();
                        
                        // Encontrar carpetas y sus hijos conectados
                        for node in nodes.iter() {
                            if node.title.starts_with("📁") {
                                let mut children = Vec::new();
                                
                                // Buscar nodos conectados a esta carpeta
                                for output_pin in &node.outputs {
                                    for link in links {
                                        if link.from == output_pin.id {
                                            if let Some(to_addr) = app.graph.locate_pin(link.to) {
                                                if let Some(child_node) = nodes.get(to_addr.node_index) {
                                                    if !child_node.title.starts_with("📁") {
                                                        children.push((child_node.id, child_node.title.clone(), child_node.language, child_node.color));
                                                        assigned_children.insert(child_node.id);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                folder_with_children.push((node.id, node.title.clone(), node.color, children));
                            }
                        }
                        
                        // Nodos sin carpeta asignada
                        let orphan_nodes: Vec<_> = nodes.iter()
                            .filter(|n| !n.title.starts_with("📁") && !assigned_children.contains(&n.id))
                            .map(|n| (n.id, n.title.clone(), n.language, n.color))
                            .collect();
                        
                        // Filtrar por búsqueda
                        if !search_query.is_empty() {
                            folder_with_children.retain(|(_, title, _, children)| {
                                title.to_lowercase().contains(&search_query) ||
                                children.iter().any(|(_, t, _, _)| t.to_lowercase().contains(&search_query))
                            });
                        }
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // MOSTRAR CARPETAS CON SUS HIJOS
                        // ═══════════════════════════════════════════════════════════════════
                        for (folder_id, folder_title, folder_color, children) in &folder_with_children {
                            let is_folder_selected = app.interaction.selected_nodes.contains(folder_id);
                            
                            // Calcular estadísticas de lenguajes en esta carpeta
                            let mut cpp_count = 0;
                            let mut java_count = 0;
                            let mut asm_count = 0;
                            let mut python_count = 0;
                            let mut rust_count = 0;
                            let mut other_count = 0;
                            
                            for (_, _, lang, _) in children {
                                match lang {
                                    crate::core::node_graph::NodeLanguage::Cpp => cpp_count += 1,
                                    crate::core::node_graph::NodeLanguage::Java => java_count += 1,
                                    crate::core::node_graph::NodeLanguage::Asm => asm_count += 1,
                                    crate::core::node_graph::NodeLanguage::Python => python_count += 1,
                                    crate::core::node_graph::NodeLanguage::Rust => rust_count += 1,
                                    _ => other_count += 1,
                                }
                            }
                            
                            // Header de carpeta con estadísticas
                            let folder_bg = if is_folder_selected {
                                Color32::from_rgba_unmultiplied(255, 180, 80, 200)
                            } else {
                                Color32::from_rgba_unmultiplied(80, 60, 40, 180)
                            };
                            
                            let folder_response = egui::Frame::none()
                                .fill(folder_bg)
                                .stroke(Stroke::new(1.5, Color32::from_rgb(255, 200, 100)))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // Primera fila: icono + nombre + contador
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("📁")
                                                .size(16.0)
                                                .color(Color32::from_rgb(255, 210, 100)));
                                            ui.add_space(6.0);
                                            
                                            let display_title = folder_title.trim_start_matches("📁 ");
                                            ui.label(egui::RichText::new(display_title)
                                                .size(12.0)
                                                .strong()
                                                .color(Color32::WHITE));
                                            
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(egui::RichText::new(format!("{}", children.len()))
                                                    .size(11.0)
                                                    .strong()
                                                    .color(Color32::from_rgb(255, 220, 150)));
                                            });
                                        });
                                        
                                        // Segunda fila: mini-estadísticas de lenguajes
                                        if !children.is_empty() {
                                            ui.add_space(4.0);
                                            ui.horizontal_wrapped(|ui| {
                                                ui.spacing_mut().item_spacing.x = 4.0;
                                                if cpp_count > 0 {
                                                    ui.label(egui::RichText::new(format!("©{}", cpp_count))
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(100, 150, 255)));
                                                }
                                                if java_count > 0 {
                                                    ui.label(egui::RichText::new(format!("☕{}", java_count))
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(237, 139, 0)));
                                                }
                                                if asm_count > 0 {
                                                    ui.label(egui::RichText::new(format!("⚡{}", asm_count))
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(255, 220, 100)));
                                                }
                                                if python_count > 0 {
                                                    ui.label(egui::RichText::new(format!("🐍{}", python_count))
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(55, 118, 171)));
                                                }
                                                if rust_count > 0 {
                                                    ui.label(egui::RichText::new(format!("🦀{}", rust_count))
                                                        .size(9.0)
                                                        .color(Color32::from_rgb(255, 140, 100)));
                                                }
                                            });
                                        }
                                    });
                                }).response;
                            
                            if folder_response.clicked() {
                                if !ui.input(|i| i.modifiers.ctrl) {
                                    app.interaction.selected_nodes.clear();
                                }
                                app.interaction.selected_nodes.insert(*folder_id);
                            }
                            
                            // Mostrar archivos hijos con indentación
                            if !children.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.add_space(20.0); // Indentación
                                    ui.vertical(|ui| {
                                        for (child_id, child_title, child_lang, child_color) in children {
                                            if !search_query.is_empty() && !child_title.to_lowercase().contains(&search_query) {
                                                continue;
                                            }
                                            
                                            let is_selected = app.interaction.selected_nodes.contains(child_id);
                                            
                                            let (lang_icon, lang_color) = match child_lang {
                                                crate::core::node_graph::NodeLanguage::Cpp => ("©", Color32::from_rgb(100, 150, 255)),
                                                crate::core::node_graph::NodeLanguage::Java => ("☕", Color32::from_rgb(237, 139, 0)),
                                                crate::core::node_graph::NodeLanguage::Asm => ("⚡", Color32::from_rgb(255, 220, 100)),
                                                crate::core::node_graph::NodeLanguage::Python => ("🐍", Color32::from_rgb(55, 118, 171)),
                                                crate::core::node_graph::NodeLanguage::Rust => ("🦀", Color32::from_rgb(255, 140, 100)),
                                                crate::core::node_graph::NodeLanguage::Text => ("📄", Color32::from_rgb(180, 180, 180)),
                                                crate::core::node_graph::NodeLanguage::Auto => ("⚙", Color32::from_rgb(150, 150, 150)),
                                            };
                                            
                                            let bg_color = if is_selected {
                                                Color32::from_rgba_unmultiplied(100, 150, 255, 200)
                                            } else {
                                                Color32::from_rgba_unmultiplied(45, 50, 60, 160)
                                            };
                                            
                                            let child_response = egui::Frame::none()
                                                .fill(bg_color)
                                                .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(child_color.r(), child_color.g(), child_color.b(), 100)))
                                                .rounding(egui::Rounding::same(6.0))
                                                .inner_margin(egui::Margin::symmetric(8.0, 5.0))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        // Barra de color
                                                        let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 16.0), egui::Sense::hover());
                                                        ui.painter().rect_filled(rect, 1.0, *child_color);
                                                        ui.add_space(4.0);
                                                        
                                                        ui.label(egui::RichText::new(lang_icon).size(11.0).color(lang_color));
                                                        ui.add_space(4.0);
                                                        
                                                        let display_title = if child_title.len() > 18 {
                                                            format!("{}...", &child_title.chars().take(15).collect::<String>())
                                                        } else {
                                                            child_title.clone()
                                                        };
                                                        ui.label(egui::RichText::new(&display_title)
                                                            .size(10.5)
                                                            .color(if is_selected { Color32::WHITE } else { Color32::from_rgb(200, 200, 210) }));
                                                    });
                                                }).response;
                                            
                                            if child_response.clicked() {
                                                if !ui.input(|i| i.modifiers.ctrl) {
                                                    app.interaction.selected_nodes.clear();
                                                }
                                                app.interaction.selected_nodes.insert(*child_id);
                                            }
                                            
                                            ui.add_space(2.0);
                                        }
                                    });
                                });
                            }
                            ui.add_space(6.0);
                        }
                        
                        // ═══════════════════════════════════════════════════════════════════
                        // NODOS SIN CARPETA (Huérfanos)
                        // ═══════════════════════════════════════════════════════════════════
                        let filtered_orphans: Vec<_> = if search_query.is_empty() {
                            orphan_nodes.clone()
                        } else {
                            orphan_nodes.iter()
                                .filter(|(_, title, _, _)| title.to_lowercase().contains(&search_query))
                                .cloned()
                                .collect()
                        };
                        
                        if !filtered_orphans.is_empty() {
                            ui.add_space(8.0);
                            
                            // Header para nodos sin carpeta
                            egui::Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(50, 55, 70, 180))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(120, 140, 180)))
                                .rounding(egui::Rounding::same(8.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new("📄")
                                            .size(14.0)
                                            .color(Color32::from_rgb(150, 180, 220)));
                                        ui.add_space(6.0);
                                        ui.label(egui::RichText::new("Otros nodos")
                                            .size(11.0)
                                            .strong()
                                            .color(Color32::from_rgb(180, 200, 230)));
                                        ui.label(egui::RichText::new(format!("({})", filtered_orphans.len()))
                                            .size(10.0)
                                            .color(Color32::from_rgb(140, 160, 190)));
                                    });
                                });
                            
                            ui.add_space(4.0);
                            
                            for (node_id, title, language, color) in filtered_orphans {
                                let is_selected = app.interaction.selected_nodes.contains(&node_id);
                                
                                let (lang_icon, lang_color) = match language {
                                    crate::core::node_graph::NodeLanguage::Cpp => ("©", Color32::from_rgb(100, 150, 255)),
                                    crate::core::node_graph::NodeLanguage::Java => ("☕", Color32::from_rgb(237, 139, 0)),
                                    crate::core::node_graph::NodeLanguage::Asm => ("⚡", Color32::from_rgb(255, 220, 100)),
                                    crate::core::node_graph::NodeLanguage::Python => ("🐍", Color32::from_rgb(55, 118, 171)),
                                    crate::core::node_graph::NodeLanguage::Rust => ("🦀", Color32::from_rgb(255, 140, 100)),
                                    crate::core::node_graph::NodeLanguage::Text => ("📄", Color32::from_rgb(180, 180, 180)),
                                    crate::core::node_graph::NodeLanguage::Auto => ("⚙", Color32::from_rgb(150, 150, 150)),
                                };
                                
                                let bg_color = if is_selected {
                                    Color32::from_rgba_unmultiplied(100, 150, 255, 200)
                                } else {
                                    Color32::from_rgba_unmultiplied(42, 47, 57, 140)
                                };
                                
                                let response = egui::Frame::none()
                                    .fill(bg_color)
                                    .stroke(Stroke::new(0.5, Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 80)))
                                    .rounding(egui::Rounding::same(6.0))
                                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let (rect, _) = ui.allocate_exact_size(egui::vec2(3.0, 16.0), egui::Sense::hover());
                                            ui.painter().rect_filled(rect, 1.0, color);
                                            ui.add_space(4.0);
                                            
                                            ui.label(egui::RichText::new(lang_icon).size(12.0).color(lang_color));
                                            ui.add_space(4.0);
                                            
                                            let display_title = if title.len() > 22 {
                                                format!("{}...", &title.chars().take(19).collect::<String>())
                                            } else {
                                                title.clone()
                                            };
                                            ui.label(egui::RichText::new(&display_title)
                                                .size(11.0)
                                                .color(if is_selected { Color32::WHITE } else { Color32::from_rgb(210, 210, 220) }));
                                        });
                                    }).response;
                                
                                if response.clicked() {
                                    if !ui.input(|i| i.modifiers.ctrl) {
                                        app.interaction.selected_nodes.clear();
                                    }
                                    app.interaction.selected_nodes.insert(node_id);
                                }
                                
                                ui.add_space(2.0);
                            }
                        }
                    }
                });
            });
        });
}
