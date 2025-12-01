use eframe::egui;
use crate::app::NodeGraphApp;
use crate::ui::layout::{LayoutStyle, LayoutConfig};

pub fn draw_view_menu(ui: &mut egui::Ui, _ctx: &egui::Context, app: &mut NodeGraphApp) {
    ui.menu_button("View", |ui| {
        // ═══════════════════════════════════════════════════════════════════
        // LAYOUT STYLE
        // ═══════════════════════════════════════════════════════════════════
        ui.menu_button("🗺️ Layout Style", |ui| {
            ui.label("Estilo de organización de nodos:");
            ui.separator();
            
            // Opción: Horizontal (Derecha)
            let is_horizontal = app.layout_config.style == LayoutStyle::Horizontal;
            if ui.selectable_label(is_horizontal, "➡️ Derecha (Trabajo Simple)").clicked() {
                app.layout_config = LayoutConfig::horizontal();
                ui.close_menu();
            }
            ui.label("   Flujo de izquierda a derecha");
            
            ui.separator();
            
            // Opción: Mapa Semántico (Abajo)
            let is_semantic = app.layout_config.style == LayoutStyle::SemanticMap;
            if ui.selectable_label(is_semantic, "⬇️ Mapa Semántico (Complejo)").clicked() {
                app.layout_config = LayoutConfig::semantic_map();
                ui.close_menu();
            }
            ui.label("   Árbol jerárquico de arriba a abajo");
            ui.label("   Ideal para CPU/GPU");
        });
        
        ui.separator();
        
        // ═══════════════════════════════════════════════════════════════════
        // AUTO-LAYOUT
        // ═══════════════════════════════════════════════════════════════════
        ui.menu_button("📐 Auto-Layout", |ui| {
            ui.label("Reorganizar nodos automáticamente:");
            ui.separator();
            
            if ui.button("🔄 Aplicar Layout Actual").clicked() {
                apply_current_layout(app);
                ui.close_menu();
            }
            
            ui.separator();
            
            if ui.button("🖥️ Layout CPU (Syscalls/Memory/Math)").clicked() {
                apply_cpu_layout(app);
                ui.close_menu();
            }
            
            if ui.button("🎮 Layout GPU (Vertex/Fragment/Compute)").clicked() {
                apply_gpu_layout(app);
                ui.close_menu();
            }
            
            if ui.button("⚡ Layout CPU + GPU Combinado").clicked() {
                apply_combined_layout(app);
                ui.close_menu();
            }
        });
        
        ui.separator();
        
        // ═══════════════════════════════════════════════════════════════════
        // VIEW CONTROLS
        // ═══════════════════════════════════════════════════════════════════
        if ui.button("🔍 Reset View").clicked() {
            app.viewport = crate::ui::viewport::Viewport2D::default();
            ui.close_menu();
        }
        
        if ui.button("🎯 Focus All Nodes").clicked() {
            focus_all_nodes(app);
            ui.close_menu();
        }
        
        ui.separator();
        ui.label(format!("Zoom: {:.0}%", app.viewport.zoom * 100.0));
        ui.label(format!("Layout: {}", app.layout_config.style.name()));
    });
}

/// Aplica el layout actual a todos los nodos
fn apply_current_layout(app: &mut NodeGraphApp) {
    let node_count = app.graph.nodes().len();
    if node_count == 0 { return; }
    
    let positions = match app.layout_config.style {
        LayoutStyle::Horizontal => {
            crate::ui::layout::calculate_horizontal_positions(
                node_count, 
                &app.layout_config,
                (node_count as f32).sqrt().ceil() as usize
            )
        }
        LayoutStyle::SemanticMap => {
            // Calcular niveles basados en conexiones
            let levels = calculate_node_levels(app);
            crate::ui::layout::calculate_semantic_map_positions(&levels, &app.layout_config)
        }
    };
    
    // Aplicar posiciones a los nodos
    apply_positions_to_nodes(app, &positions);
}

/// Aplica layout específico para CPU templates
fn apply_cpu_layout(app: &mut NodeGraphApp) {
    let node_count = app.graph.nodes().len();
    if node_count == 0 { return; }
    
    let mut config = LayoutConfig::semantic_map();
    config.start_x = 300.0;
    
    // Organizar en 4 niveles: Root, Syscalls, Memory/Math, Output
    let syscalls = (node_count / 3).max(1);
    let memory = (node_count / 3).max(1);
    let math = node_count.saturating_sub(syscalls).saturating_sub(memory);
    
    let levels = vec![1, syscalls.max(1), memory.max(1), math.max(1)];
    let positions = crate::ui::layout::calculate_semantic_map_positions(&levels, &config);
    
    apply_positions_to_nodes(app, &positions);
}

/// Aplica layout específico para GPU templates
fn apply_gpu_layout(app: &mut NodeGraphApp) {
    let node_count = app.graph.nodes().len();
    if node_count == 0 { return; }
    
    let mut config = LayoutConfig::semantic_map();
    config.start_x = 400.0;
    
    // Organizar en niveles: Root, Vertex, Fragment, Compute, Output
    let per_level = (node_count / 4).max(1);
    let levels = vec![1, per_level, per_level, per_level, 1];
    let positions = crate::ui::layout::calculate_semantic_map_positions(&levels, &config);
    
    apply_positions_to_nodes(app, &positions);
}

/// Aplica layout combinado CPU + GPU lado a lado
fn apply_combined_layout(app: &mut NodeGraphApp) {
    let node_count = app.graph.nodes().len();
    if node_count == 0 { return; }
    
    let half = node_count / 2;
    let (cpu_positions, gpu_positions) = crate::ui::layout::combined_cpu_gpu_layout(
        half.max(1), 
        (node_count - half).max(1)
    );
    
    // Combinar posiciones
    let mut positions = cpu_positions;
    positions.extend(gpu_positions);
    
    apply_positions_to_nodes(app, &positions);
}

/// Calcula los niveles de nodos basados en conexiones (para mapa semántico)
fn calculate_node_levels(app: &NodeGraphApp) -> Vec<usize> {
    let node_count = app.graph.nodes().len();
    if node_count == 0 { return vec![]; }
    
    // Algoritmo simple: contar niveles basados en profundidad de conexiones
    use std::collections::{HashMap, HashSet};
    
    let nodes = app.graph.nodes();
    let links = app.graph.links();
    
    // Construir mapa de conexiones
    let mut has_input: HashSet<usize> = HashSet::new();
    
    for link in links {
        if let Some(to_addr) = app.graph.locate_pin(link.to) {
            has_input.insert(to_addr.node_index);
        }
    }
    
    // Nodos raíz (sin inputs)
    let roots: Vec<usize> = (0..nodes.len())
        .filter(|i| !has_input.contains(i))
        .collect();
    
    // Calcular profundidad de cada nodo
    let mut depths: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<(usize, usize)> = roots.iter().map(|&i| (i, 0)).collect();
    
    while let Some((node_idx, depth)) = stack.pop() {
        if depths.get(&node_idx).map(|&d| d >= depth).unwrap_or(false) {
            continue;
        }
        depths.insert(node_idx, depth);
        
        // Buscar hijos
        if let Some(node) = nodes.get(node_idx) {
            for output_pin in &node.outputs {
                for link in links {
                    if link.from == output_pin.id {
                        if let Some(to_addr) = app.graph.locate_pin(link.to) {
                            stack.push((to_addr.node_index, depth + 1));
                        }
                    }
                }
            }
        }
    }
    
    // Contar nodos por nivel
    let max_depth = depths.values().max().copied().unwrap_or(0);
    let mut levels = vec![0; max_depth + 1];
    
    for &depth in depths.values() {
        levels[depth] += 1;
    }
    
    // Si hay nodos sin clasificar, agregarlos al primer nivel
    let classified = depths.len();
    if classified < node_count {
        levels[0] += node_count - classified;
    }
    
    levels
}

/// Aplica las posiciones calculadas a los nodos del grafo
fn apply_positions_to_nodes(app: &mut NodeGraphApp, positions: &[egui::Pos2]) {
    let node_ids: Vec<_> = app.graph.nodes().iter().map(|n| n.id).collect();
    
    for (idx, &pos) in positions.iter().enumerate() {
        if let Some(node_id) = node_ids.get(idx) {
            if let Some(node) = app.graph.node_mut(*node_id) {
                node.position = pos;
            }
        }
    }
}

/// Centra la vista en todos los nodos
fn focus_all_nodes(app: &mut NodeGraphApp) {
    let nodes = app.graph.nodes();
    if nodes.is_empty() { return; }
    
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    
    for node in nodes {
        min_x = min_x.min(node.position.x);
        min_y = min_y.min(node.position.y);
        max_x = max_x.max(node.position.x + 150.0); // Estimado de ancho del nodo
        max_y = max_y.max(node.position.y + 80.0);  // Estimado de alto del nodo
    }
    
    let bounds = egui::Rect::from_min_max(
        egui::pos2(min_x, min_y),
        egui::pos2(max_x, max_y)
    );
    
    // Usar un rect de canvas aproximado
    let canvas = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1200.0, 700.0));
    app.viewport.focus_on(bounds, canvas);
}

