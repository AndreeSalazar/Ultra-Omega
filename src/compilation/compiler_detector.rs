// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Detector de Compiladores
// Detecta automáticamente compiladores instalados en el sistema
// 100% enfocado en Rust (rustc, cargo)
// ═══════════════════════════════════════════════════════════════════════════════

use std::process::Command;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════════
// ESTRUCTURAS DE INFORMACIÓN DEL COMPILADOR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct CompilerInfo {
    pub name: String,
    pub version: String,
    pub path: Option<PathBuf>,
    pub available: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompilerStatus {
    pub rustc: CompilerInfo,
    pub cargo: CompilerInfo,
}

impl Default for CompilerStatus {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE DETECCIÓN
// ═══════════════════════════════════════════════════════════════════════════════

impl CompilerStatus {
    pub fn new() -> Self {
        Self {
            rustc: detect_rustc(),
            cargo: detect_cargo(),
        }
    }

    /// Obtener un resumen en formato texto
    pub fn summary(&self) -> String {
        let mut summary = String::from("=== Estado de Compiladores (100% Rust) ===\n\n");
        
        summary.push_str(&format_compiler_status("Rust (rustc)", &self.rustc));
        summary.push_str(&format_compiler_status("Cargo", &self.cargo));
        
        summary.push_str("\n=== Resumen ===\n");
        let available_count = [&self.rustc, &self.cargo]
            .iter()
            .filter(|c| c.available)
            .count();
        
        summary.push_str(&format!("Compiladores disponibles: {}/2\n", available_count));
        
        if available_count == 2 {
            summary.push_str("✅ ¡Rust está completamente instalado y listo!\n");
        } else {
            summary.push_str("❌ Falta instalar Rust. Descarga desde: https://rustup.rs/\n");
        }
        
        summary
    }

    /// Verificar si todos los compiladores necesarios están disponibles
    pub fn is_ready(&self) -> bool {
        self.rustc.available && self.cargo.available
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DETECCIÓN DE RUST
// ═══════════════════════════════════════════════════════════════════════════════

fn detect_rustc() -> CompilerInfo {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return CompilerInfo {
                    name: "rustc".to_string(),
                    version,
                    path: find_executable("rustc"),
                    available: true,
                    error_message: None,
                };
            }
        }
        Err(_) => {}
    }

    CompilerInfo {
        name: "rustc".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("rustc no encontrado. Instala Rust desde: https://rustup.rs/".to_string()),
    }
}

fn detect_cargo() -> CompilerInfo {
    match Command::new("cargo").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return CompilerInfo {
                    name: "cargo".to_string(),
                    version,
                    path: find_executable("cargo"),
                    available: true,
                    error_message: None,
                };
            }
        }
        Err(_) => {}
    }

    CompilerInfo {
        name: "cargo".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("cargo no encontrado. Instala Rust desde: https://rustup.rs/".to_string()),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES AUXILIARES
// ═══════════════════════════════════════════════════════════════════════════════

pub fn find_executable(name: &str) -> Option<PathBuf> {
    let name_with_ext = if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };

    // Buscar en PATH usando which-like logic
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(&name_with_ext);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // Buscar en ubicaciones comunes de Rust
    if let Ok(home) = std::env::var(if cfg!(target_os = "windows") { "USERPROFILE" } else { "HOME" }) {
        let cargo_bin = PathBuf::from(home).join(".cargo").join("bin").join(&name_with_ext);
        if cargo_bin.exists() {
            return Some(cargo_bin);
        }
    }

    None
}

/// Búsqueda profunda en directorios del sistema (Windows/Linux/macOS)
pub fn deep_search_executable(name: &str) -> Option<PathBuf> {
    let name_with_ext = if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.to_string()
    };

    let search_dirs: Vec<PathBuf> = if cfg!(target_os = "windows") {
        // Windows: buscar en Program Files, System32, etc.
        let mut dirs = Vec::new();
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            dirs.push(PathBuf::from(program_files));
        }
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            dirs.push(PathBuf::from(program_files_x86));
        }
        if let Ok(system_root) = std::env::var("SystemRoot") {
            let sys_root = PathBuf::from(system_root);
            dirs.push(sys_root.join("System32"));
            dirs.push(sys_root);
        }
        // Rustup default locations
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            let base = PathBuf::from(user_profile);
            dirs.push(base.join(".rustup").join("toolchains"));
            dirs.push(base.join(".cargo").join("bin"));
        }
        dirs
    } else {
        // Linux/macOS
        vec![
            PathBuf::from("/usr/bin"),
            PathBuf::from("/usr/local/bin"),
            PathBuf::from("/opt"),
        ]
    };

    for base_dir in &search_dirs {
        if let Ok(walker) = std::fs::read_dir(base_dir) {
            for entry in walker.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    if !dir_name.starts_with('.') {
                        let candidate = path.join(&name_with_ext);
                        if candidate.is_file() {
                            return Some(candidate);
                        }
                    }
                }
            }
        }
    }

    None
}

fn format_compiler_status(name: &str, info: &CompilerInfo) -> String {
    let status = if info.available { "✅" } else { "❌" };
    let mut result = format!("{} {}: {}\n", status, name, info.version);
    
    if let Some(path) = &info.path {
        result.push_str(&format!("   Ubicación: {}\n", path.display()));
    }
    
    if let Some(error) = &info.error_message {
        result.push_str(&format!("   {}\n", error));
    }
    
    result.push('\n');
    result
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES PÚBLICAS DE UTILIDAD
// ═══════════════════════════════════════════════════════════════════════════════

/// Detectar todos los compiladores y retornar un resumen
pub fn detect_all_compilers() -> CompilerStatus {
    CompilerStatus::new()
}
