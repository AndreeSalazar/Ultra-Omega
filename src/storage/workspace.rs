use std::path::PathBuf;
use crate::core::node_graph::NodeGraph;
use super::node_storage::NodeStorage;

#[derive(Default, Clone)]
pub struct Workspace {
    pub root_path: Option<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub auto_save: bool,
}

impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_root(&mut self, path: PathBuf) {
        self.root_path = Some(path);
    }

    pub fn clear_root(&mut self) {
        self.root_path = None;
        self.current_file = None;
    }

    pub fn has_root(&self) -> bool {
        self.root_path.is_some()
    }

    pub fn get_node_map_path(&self) -> Option<PathBuf> {
        self.root_path.as_ref().map(|root| root.join("node_map.json"))
    }

    /// Guardar grafo (formato nuevo: código separado)
    /// Incluye guardado recursivo de subnetworks
    pub fn save_graph(&self, graph: &mut NodeGraph) -> Result<(), String> {
        self.save_graph_recursive(graph)
    }
    
    /// Guardar grafo recursivamente (incluye subnetworks)
    fn save_graph_recursive(&self, graph: &mut NodeGraph) -> Result<(), String> {
        let storage = NodeStorage::new(self.clone());
        
        // Asegurar que el directorio nodes/ existe
        storage.ensure_nodes_directory()?;

        // 1. Guardar código de cada nodo en archivos separados
        for node in graph.nodes_mut() {
            // Si ya tiene code_path, actualizar código en archivo
            if let Some(code_path) = &node.code_path {
                // Guardar código (siempre actualizar si hay cambios)
                let _ = storage.save_node_code(node.id, &node.code, node.language)?;
            } else if !node.code.is_empty() {
                // Formato antiguo o nuevo nodo sin code_path: guardar código y asignar path
                let code_path = storage.save_node_code(node.id, &node.code, node.language)?;
                node.code_path = Some(code_path);
            }
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 GUARDAR SUBNETWORKS RECURSIVAMENTE
            // ═══════════════════════════════════════════════════════════════════
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                // Guardar el grafo interno recursivamente
                self.save_graph_recursive(sub_graph)?;
            }
        }

        // 2. Guardar node_map.json (con code_path, pero código NO embebido para serialización)
        // Necesitamos serializar sin el código embebido para nodos con code_path
        let path = self.get_node_map_path()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        // Crear una copia del grafo para serialización (sin código embebido si hay code_path)
        let mut graph_for_serialization = graph.clone();
        for node in graph_for_serialization.nodes_mut() {
            if node.code_path.is_some() {
                // Limpiar código embebido para nodos con code_path (se carga desde archivo)
                node.code = String::new();
            }
        }
        
        let json = serde_json::to_string_pretty(&graph_for_serialization)
            .map_err(|e| format!("Failed to serialize graph: {}", e))?;
        
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    /// Cargar grafo (carga código desde archivos separados si existe code_path)
    /// Incluye carga recursiva de subnetworks
    pub fn load_graph(&self) -> Result<NodeGraph, String> {
        self.load_graph_recursive()
    }
    
    /// Cargar grafo recursivamente (incluye subnetworks)
    fn load_graph_recursive(&self) -> Result<NodeGraph, String> {
        let path = self.get_node_map_path()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        if !path.exists() {
            return Ok(NodeGraph::default());
        }
        
        let json = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let mut graph: NodeGraph = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse graph: {}", e))?;
        
        // Cargar código desde archivos separados si tienen code_path
        let storage = NodeStorage::new(self.clone());
        for node in graph.nodes_mut() {
            if let Some(code_path) = &node.code_path {
                // Cargar código desde archivo separado
                match storage.load_node_code(code_path) {
                    Ok(code) => {
                        node.code = code;
                    }
                    Err(e) => {
                        // Si falla, limpiar code_path si el archivo no existe
                        // Esto evita warnings repetidos para archivos eliminados
                        if e.contains("cannot find") || e.contains("No such file") || e.contains("os error 3") {
                            node.code_path = None;
                        } else {
                            // Solo mostrar warning si es otro tipo de error
                            eprintln!("Warning: Failed to load code from {}: {}", code_path, e);
                        }
                    }
                }
            }
            // Si no hay code_path, el código ya está en node.code (formato antiguo o sin código)
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 CARGAR SUBNETWORKS RECURSIVAMENTE
            // ═══════════════════════════════════════════════════════════════════
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                // Cargar el grafo interno recursivamente
                // Nota: Los subnetworks se guardan en el mismo node_map.json, así que
                // ya están cargados desde la deserialización. Solo necesitamos cargar
                // el código de sus nodos.
                for sub_node in sub_graph.nodes_mut() {
                    if let Some(sub_code_path) = &sub_node.code_path {
                        match storage.load_node_code(sub_code_path) {
                            Ok(code) => {
                                sub_node.code = code;
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to load subnetwork code from {}: {}", sub_code_path, e);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(graph)
    }

    pub fn get_folder_name(&self) -> Option<String> {
        self.root_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }

    pub fn list_files(&self) -> Result<Vec<(String, bool)>, String> {
        let root = self.root_path.as_ref()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        let mut items = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let is_dir = metadata.is_dir();
                        items.push((name, is_dir));
                    }
                }
            }
        }
        
        items.sort_by(|a, b| {
            match (a.1, b.1) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.cmp(&b.0),
            }
        });
        
        Ok(items)
    }
}

