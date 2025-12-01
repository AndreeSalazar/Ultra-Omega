// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Layout para Nodos
// Estilos de visualización: Derecha (trabajo simple) y Mapa Semántico (complejo)
// ═══════════════════════════════════════════════════════════════════════════════

use eframe::egui::{Pos2, pos2};
use serde::{Deserialize, Serialize};

/// Estilo de layout para organización de nodos
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LayoutStyle {
    /// Layout horizontal: flujo de izquierda a derecha
    /// Ideal para trabajo simple, nodos individuales
    #[default]
    Horizontal,
    
    /// Mapa Semántico: flujo de arriba hacia abajo en árbol jerárquico
    /// Ideal para trabajo complejo con múltiples ramas (CPU/GPU)
    SemanticMap,
}

impl LayoutStyle {
    pub fn name(&self) -> &'static str {
        match self {
            LayoutStyle::Horizontal => "➡️ Derecha (Trabajo Simple)",
            LayoutStyle::SemanticMap => "⬇️ Mapa Semántico (Complejo)",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            LayoutStyle::Horizontal => "Nodos fluyen de izquierda a derecha. Ideal para trabajo lineal.",
            LayoutStyle::SemanticMap => "Árbol jerárquico de arriba a abajo. Ideal para CPU/GPU y proyectos complejos.",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            LayoutStyle::Horizontal => "⏩",
            LayoutStyle::SemanticMap => "🗺️",
        }
    }
}

/// Configuración del layout
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub style: LayoutStyle,
    pub node_spacing_x: f32,
    pub node_spacing_y: f32,
    pub group_spacing: f32,
    pub start_x: f32,
    pub start_y: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            style: LayoutStyle::Horizontal,
            node_spacing_x: 250.0,
            node_spacing_y: 120.0,
            group_spacing: 80.0,
            start_x: 100.0,
            start_y: 100.0,
        }
    }
}

impl LayoutConfig {
    /// Configuración para Mapa Semántico
    pub fn semantic_map() -> Self {
        Self {
            style: LayoutStyle::SemanticMap,
            node_spacing_x: 180.0,
            node_spacing_y: 100.0,
            group_spacing: 120.0,
            start_x: 400.0,  // Centrado
            start_y: 50.0,
        }
    }
    
    /// Configuración para layout Horizontal
    pub fn horizontal() -> Self {
        Self {
            style: LayoutStyle::Horizontal,
            node_spacing_x: 250.0,
            node_spacing_y: 100.0,
            group_spacing: 80.0,
            start_x: 100.0,
            start_y: 100.0,
        }
    }
}

/// Calcula posiciones para layout horizontal (flujo a la derecha)
pub fn calculate_horizontal_positions(
    node_count: usize,
    config: &LayoutConfig,
    columns: usize,
) -> Vec<Pos2> {
    let mut positions = Vec::with_capacity(node_count);
    let cols = columns.max(1);
    
    for i in 0..node_count {
        let col = i / cols;
        let row = i % cols;
        
        let x = config.start_x + (col as f32) * config.node_spacing_x;
        let y = config.start_y + (row as f32) * config.node_spacing_y;
        
        positions.push(pos2(x, y));
    }
    
    positions
}

/// Calcula posiciones para mapa semántico (árbol jerárquico hacia abajo)
/// 
/// ```text
///              [Root]
///            /   |   \
///        [A]   [B]   [C]
///         |     |     |
///        [D]   [E]   [F]
///         \     |     /
///           [Merge]
/// ```
pub fn calculate_semantic_map_positions(
    levels: &[usize],  // Número de nodos por nivel
    config: &LayoutConfig,
) -> Vec<Pos2> {
    let mut positions = Vec::new();
    
    // Encontrar el nivel más ancho para calcular el centro
    let max_width = levels.iter().max().copied().unwrap_or(1) as f32;
    let total_width = max_width * config.node_spacing_x;
    let center_x = config.start_x;
    
    let mut y = config.start_y;
    
    for &nodes_in_level in levels {
        if nodes_in_level == 0 {
            y += config.node_spacing_y;
            continue;
        }
        
        // Calcular posiciones centradas para este nivel
        let level_width = (nodes_in_level as f32 - 1.0) * config.node_spacing_x;
        let start_x = center_x - level_width / 2.0;
        
        for i in 0..nodes_in_level {
            let x = start_x + (i as f32) * config.node_spacing_x;
            positions.push(pos2(x, y));
        }
        
        y += config.node_spacing_y;
    }
    
    positions
}

/// Estructura para definir grupos en el mapa semántico
#[derive(Clone, Debug)]
pub struct SemanticGroup {
    pub name: String,
    pub color: (u8, u8, u8),
    pub node_indices: Vec<usize>,
}

/// Calcula posiciones para mapa semántico con grupos (CPU/GPU)
/// 
/// ```text
///           [CPU Root]        [GPU Root]
///               |                  |
///     +---------+--------+    +----+----+
///     |         |        |    |         |
/// [syscall] [memory] [math]  [vert] [frag]
///     |         |        |    |         |
///     +---------+--------+    +----+----+
///               |                  |
///           [CPU Out]          [GPU Out]
///               \                  /
///                \                /
///                 +------+-------+
///                        |
///                    [FINAL]
/// ```
pub fn calculate_grouped_semantic_map(
    groups: &[SemanticGroup],
    nodes_per_group_level: &[Vec<usize>],  // [group_idx][level] = node_count
    config: &LayoutConfig,
) -> Vec<Pos2> {
    let mut positions = Vec::new();
    let num_groups = groups.len();
    
    if num_groups == 0 {
        return positions;
    }
    
    // Calcular ancho total de todos los grupos
    let group_width = 300.0;  // Ancho estimado por grupo
    let total_groups_width = (num_groups as f32) * group_width + ((num_groups - 1) as f32) * config.group_spacing;
    let start_x = config.start_x - total_groups_width / 2.0;
    
    // Procesar cada grupo
    for (group_idx, group) in groups.iter().enumerate() {
        let group_center_x = start_x + (group_idx as f32) * (group_width + config.group_spacing) + group_width / 2.0;
        
        // Obtener niveles de este grupo
        let levels = nodes_per_group_level.get(group_idx)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        
        let mut y = config.start_y;
        
        for (level_idx, &nodes_in_level) in levels.iter().enumerate() {
            if nodes_in_level == 0 {
                y += config.node_spacing_y;
                continue;
            }
            
            let level_width = (nodes_in_level as f32 - 1.0) * config.node_spacing_x.min(group_width / nodes_in_level as f32);
            let level_start_x = group_center_x - level_width / 2.0;
            
            for i in 0..nodes_in_level {
                let x = level_start_x + (i as f32) * config.node_spacing_x.min(group_width / nodes_in_level as f32);
                positions.push(pos2(x, y));
            }
            
            y += config.node_spacing_y;
        }
    }
    
    positions
}

/// Layout predefinido para templates CPU
pub fn cpu_template_layout(num_templates: usize) -> Vec<Pos2> {
    let config = LayoutConfig::semantic_map();
    
    // Organizar en niveles:
    // Nivel 0: Título/Root (1)
    // Nivel 1: Syscalls (variable)
    // Nivel 2: Memory (variable)
    // Nivel 3: Math (variable)
    // Nivel 4: Output (1)
    
    let syscalls = (num_templates / 3).max(1);
    let memory = (num_templates / 4).max(1);
    let math = num_templates - syscalls - memory;
    
    let levels = vec![1, syscalls, memory, math.max(1), 1];
    calculate_semantic_map_positions(&levels, &config)
}

/// Layout predefinido para templates GPU
pub fn gpu_template_layout(num_templates: usize) -> Vec<Pos2> {
    let config = LayoutConfig::semantic_map();
    
    // Organizar en niveles:
    // Nivel 0: Root (1)
    // Nivel 1: Vertex shaders
    // Nivel 2: Fragment shaders
    // Nivel 3: Compute shaders
    // Nivel 4: Output (1)
    
    let vertex = (num_templates / 3).max(1);
    let fragment = (num_templates / 3).max(1);
    let compute = num_templates - vertex - fragment;
    
    let levels = vec![1, vertex, fragment, compute.max(0), 1];
    calculate_semantic_map_positions(&levels, &config)
}

/// Layout combinado CPU + GPU lado a lado
pub fn combined_cpu_gpu_layout(cpu_count: usize, gpu_count: usize) -> (Vec<Pos2>, Vec<Pos2>) {
    let mut config = LayoutConfig::semantic_map();
    
    // CPU a la izquierda
    config.start_x = 200.0;
    let cpu_levels = vec![1, (cpu_count / 2).max(1), cpu_count - (cpu_count / 2), 1];
    let cpu_positions = calculate_semantic_map_positions(&cpu_levels, &config);
    
    // GPU a la derecha
    config.start_x = 600.0;
    let gpu_levels = vec![1, (gpu_count / 2).max(1), gpu_count - (gpu_count / 2), 1];
    let gpu_positions = calculate_semantic_map_positions(&gpu_levels, &config);
    
    (cpu_positions, gpu_positions)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_horizontal_positions() {
        let config = LayoutConfig::horizontal();
        let positions = calculate_horizontal_positions(4, &config, 2);
        assert_eq!(positions.len(), 4);
    }
    
    #[test]
    fn test_semantic_map_positions() {
        let config = LayoutConfig::semantic_map();
        let levels = vec![1, 3, 2, 1];
        let positions = calculate_semantic_map_positions(&levels, &config);
        assert_eq!(positions.len(), 7);
    }
}

