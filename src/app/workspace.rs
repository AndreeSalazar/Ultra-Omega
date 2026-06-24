use std::path::{Path, PathBuf};
use crate::core::NodeGraph;
use crate::storage::workspace::Workspace as StorageWorkspace;

#[derive(Debug, Default)]
pub struct WorkspaceState {
    root: Option<PathBuf>,
    storage: Option<StorageWorkspace>,
}

impl WorkspaceState {
    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    pub fn select_folder(&mut self) -> Option<&Path> {
        let mut dialog = rfd::FileDialog::new().set_title("Ultra-Omega: seleccionar carpeta de trabajo");

        if let Some(root) = &self.root {
            dialog = dialog.set_directory(root);
        } else if let Ok(current_dir) = std::env::current_dir() {
            dialog = dialog.set_directory(current_dir);
        }

        if let Some(folder) = dialog.pick_folder() {
            log::info!("Workspace seleccionado: {}", folder.display());
            self.root = Some(folder.clone());

            let mut ws = StorageWorkspace::new();
            ws.set_root(folder);
            self.storage = Some(ws);
        }

        self.root()
    }

    pub fn load_graph(&self) -> Option<NodeGraph> {
        self.storage.as_ref().and_then(|ws| {
            ws.load_graph().ok()
        })
    }

    pub fn save_graph(&self, graph: &mut NodeGraph) -> Result<(), String> {
        if let Some(ws) = &self.storage {
            ws.save_graph(graph)?;
        }
        Ok(())
    }

    pub fn label(&self) -> String {
        self.root
            .as_ref()
            .map(|path| format!("Workspace: {}", path.display()))
            .unwrap_or_else(|| "Workspace: presiona O para seleccionar carpeta".to_string())
    }

    /// Lista archivos del workspace para mostrar en el sidebar
    pub fn list_files_for_sidebar(&self) -> Vec<SidebarEntry> {
        let mut entries: Vec<SidebarEntry> = Vec::new();
        let Some(root) = &self.root else { return entries; };
        collect_entries(root, "", 0, &mut entries, 200); // max 200 entries
        entries
    }

    pub fn is_folder_open(&self, rel_path: &str) -> bool {
        // Para implementación simple: siempre abierto si existe
        let Some(root) = &self.root else { return false; };
        root.join(rel_path).is_dir()
    }
}

#[derive(Clone, Debug)]
pub struct SidebarEntry {
    pub name: String,
    pub rel_path: String,
    pub depth: usize,
    pub is_dir: bool,
    pub is_expanded: bool,
}

fn collect_entries(base: &Path, rel: &str, depth: usize, out: &mut Vec<SidebarEntry>, max: usize) {
    if out.len() >= max { return; }
    let full = base.join(rel);
    let Ok(read_dir) = std::fs::read_dir(&full) else { return; };
    let mut entries: Vec<_> = read_dir.flatten().collect();
    entries.sort_by_key(|e| {
        let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
        // Carpetas primero, luego archivos
        (!is_dir, e.file_name().to_string_lossy().to_lowercase())
    });
    for entry in entries {
        if out.len() >= max { break; }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') { continue; } // Ocultar dotfiles
        if name == "target" || name == "node_modules" { continue; } // Ocultar build dirs
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let new_rel = if rel.is_empty() { name.clone() } else { format!("{}/{}", rel, name) };
        out.push(SidebarEntry {
            name: name.clone(),
            rel_path: new_rel.clone(),
            depth,
            is_dir,
            is_expanded: true,
        });
        if is_dir {
            collect_entries(base, &new_rel, depth + 1, out, max);
        }
    }
}
