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
}
