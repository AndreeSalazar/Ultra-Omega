use eframe::egui;
use crate::app::NodeGraphApp;

pub fn draw_file_menu(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("File", |ui| {
        // New Operations
        if ui.add(egui::Button::new("New Text File").shortcut_text("Ctrl+N")).clicked() {
            // TODO: Implement new text file
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("New Window").shortcut_text("Ctrl+Shift+N")).clicked() {
            // TODO: Implement new window
            ui.close_menu();
        }
        
        ui.menu_button("New Window with Profile", |_ui| {
            // TODO: Submenu for profiles
        });
        
        ui.separator();
        
        // Open Operations
        if ui.add(egui::Button::new("Open File...").shortcut_text("Ctrl+O")).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Node Map", &["json"])
                .add_filter("All Files", &["*"])
                .pick_file()
            {
                if let Ok(json) = std::fs::read_to_string(&path) {
                    if let Ok(graph) = serde_json::from_str::<crate::node_graph::NodeGraph>(&json) {
                        app.graph = graph;
                        app.graph.recalculate_ids();
                        app.interaction.selected_nodes.clear();
                        app.workspace.current_file = Some(path);
                    }
                }
            }
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Open Folder...").shortcut_text("Ctrl+M Ctrl+O")).clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                app.workspace.set_root(path);
                // Auto-load node_map.json if exists
                if let Err(e) = app.load_graph_from_workspace() {
                    eprintln!("Error loading graph: {}", e);
                }
            }
            ui.close_menu();
        }
        
        if ui.button("Open Workspace from File...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Workspace", &["json"])
                .add_filter("All Files", &["*"])
                .pick_file()
            {
                if let Some(parent) = path.parent() {
                    app.workspace.set_root(parent.to_path_buf());
                    if let Err(e) = app.load_graph_from_workspace() {
                        eprintln!("Error loading workspace: {}", e);
                    }
                }
            }
            ui.close_menu();
        }
        
        ui.menu_button("Open Recent", |_ui| {
            // TODO: Submenu for recent files
        });
        
        ui.separator();
        
        // Workspace Management
        if ui.button("Add Folder to Workspace...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                // For now, we'll just set it as the root
                // In a full implementation, you'd maintain a list of folders
                app.workspace.set_root(path);
            }
            ui.close_menu();
        }
        
        if ui.button("Save Workspace As...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Workspace", &["json"])
                .set_file_name("workspace.json")
                .save_file()
            {
                // Save workspace config
                let workspace_data = serde_json::json!({
                    "root": app.workspace.root_path.as_ref().map(|p| p.to_string_lossy()),
                    "auto_save": app.workspace.auto_save
                });
                if let Ok(json) = serde_json::to_string_pretty(&workspace_data) {
                    let _ = std::fs::write(&path, json);
                    if let Some(parent) = path.parent() {
                        app.workspace.set_root(parent.to_path_buf());
                    }
                }
            }
            ui.close_menu();
        }
        
        if ui.button("Duplicate Workspace").clicked() {
            if app.workspace.has_root() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Workspace", &["json"])
                    .set_file_name("workspace_copy.json")
                    .save_file()
                {
                    // Copy workspace config
                    let workspace_data = serde_json::json!({
                        "root": app.workspace.root_path.as_ref().map(|p| p.to_string_lossy()),
                        "auto_save": app.workspace.auto_save
                    });
                    if let Ok(json) = serde_json::to_string_pretty(&workspace_data) {
                        let _ = std::fs::write(&path, json);
                    }
                }
            }
            ui.close_menu();
        }
        
        ui.separator();
        
        // Save Operations
        if ui.add(egui::Button::new("Save").shortcut_text("Ctrl+S")).clicked() {
            if app.workspace.has_root() {
                if let Err(e) = app.save_current_graph() {
                    eprintln!("Error saving: {}", e);
                }
            } else {
                // Fallback to Save As if no workspace
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Node Map", &["json"])
                    .set_file_name("node_map.json")
                    .save_file()
                {
                    if let Some(parent) = path.parent() {
                        app.workspace.set_root(parent.to_path_buf());
                        app.workspace.current_file = Some(path.clone());
                        if let Err(e) = app.save_current_graph() {
                            eprintln!("Error saving: {}", e);
                        }
                    }
                }
            }
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Save As...").shortcut_text("Ctrl+Shift+S")).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Node Map", &["json"])
                .set_file_name("node_map.json")
                .save_file()
            {
                if let Some(parent) = path.parent() {
                    app.workspace.set_root(parent.to_path_buf());
                    app.workspace.current_file = Some(path.clone());
                    if let Err(e) = app.save_current_graph() {
                        eprintln!("Error saving: {}", e);
                    }
                }
            }
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Save All").shortcut_text("Ctrl+M S")).clicked() {
            if app.workspace.has_root() {
                if let Err(e) = app.save_current_graph() {
                    eprintln!("Error saving: {}", e);
                }
            }
            ui.close_menu();
        }
        
        ui.separator();
        
        // Share and Preferences
        ui.menu_button("Share", |_ui| {
            // TODO: Submenu for share options
        });
        
        // Auto Save toggle
        ui.checkbox(&mut app.workspace.auto_save, "Auto Save");
        
        ui.menu_button("Preferences", |_ui| {
            // TODO: Submenu for preferences
        });
        
        ui.separator();
        
        // Close and Revert Operations
        if ui.button("Revert File").clicked() {
            // TODO: Implement revert file
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Close Editor").shortcut_text("Ctrl+F4")).clicked() {
            // TODO: Implement close editor
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Close Folder").shortcut_text("Ctrl+M F")).clicked() {
            app.workspace.clear_root();
            ui.close_menu();
        }
        
        if ui.add(egui::Button::new("Close Window").shortcut_text("Alt+F4")).clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            ui.close_menu();
        }
        
        ui.separator();
        
        // Show workspace status
        if app.workspace.has_root() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Workspace:");
                if let Some(root) = &app.workspace.root_path {
                    ui.label(egui::RichText::new(root.to_string_lossy()).small().color(egui::Color32::GRAY));
                }
            });
        }
        
        ui.separator();
        
        // Exit
        if ui.button("Exit").clicked() {
            // Auto-save before exit if enabled
            if app.workspace.auto_save && app.workspace.has_root() {
                let _ = app.save_current_graph();
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            ui.close_menu();
        }
    });
}

