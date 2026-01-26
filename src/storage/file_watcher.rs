// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega File Watcher - Sistema de Detección en Tiempo Real
// ═══════════════════════════════════════════════════════════════════════════
// Inspirado en Houdini: Detecta archivos y carpetas para crear nodos automáticamente

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Instant};
use crate::core::node_graph::{NodeLanguage, NodeId};
use std::hash::Hash;

/// Información de un archivo detectado
#[derive(Debug, Clone)]
pub struct DetectedFile {
    pub path: PathBuf,
    pub name: String,
    pub extension: String,
    pub language: NodeLanguage,
    pub is_directory: bool,
    pub modified_time: Option<SystemTime>,
    pub size: u64,
    /// ID del nodo asociado (si existe)
    pub node_id: Option<NodeId>,
}

/// Información de una carpeta detectada
#[derive(Debug, Clone)]
pub struct DetectedFolder {
    pub path: PathBuf,
    pub name: String,
    pub files: Vec<DetectedFile>,
    pub subfolders: Vec<DetectedFolder>,
    pub file_count: usize,
    pub folder_count: usize,
    /// Lenguaje predominante en la carpeta
    pub dominant_language: Option<NodeLanguage>,
}

/// Estado del sistema de detección de archivos
#[derive(Debug, Clone)]
pub struct FileWatcherState {
    /// Carpeta raíz que se está monitoreando
    pub root_path: Option<PathBuf>,
    /// Estructura de archivos detectada
    pub detected_structure: Option<DetectedFolder>,
    /// Última vez que se escaneó
    pub last_scan: Option<Instant>,
    /// Intervalo de escaneo (en segundos)
    pub scan_interval_secs: f32,
    /// Archivos que han cambiado desde el último escaneo
    pub changed_files: Vec<PathBuf>,
    /// Archivos nuevos detectados
    pub new_files: Vec<PathBuf>,
    /// Archivos eliminados
    pub deleted_files: Vec<PathBuf>,
    /// Cache de tiempos de modificación
    file_times: HashMap<PathBuf, SystemTime>,
    /// Si el escaneo automático está activo
    pub auto_scan_enabled: bool,
}

impl Default for FileWatcherState {
    fn default() -> Self {
        Self {
            root_path: None,
            detected_structure: None,
            last_scan: None,
            scan_interval_secs: 2.0, // Escanear cada 2 segundos
            changed_files: Vec::new(),
            new_files: Vec::new(),
            deleted_files: Vec::new(),
            file_times: HashMap::new(),
            auto_scan_enabled: true,
        }
    }
}

impl FileWatcherState {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Establecer la carpeta raíz a monitorear
    pub fn set_root(&mut self, path: PathBuf) {
        self.root_path = Some(path);
        self.detected_structure = None;
        self.last_scan = None;
        self.file_times.clear();
        self.changed_files.clear();
        self.new_files.clear();
        self.deleted_files.clear();
    }
    
    /// Verificar si necesita escanear
    pub fn needs_scan(&self) -> bool {
        if !self.auto_scan_enabled {
            return false;
        }
        
        match self.last_scan {
            None => true,
            Some(last) => last.elapsed().as_secs_f32() >= self.scan_interval_secs,
        }
    }
    
    /// Escanear la carpeta raíz y detectar cambios
    pub fn scan(&mut self) -> Result<(), String> {
        let root = self.root_path.clone()
            .ok_or_else(|| "No root path set".to_string())?;
        
        if !root.exists() {
            return Err(format!("Root path does not exist: {:?}", root));
        }
        
        // Limpiar listas de cambios
        self.changed_files.clear();
        self.new_files.clear();
        self.deleted_files.clear();
        
        // Escanear estructura
        let structure = scan_folder(&root, &root)?;
        
        // Detectar cambios comparando con el cache
        let mut current_files: HashMap<PathBuf, SystemTime> = HashMap::new();
        collect_file_times(&structure, &mut current_files);
        
        // Detectar archivos nuevos y modificados
        for (path, time) in &current_files {
            if let Some(old_time) = self.file_times.get(path) {
                if time != old_time {
                    self.changed_files.push(path.clone());
                }
            } else {
                self.new_files.push(path.clone());
            }
        }
        
        // Detectar archivos eliminados
        for path in self.file_times.keys() {
            if !current_files.contains_key(path) {
                self.deleted_files.push(path.clone());
            }
        }
        
        // Actualizar cache
        self.file_times = current_files;
        self.detected_structure = Some(structure);
        self.last_scan = Some(Instant::now());
        
        Ok(())
    }
    
    /// Obtener todos los archivos de código detectados
    pub fn get_code_files(&self) -> Vec<&DetectedFile> {
        let mut files = Vec::new();
        if let Some(ref structure) = self.detected_structure {
            collect_code_files(structure, &mut files);
        }
        files
    }
    
    /// Obtener archivos por lenguaje
    pub fn get_files_by_language(&self, language: NodeLanguage) -> Vec<&DetectedFile> {
        self.get_code_files()
            .into_iter()
            .filter(|f| f.language == language)
            .collect()
    }
    
    /// Obtener estadísticas de la estructura
    pub fn get_stats(&self) -> FileWatcherStats {
        let mut stats = FileWatcherStats::default();
        
        if let Some(ref structure) = self.detected_structure {
            count_stats(structure, &mut stats);
        }
        
        stats
    }
    
    /// Verificar si hay cambios pendientes
    pub fn has_changes(&self) -> bool {
        !self.changed_files.is_empty() || 
        !self.new_files.is_empty() || 
        !self.deleted_files.is_empty()
    }
}

/// Estadísticas del sistema de archivos
#[derive(Debug, Clone, Default)]
pub struct FileWatcherStats {
    pub total_files: usize,
    pub total_folders: usize,
    pub cpp_files: usize,
    pub java_files: usize,
    pub asm_files: usize,
    pub python_files: usize,
    pub rust_files: usize,
    pub other_files: usize,
}

/// Escanear una carpeta recursivamente
fn scan_folder(path: &Path, root: &Path) -> Result<DetectedFolder, String> {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let mut folder = DetectedFolder {
        path: path.to_path_buf(),
        name,
        files: Vec::new(),
        subfolders: Vec::new(),
        file_count: 0,
        folder_count: 0,
        dominant_language: None,
    };
    
    let entries = std::fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {:?}: {}", path, e))?;
    
    let mut language_counts: HashMap<NodeLanguage, usize> = HashMap::new();
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let entry_path = entry.path();
        let metadata = entry.metadata().ok();
        
        if entry_path.is_dir() {
            // Ignorar carpetas ocultas y especiales
            let dir_name = entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            if !dir_name.starts_with('.') && 
               dir_name != "node_modules" && 
               dir_name != "target" &&
               dir_name != "__pycache__" &&
               dir_name != ".git" {
                if let Ok(subfolder) = scan_folder(&entry_path, root) {
                    folder.folder_count += 1 + subfolder.folder_count;
                    folder.file_count += subfolder.file_count;
                    
                    // Acumular lenguajes de subcarpetas
                    for file in &subfolder.files {
                        *language_counts.entry(file.language).or_insert(0) += 1;
                    }
                    
                    folder.subfolders.push(subfolder);
                }
            }
        } else if entry_path.is_file() {
            let file = detect_file(&entry_path, metadata.as_ref());
            
            // Solo contar archivos de código
            if file.language != NodeLanguage::Cpp || !file.extension.is_empty() {
                *language_counts.entry(file.language).or_insert(0) += 1;
            }
            
            folder.files.push(file);
            folder.file_count += 1;
        }
    }
    
    // Determinar lenguaje dominante
    folder.dominant_language = language_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(lang, _)| lang);
    
    // Ordenar archivos y carpetas
    folder.files.sort_by(|a, b| a.name.cmp(&b.name));
    folder.subfolders.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(folder)
}

/// Detectar información de un archivo
fn detect_file(path: &Path, metadata: Option<&std::fs::Metadata>) -> DetectedFile {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let language = detect_language_from_extension(&extension);
    
    let (modified_time, size) = if let Some(meta) = metadata {
        (meta.modified().ok(), meta.len())
    } else {
        (None, 0)
    };
    
    DetectedFile {
        path: path.to_path_buf(),
        name,
        extension,
        language,
        is_directory: false,
        modified_time,
        size,
        node_id: None,
    }
}

/// Detectar lenguaje basado en extensión
pub fn detect_language_from_extension(ext: &str) -> NodeLanguage {
    match ext {
        "cpp" | "cc" | "cxx" | "c++" | "c" | "h" | "hpp" | "hxx" => NodeLanguage::Cpp,
        "java" => NodeLanguage::Java,
        "asm" | "s" | "nasm" => NodeLanguage::Asm,
        "py" | "pyw" => NodeLanguage::Python,
        "rs" => NodeLanguage::Rust,
        "js" | "ts" | "jsx" | "tsx" | "txt" | "md" => NodeLanguage::Text,
        _ => NodeLanguage::Cpp, // Default
    }
}

/// Obtener icono para un lenguaje
pub fn get_language_icon(language: NodeLanguage) -> &'static str {
    match language {
        NodeLanguage::Cpp => "©",
        NodeLanguage::Java => "☕",
        NodeLanguage::Asm => "⚡",
        NodeLanguage::Python => "🐍",
        NodeLanguage::Rust => "🦀",
        NodeLanguage::Text => "📄",
        NodeLanguage::Auto => "�",
    }
}

/// Obtener icono para una extensión de archivo
pub fn get_file_icon(extension: &str) -> &'static str {
    match extension {
        "cpp" | "cc" | "cxx" | "c++" | "c" => "©",
        "h" | "hpp" | "hxx" => "📋",
        "java" => "☕",
        "asm" | "s" | "nasm" => "⚡",
        "py" | "pyw" => "🐍",
        "rs" => "🦀",
        "js" | "ts" => "🟨",
        "json" => "📋",
        "txt" | "md" => "📄",
        "exe" => "⚙️",
        "dll" | "so" | "dylib" => "🔧",
        _ => "📄",
    }
}

/// Recolectar tiempos de modificación de archivos
fn collect_file_times(folder: &DetectedFolder, times: &mut HashMap<PathBuf, SystemTime>) {
    for file in &folder.files {
        if let Some(time) = file.modified_time {
            times.insert(file.path.clone(), time);
        }
    }
    
    for subfolder in &folder.subfolders {
        collect_file_times(subfolder, times);
    }
}

/// Recolectar archivos de código
fn collect_code_files<'a>(folder: &'a DetectedFolder, files: &mut Vec<&'a DetectedFile>) {
    for file in &folder.files {
        // Solo archivos de código
        match file.extension.as_str() {
            "cpp" | "cc" | "cxx" | "c++" | "c" | "h" | "hpp" | "hxx" |
            "java" | "asm" | "s" | "nasm" | "py" | "pyw" | "rs" | "js" | "ts" => {
                files.push(file);
            }
            _ => {}
        }
    }
    
    for subfolder in &folder.subfolders {
        collect_code_files(subfolder, files);
    }
}

/// Contar estadísticas
fn count_stats(folder: &DetectedFolder, stats: &mut FileWatcherStats) {
    stats.total_folders += 1;
    
    for file in &folder.files {
        stats.total_files += 1;
        
        match file.language {
            NodeLanguage::Cpp => stats.cpp_files += 1,
            NodeLanguage::Java => stats.java_files += 1,
            NodeLanguage::Asm => stats.asm_files += 1,
            NodeLanguage::Python => stats.python_files += 1,
            NodeLanguage::Rust => stats.rust_files += 1,
            NodeLanguage::Text => stats.other_files += 1,
            NodeLanguage::Auto => stats.other_files += 1,
        }
    }
    
    for subfolder in &folder.subfolders {
        count_stats(subfolder, stats);
    }
}

/// Crear nodos automáticamente basados en archivos detectados
pub fn create_nodes_from_files(
    files: &[&DetectedFile],
    graph: &mut crate::core::node_graph::NodeGraph,
    start_position: eframe::egui::Pos2,
    spacing: f32,
) -> Vec<NodeId> {
    use eframe::egui::Color32;
    
    let mut created_nodes = Vec::new();
    let mut current_pos = start_position;
    let mut row_count = 0;
    let max_per_row = 5;
    
    for file in files {
        // Leer contenido del archivo
        let code = std::fs::read_to_string(&file.path).unwrap_or_default();
        
        // Color basado en lenguaje
        let color = match file.language {
            NodeLanguage::Cpp => Color32::from_rgb(100, 150, 255),
            NodeLanguage::Java => Color32::from_rgb(237, 139, 0),
            NodeLanguage::Asm => Color32::from_rgb(255, 220, 100),
            NodeLanguage::Python => Color32::from_rgb(55, 118, 171),
            NodeLanguage::Rust => Color32::from_rgb(255, 140, 100),
            NodeLanguage::Text => Color32::from_rgb(180, 180, 180),
            NodeLanguage::Auto => Color32::from_rgb(150, 150, 150),
        };
        
        // Crear nodo con la firma correcta
        let node_id = graph.add_node(
            file.name.clone(),
            current_pos,
            color,
            &["in"],
            &["out"],
            file.language,
        );
        
        // Establecer el código del nodo
        if let Some(node) = graph.node_mut(node_id) {
            node.code = code;
            node.code_path = Some(file.path.to_string_lossy().to_string());
        }
        
        created_nodes.push(node_id);
        
        // Actualizar posición
        row_count += 1;
        if row_count >= max_per_row {
            row_count = 0;
            current_pos.x = start_position.x;
            current_pos.y += spacing;
        } else {
            current_pos.x += spacing;
        }
    }
    
    created_nodes
}

/// Crear estructura de carpetas como nodos (estilo Houdini)
pub fn create_folder_structure_as_nodes(
    folder: &DetectedFolder,
    graph: &mut crate::core::node_graph::NodeGraph,
    start_position: eframe::egui::Pos2,
    spacing: f32,
    depth: usize,
) -> Vec<NodeId> {
    use eframe::egui::Color32;
    
    let mut created_nodes = Vec::new();
    let mut current_pos = start_position;
    
    // Crear nodo para la carpeta (como nodo contenedor)
    let folder_code = format!(
        "// Carpeta: {}\n// Archivos: {}\n// Subcarpetas: {}\n// Lenguaje dominante: {:?}",
        folder.name,
        folder.file_count,
        folder.folder_count,
        folder.dominant_language
    );
    
    let folder_language = folder.dominant_language.unwrap_or(NodeLanguage::Cpp);
    
    // Color para carpeta
    let folder_color = Color32::from_rgb(255, 200, 100);
    
    let folder_node_id = graph.add_node(
        format!("📁 {}", folder.name),
        current_pos,
        folder_color,
        &["in"],
        &["out"],
        folder_language,
    );
    
    // Establecer código del nodo carpeta
    if let Some(node) = graph.node_mut(folder_node_id) {
        node.code = folder_code;
    }
    
    created_nodes.push(folder_node_id);
    
    // Crear nodos para archivos de código en esta carpeta
    let mut file_pos = eframe::egui::pos2(
        current_pos.x + spacing,
        current_pos.y + spacing * 0.5,
    );
    
    for file in &folder.files {
        // Solo archivos de código
        match file.extension.as_str() {
            "cpp" | "cc" | "cxx" | "c++" | "c" | "h" | "hpp" | "hxx" |
            "java" | "asm" | "s" | "nasm" | "py" | "pyw" | "rs" | "js" | "ts" => {
                let code = std::fs::read_to_string(&file.path).unwrap_or_default();
                
                // Color basado en lenguaje
                let color = match file.language {
                    NodeLanguage::Cpp => Color32::from_rgb(100, 150, 255),
                    NodeLanguage::Java => Color32::from_rgb(237, 139, 0),
                    NodeLanguage::Asm => Color32::from_rgb(255, 220, 100),
                    NodeLanguage::Python => Color32::from_rgb(55, 118, 171),
                    NodeLanguage::Rust => Color32::from_rgb(255, 140, 100),
                    NodeLanguage::Text => Color32::from_rgb(180, 180, 180),
                    NodeLanguage::Auto => Color32::from_rgb(150, 150, 150),
                };
                
                let node_id = graph.add_node(
                    file.name.clone(),
                    file_pos,
                    color,
                    &["in"],
                    &["out"],
                    file.language,
                );
                
                // Establecer código del nodo
                if let Some(node) = graph.node_mut(node_id) {
                    node.code = code;
                    node.code_path = Some(file.path.to_string_lossy().to_string());
                }
                
                created_nodes.push(node_id);
                file_pos.y += spacing * 0.6;
            }
            _ => {}
        }
    }
    
    // Procesar subcarpetas recursivamente
    let mut subfolder_pos = eframe::egui::pos2(
        current_pos.x,
        current_pos.y + spacing * (1.0 + folder.files.len() as f32 * 0.3),
    );
    
    for subfolder in &folder.subfolders {
        let subfolder_nodes = create_folder_structure_as_nodes(
            subfolder,
            graph,
            subfolder_pos,
            spacing,
            depth + 1,
        );
        
        created_nodes.extend(subfolder_nodes);
        subfolder_pos.y += spacing * 2.0;
    }
    
    created_nodes
}
