// ═══════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Nodo Carpeta
// Contenedor de trabajo y unidad de herencia
// ═══════════════════════════════════════════════════════════════════

use crate::core::node_graph::{Node, NodeGraph, NodeId, NodeLanguage, PinId, PinAddress};
use crate::core::types::{Color32, Pos2, pos2};

/// Tipo de nodo carpeta
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderNodeMode {
    /// Modo organización: solo guarda nodos, no se puede heredar
    Organization,
    /// Modo heredable: otros nodos pueden heredar su contenido
    Inheritable,
}

impl Default for FolderNodeMode {
    fn default() -> Self {
        FolderNodeMode::Organization
    }
}

/// Información sobre un nodo carpeta
#[derive(Clone, Debug)]
pub struct FolderNodeInfo {
    pub node_id: NodeId,
    pub mode: FolderNodeMode,
    pub content_graph: NodeGraph,
}

impl FolderNodeInfo {
    pub fn new(node_id: NodeId, mode: FolderNodeMode) -> Self {
        Self {
            node_id,
            mode,
            content_graph: NodeGraph::default(),
        }
    }
}

/// Funciones para manejar Nodos Carpeta
impl NodeGraph {
    /// Crear un nuevo nodo carpeta
    pub fn create_folder_node(
        &mut self,
        title: impl Into<String>,
        position: Pos2,
        mode: FolderNodeMode,
        required_language: Option<NodeLanguage>, // Lenguaje requerido para carpetas heredables
    ) -> NodeId {
        // Color especial para nodos carpeta (azul más claro)
        let color = Color32::from_rgb(100, 150, 255);
        
        let mut folder_title = title.into();
        // Marcar como nodo carpeta en el título
        if !folder_title.starts_with("📁 ") {
            folder_title = format!("📁 {}", folder_title);
        }
        
        // Determinar el lenguaje del nodo carpeta
        let folder_language = if matches!(mode, FolderNodeMode::Inheritable) {
            // Para carpetas heredables, usar el lenguaje especificado o Auto por defecto
            required_language.unwrap_or(NodeLanguage::Auto)
        } else {
            // Para carpetas de organización, usar Text
            NodeLanguage::Text
        };
        
        // Crear nodo usando add_node y luego modificar
        let id = self.add_node(
            folder_title,
            position,
            color,
            &[], // Sin inputs
            &[], // Sin outputs
            folder_language, // Usar el lenguaje determinado
        );
        
        // Configurar como nodo carpeta
        if let Some(node) = self.node_mut(id) {
            node.subnetwork_graph = Some(NodeGraph::default()); // Usar subnetwork_graph como contenedor
            if matches!(mode, FolderNodeMode::Inheritable) {
                if !node.title.contains("(Heredable)") {
                    // Agregar el lenguaje requerido al título para que sea visible
                    let lang_suffix = if !matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
                        format!(" [{}]", Self::language_display_name(folder_language))
                    } else {
                        String::new()
                    };
                    node.title = format!("{} (Heredable){}", node.title, lang_suffix);
                }
            }
        }
        
        id
    }
    
    /// Verificar si un nodo es un nodo carpeta
    pub fn is_folder_node(&self, node_id: NodeId) -> bool {
        if let Some(node) = self.node(node_id) {
            // Por ahora, verificamos por el título que empiece con 📁
            // En el futuro, agregaremos un campo específico
            node.title.starts_with("📁 ") && node.subnetwork_graph.is_some()
        } else {
            false
        }
    }
    
    /// Obtener el grafo interno de un nodo carpeta
    pub fn get_folder_content(&self, folder_node_id: NodeId) -> Option<&NodeGraph> {
        if let Some(node) = self.node(folder_node_id) {
            // Verificar si es nodo carpeta
            if node.title.starts_with("📁 ") && node.subnetwork_graph.is_some() {
                node.subnetwork_graph.as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Obtener el grafo interno mutable de un nodo carpeta
    pub fn get_folder_content_mut(&mut self, folder_node_id: NodeId) -> Option<&mut NodeGraph> {
        if let Some(node) = self.node_mut(folder_node_id) {
            // Verificar si es nodo carpeta
            if node.title.starts_with("📁 ") && node.subnetwork_graph.is_some() {
                node.subnetwork_graph.as_mut()
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Agregar un nodo dentro de un nodo carpeta
    pub fn add_node_to_folder(
        &mut self,
        folder_node_id: NodeId,
        title: impl Into<String>,
        position: Pos2,
        color: Color32,
        inputs: &[&str],
        outputs: &[&str],
        language: NodeLanguage,
    ) -> Result<NodeId, String> {
        // ═══════════════════════════════════════════════════════════════════
        // 🆕 VALIDAR LENGUAJE SI ES CARPETA HEREDABLE
        // ═══════════════════════════════════════════════════════════════════
        if let Some(folder_node) = self.node(folder_node_id) {
            // Verificar si es carpeta heredable
            if folder_node.title.contains("(Heredable)") {
                // Obtener el lenguaje requerido de la carpeta
                let mut folder_language = folder_node.language;
                
                // Si el lenguaje es Auto o Text, intentar extraer del título
                if matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
                    if folder_node.title.contains("[Rust]") {
                        folder_language = NodeLanguage::Rust;
                    } else if folder_node.title.contains("[Python]") {
                        folder_language = NodeLanguage::Python;
                    } else if folder_node.title.contains("[Java]") {
                        folder_language = NodeLanguage::Java;
                    } else if folder_node.title.contains("[Assembly]") || folder_node.title.contains("[ASM]") {
                        folder_language = NodeLanguage::Asm;
                    }
                }
                
                // Si el lenguaje de la carpeta no es Auto o Text, validar compatibilidad
                if !matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
                    // SOLO bloquear si el lenguaje NO coincide
                    if language != folder_language {
                        return Err(format!(
                            "❌ ERROR: El nodo tiene lenguaje '{}' pero la carpeta heredable solo acepta '{}'.\n\
                            Las carpetas heredables solo pueden contener un único lenguaje de programación.",
                            Self::language_display_name(language),
                            Self::language_display_name(folder_language)
                        ));
                    }
                    // Si el lenguaje SÍ coincide, continuar normalmente
                }
            }
        }
        
        if let Some(folder_graph) = self.get_folder_content_mut(folder_node_id) {
            Ok(folder_graph.add_node(title, position, color, inputs, outputs, language))
        } else {
            Err("No se pudo acceder al contenido de la carpeta".to_string())
        }
    }
    
    /// Obtener nombre de visualización para un lenguaje
    fn language_display_name(lang: NodeLanguage) -> String {
        match lang {
            NodeLanguage::Rust => "Rust".to_string(),
            NodeLanguage::Python => "Python".to_string(),
            NodeLanguage::Java => "Java".to_string(),
            NodeLanguage::Asm => "Assembly".to_string(),
            NodeLanguage::Cpp => "C++".to_string(),
            NodeLanguage::Text => "Text".to_string(),
            NodeLanguage::Auto => "Auto".to_string(),
        }
    }
    
    /// Mover nodos existentes a un nodo carpeta
    /// Nota: Esta función es compleja porque necesita preservar conexiones
    /// Por ahora, simplificamos moviendo solo los nodos (sin links)
    pub fn move_nodes_to_folder(
        &mut self,
        folder_node_id: NodeId,
        node_ids: &[NodeId],
    ) -> Result<(), String> {
        // Verificar que es nodo carpeta ANTES de obtener referencias
        if !self.is_folder_node(folder_node_id) {
            return Err("El nodo especificado no es un nodo carpeta".to_string());
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // 🆕 VALIDAR LENGUAJE SI ES CARPETA HEREDABLE
        // ═══════════════════════════════════════════════════════════════════
        let folder_language_opt = {
            if let Some(folder_node) = self.node(folder_node_id) {
                if folder_node.title.contains("(Heredable)") {
                    let lang = folder_node.language;
                    // Si no es Auto o Text, necesitamos validar
                    if !matches!(lang, NodeLanguage::Auto | NodeLanguage::Text) {
                        Some(lang)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        
        // Recolectar TODA la información necesaria ANTES de cualquier referencia mutable
        let node_ids_set: std::collections::HashSet<NodeId> = node_ids.iter().copied().collect();
        
        // Obtener todos los PinIds de los nodos que se moverán
        let mut pins_to_move: std::collections::HashSet<PinId> = std::collections::HashSet::new();
        let mut incompatible_nodes: Vec<String> = Vec::new();
        
        let nodes_to_move: Vec<_> = {
            node_ids.iter()
                .filter_map(|&node_id| {
                    if let Some(node) = self.node(node_id) {
                        // Validar lenguaje si es carpeta heredable
                        if let Some(required_lang) = folder_language_opt {
                            if node.language != required_lang {
                                incompatible_nodes.push(node.title.clone());
                                return None; // Filtrar nodos incompatibles
                            }
                        }
                        
                        for pin in node.inputs.iter().chain(node.outputs.iter()) {
                            pins_to_move.insert(pin.id);
                        }
                        Some(node.clone())
                    } else {
                        None
                    }
                })
                .collect()
        };
        
        // Verificar si se filtraron nodos por incompatibilidad de lenguaje
        if !incompatible_nodes.is_empty() {
            if let Some(required_lang) = folder_language_opt {
                let nodes_list = incompatible_nodes.join(", ");
                return Err(format!(
                    "❌ ERROR: Los siguientes nodos tienen lenguajes incompatibles: {}\n\
                    La carpeta heredable solo acepta '{}'.\n\
                    Las carpetas heredables solo pueden contener un único lenguaje de programación.",
                    nodes_list,
                    Self::language_display_name(required_lang)
                ));
            }
        }
        
        // Recolectar links que conectan estos pines
        let links_to_remove: Vec<_> = {
            let links = self.links();
            links.iter()
                .filter(|link| {
                    pins_to_move.contains(&link.from) || pins_to_move.contains(&link.to)
                })
                .cloned()
                .collect()
        };
        
        // Ahora obtener referencia mutable al folder_graph
        let folder_graph = self.get_folder_content_mut(folder_node_id)
            .ok_or("No se pudo acceder al contenido de la carpeta")?;
        
        // Mover nodos al grafo interno
        for node in nodes_to_move {
            // Agregar nodo al grafo interno usando add_node
            let input_labels: Vec<&str> = node.inputs.iter().map(|p| p.label.as_str()).collect();
            let output_labels: Vec<&str> = node.outputs.iter().map(|p| p.label.as_str()).collect();
            
            let inner_id = folder_graph.add_node(
                node.title.clone(),
                node.position,
                node.color,
                &input_labels,
                &output_labels,
                node.language,
            );
            
            // Copiar código y otros campos
            if let Some(inner_node) = folder_graph.node_mut(inner_id) {
                inner_node.code = node.code;
                // ═══════════════════════════════════════════════════════════════════
                // IMPORTANTE: No copiar code_path porque el nuevo nodo tiene un ID diferente
                // El code_path se generará automáticamente cuando se guarde el grafo
                // basándose en el nuevo ID del nodo dentro de la carpeta
                // ═══════════════════════════════════════════════════════════════════
                inner_node.code_path = None; // Se generará nuevo code_path basado en inner_id
                inner_node.parent_node = node.parent_node;
                inner_node.inherits_from_folder = node.inherits_from_folder;
            }
        }
        
        // Liberar referencia mutable antes de remover links
        drop(folder_graph);
        
        // Remover links del grafo principal
        for link in links_to_remove {
            self.remove_link(link.from, link.to);
        }
        
        // Remover nodos del grafo principal
        self.remove_nodes(&node_ids_set);
        
        Ok(())
    }
    
    /// Obtener todo el código de un nodo carpeta (para herencia)
    pub fn get_folder_node_code(&self, folder_node_id: NodeId) -> String {
        if let Some(folder_graph) = self.get_folder_content(folder_node_id) {
            let mut combined_code = String::new();
            
            // Combinar código de todos los nodos dentro de la carpeta
            for node in folder_graph.nodes() {
                if !node.code.is_empty() {
                    combined_code.push_str(&format!("// === {} ===\n", node.title));
                    combined_code.push_str(&node.code);
                    combined_code.push_str("\n\n");
                }
            }
            
            combined_code
        } else {
            String::new()
        }
    }
    
    /// Aplicar herencia de nodo carpeta a un nodo
    pub fn apply_folder_inheritance(&self, node_id: NodeId, folder_node_id: NodeId) -> String {
        let folder_code = self.get_folder_node_code(folder_node_id);
        
        if let Some(node) = self.node(node_id) {
            // Combinar código de la carpeta + código propio del nodo
            let mut combined = String::new();
            combined.push_str("// === Código heredado de carpeta ===\n");
            combined.push_str(&folder_code);
            combined.push_str("\n// === Código propio ===\n");
            combined.push_str(&node.code);
            combined
        } else {
            folder_code
        }
    }
    
    /// Establecer si un nodo carpeta es heredable
    pub fn set_folder_inheritable(&mut self, folder_node_id: NodeId, inheritable: bool) -> Result<(), String> {
        // Verificar primero si es nodo carpeta
        let is_folder = self.is_folder_node(folder_node_id);
        if !is_folder {
            return Err("El nodo especificado no es un nodo carpeta".to_string());
        }
        
        if let Some(node) = self.node_mut(folder_node_id) {
            // Por ahora, usamos el título para indicar modo
            // En el futuro, agregaremos un campo específico
            if inheritable {
                if !node.title.contains("(Heredable)") {
                    node.title = format!("{} (Heredable)", node.title);
                }
            } else {
                node.title = node.title.replace(" (Heredable)", "");
            }
            Ok(())
        } else {
            Err("Nodo no encontrado".to_string())
        }
    }
    
    /// Verificar si un nodo carpeta es heredable
    pub fn is_folder_inheritable(&self, folder_node_id: NodeId) -> bool {
        if let Some(node) = self.node(folder_node_id) {
            node.title.contains("(Heredable)")
        } else {
            false
        }
    }
    
    /// Obtener todos los nodos carpeta en el grafo
    pub fn get_all_folder_nodes(&self) -> Vec<NodeId> {
        self.nodes().iter()
            .filter(|node| self.is_folder_node(node.id))
            .map(|node| node.id)
            .collect()
    }
    
    /// Convertir un grupo de nodos en un nodo carpeta
    pub fn create_folder_from_nodes(
        &mut self,
        title: impl Into<String>,
        position: Pos2,
        node_ids: &[NodeId],
        mode: FolderNodeMode,
    ) -> Result<NodeId, String> {
        if node_ids.is_empty() {
            return Err("No se pueden crear nodos carpeta vacíos".to_string());
        }
        
        // Crear el nodo carpeta
        let folder_id = self.create_folder_node(title, position, mode, None);
        
        // Mover los nodos seleccionados a la carpeta
        self.move_nodes_to_folder(folder_id, node_ids)?;
        
        Ok(folder_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_folder_node() {
        let mut graph = NodeGraph::default();
        let folder_id = graph.create_folder_node(
            "Test Folder",
            pos2(100.0, 100.0),
            FolderNodeMode::Organization,
            None,
        );
        
        assert!(graph.is_folder_node(folder_id));
    }
    
    #[test]
    fn test_add_node_to_folder() {
        let mut graph = NodeGraph::default();
        let folder_id = graph.create_folder_node(
            "Test Folder",
            pos2(100.0, 100.0),
            FolderNodeMode::Organization,
            None,
        );
        
        let inner_node_id = graph.add_node_to_folder(
            folder_id,
            "Inner Node",
            pos2(50.0, 50.0),
            Color32::WHITE,
            &[],
            &["Output"],
            NodeLanguage::Rust,
        );
        
        assert!(inner_node_id.is_ok());
        
        if let Some(folder_graph) = graph.get_folder_content(folder_id) {
            assert_eq!(folder_graph.nodes().len(), 1);
        }
    }
}

