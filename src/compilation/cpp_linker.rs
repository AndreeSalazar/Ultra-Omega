use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub enum CppLinker {
    Gcc,
    Clang,
    Msvc,
}

#[derive(Debug, Clone)]
pub struct CppLinkerConfig {
    pub linker: CppLinker,
    pub version: String, // "c++11", "c++14", "c++17"
    pub command: String,
    pub args: Vec<String>,
    pub available: bool,
}

impl CppLinkerConfig {
    pub fn new(linker: CppLinker, version: &str) -> Self {
        let (command, args) = match (&linker, version) {
            // GCC configurations
            (CppLinker::Gcc, "c++11") => (
                "g++".to_string(),
                vec![
                    "-std=c++11".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            (CppLinker::Gcc, "c++14") => (
                "g++".to_string(),
                vec![
                    "-std=c++14".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            (CppLinker::Gcc, "c++17") => (
                "g++".to_string(),
                vec![
                    "-std=c++17".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            
            // Clang configurations
            (CppLinker::Clang, "c++11") => (
                "clang++".to_string(),
                vec![
                    "-std=c++11".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            (CppLinker::Clang, "c++14") => (
                "clang++".to_string(),
                vec![
                    "-std=c++14".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            (CppLinker::Clang, "c++17") => (
                "clang++".to_string(),
                vec![
                    "-std=c++17".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
            
            // MSVC configurations
            (CppLinker::Msvc, "c++11") => (
                "cl".to_string(),
                vec![
                    "/std:c++11".to_string(),
                    "/W4".to_string(),
                    "/O2".to_string(),
                    "/EHsc".to_string(),
                ]
            ),
            (CppLinker::Msvc, "c++14") => (
                "cl".to_string(),
                vec![
                    "/std:c++14".to_string(),
                    "/W4".to_string(),
                    "/O2".to_string(),
                    "/EHsc".to_string(),
                ]
            ),
            (CppLinker::Msvc, "c++17") => (
                "cl".to_string(),
                vec![
                    "/std:c++17".to_string(),
                    "/W4".to_string(),
                    "/O2".to_string(),
                    "/EHsc".to_string(),
                ]
            ),
            
            // Default fallback
            _ => (
                "g++".to_string(),
                vec![
                    "-std=c++11".to_string(),
                    "-Wall".to_string(),
                    "-Wextra".to_string(),
                    "-O2".to_string(),
                ]
            ),
        };
        
        Self {
            linker,
            version: version.to_string(),
            command,
            args,
            available: false, // Will be checked later
        }
    }

    pub fn check_availability(&mut self) -> bool {
        self.available = find_executable(&self.command).is_some();
        self.available
    }

    pub fn get_full_command(&self, source_file: &str, output_file: &str) -> Vec<String> {
        let mut cmd = vec![self.command.clone()];
        
        // Add version-specific flags
        for arg in &self.args {
            if arg != "-o" && arg != "/Fe:" {
                cmd.push(arg.clone());
            }
        }
        
        // Add output file
        if self.command == "cl" {
            // MSVC uses /Fe:output
            cmd.push(format!("/Fe:{}", output_file));
        } else {
            // GCC/Clang use -o output
            cmd.push("-o".to_string());
            cmd.push(output_file.to_string());
        }
        
        // Add source file
        cmd.push(source_file.to_string());
        
        cmd
    }

    pub fn get_description(&self) -> String {
        format!("{} {} ({})", 
            self.command, 
            self.version, 
            match self.linker {
                CppLinker::Gcc => "GCC",
                CppLinker::Clang => "Clang",
                CppLinker::Msvc => "MSVC",
            }
        )
    }
}

pub struct CppLinkerManager {
    pub configs: Vec<CppLinkerConfig>,
}

impl CppLinkerManager {
    pub fn new() -> Self {
        let mut configs = Vec::new();
        
        // Priorizar Clang ya que está disponible
        let linkers = vec![CppLinker::Clang, CppLinker::Gcc, CppLinker::Msvc];
        let versions = vec!["c++11", "c++14", "c++17"];
        
        for linker in linkers {
            for version in &versions {
                let mut config = CppLinkerConfig::new(linker.clone(), version);
                config.check_availability();
                configs.push(config);
            }
        }
        
        Self { configs }
    }
    
    pub fn get_best_config_for_version(&self, version: &str) -> Option<&CppLinkerConfig> {
        // Try to find available config for the requested version
        for config in &self.configs {
            if config.version == version && config.available {
                return Some(config);
            }
        }
        None
    }
    
    pub fn compile_and_link(
        &self,
        source_file: &Path,
        output_file: &str,
        version: &str,
        work_dir: &Path,
        output: &mut String,
    ) -> Result<PathBuf, String> {
        if let Some(config) = self.get_best_config_for_version(version) {
            let source_file_str = source_file.to_string_lossy();
            let output_path = work_dir.join(output_file);
            
            let cmd = config.get_full_command(&source_file_str, output_file);
            
            output.push_str(&format!(">>> Usando compilador: {}\n", config.get_description()));
            output.push_str(&format!(">>> Comando: {}\n", cmd.join(" ")));
            
            match Command::new(&cmd[0])
                .args(&cmd[1..])
                .current_dir(work_dir)
                .output()
            {
                Ok(result) => {
                    if result.status.success() {
                        output.push_str(">>> ✅ Compilación exitosa\n");
                        Ok(output_path)
                    } else {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        let stdout = String::from_utf8_lossy(&result.stdout);
                        output.push_str(&format!("❌ Error de compilación:\n{}\n", stderr));
                        if !stdout.is_empty() {
                            output.push_str(&format!("Salida: {}\n", stdout));
                        }
                        Err(format!("Error de compilación: {}", stderr))
                    }
                }
                Err(e) => {
                    output.push_str(&format!("❌ Error ejecutando compilador: {}\n", e));
                    Err(format!("Error ejecutando compilador: {}", e))
                }
            }
        } else {
            Err(format!("No hay compilador disponible para {}", version))
        }
    }
    
    pub fn get_status_report(&self) -> String {
        let mut report = String::new();
        report.push_str("📊 Estado de los compiladores C++:\n\n");
        
        for config in &self.configs {
            let status = if config.available { "✅ Disponible" } else { "❌ No encontrado" };
            report.push_str(&format!("{} {}\n", config.get_description(), status));
        }
        
        report.push_str("\n📋 Versiones C++ soportadas:\n");
        report.push_str("   • C++11 (2011) - Fundamentos modernos\n");
        report.push_str("   • C++14 (2014) - Mejoras intermedias\n");
        report.push_str("   • C++17 (2017) - Características modernas\n");
        
        report
    }
}

/// Buscar compiladores C++ en rutas comunes del sistema
pub fn find_cpp_compilers() -> Vec<String> {
    let mut compilers = Vec::new();
    
    // 1. Buscar en PATH del sistema
    if let Ok(path) = std::env::var("PATH") {
        for path_dir in path.split(';') {
            let compilers_in_dir = search_compilers_in_directory(path_dir);
            compilers.extend(compilers_in_dir);
        }
    }
    
    // 2. Buscar en rutas comunes de Windows
    let common_paths = vec![
        r"C:\MinGW\bin",
        r"C:\msys64\mingw64\bin", 
        r"C:\msys64\mingw32\bin",
        r"C:\TDM-GCC-64\bin",
        r"C:\TDM-GCC-32\bin",
        r"C:\cygwin64\bin",
        r"C:\Program Files\LLVM\bin",
        r"C:\Program Files (x86)\LLVM\bin",
    ];
    
    for path_dir in common_paths {
        if std::path::Path::new(path_dir).exists() {
            let compilers_in_dir = search_compilers_in_directory(path_dir);
            compilers.extend(compilers_in_dir);
        }
    }
    
    // Eliminar duplicados y ordenar
    compilers.sort();
    compilers.dedup();
    
    compilers
}

/// Buscar compiladores en un directorio específico
fn search_compilers_in_directory(dir: &str) -> Vec<String> {
    let mut compilers = Vec::new();
    
    let compiler_names = vec![
        "g++.exe", "g++.bat", "g++.cmd",
        "clang++.exe", "clang++.bat", "clang++.cmd", 
        "cl.exe", "cl.bat", "cl.cmd",
        "gcc.exe", "gcc.bat", "gcc.cmd",
        "clang.exe", "clang.bat", "clang.cmd",
    ];
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if compiler_names.iter().any(|&name| file_name.to_lowercase() == name.to_lowercase()) {
                    let full_path = entry.path().to_string_lossy().into_owned();
                    compilers.push(full_path);
                }
            }
        }
    }
    
    compilers
}

/// Encontrar ejecutable en el sistema
fn find_executable(command: &str) -> Option<String> {
    // First try to find it in PATH
    if let Ok(output) = Command::new("where").arg(command).output() {
        if output.status.success() {
            let paths = String::from_utf8_lossy(&output.stdout);
            for line in paths.lines() {
                if !line.trim().is_empty() {
                    return Some(line.trim().to_string());
                }
            }
        }
    }
    
    // Try common paths
    let common_paths = find_cpp_compilers();
    for path in common_paths {
        if path.contains(command) {
            return Some(path);
        }
    }
    
    None
}

/// Detectar automáticamente la versión C++ del código
pub fn detect_cpp_version(code: &str) -> &'static str {
    // Verificar si el código usa cabeceras modernas de C++ que chocan con C++11
    let uses_modern_headers = code.contains("#include <iostream>") ||
                             code.contains("#include <vector>") ||
                             code.contains("#include <string>") ||
                             code.contains("#include <memory>") ||
                             code.contains("#include <algorithm>") ||
                             code.contains("#include <map>") ||
                             code.contains("#include <set>") ||
                             code.contains("std::");
    
    // Buscar características específicas de cada versión
    if code.contains("std::invoke") || 
       code.contains("std::apply") || 
       code.contains("std::make_from_tuple") ||
       code.contains("constexpr if") ||
       code.contains("inline variable") ||
       code.contains("[[nodiscard]]") ||
       code.contains("std::string_view") {
        "c++17"
    } else if code.contains("auto") && 
              code.contains("->") && 
              (code.contains("decltype") || 
               code.contains("std::move") ||
               code.contains("std::forward") ||
               code.contains("constexpr") ||
               code.contains("noexcept") ||
               code.contains("override") ||
               code.contains("final")) {
        "c++14"
    } else if uses_modern_headers && 
              (code.contains("auto") || 
               code.contains("nullptr") ||
               code.contains("std::unique_ptr") ||
               code.contains("std::shared_ptr") ||
               code.contains("std::make_shared") ||
               code.contains("std::make_unique") ||
               code.contains("std::array") ||
               code.contains("std::tuple") ||
               (code.contains("for.*:") && code.contains("auto")) ||
               code.contains("lambda") ||
               code.contains("[](")) {
        "c++14" // Código C++11 con cabeceras modernas necesita C++14 mínimo
    } else if code.contains("auto") || 
              code.contains("nullptr") ||
              code.contains("std::unique_ptr") ||
              code.contains("std::shared_ptr") ||
              code.contains("std::make_shared") ||
              code.contains("std::make_unique") ||
              code.contains("std::array") ||
              code.contains("std::tuple") ||
              (code.contains("for.*:") && code.contains("auto")) ||
              code.contains("lambda") ||
              code.contains("[](") {
        "c++11" // C++11 puro sin cabeceras modernas
    } else {
        "c++11" // Default a C++11 para código simple
    }
}

// Función de utilidad para obtener el mejor linker automáticamente
pub fn get_best_cpp_linker() -> Option<CppLinkerConfig> {
    let manager = CppLinkerManager::new();
    
    // Prioridad: C++17 > C++14 > C++11
    for version in ["c++17", "c++14", "c++11"] {
        if let Some(config) = manager.get_best_config_for_version(version) {
            return Some(config.clone());
        }
    }
    
    None
}

// Función de utilidad para compilar código C++ con detección automática
pub fn compile_cpp_auto(
    source_file: &Path,
    output_file: &str,
    work_dir: &Path,
    output: &mut String,
) -> Result<PathBuf, String> {
    let manager = CppLinkerManager::new();
    
    // Intentar con C++17 primero, luego retroceder
    for version in ["c++17", "c++14", "c++11"] {
        match manager.compile_and_link(source_file, output_file, version, work_dir, output) {
            Ok(path) => return Ok(path),
            Err(_) => {
                output.push_str(&format!("⚠️ Intentando con versión anterior...\n"));
                continue;
            }
        }
    }
    
    Err("❌ No se pudo compilar con ninguna versión C++".to_string())
}
