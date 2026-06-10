use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct WorkspaceState {
    root: Option<PathBuf>,
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
            println!("Workspace seleccionado: {}", folder.display());
            self.root = Some(folder);
        }

        self.root()
    }

    pub fn label(&self) -> String {
        self.root
            .as_ref()
            .map(|path| format!("Workspace: {}", path.display()))
            .unwrap_or_else(|| "Workspace: presiona O para seleccionar carpeta".to_string())
    }
}
