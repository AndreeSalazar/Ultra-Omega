// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Detector de Compiladores
// Detecta automáticamente compiladores instalados en el sistema
// Soporta: NASM, Rust, Zig, C++ (g++, clang++, MSVC)
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
    pub rust: CompilerInfo,
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
            rust: detect_rust(),
        }
    }

    /// Obtener un resumen en formato texto
    pub fn summary(&self) -> String {
        let mut summary = String::from("=== Estado de Compiladores ===\n\n");
        
        summary.push_str(&format_compiler_status("Rust (Cargo)", &self.rust));
            cpp_msvc: detect_cpp_msvc(),
        }
    }

    /// Obtener un resumen en formato texto
    pub fn summary(&self) -> String {
        let mut summary = String::from("=== Estado de Compiladores ===\n\n");
        
        summary.push_str(&format_compiler_status("NASM (ASM)", &self.nasm));
        summary.push_str(&format_compiler_status("Rust (Cargo)", &self.rust));
        summary.push_str(&format_compiler_status("Zig", &self.zig));
        summary.push_str(&format_compiler_status("C++ (GCC/g++)", &self.cpp_gcc));
        summary.push_str(&format_compiler_status("C++ (Clang++)", &self.cpp_clang));
        summary.push_str(&format_compiler_status("C++ (MSVC)", &self.cpp_msvc));
        
        summary.push_str("\n=== Resumen ===\n");
        let available_count = [
            &self.nasm,
            &self.rust,
            &self.zig,
            &self.cpp_gcc,
            &self.cpp_clang,
            &self.cpp_msvc,
        ]
        .iter()
        .filter(|c| c.available)
        .count();
        
        summary.push_str(&format!("Compiladores disponibles: {}/6\n", available_count));
        
        if available_count == 6 {
            summary.push_str("✅ ¡Todos los compiladores están disponibles!\n");
        } else if available_count >= 4 {
            summary.push_str("⚠️ La mayoría de compiladores están disponibles\n");
        } else {
            summary.push_str("❌ Faltan varios compiladores. Instala los que necesites.\n");
        }
        
        summary
    }

    /// Verificar si hay al menos un compilador C++ disponible
    pub fn has_cpp_compiler(&self) -> bool {
        self.cpp_gcc.available || self.cpp_clang.available || self.cpp_msvc.available
    }

    /// Obtener el mejor compilador C++ disponible (prioridad: GCC > Clang > MSVC)
    pub fn best_cpp_compiler(&self) -> Option<&CompilerInfo> {
        if self.cpp_gcc.available {
            Some(&self.cpp_gcc)
        } else if self.cpp_clang.available {
            Some(&self.cpp_clang)
        } else if self.cpp_msvc.available {
            Some(&self.cpp_msvc)
        } else {
            None
        }
    }

    /// Verificar si todos los compiladores necesarios están disponibles
    pub fn is_ready(&self) -> bool {
        self.rust.available
    }

// ═══════════════════════════════════════════════════════════════════════════════
// DETECCIÓN DE COMPILADORES ESPECÍFICOS
// ═══════════════════════════════════════════════════════════════════════════════

fn detect_nasm() -> CompilerInfo {
    // Primero intentar en PATH
    match Command::new("nasm").arg("-v").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .unwrap_or("Desconocida")
                    .to_string();
                
                return CompilerInfo {
                    name: "NASM".to_string(),
                    version,
                    path: find_executable("nasm"),
                    available: true,
                    error_message: None,
                };
            }
        }
        Err(_) => {}
    }
    
    // Si no se encuentra en PATH, buscar profundamente
    if let Some(path) = deep_search_executable("nasm") {
        match Command::new(&path).arg("-v").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .unwrap_or("Desconocida")
                        .to_string();
                    
                    return CompilerInfo {
                        name: "NASM".to_string(),
                        version,
                        path: Some(path),
                        available: true,
                        error_message: None,
                    };
                }
            }
            Err(_) => {}
        }
    }
    
    CompilerInfo {
        name: "NASM".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("NASM no encontrado.\nDescarga desde: https://nasm.us\nWindows: Agrega NASM a PATH o instálalo en C:\\NASM".to_string()),
    }
}

fn detect_rust() -> CompilerInfo {
    // Intentar rustc primero
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                
                return CompilerInfo {
                    name: "Rust".to_string(),
                    version,
                    path: find_executable("rustc"),
                    available: true,
                    error_message: None,
                };
            }
        }
        Err(_) => {}
    }
    
    // Si rustc no está en PATH, buscar profundamente
    if let Some(rustc_path) = deep_search_executable("rustc") {
        match Command::new(&rustc_path).arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    
                    return CompilerInfo {
                        name: "Rust".to_string(),
                        version,
                        path: Some(rustc_path),
                        available: true,
                        error_message: None,
                    };
                }
            }
            Err(_) => {}
        }
    }
    
    // También buscar cargo y deducir rustc
    if let Some(cargo_path) = deep_search_executable("cargo") {
        match Command::new(&cargo_path).arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let cargo_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    
                    // Si encontramos cargo, buscar rustc en el mismo directorio
                    if let Some(cargo_dir) = cargo_path.parent() {
                        let rustc_in_cargo_dir = cargo_dir.join("rustc.exe");
                        if rustc_in_cargo_dir.exists() {
                            return CompilerInfo {
                                name: "Rust (via cargo)".to_string(),
                                version: format!("Rust (cargo: {})", cargo_version),
                                path: Some(rustc_in_cargo_dir),
                                available: true,
                                error_message: None,
                            };
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
    
    CompilerInfo {
        name: "Rust".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("Rust no encontrado.\nInstala desde: https://rustup.rs/\nWindows: Descarga rustup-init.exe\nLinux/Mac: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh".to_string()),
    }
}

fn detect_zig() -> CompilerInfo {
    // Primero intentar en PATH
    match Command::new("zig").arg("version").output() {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                
                return CompilerInfo {
                    name: "Zig".to_string(),
                    version,
                    path: find_executable("zig"),
                    available: true,
                    error_message: None,
                };
            }
        }
        Err(_) => {}
    }
    
    // Si no se encuentra en PATH, buscar profundamente
    if let Some(path) = deep_search_executable("zig") {
        match Command::new(&path).arg("version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    
                    return CompilerInfo {
                        name: "Zig".to_string(),
                        version,
                        path: Some(path),
                        available: true,
                        error_message: None,
                    };
                }
            }
            Err(_) => {}
        }
    }
    
    CompilerInfo {
        name: "Zig".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("Zig no encontrado.\nDescarga desde: https://ziglang.org/download/\nWindows: Extrae en C:\\Zig y agrega a PATH".to_string()),
    }
}

fn detect_cpp_gcc() -> CompilerInfo {
    // En Windows, buscar g++ o mingw32-g++ con múltiples variantes
    let commands = if cfg!(target_os = "windows") {
        vec!["g++", "mingw32-g++", "x86_64-w64-mingw32-g++", "i686-w64-mingw32-g++"]
    } else {
        vec!["g++"]
    };
    
    // Primero intentar con PATH
    for cmd in &commands {
        match Command::new(cmd).arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .next()
                        .unwrap_or("Desconocida")
                        .to_string();
                    
                    return CompilerInfo {
                        name: format!("C++ ({})", cmd),
                        version,
                        path: find_executable(cmd),
                        available: true,
                        error_message: None,
                    };
                }
            }
            Err(_) => continue,
        }
    }
    
    // Si no se encuentra en PATH, buscar profundamente
    for cmd in &commands {
        if let Some(path) = deep_search_executable(cmd) {
            // Verificar que el ejecutable funciona
            match Command::new(&path).arg("--version").output() {
                Ok(output) => {
                    if output.status.success() {
                        let version = String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .next()
                            .unwrap_or("Desconocida")
                            .to_string();
                        
                        return CompilerInfo {
                            name: format!("C++ ({})", cmd),
                            version,
                            path: Some(path),
                            available: true,
                            error_message: None,
                        };
                    }
                }
                Err(_) => continue,
            }
        }
    }
    
    CompilerInfo {
        name: "C++ (GCC/g++)".to_string(),
        version: "No disponible".to_string(),
        path: None,
        available: false,
        error_message: Some("GCC/g++ no encontrado.\nLinux: sudo apt-get install g++\nWindows: Instala MinGW-w64\nMac: xcode-select --install".to_string()),
    }
}

fn detect_cpp_clang() -> CompilerInfo {
    match Command::new("clang++").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("Desconocida")
                .to_string();
            
            CompilerInfo {
                name: "C++ (Clang++)".to_string(),
                version,
                path: find_executable("clang++"),
                available: true,
                error_message: None,
            }
        }
        Err(e) => CompilerInfo {
            name: "C++ (Clang++)".to_string(),
            version: "No disponible".to_string(),
            path: None,
            available: false,
            error_message: Some(format!("Clang++ no encontrado: {}\nInstala desde: https://releases.llvm.org/download.html", e)),
        },
    }
}

fn detect_cpp_msvc() -> CompilerInfo {
    // En Windows, buscar cl.exe (compilador MSVC)
    if !cfg!(target_os = "windows") {
        return CompilerInfo {
            name: "C++ (MSVC)".to_string(),
            version: "Solo Windows".to_string(),
            path: None,
            available: false,
            error_message: Some("MSVC solo está disponible en Windows".to_string()),
        };
    }
    
    // Buscar en las ubicaciones comunes de Visual Studio
    let possible_paths = vec![
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC",
        r"C:\Program Files\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC",
    ];
    
    for base_path in possible_paths {
        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let cl_path = entry.path().join("bin\\Hostx64\\x64\\cl.exe");
                if cl_path.exists() {
                    match Command::new(&cl_path).output() {
                        Ok(_) => {
                            return CompilerInfo {
                                name: "C++ (MSVC)".to_string(),
                                version: "MSVC (encontrado)".to_string(),
                                path: Some(cl_path.clone()),
                                available: true,
                                error_message: None,
                            };
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }
    
    // Intentar con el comando cl directamente
    match Command::new("cl").output() {
        Ok(_) => CompilerInfo {
            name: "C++ (MSVC)".to_string(),
            version: "MSVC (disponible)".to_string(),
            path: find_executable("cl"),
            available: true,
            error_message: None,
        },
        Err(_) => CompilerInfo {
            name: "C++ (MSVC)".to_string(),
            version: "No disponible".to_string(),
            path: None,
            available: false,
            error_message: Some("MSVC no encontrado.\nInstala Visual Studio con herramientas de C++ desde: https://visualstudio.microsoft.com/".to_string()),
        },
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES AUXILIARES
// ═══════════════════════════════════════════════════════════════════════════════

pub fn find_executable(name: &str) -> Option<PathBuf> {
    // Primero intentar con `which` (PATH del sistema)
    if cfg!(target_os = "windows") {
        let name_with_ext = format!("{}.exe", name);
        if let Ok(path) = which::which(&name_with_ext) {
            return Some(path);
        }
        if let Ok(path) = which::which(name) {
            return Some(path);
        }
    } else {
        if let Ok(path) = which::which(name) {
            return Some(path);
        }
    }
    
    // Si no se encuentra en PATH, buscar en ubicaciones comunes
    deep_search_executable(name)
}

/// Búsqueda profunda de ejecutables en ubicaciones comunes
pub fn deep_search_executable(name: &str) -> Option<PathBuf> {
    #[cfg(target_os = "windows")] {
        let name_exe = format!("{}.exe", name);
        
        // Lista de ubicaciones comunes para buscar compiladores
        let mut search_paths: Vec<&str> = vec![
            // MinGW-w64 ubicaciones comunes
            r"C:\MinGW\bin",
            r"C:\MinGW-w64\bin",
            r"C:\msys64\mingw64\bin",
            r"C:\msys64\ucrt64\bin",
            
            // LLVM/Clang ubicaciones
            r"C:\Program Files\LLVM\bin",
            r"C:\Program Files (x86)\LLVM\bin",
        ];
        
        // Agregar paths específicos según el comando
        if name == "link" || name == "cl" {
            // Visual Studio paths (buscar manualmente más abajo)
            search_paths.extend(&[
                r"C:\Program Files\Microsoft Visual Studio\2022",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2022",
                r"C:\Program Files\Microsoft Visual Studio\2019",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2019",
            ]);
        }
        
        if name == "nasm" {
            // Agregar ubicaciones comunes de NASM
            search_paths.extend(&[
                r"C:\Program Files\NASM",
                r"C:\NASM",
            ]);
            
            // Buscar en AppData local (donde el usuario lo tiene instalado)
            if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
                let nasm_paths = vec![
                    format!(r"{}\bin\NASM", local_appdata),
                    format!(r"{}\NASM", local_appdata),
                    format!(r"{}\bin", local_appdata),
                ];
                for nasm_path in nasm_paths {
                    let full_path = PathBuf::from(&nasm_path).join(&name_exe);
                    if full_path.exists() {
                        return Some(full_path);
                    }
                }
            }
            
            // Buscar en ProgramData
            if let Ok(programdata) = std::env::var("ProgramData") {
                let nasm_path = format!(r"{}\NASM", programdata);
                let full_path = PathBuf::from(&nasm_path).join(&name_exe);
                if full_path.exists() {
                    return Some(full_path);
                }
            }
        }
        
        if name == "zig" {
            search_paths.extend(&[
                r"C:\Program Files\Zig",
                r"C:\Zig",
                r"C:\Tools\Zig",
            ]);
        }
        
        // Rust ubicaciones comunes (buscar primero antes de búsqueda general)
        if name == "rustc" || name == "cargo" {
            // Rust generalmente se instala en el directorio del usuario
            if let Ok(home) = std::env::var("USERPROFILE") {
                // Buscar en .cargo\bin (instalación estándar de rustup)
                let cargo_bin = format!(r"{}\.cargo\bin", home);
                if std::path::Path::new(&cargo_bin).exists() {
                    let full_path = std::path::Path::new(&cargo_bin).join(&name_exe);
                    if full_path.exists() {
                        return Some(full_path);
                    }
                }
            }
            // También buscar en ubicaciones comunes
            let rust_paths = vec![
                r"C:\Program Files\Rust",
                r"C:\Rust\bin",
                r"C:\tools\rust\bin",
            ];
            for rust_path in rust_paths {
                let full_path = PathBuf::from(rust_path).join(&name_exe);
                if full_path.exists() {
                    return Some(full_path);
                }
            }
        }
        
        // Ubicaciones comunes de programas (solo si no es VS)
        if name != "link" && name != "cl" {
            search_paths.extend(&[
                r"C:\Program Files",
                r"C:\Program Files (x86)",
                r"C:\tools",
            ]);
        }
        
        // Buscar Visual Studio específicamente (para cl.exe y link.exe)
        if name == "cl" || name == "link" {
            let vs_base_paths = vec![
                r"C:\Program Files\Microsoft Visual Studio\2022",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2022",
                r"C:\Program Files\Microsoft Visual Studio\2019",
                r"C:\Program Files (x86)\Microsoft Visual Studio\2019",
            ];
            
            for vs_base in vs_base_paths {
                if let Ok(entries) = std::fs::read_dir(vs_base) {
                    for entry in entries.flatten() {
                        let edition_path = entry.path();
                        if edition_path.is_dir() {
                            let msvc_path = edition_path.join("VC/Tools/MSVC");
                            if let Ok(msvc_entries) = std::fs::read_dir(&msvc_path) {
                                for msvc_entry in msvc_entries.flatten() {
                                    let version_path = msvc_entry.path();
                                    let tool_path = version_path.join(format!("bin/Hostx64/x64/{}.exe", name));
                                    if tool_path.exists() {
                                        return Some(tool_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Buscar en cada ruta
        for base_path_str in search_paths {
            // Verificar directamente
            let full_path = PathBuf::from(base_path_str).join(&name_exe);
            if full_path.exists() {
                return Some(full_path);
            }
            
            // Buscar recursivamente en subdirectorios
            if let Ok(entries) = std::fs::read_dir(base_path_str) {
                for entry in entries.flatten() {
                    let sub_path = entry.path();
                    if sub_path.is_dir() {
                        // Verificar en el directorio directamente
                        let test_path = sub_path.join(&name_exe);
                        if test_path.exists() {
                            return Some(test_path);
                        }
                        
                        // Buscar en bin/ subdirectorio común
                        let bin_path = sub_path.join("bin").join(&name_exe);
                        if bin_path.exists() {
                            return Some(bin_path);
                        }
                        
                        // Para algunos compiladores, buscar más profundo (solo 1 nivel más)
                        if name == "nasm" || name == "zig" {
                            // Buscar directamente en subdirectorios comunes
                            for sub_entry in std::fs::read_dir(&sub_path).ok()?.flatten() {
                                let deep_path = sub_entry.path();
                                if deep_path.is_dir() {
                                    let deep_test = deep_path.join(&name_exe);
                                    if deep_test.exists() {
                                        return Some(deep_test);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))] {
        // Linux/Mac: Buscar en ubicaciones comunes
        let search_paths = vec![
            "/usr/bin",
            "/usr/local/bin",
            "/opt/homebrew/bin",  // macOS Apple Silicon
            "/usr/local/opt",     // macOS Homebrew
            "/opt",
            "/snap/bin",
        ];
        
        for base_path in search_paths {
            let full_path = PathBuf::from(base_path).join(name);
            if full_path.exists() && is_executable(&full_path) {
                return Some(full_path);
            }
        }
        
        // Buscar en directorios del usuario
        if let Ok(home) = std::env::var("HOME") {
            let home_paths = vec![
                format!("{}/.cargo/bin", home),
                format!("{}/.local/bin", home),
                format!("{}/bin", home),
            ];
            
            for home_path_str in home_paths {
                let full_path = PathBuf::from(&home_path_str).join(name);
                if full_path.exists() && is_executable(&full_path) {
                    return Some(full_path);
                }
            }
        }
    }
    
    None
}


#[cfg(not(target_os = "windows"))]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(metadata) = std::fs::metadata(path) {
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        // Verificar si tiene permisos de ejecución
        (mode & 0o111) != 0
    } else {
        false
    }
}

#[cfg(target_os = "windows")]
fn is_executable(_path: &std::path::Path) -> bool {
    true // En Windows, si existe un .exe, es ejecutable
}

fn format_compiler_status(name: &str, info: &CompilerInfo) -> String {
    let status = if info.available {
        "✅"
    } else {
        "❌"
    };
    
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

/// Verificar si un compilador específico está disponible
pub fn is_compiler_available(compiler: &str) -> bool {
    match compiler.to_lowercase().as_str() {
        "nasm" | "asm" => detect_nasm().available,
        "rust" | "cargo" => detect_rust().available,
        "zig" => detect_zig().available,
        "g++" | "gcc" => detect_cpp_gcc().available,
        "clang++" | "clang" => detect_cpp_clang().available,
        "msvc" | "cl" => detect_cpp_msvc().available,
        _ => false,
    }
}

/// Obtener información de un compilador específico
pub fn get_compiler_info(compiler: &str) -> CompilerInfo {
    match compiler.to_lowercase().as_str() {
        "nasm" | "asm" => detect_nasm(),
        "rust" | "cargo" => detect_rust(),
        "zig" => detect_zig(),
        "g++" | "gcc" => detect_cpp_gcc(),
        "clang++" | "clang" => detect_cpp_clang(),
        "msvc" | "cl" => detect_cpp_msvc(),
        _ => CompilerInfo {
            name: compiler.to_string(),
            version: "Desconocido".to_string(),
            path: None,
            available: false,
            error_message: Some(format!("Compilador '{}' no reconocido", compiler)),
        },
    }
}

