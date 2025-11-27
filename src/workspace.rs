use std::path::PathBuf;
use crate::node_graph::NodeGraph;

#[derive(Default)]
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

    pub fn save_graph(&self, graph: &NodeGraph) -> Result<(), String> {
        let path = self.get_node_map_path()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        let json = serde_json::to_string_pretty(graph)
            .map_err(|e| format!("Failed to serialize graph: {}", e))?;
        
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    pub fn load_graph(&self) -> Result<NodeGraph, String> {
        let path = self.get_node_map_path()
            .ok_or_else(|| "No workspace root set".to_string())?;
        
        if !path.exists() {
            return Ok(NodeGraph::default());
        }
        
        let json = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let graph: NodeGraph = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse graph: {}", e))?;
        
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

