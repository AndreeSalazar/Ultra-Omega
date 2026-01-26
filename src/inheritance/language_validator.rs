// ═══════════════════════════════════════════════════════════════════
// VALIDADOR DE LENGUAJES ÚNICOS PARA CARPETAS HEREDABLES
// ═══════════════════════════════════════════════════════════════════

use crate::core::node_graph::{NodeGraph, NodeId, NodeLanguage};

/// Resultado de validación de lenguaje
#[derive(Debug, Clone)]
pub enum LanguageValidationResult {
    /// Validación exitosa
    Valid,
    /// Error: lenguaje incompatible
    IncompatibleLanguage {
        folder_language: NodeLanguage,
        node_language: NodeLanguage,
        node_title: String,
    },
    /// Error: carpeta no es heredable
    NotInheritable,
    /// Error: carpeta no encontrada
    FolderNotFound,
}

impl LanguageValidationResult {
    /// Obtener mensaje de error si existe
    pub fn error_message(&self) -> Option<String> {
        match self {
            LanguageValidationResult::Valid => None,
            LanguageValidationResult::IncompatibleLanguage { folder_language, node_language, node_title } => {
                Some(format!(
                    "❌ ERROR: El nodo '{}' tiene lenguaje '{}' pero la carpeta heredable solo acepta '{}'.\n\
                    Las carpetas heredables solo pueden contener un único lenguaje de programación.",
                    node_title,
                    language_display_name(*node_language),
                    language_display_name(*folder_language)
                ))
            },
            LanguageValidationResult::NotInheritable => {
                Some("❌ ERROR: Esta carpeta no es heredable. Solo las carpetas heredables requieren un lenguaje único.".to_string())
            },
            LanguageValidationResult::FolderNotFound => {
                Some("❌ ERROR: Carpeta no encontrada.".to_string())
            },
        }
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

/// Validar que un nodo puede ser agregado a una carpeta heredable
pub fn validate_node_for_inheritable_folder(
    graph: &NodeGraph,
    folder_id: NodeId,
    node_id: NodeId,
) -> LanguageValidationResult {
    // Verificar que la carpeta existe y es heredable
    let folder_node = match graph.node(folder_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };
    
    // Verificar que es una carpeta heredable
    if !folder_node.title.contains("(Heredable)") {
        return LanguageValidationResult::NotInheritable;
    }
    
    // Obtener el lenguaje requerido de la carpeta
    let folder_language = folder_node.language;
    
    // Si el lenguaje de la carpeta es Auto o Text, no hay restricción
    if matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
        return LanguageValidationResult::Valid;
    }
    
    // Obtener el nodo que se quiere agregar
    let node = match graph.node(node_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };
    
    // Validar que el lenguaje del nodo coincide con el de la carpeta
    if node.language != folder_language {
        return LanguageValidationResult::IncompatibleLanguage {
            folder_language,
            node_language: node.language,
            node_title: node.title.clone(),
        };
    }
    
    LanguageValidationResult::Valid
}

/// Validar que todos los nodos dentro de una carpeta heredable tienen el mismo lenguaje
pub fn validate_folder_language_consistency(
    graph: &NodeGraph,
    folder_id: NodeId,
) -> LanguageValidationResult {
    // Verificar que la carpeta existe y es heredable
    let folder_node = match graph.node(folder_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };
    
    // Verificar que es una carpeta heredable
    if !folder_node.title.contains("(Heredable)") {
        return LanguageValidationResult::NotInheritable;
    }
    
    // Obtener el lenguaje requerido de la carpeta
    let folder_language = folder_node.language;
    
    // Si el lenguaje de la carpeta es Auto o Text, no hay restricción
    if matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
        return LanguageValidationResult::Valid;
    }
    
    // Obtener el grafo interno de la carpeta
    let folder_graph = match folder_node.subnetwork_graph.as_ref() {
        Some(graph) => graph,
        None => return LanguageValidationResult::Valid, // Carpeta vacía es válida
    };
    
    // Validar todos los nodos dentro de la carpeta
    for node in folder_graph.nodes() {
        if node.language != folder_language {
            return LanguageValidationResult::IncompatibleLanguage {
                folder_language,
                node_language: node.language,
                node_title: node.title.clone(),
            };
        }
    }
    
    LanguageValidationResult::Valid
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

