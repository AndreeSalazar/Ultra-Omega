// ═══════════════════════════════════════════════════════════════════════════════
// Node Storage: Gestión de código fuente separado
// Guarda y carga el código de los nodos en archivos separados
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use crate::core::node_graph::{NodeId, NodeLanguage};
use super::workspace::Workspace;

/// Maneja el almacenamiento de código fuente de nodos en archivos separados
pub struct NodeStorage {
    workspace: Workspace,
}

impl NodeStorage {
    pub fn new(workspace: Workspace) -> Self {
        Self { workspace }
    }

    /// Obtener la ruta completa para el código de un nodo
    pub fn get_node_code_path(&self, node_id: NodeId, language: NodeLanguage) -> Option<PathBuf> {
        self.workspace.root_path.as_ref().map(|root| {
            root.join("nodes").join(format!("node_{:06}.{}", node_id.0, Self::get_file_extension(language)))
        })
    }

    /// Obtener la ruta relativa (desde el workspace) para el código de un nodo
    pub fn get_node_code_path_relative(&self, node_id: NodeId, language: NodeLanguage) -> String {
        format!("nodes/node_{:06}.{}", node_id.0, Self::get_file_extension(language))
    }

    /// Guardar código de un nodo en archivo separado
    pub fn save_node_code(&self, node_id: NodeId, code: &str, language: NodeLanguage) -> Result<String, String> {
        let root = self.workspace.root_path.as_ref()
            .ok_or_else(|| "No workspace root set".to_string())?;

        // Asegurar que el directorio nodes/ existe
        let nodes_dir = root.join("nodes");
        std::fs::create_dir_all(&nodes_dir)
            .map_err(|e| format!("Failed to create nodes directory: {}", e))?;

        // Generar ruta del archivo
        let relative_path = self.get_node_code_path_relative(node_id, language);
        let full_path = root.join(&relative_path);

        // Guardar código
        std::fs::write(&full_path, code)
            .map_err(|e| format!("Failed to write node code file: {}", e))?;

        Ok(relative_path)
    }

    /// Cargar código de un nodo desde archivo separado
    pub fn load_node_code(&self, code_path: &str) -> Result<String, String> {
        let root = self.workspace.root_path.as_ref()
            .ok_or_else(|| "No workspace root set".to_string())?;

        let full_path = root.join(code_path);

        // Validar que la ruta está dentro del workspace (seguridad)
        if !full_path.starts_with(root) {
            return Err("Invalid code path: outside workspace".to_string());
        }

        std::fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read node code file: {}", e))
    }

    /// Eliminar archivo de código de un nodo
    pub fn delete_node_code(&self, code_path: &str) -> Result<(), String> {
        let root = self.workspace.root_path.as_ref()
            .ok_or_else(|| "No workspace root set".to_string())?;

        let full_path = root.join(code_path);

        // Validar que la ruta está dentro del workspace
        if !full_path.starts_with(root) {
            return Err("Invalid code path: outside workspace".to_string());
        }

        if full_path.exists() {
            std::fs::remove_file(&full_path)
                .map_err(|e| format!("Failed to delete node code file: {}", e))?;
        }

        Ok(())
    }

    /// Obtener extensión de archivo según el lenguaje
    fn get_file_extension(language: NodeLanguage) -> &'static str {
        match language {
            NodeLanguage::Rust => "rs",
            NodeLanguage::Asm => "asm",
            NodeLanguage::Java => "java",
            NodeLanguage::Python => "py",
            NodeLanguage::Text => "txt",
            NodeLanguage::Auto => "txt",
        }
    }

    /// Asegurar que el directorio nodes/ existe
    pub fn ensure_nodes_directory(&self) -> Result<(), String> {
        let root = self.workspace.root_path.as_ref()
            .ok_or_else(|| "No workspace root set".to_string())?;

        let nodes_dir = root.join("nodes");
        std::fs::create_dir_all(&nodes_dir)
            .map_err(|e| format!("Failed to create nodes directory: {}", e))?;

        Ok(())
    }

    /// Verificar si un archivo de código existe
    pub fn code_file_exists(&self, code_path: &str) -> bool {
        if let Some(root) = &self.workspace.root_path {
            let full_path = root.join(code_path);
            full_path.exists() && full_path.is_file()
        } else {
            false
        }
    }
}

