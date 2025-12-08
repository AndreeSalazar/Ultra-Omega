// ═══════════════════════════════════════════════════════════════════════════════
// Migration: Migración de proyectos antiguos al nuevo formato
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use crate::core::node_graph::NodeGraph;
use super::workspace::Workspace;
use super::node_storage::NodeStorage;

/// Resultado de la migración
#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub migrated: bool,
    pub nodes_migrated: usize,
    pub errors: Vec<String>,
}

/// Detectar si un proyecto necesita migración
pub fn needs_migration(workspace: &Workspace) -> bool {
    // Verificar si existe node_map.json con código embebido
    if let Some(map_path) = workspace.get_node_map_path() {
        if map_path.exists() {
            // Intentar cargar y verificar si hay código embebido
            if let Ok(json) = std::fs::read_to_string(&map_path) {
                // Buscar si hay nodos con campo "code" no vacío
                return json.contains("\"code\":") && json.contains("\"code\":\"");
            }
        }
    }
    
    // Si no existe node_map.json, no necesita migración
    false
}

/// Migrar proyecto del formato antiguo (código embebido) al nuevo (código separado)
pub fn migrate_project(workspace: &Workspace, graph: &mut NodeGraph) -> Result<MigrationResult, String> {
    let storage = NodeStorage::new(workspace.clone());
    
    // Asegurar que el directorio nodes/ existe
    storage.ensure_nodes_directory()
        .map_err(|e| format!("Failed to create nodes directory: {}", e))?;

    let mut result = MigrationResult {
        migrated: false,
        nodes_migrated: 0,
        errors: Vec::new(),
    };

    // Migrar cada nodo que tenga código embebido
    for node in graph.nodes_mut() {
        // Si ya tiene code_path, ya está migrado
        if node.code_path.is_some() {
            continue;
        }

        // Si no tiene código, saltar
        if node.code.is_empty() {
            continue;
        }

        // Migrar: guardar código en archivo separado
        match storage.save_node_code(node.id, &node.code, node.language) {
            Ok(relative_path) => {
                // Actualizar el nodo con la ruta del código
                node.code_path = Some(relative_path);
                // Mantener código en memoria por compatibilidad (se puede limpiar después)
                result.nodes_migrated += 1;
                result.migrated = true;
            }
            Err(e) => {
                result.errors.push(format!("Node {}: {}", node.id.0, e));
            }
        }
    }

    Ok(result)
}

/// Crear backup del proyecto antes de migrar
pub fn create_backup(workspace: &Workspace) -> Result<PathBuf, String> {
    let root = workspace.root_path.as_ref()
        .ok_or_else(|| "No workspace root set".to_string())?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_dir = root.parent()
        .ok_or_else(|| "Cannot determine parent directory".to_string())?
        .join(format!("{}_backup_{}", 
            root.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project"),
            timestamp));

    // Copiar todo el contenido del workspace
    if backup_dir.exists() {
        std::fs::remove_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to remove existing backup: {}", e))?;
    }

    copy_directory(root, &backup_dir)
        .map_err(|e| format!("Failed to create backup: {}", e))?;

    Ok(backup_dir)
}

/// Copiar directorio recursivamente
fn copy_directory(src: &std::path::Path, dst: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if path.is_dir() {
            copy_directory(&path, &dst_path)?;
        } else {
            std::fs::copy(&path, &dst_path)?;
        }
    }

    Ok(())
}

