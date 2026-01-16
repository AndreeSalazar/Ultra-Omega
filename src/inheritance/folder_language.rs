// ═══════════════════════════════════════════════════════════════════
// GESTIÓN DE LENGUAJES EN CARPETAS HEREDABLES
// ═══════════════════════════════════════════════════════════════════

use crate::core::node_graph::{NodeGraph, NodeId, NodeLanguage};

/// Establecer el lenguaje requerido para una carpeta heredable
pub fn set_folder_required_language(
    graph: &mut NodeGraph,
    folder_id: NodeId,
    language: NodeLanguage,
) -> Result<(), String> {
    let folder_node = graph.node_mut(folder_id)
        .ok_or_else(|| "Carpeta no encontrada".to_string())?;
    
    // Verificar que es una carpeta heredable
    if !folder_node.title.contains("(Heredable)") {
        return Err("Solo las carpetas heredables pueden tener un lenguaje requerido".to_string());
    }
    
    // Establecer el lenguaje
    folder_node.language = language;
    
    // Validar que todos los nodos existentes son compatibles
    if let Some(ref folder_graph) = folder_node.subnetwork_graph {
        for node in folder_graph.nodes() {
            if node.language != language && !matches!(language, NodeLanguage::Auto | NodeLanguage::Text) {
                return Err(format!(
                    "El nodo '{}' tiene lenguaje '{}' que no coincide con el lenguaje requerido '{}'",
                    node.title,
                    language_display_name(node.language),
                    language_display_name(language)
                ));
            }
        }
    }
    
    Ok(())
}

/// Obtener el lenguaje requerido de una carpeta heredable
pub fn get_folder_required_language(
    graph: &NodeGraph,
    folder_id: NodeId,
) -> Option<NodeLanguage> {
    let folder_node = graph.node(folder_id)?;
    
    if folder_node.title.contains("(Heredable)") {
        Some(folder_node.language)
    } else {
        None
    }
}

/// Obtener nombre de visualización para un lenguaje
fn language_display_name(lang: NodeLanguage) -> String {
    match lang {
        NodeLanguage::Rust => "Rust".to_string(),
        NodeLanguage::Python => "Python".to_string(),
        NodeLanguage::Java => "Java".to_string(),        
        NodeLanguage::Asm => "Assembly".to_string(),
        NodeLanguage::Text => "Text".to_string(),
        NodeLanguage::Auto => "Auto".to_string(),
    }
}

