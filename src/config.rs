use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub workspace_path: Option<String>,
    pub auto_save: bool,
    pub window_size: Option<(f32, f32)>,
    pub window_pos: Option<(f32, f32)>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            workspace_path: None,
            auto_save: false,
            window_size: None,
            window_pos: None,
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ultra-Omega")
            .join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(json) = std::fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str(&json) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        Ok(())
    }
}

