// ═══════════════════════════════════════════════════════════════════════════════
// Project: Metadatos y configuración del proyecto
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metadatos del proyecto Ultra-Omega
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub format_version: u32, // Versión del formato (para migraciones futuras)
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            name: "Untitled Project".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            created_at: now.clone(),
            updated_at: now,
            format_version: 1, // Versión 1 = código separado
        }
    }
}

/// Configuración del proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub metadata: ProjectMetadata,
    pub auto_save: bool,
    pub auto_format: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            metadata: ProjectMetadata::default(),
            auto_save: true,
            auto_format: false,
        }
    }
}

impl ProjectConfig {
    /// Guardar configuración del proyecto
    pub fn save(&self, workspace_root: &PathBuf) -> Result<(), String> {
        let config_path = workspace_root.join(".ultra-omega").join("project.json");
        
        // Asegurar que el directorio existe
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize project config: {}", e))?;

        std::fs::write(&config_path, json)
            .map_err(|e| format!("Failed to write project config: {}", e))?;

        Ok(())
    }

    /// Cargar configuración del proyecto
    pub fn load(workspace_root: &PathBuf) -> Result<Self, String> {
        let config_path = workspace_root.join(".ultra-omega").join("project.json");

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let json = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read project config: {}", e))?;

        let config: Self = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse project config: {}", e))?;

        Ok(config)
    }

    /// Actualizar timestamp de última modificación
    pub fn touch(&mut self) {
        self.metadata.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

