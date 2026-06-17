// ═══════════════════════════════════════════════════════════════════
// VALIDADOR DE LENGUAJES ÚNICOS PARA CARPETAS HEREDABLES
// ═══════════════════════════════════════════════════════════════════

use crate::core::node_graph::{NodeGraph, NodeId, NodeLanguage};

#[derive(Debug, Clone)]
pub enum LanguageValidationResult {
    Valid,
    IncompatibleLanguage {
        folder_language: NodeLanguage,
        node_language: NodeLanguage,
        node_title: String,
    },
    NotInheritable,
    FolderNotFound,
}

impl LanguageValidationResult {
    pub fn error_message(&self) -> Option<String> {
        match self {
            LanguageValidationResult::Valid => None,
            LanguageValidationResult::IncompatibleLanguage { folder_language, node_language, node_title } => {
                Some(format!(
                    "ERROR: El nodo '{}' tiene lenguaje '{}' pero la carpeta heredable solo acepta '{}'.",
                    node_title,
                    NodeGraph::language_display_name(*node_language),
                    NodeGraph::language_display_name(*folder_language)
                ))
            },
            LanguageValidationResult::NotInheritable => {
                Some("ERROR: Esta carpeta no es heredable.".to_string())
            },
            LanguageValidationResult::FolderNotFound => {
                Some("ERROR: Carpeta no encontrada.".to_string())
            },
        }
    }
}

pub fn validate_node_for_inheritable_folder(
    graph: &NodeGraph,
    folder_id: NodeId,
    node_id: NodeId,
) -> LanguageValidationResult {
    let folder_node = match graph.node(folder_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };

    if !folder_node.is_folder || !folder_node.title.contains("(Heredable)") {
        return LanguageValidationResult::NotInheritable;
    }

    let folder_language = folder_node.language;

    if matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
        return LanguageValidationResult::Valid;
    }

    let node = match graph.node(node_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };

    if node.language != folder_language {
        return LanguageValidationResult::IncompatibleLanguage {
            folder_language,
            node_language: node.language,
            node_title: node.title.clone(),
        };
    }

    LanguageValidationResult::Valid
}

pub fn validate_folder_language_consistency(
    graph: &NodeGraph,
    folder_id: NodeId,
) -> LanguageValidationResult {
    let folder_node = match graph.node(folder_id) {
        Some(node) => node,
        None => return LanguageValidationResult::FolderNotFound,
    };

    if !folder_node.is_folder || !folder_node.title.contains("(Heredable)") {
        return LanguageValidationResult::NotInheritable;
    }

    let folder_language = folder_node.language;

    if matches!(folder_language, NodeLanguage::Auto | NodeLanguage::Text) {
        return LanguageValidationResult::Valid;
    }

    let folder_graph = match folder_node.subnetwork_graph.as_ref() {
        Some(graph) => graph,
        None => return LanguageValidationResult::Valid,
    };

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

pub fn get_folder_required_language(
    graph: &NodeGraph,
    folder_id: NodeId,
) -> Option<NodeLanguage> {
    let folder_node = graph.node(folder_id)?;
    if folder_node.is_folder && folder_node.title.contains("(Heredable)") {
        Some(folder_node.language)
    } else {
        None
    }
}

