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

    if let Ok(path) = which::which(&name_with_ext) {
        return Some(path);
    }
    if let Ok(path) = which::which(name) {
        return Some(path);
    }

    // Buscar en ubicaciones comunes
    if let Ok(home) = std::env::var(if cfg!(target_os = "windows") { "USERPROFILE" } else { "HOME" }) {
        let cargo_bin = PathBuf::from(home).join(".cargo").join("bin").join(&name_with_ext);
        if cargo_bin.exists() {
            return Some(cargo_bin);
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
