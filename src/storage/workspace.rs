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
    
    /// Limpiar código embebido recursivamente de todos los nodos (para serialización)
    fn clear_embedded_code_recursive(graph: &mut NodeGraph) {
        for node in graph.nodes_mut() {
            if node.code_path.is_some() {
                // Limpiar código embebido para nodos con code_path (se carga desde archivo)
                node.code = String::new();
            }
            
            // Limpiar código de nodos dentro de subnetworks/carpetas recursivamente
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                Self::clear_embedded_code_recursive(sub_graph);
            }
        }
    }

    /// Guardar grafo recursivamente (incluye subnetworks)
    fn save_graph_recursive(&self, graph: &mut NodeGraph) -> Result<(), String> {
        let storage = NodeStorage::new(self.clone());
        
        // Asegurar que el directorio nodes/ existe
        storage.ensure_nodes_directory()?;

        // 1. Guardar código de cada nodo en archivos separados (recursivamente)
        for node in graph.nodes_mut() {
            // ═══════════════════════════════════════════════════════════════════
            // GUARDAR CÓDIGO DEL NODO ACTUAL (siempre, incluso si está vacío para asignar code_path)
            // ═══════════════════════════════════════════════════════════════════
            // Verificar si el code_path actual coincide con el ID del nodo
            let expected_path = storage.get_node_code_path_relative(node.id, node.language);
            let needs_path_update = node.code_path.as_ref()
                .map(|path| path != &expected_path)
                .unwrap_or(true);
            
            // ═══════════════════════════════════════════════════════════════════
            // IMPORTANTE: Siempre guardar el código ANTES de cualquier otra operación
            // Esto asegura que los cambios se persistan incluso si el nodo está dentro de una carpeta
            // ═══════════════════════════════════════════════════════════════════
            
            // Si el nodo tiene código, SIEMPRE guardarlo (incluso si code_path existe)
            // Esto asegura que los cambios recientes se guarden
            if !node.code.is_empty() {
                // Guardar código y obtener/actualizar code_path
                let code_path = storage.save_node_code(node.id, &node.code, node.language)?;
                node.code_path = Some(code_path);
            } else if node.code_path.is_none() {
                // Si no hay código pero tampoco code_path, asignar code_path para consistencia
                node.code_path = Some(expected_path);
            }
            // Si code_path existe pero el código está vacío, mantener el code_path
            // (el código vacío se guardará como archivo vacío si es necesario)
            
            // ═══════════════════════════════════════════════════════════════════
            // 🆕 GUARDAR SUBNETWORKS/CARPETAS RECURSIVAMENTE
            // ═══════════════════════════════════════════════════════════════════
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                // Guardar el grafo interno recursivamente (esto guarda código de nodos dentro de carpetas)
                // IMPORTANTE: Esto guarda TODOS los nodos dentro de la carpeta, incluso los modificados
                self.save_graph_recursive(sub_graph)?;
            }
        }

        // 2. Guardar node_map.json (con code_path, pero código NO embebido para serialización)
        // Necesitamos serializar sin el código embebido para nodos con code_path
        let path = self.get_node_map_path()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        // Crear una copia del grafo para serialización (sin código embebido si hay code_path)
        let mut graph_for_serialization = graph.clone();
        
        // Limpiar código embebido recursivamente (incluye nodos dentro de carpetas)
        Self::clear_embedded_code_recursive(&mut graph_for_serialization);
        
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
    
    /// Cargar código de nodos recursivamente (incluye todos los niveles de carpetas)
    fn load_node_code_recursive(graph: &mut NodeGraph, storage: &NodeStorage) {
        for node in graph.nodes_mut() {
            // Cargar código del nodo actual si tiene code_path
            if let Some(code_path) = &node.code_path.clone() {
                match storage.load_node_code(code_path) {
                    Ok(code) => {
                        node.code = code;
                    }
                    Err(e) => {
                        // Verificar si es un error de archivo no encontrado (varios formatos y idiomas)
                        let is_file_not_found = e.contains("cannot find") 
                            || e.contains("No such file") 
                            || e.contains("No se puede encontrar")
                            || e.contains("El sistema no puede encontrar")
                            || e.contains("El sistema no puede encontrar el archivo especificado")
                            || e.contains("os error 2")
                            || e.contains("os error 3")
                            || e.contains("not found")
                            || e.contains("No existe el archivo")
                            || e.contains("does not exist");
                        
                        if is_file_not_found {
                            // Archivo no encontrado: limpiar code_path silenciosamente
                            // No mostrar warning porque es un caso común (archivos eliminados manualmente, etc.)
                            node.code_path = None;
                        } else {
                            // Otro tipo de error (permisos, I/O, etc.): mostrar warning
                            eprintln!("Warning: Failed to load code from {}: {}", code_path, e);
                        }
                    }
                }
            }
            
            // Cargar código de nodos dentro de subnetworks/carpetas recursivamente
            if let Some(ref mut sub_graph) = node.subnetwork_graph {
                Self::load_node_code_recursive(sub_graph, storage);
            }
        }
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
        
        // Cargar código desde archivos separados recursivamente (incluye todos los niveles)
        let storage = NodeStorage::new(self.clone());
        Self::load_node_code_recursive(&mut graph, &storage);
        
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

