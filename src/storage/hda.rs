/* ═══════════════════════════════════════════════════════════════════════════
 * HDA (Houdini Digital Asset) - Sistema de Assets Exportables
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * Este módulo implementa un sistema similar a los HDAs de Houdini:
 * - Exportar grupos de nodos como assets reutilizables
 * - Importar assets en otros proyectos
 * - Parámetros configurables expuestos
 * - Documentación integrada
 * ═══════════════════════════════════════════════════════════════════════════
 */

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::core::node_graph::{NodeGraph, Node, NodeId, PinId, PinKind};

// ═══════════════════════════════════════════════════════════════════════════
// ESTRUCTURAS DE DATOS
// ═══════════════════════════════════════════════════════════════════════════

/// Parámetro expuesto de un HDA
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HDAParameter {
    pub name: String,
    pub label: String,
    pub description: String,
    pub param_type: ParameterType,
    pub default_value: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub choices: Option<Vec<String>>, // Para parámetros de tipo enum/dropdown
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParameterType {
    Float,
    Int,
    String,
    Bool,
    Enum,
    Vector3,
    Color,
}

impl ParameterType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ParameterType::Float => "Float",
            ParameterType::Int => "Int",
            ParameterType::String => "String",
            ParameterType::Bool => "Bool",
            ParameterType::Enum => "Enum",
            ParameterType::Vector3 => "Vector3",
            ParameterType::Color => "Color",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Float" => Some(ParameterType::Float),
            "Int" => Some(ParameterType::Int),
            "String" => Some(ParameterType::String),
            "Bool" => Some(ParameterType::Bool),
            "Enum" => Some(ParameterType::Enum),
            "Vector3" => Some(ParameterType::Vector3),
            "Color" => Some(ParameterType::Color),
            _ => None,
        }
    }
    
    pub fn all_types() -> Vec<Self> {
        vec![
            ParameterType::Float,
            ParameterType::Int,
            ParameterType::String,
            ParameterType::Bool,
            ParameterType::Enum,
            ParameterType::Vector3,
            ParameterType::Color,
        ]
    }
}

/// HDA (Houdini Digital Asset) - Asset exportable
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HDA {
    /// Nombre único del asset
    pub name: String,
    
    /// Etiqueta visible para el usuario
    pub label: String,
    
    /// Descripción del asset
    pub description: String,
    
    /// Versión del asset (formato semver)
    pub version: String,
    
    /// Autor del asset
    pub author: String,
    
    /// Categoría del asset (para organización)
    pub category: String,
    
    /// Tags para búsqueda
    pub tags: Vec<String>,
    
    /// Grafo de nodos que constituye este asset
    pub graph: NodeGraph,
    
    /// Parámetros expuestos (configurables desde fuera)
    pub parameters: Vec<HDAParameter>,
    
    /// Pines de entrada expuestos
    pub exposed_inputs: Vec<String>,
    
    /// Pines de salida expuestos
    pub exposed_outputs: Vec<String>,
    
    /// Documentación en formato Markdown
    pub documentation: String,
    
    /// Icono (emoji o nombre de archivo)
    pub icon: String,
    
    /// Fecha de creación
    pub created_at: String,
    
    /// Fecha de última modificación
    pub updated_at: String,
}

impl Default for HDA {
    fn default() -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        HDA {
            name: String::new(),
            label: String::new(),
            description: String::new(),
            version: "1.0.0".to_string(),
            author: String::new(),
            category: "General".to_string(),
            tags: Vec::new(),
            graph: NodeGraph::default(),
            parameters: Vec::new(),
            exposed_inputs: Vec::new(),
            exposed_outputs: Vec::new(),
            documentation: String::new(),
            icon: "📦".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GESTIÓN DE HDAs
// ═══════════════════════════════════════════════════════════════════════════

pub struct HDAManager {
    assets_dir: PathBuf,
}

impl HDAManager {
    /// Crear un nuevo gestor de HDAs
    pub fn new(workspace_root: &Path) -> Self {
        let assets_dir = workspace_root.join(".ultra-omega").join("assets");
        HDAManager { assets_dir }
    }
    
    /// Obtener directorio de assets global (para assets compartidos)
    pub fn global_assets_dir() -> Option<PathBuf> {
        // En Windows: %APPDATA%/Ultra-Omega/assets
        // En Linux/Mac: ~/.ultra-omega/assets
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA")
                .ok()
                .map(|appdata| PathBuf::from(appdata).join("Ultra-Omega").join("assets"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("HOME")
                .ok()
                .map(|home| PathBuf::from(home).join(".ultra-omega").join("assets"))
        }
    }
    
    /// Asegurar que el directorio de assets existe
    pub fn ensure_assets_dir(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.assets_dir)
            .map_err(|e| format!("Failed to create assets directory: {}", e))?;
        Ok(())
    }
    
    /// Exportar un grafo o subgrafo como HDA
    pub fn export_hda(
        &self,
        hda: &HDA,
        export_to_global: bool,
    ) -> Result<PathBuf, String> {
        self.ensure_assets_dir()?;
        
        let assets_dir = if export_to_global {
            Self::global_assets_dir()
                .ok_or_else(|| "Failed to get global assets directory".to_string())?
        } else {
            self.assets_dir.clone()
        };
        
        // Crear directorio para este asset
        let asset_dir = assets_dir.join(&hda.name);
        std::fs::create_dir_all(&asset_dir)
            .map_err(|e| format!("Failed to create asset directory: {}", e))?;
        
        // Guardar metadatos del HDA
        let metadata_path = asset_dir.join("asset.json");
        let metadata_json = serde_json::to_string_pretty(hda)
            .map_err(|e| format!("Failed to serialize HDA: {}", e))?;
        std::fs::write(&metadata_path, metadata_json)
            .map_err(|e| format!("Failed to write asset.json: {}", e))?;
        
        // Guardar código de nodos
        let nodes_dir = asset_dir.join("nodes");
        std::fs::create_dir_all(&nodes_dir)
            .map_err(|e| format!("Failed to create nodes directory: {}", e))?;
        
        for node in hda.graph.nodes() {
            if !node.code.is_empty() {
                let extension = match node.language {
                    crate::core::node_graph::NodeLanguage::Rust => "rs",
                    crate::core::node_graph::NodeLanguage::C => "c",
                    crate::core::node_graph::NodeLanguage::Cpp => "cpp",
                    crate::core::node_graph::NodeLanguage::Asm => "asm",
                    crate::core::node_graph::NodeLanguage::Zig => "zig",
                    _ => "txt",
                };
                
                let code_file = nodes_dir.join(format!("node_{:06}.{}", node.id.0, extension));
                std::fs::write(&code_file, &node.code)
                    .map_err(|e| format!("Failed to write node code: {}", e))?;
            }
            
            // Guardar subnetworks recursivamente si existen
            if let Some(ref sub_graph) = node.subnetwork_graph {
                self.export_subgraph_recursive(sub_graph, &nodes_dir, &format!("node_{:06}", node.id.0))?;
            }
        }
        
        // Guardar documentación si existe
        if !hda.documentation.is_empty() {
            let doc_path = asset_dir.join("README.md");
            std::fs::write(&doc_path, &hda.documentation)
                .map_err(|e| format!("Failed to write documentation: {}", e))?;
        }
        
        Ok(asset_dir)
    }
    
    /// Función auxiliar para exportar subgrafos recursivamente
    fn export_subgraph_recursive(
        &self,
        graph: &NodeGraph,
        base_dir: &Path,
        prefix: &str,
    ) -> Result<(), String> {
        let subnodes_dir = base_dir.join(prefix);
        std::fs::create_dir_all(&subnodes_dir)
            .map_err(|e| format!("Failed to create subnodes directory: {}", e))?;
        
        for node in graph.nodes() {
            if !node.code.is_empty() {
                let extension = match node.language {
                    crate::core::node_graph::NodeLanguage::Rust => "rs",
                    crate::core::node_graph::NodeLanguage::C => "c",
                    crate::core::node_graph::NodeLanguage::Cpp => "cpp",
                    crate::core::node_graph::NodeLanguage::Asm => "asm",
                    crate::core::node_graph::NodeLanguage::Zig => "zig",
                    _ => "txt",
                };
                
                let code_file = subnodes_dir.join(format!("node_{:06}.{}", node.id.0, extension));
                std::fs::write(&code_file, &node.code)
                    .map_err(|e| format!("Failed to write subnode code: {}", e))?;
            }
            
            if let Some(ref sub_graph) = node.subnetwork_graph {
                self.export_subgraph_recursive(sub_graph, &subnodes_dir, &format!("node_{:06}", node.id.0))?;
            }
        }
        
        Ok(())
    }
    
    /// Importar un HDA desde un directorio
    pub fn import_hda(&self, asset_path: &Path) -> Result<HDA, String> {
        let metadata_path = asset_path.join("asset.json");
        
        if !metadata_path.exists() {
            return Err(format!("asset.json not found in {}", asset_path.display()));
        }
        
        let metadata_json = std::fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read asset.json: {}", e))?;
        
        let mut hda: HDA = serde_json::from_str(&metadata_json)
            .map_err(|e| format!("Failed to parse asset.json: {}", e))?;
        
        // Cargar código de nodos
        let nodes_dir = asset_path.join("nodes");
        if nodes_dir.exists() {
            self.load_node_code_recursive(&mut hda.graph, &nodes_dir)?;
        }
        
        // Cargar documentación si existe
        let doc_path = asset_path.join("README.md");
        if doc_path.exists() {
            hda.documentation = std::fs::read_to_string(&doc_path)
                .unwrap_or_default();
        }
        
        Ok(hda)
    }
    
    /// Función auxiliar para cargar código de nodos recursivamente
    fn load_node_code_recursive(
        &self,
        graph: &mut NodeGraph,
        base_dir: &Path,
    ) -> Result<(), String> {
        for node in graph.nodes_mut() {
            // Buscar archivo de código para este nodo
            let pattern = format!("node_{:06}.*", node.id.0);
            
            if let Ok(entries) = std::fs::read_dir(base_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        
                        if file_name_str.starts_with(&format!("node_{:06}.", node.id.0)) {
                            let code = std::fs::read_to_string(entry.path())
                                .map_err(|e| format!("Failed to read node code: {}", e))?;
                            node.code = code;
                            
                            // Determinar extensión para el code_path
                            if let Some(extension) = entry.path().extension() {
                                let ext_str = extension.to_string_lossy();
                                let relative_path = format!("nodes/node_{:06}.{}", node.id.0, ext_str);
                                node.code_path = Some(relative_path);
                            }
                            break;
                        }
                    }
                }
            }
            
            // Cargar subnetworks recursivamente
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                let subnodes_dir = base_dir.join(&format!("node_{:06}", node.id.0));
                if subnodes_dir.exists() {
                    self.load_node_code_recursive(sub_graph, &subnodes_dir)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Listar todos los HDAs disponibles (locales y globales)
    pub fn list_available_hdas(&self) -> Vec<(PathBuf, HDAInfo)> {
        let mut hdas = Vec::new();
        
        // Buscar en assets locales
        if let Ok(entries) = std::fs::read_dir(&self.assets_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(info) = self.get_hda_info(&path) {
                            hdas.push((path, info));
                        }
                    }
                }
            }
        }
        
        // Buscar en assets globales
        if let Some(global_dir) = Self::global_assets_dir() {
            if let Ok(entries) = std::fs::read_dir(&global_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Ok(info) = self.get_hda_info(&path) {
                                hdas.push((path, info));
                            }
                        }
                    }
                }
            }
        }
        
        hdas
    }
    
    /// Obtener información básica de un HDA sin cargarlo completamente
    pub fn get_hda_info(&self, asset_path: &Path) -> Result<HDAInfo, String> {
        let metadata_path = asset_path.join("asset.json");
        
        if !metadata_path.exists() {
            return Err("asset.json not found".to_string());
        }
        
        let metadata_json = std::fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read asset.json: {}", e))?;
        
        let hda: HDA = serde_json::from_str(&metadata_json)
            .map_err(|e| format!("Failed to parse asset.json: {}", e))?;
        
        Ok(HDAInfo {
            name: hda.name,
            label: hda.label,
            description: hda.description,
            version: hda.version,
            author: hda.author,
            category: hda.category,
            tags: hda.tags,
            icon: hda.icon,
            node_count: hda.graph.nodes().len(),
            parameter_count: hda.parameters.len(),
            is_global: Self::global_assets_dir()
                .map(|g| asset_path.starts_with(&g))
                .unwrap_or(false),
        })
    }
}

/// Información básica de un HDA (para listado sin cargar completo)
#[derive(Clone, Debug)]
pub struct HDAInfo {
    pub name: String,
    pub label: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub icon: String,
    pub node_count: usize,
    pub parameter_count: usize,
    pub is_global: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES PARA CREAR HDAs DESDE NODOS
// ═══════════════════════════════════════════════════════════════════════════

/// Crear un HDA desde un subgrafo o conjunto de nodos seleccionados
pub fn create_hda_from_nodes(
    graph: &NodeGraph,
    node_ids: &[NodeId],
    name: String,
    label: String,
    description: String,
) -> Result<HDA, String> {
    if node_ids.is_empty() {
        return Err("No nodes selected for HDA".to_string());
    }
    
    // Crear un nuevo grafo con solo los nodos seleccionados
    let mut hda_graph = NodeGraph::default();
    let mut id_mapping: std::collections::HashMap<NodeId, NodeId> = std::collections::HashMap::new();
    
    // Copiar nodos seleccionados
    for &node_id in node_ids {
        if let Some(node) = graph.node(node_id) {
            let input_labels: Vec<&str> = node.inputs.iter().map(|p| p.label.as_str()).collect();
            let output_labels: Vec<&str> = node.outputs.iter().map(|p| p.label.as_str()).collect();
            let new_id = hda_graph.add_node(
                node.title.clone(),
                node.position,
                node.color,
                &input_labels,
                &output_labels,
                node.language,
            );
            id_mapping.insert(node_id, new_id);
            
            // Copiar código y subnetwork
            if let Some(new_node) = hda_graph.node_mut(new_id) {
                new_node.code = node.code.clone();
                if let Some(ref sub_graph) = node.subnetwork_graph {
                    new_node.subnetwork_graph = Some(sub_graph.clone());
                }
            }
        }
    }
    
    // Copiar links entre nodos seleccionados
    // Primero necesitamos crear un mapeo de PinId antiguo -> PinId nuevo
    let mut pin_id_mapping: std::collections::HashMap<PinId, PinId> = std::collections::HashMap::new();
    
    for &node_id in node_ids {
        if let (Some(node), Some(&new_node_id)) = (graph.node(node_id), id_mapping.get(&node_id)) {
            if let Some(new_node) = hda_graph.node(new_node_id) {
                // Mapear inputs
                for (i, input) in node.inputs.iter().enumerate() {
                    if let Some(new_input) = new_node.inputs.get(i) {
                        pin_id_mapping.insert(input.id, new_input.id);
                    }
                }
                // Mapear outputs
                for (i, output) in node.outputs.iter().enumerate() {
                    if let Some(new_output) = new_node.outputs.get(i) {
                        pin_id_mapping.insert(output.id, new_output.id);
                    }
                }
            }
        }
    }
    
    // Ahora copiar links
    for link in graph.links() {
        if let (Some(new_from_pin), Some(new_to_pin)) = (pin_id_mapping.get(&link.from), pin_id_mapping.get(&link.to)) {
            hda_graph.add_link(*new_from_pin, *new_to_pin, link.color);
        }
    }
    
    // Detectar pines expuestos (inputs sin conexión externa, outputs usados externamente)
    let mut exposed_inputs = Vec::new();
    let mut exposed_outputs = Vec::new();
    
    for &node_id in node_ids {
        if let Some(node) = graph.node(node_id) {
            // Inputs: si no tienen conexión desde fuera del conjunto seleccionado
            for input in &node.inputs {
                let has_external_connection = graph.links().iter().any(|l| {
                    if l.to == input.id {
                        // Encontrar el nodo que tiene el pin de entrada (link.to)
                        if let Some(to_addr) = graph.locate_pin(l.to) {
                            let to_node_id = graph.nodes()[to_addr.node_index].id;
                            if to_node_id == node_id {
                                // Verificar si el pin de origen está fuera del conjunto seleccionado
                                if let Some(from_addr) = graph.locate_pin(l.from) {
                                    let from_node_id = graph.nodes()[from_addr.node_index].id;
                                    return !node_ids.contains(&from_node_id);
                                }
                            }
                        }
                    }
                    false
                });
                
                if !has_external_connection {
                    exposed_inputs.push(input.label.clone());
                }
            }
            
            // Outputs: si tienen conexión fuera del conjunto seleccionado
            for output in &node.outputs {
                let has_external_connection = graph.links().iter().any(|l| {
                    if l.from == output.id {
                        // Encontrar el nodo que tiene el pin de salida (link.from)
                        if let Some(from_addr) = graph.locate_pin(l.from) {
                            let from_node_id = graph.nodes()[from_addr.node_index].id;
                            if from_node_id == node_id {
                                // Verificar si el pin de destino está fuera del conjunto seleccionado
                                if let Some(to_addr) = graph.locate_pin(l.to) {
                                    let to_node_id = graph.nodes()[to_addr.node_index].id;
                                    return !node_ids.contains(&to_node_id);
                                }
                            }
                        }
                    }
                    false
                });
                
                if has_external_connection {
                    exposed_outputs.push(output.label.clone());
                }
            }
        }
    }
    
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    Ok(HDA {
        name: name.clone(),
        label: if label.is_empty() { name } else { label },
        description,
        version: "1.0.0".to_string(),
        author: String::new(),
        category: "General".to_string(),
        tags: Vec::new(),
        graph: hda_graph,
        parameters: Vec::new(),
        exposed_inputs,
        exposed_outputs,
        documentation: String::new(),
        icon: "📦".to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Crear un HDA desde un subnetwork (nodo que contiene un grafo)
pub fn create_hda_from_subnetwork(
    node: &Node,
) -> Result<HDA, String> {
    let sub_graph = node.subnetwork_graph.as_ref()
        .ok_or_else(|| "Node is not a subnetwork".to_string())?;
    
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    Ok(HDA {
        name: node.title.clone().replace(" ", "_").to_lowercase(),
        label: node.title.clone(),
        description: format!("HDA exported from subnetwork: {}", node.title),
        version: "1.0.0".to_string(),
        author: String::new(),
        category: "Subnetwork".to_string(),
        tags: vec!["subnetwork".to_string()],
        graph: sub_graph.clone(),
        parameters: Vec::new(),
        exposed_inputs: node.exposed_inputs.iter().map(|p| p.name.clone()).collect(),
        exposed_outputs: node.exposed_outputs.iter().map(|p| p.name.clone()).collect(),
        documentation: String::new(),
        icon: "📁".to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}

