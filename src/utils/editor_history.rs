use std::path::PathBuf;
use std::time::Instant;
use crate::core::node_graph::NodeId;

pub struct EditorHistory {
    pub node_id: NodeId,
    pub history: Vec<String>, // Historial de versiones del código
    pub current_index: usize,  // Índice actual en el historial
    pub last_save_time: Option<Instant>,
    pub last_code_hash: u64,
    pub temp_file_path: Option<PathBuf>,
}

impl EditorHistory {
    pub fn new(node_id: NodeId, initial_code: String) -> Self {
        let mut history = Self {
            node_id,
            history: vec![initial_code.clone()],
            current_index: 0,
            last_save_time: None,
            last_code_hash: Self::hash_code(&initial_code),
            temp_file_path: None,
        };
        
        // Crear archivo temporal inicial
        history.save_to_temp(&initial_code);
        
        history
    }
    
    pub fn add_version(&mut self, code: String) {
        let code_hash = Self::hash_code(&code);
        
        // Solo agregar si el código cambió
        if code_hash != self.last_code_hash {
            // Eliminar versiones futuras si estamos en medio del historial
            if self.current_index < self.history.len() - 1 {
                self.history.truncate(self.current_index + 1);
            }
            
            // Agregar nueva versión
            self.history.push(code.clone());
            self.current_index = self.history.len() - 1;
            self.last_code_hash = code_hash;
            
            // Limitar historial a 50 versiones
            if self.history.len() > 50 {
                self.history.remove(0);
                self.current_index -= 1;
            }
        }
    }
    
    pub fn get_current(&self) -> Option<&String> {
        self.history.get(self.current_index)
    }
    
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }
    
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len() - 1
    }
    
    pub fn undo(&mut self) -> Option<String> {
        if self.can_undo() {
            self.current_index -= 1;
            if let Some(code) = self.get_current() {
                let code_clone = code.clone();
                self.last_code_hash = Self::hash_code(&code_clone);
                self.save_to_temp(&code_clone);
                return Some(code_clone);
            }
        }
        None
    }
    
    pub fn redo(&mut self) -> Option<String> {
        if self.can_redo() {
            self.current_index += 1;
            if let Some(code) = self.get_current() {
                let code_clone = code.clone();
                self.last_code_hash = Self::hash_code(&code_clone);
                self.save_to_temp(&code_clone);
                return Some(code_clone);
            }
        }
        None
    }
    
    pub fn should_auto_save(&self) -> bool {
        // Auto-save cada 10 segundos
        if let Some(last_save) = self.last_save_time {
            last_save.elapsed().as_secs() >= 10
        } else {
            true // Primera vez, guardar inmediatamente
        }
    }
    
    pub fn mark_saved(&mut self) {
        self.last_save_time = Some(Instant::now());
    }
    
    pub fn save_to_temp(&mut self, code: &str) {
        // Crear directorio temp si no existe
        let temp_dir = std::env::temp_dir().join("Ultra-Omega");
        if let Err(e) = std::fs::create_dir_all(&temp_dir) {
            eprintln!("Error creating temp directory: {}", e);
            return;
        }
        
        // Crear nombre de archivo temporal basado en node_id
        let filename = format!("node_{}_code.tmp", self.node_id.0);
        let temp_path = temp_dir.join(&filename);
        
        // Guardar código en archivo temporal
        if let Err(e) = std::fs::write(&temp_path, code) {
            eprintln!("Error saving to temp file: {}", e);
        } else {
            self.temp_file_path = Some(temp_path);
        }
    }
    
    #[allow(dead_code)] // Puede ser útil en el futuro
    pub fn load_from_temp(&self) -> Option<String> {
        if let Some(ref temp_path) = self.temp_file_path {
            if temp_path.exists() {
                if let Ok(code) = std::fs::read_to_string(temp_path) {
                    return Some(code);
                }
            }
        }
        None
    }
    
    pub fn cleanup_temp(&self) {
        if let Some(ref temp_path) = self.temp_file_path {
            if temp_path.exists() {
                let _ = std::fs::remove_file(temp_path);
            }
        }
    }
    
    fn hash_code(code: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        code.hash(&mut hasher);
        hasher.finish()
    }
}

