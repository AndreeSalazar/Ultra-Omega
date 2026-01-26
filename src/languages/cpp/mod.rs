// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: C++ Language Support
// Soporte completo para C++11, C++14, C++17 con detección automática
// ═══════════════════════════════════════════════════════════════════════════════

use std::process::Command;
use std::path::PathBuf;
use crate::compilation::compiler_detector::{CompilerInfo, find_executable};

pub mod templates;
pub use templates::CppVersionManager;

#[derive(Debug, Clone, PartialEq)]
pub enum CppVersion {
    Cpp11,
    Cpp14,
    Cpp17,
    Auto, // Detectar automáticamente
}

impl CppVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            CppVersion::Cpp11 => "c++11",
            CppVersion::Cpp14 => "c++14", 
            CppVersion::Cpp17 => "c++17",
            CppVersion::Auto => "c++17", // Por defecto la más moderna
        }
    }

    pub fn as_flag(&self) -> String {
        format!("-std={}", self.as_str())
    }

    pub fn get_features(&self) -> Vec<&'static str> {
        match self {
            CppVersion::Cpp11 => vec![
                "auto", "lambda", "smart_pointers", "move_semantics", 
                "range_based_for", "constexpr", "noexcept", "enum_class"
            ],
            CppVersion::Cpp14 => vec![
                "auto", "lambda", "smart_pointers", "move_semantics",
                "range_based_for", "constexpr", "noexcept", "enum_class",
                "generic_lambdas", "auto_return_type", "variable_templates",
                "std_make_unique", "chrono_literals"
            ],
            CppVersion::Cpp17 => vec![
                "auto", "lambda", "smart_pointers", "move_semantics",
                "range_based_for", "constexpr", "noexcept", "enum_class",
                "generic_lambdas", "auto_return_type", "variable_templates",
                "std_make_unique", "chrono_literals", "structured_bindings",
                "std_optional", "std_variant", "std_string_view", "if_constexpr",
                "std_filesystem", "parallel_algorithms", "fold_expressions"
            ],
            CppVersion::Auto => vec![], // Se determina en runtime
        }
    }
}

#[derive(Debug, Clone)]
pub struct CppCompiler {
    pub version: CppVersion,
    pub compiler_type: CppCompilerType,
    pub info: CompilerInfo,
}

#[derive(Debug, Clone)]
pub enum CppCompilerType {
    GCC,
    Clang,
    MSVC,
}

impl CppCompiler {
    pub fn new(version: CppVersion) -> Option<Self> {
        // Intentar encontrar el mejor compilador disponible
        if let Some(gcc_info) = Self::find_gcc() {
            return Some(Self {
                version,
                compiler_type: CppCompilerType::GCC,
                info: gcc_info,
            });
        }

        if let Some(clang_info) = Self::find_clang() {
            return Some(Self {
                version,
                compiler_type: CppCompilerType::Clang,
                info: clang_info,
            });
        }

        #[cfg(target_os = "windows")]
        if let Some(msvc_info) = Self::find_msvc() {
            return Some(Self {
                version,
                compiler_type: CppCompilerType::MSVC,
                info: msvc_info,
            });
        }

        None
    }

    fn find_gcc() -> Option<CompilerInfo> {
        let commands = vec!["g++", "gcc", "mingw32-g++", "x86_64-w64-mingw32-g++"];
        
        for cmd in commands {
            if let Some(path) = find_executable(cmd) {
                if let Ok(output) = Command::new(&path).arg("--version").output() {
                    if output.status.success() {
                        let version = String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .next()
                            .unwrap_or("Unknown")
                            .to_string();
                        
                        return Some(CompilerInfo {
                            name: format!("GCC ({})", cmd),
                            version,
                            path: Some(path),
                            available: true,
                            error_message: None,
                        });
                    }
                }
            }
        }
        None
    }

    fn find_clang() -> Option<CompilerInfo> {
        let commands = vec!["clang++", "clang"];
        
        for cmd in commands {
            if let Some(path) = find_executable(cmd) {
                if let Ok(output) = Command::new(&path).arg("--version").output() {
                    if output.status.success() {
                        let version = String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .next()
                            .unwrap_or("Unknown")
                            .to_string();
                        
                        return Some(CompilerInfo {
                            name: format!("Clang ({})", cmd),
                            version,
                            path: Some(path),
                            available: true,
                            error_message: None,
                        });
                    }
                }
            }
        }
        None
    }

    #[cfg(target_os = "windows")]
    fn find_msvc() -> Option<CompilerInfo> {
        // Implementación básica para MSVC
        if let Ok(output) = Command::new("cl").arg("/?").output() {
            if output.status.success() || output.status.code().is_some() {
                return Some(CompilerInfo {
                    name: "MSVC (cl.exe)".to_string(),
                    version: "Visual Studio".to_string(),
                    path: Some(PathBuf::from("cl.exe")),
                    available: true,
                    error_message: None,
                });
            }
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    fn find_msvc() -> Option<CompilerInfo> {
        None // MSVC solo disponible en Windows
    }

    pub fn compile(&self, source_file: &str, output_file: &str, work_dir: &str) -> Result<String, String> {
        let source_path = std::path::Path::new(source_file);
        let output_path = std::path::Path::new(output_file);
        let work_path = std::path::Path::new(work_dir);

        if !source_path.exists() {
            return Err(format!("Archivo fuente no encontrado: {}", source_file));
        }

        // Construir comando de compilación
        let mut cmd = Command::new(&self.info.path.as_ref().unwrap());
        cmd.current_dir(work_path);

        // Argumentos base según el tipo de compilador
        match self.compiler_type {
            CppCompilerType::GCC | CppCompilerType::Clang => {
                cmd.args([
                    "-o", output_file,
                    source_file,
                    &self.version.as_flag(),
                    "-Wall",  // Warnings
                    "-Wextra", // Más warnings
                    "-O2",    // Optimización
                ]);

                // Agregar librerías específicas de Windows si es necesario
                #[cfg(target_os = "windows")]
                {
                    cmd.args(["-static-libgcc", "-static-libstdc++"]);
                }
            }
            CppCompilerType::MSVC => {
                cmd.args([
                    "/Fe:", output_file,
                    source_file,
                    "/std:cpp17", // MSVC usa diferente formato
                    "/W3",        // Warnings nivel 3
                    "/O2",        // Optimización
                ]);
            }
        }

        // Ejecutar compilación
        let output = cmd.output()
            .map_err(|e| format!("Error al ejecutar compilador: {}", e))?;

        if output.status.success() {
            Ok(format!("✅ Compilación exitosa: {}", output_file))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(format!("❌ Error de compilación:\nSTDOUT:\n{}\nSTDERR:\n{}", stdout, stderr))
        }
    }

    pub fn get_compile_command(&self, source_file: &str, output_file: &str) -> String {
        let path = self.info.path.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "compilador".to_string());

        match self.compiler_type {
            CppCompilerType::GCC | CppCompilerType::Clang => {
                format!("{} -o {} {} {} -Wall -Wextra -O2", 
                    path, output_file, source_file, self.version.as_flag())
            }
            CppCompilerType::MSVC => {
                format!("{} /Fe:{} {} /std:cpp17 /W3 /O2", 
                    path, output_file, source_file)
            }
        }
    }

    pub fn detect_best_version(&self) -> CppVersion {
        match self.compiler_type {
            CppCompilerType::GCC | CppCompilerType::Clang => {
                // Intentar compilar con C++17 para ver si está disponible
                let test_code = r#"
#include <iostream>
#include <optional>
#include <variant>
int main() {
    std::optional<int> opt = 42;
    std::variant<int, float> var = 42;
    std::cout << "C++17 test";
    return 0;
}
"#;

                // Si la compilación con C++17 tiene éxito, usar C++17
                if self.can_compile_version(test_code, CppVersion::Cpp17) {
                    CppVersion::Cpp17
                } else if self.can_compile_version(test_code, CppVersion::Cpp14) {
                    CppVersion::Cpp14
                } else {
                    CppVersion::Cpp11
                }
            }
            CppCompilerType::MSVC => {
                // MSVC generalmente soporta C++17 por defecto
                CppVersion::Cpp17
            }
        }
    }

    fn can_compile_version(&self, test_code: &str, version: CppVersion) -> bool {
        // Crear archivo temporal
        let temp_file = "temp_test.cpp";
        let temp_exe = "temp_test.exe";
        
        if let Ok(_) = std::fs::write(temp_file, test_code) {
            let mut cmd = Command::new(&self.info.path.as_ref().unwrap());
            
            match self.compiler_type {
                CppCompilerType::GCC | CppCompilerType::Clang => {
                    cmd.args(["-o", temp_exe, temp_file, &version.as_flag()]);
                }
                CppCompilerType::MSVC => {
                    cmd.args(["/Fe:", temp_exe, temp_file, "/std:cpp17"]);
                }
            }

            let result = cmd.output();
            
            // Limpiar archivos temporales
            let _ = std::fs::remove_file(temp_file);
            let _ = std::fs::remove_file(temp_exe);
            
            result.map(|output| output.status.success()).unwrap_or(false)
        } else {
            false
        }
    }
}

pub fn get_available_cpp_compilers() -> Vec<CppCompiler> {
    let mut compilers = Vec::new();
    
    // Intentar con cada versión en orden de preferencia
    for version in [CppVersion::Cpp17, CppVersion::Cpp14, CppVersion::Cpp11] {
        if let Some(compiler) = CppCompiler::new(version.clone()) {
            // Detectar la mejor versión disponible
            let best_version = compiler.detect_best_version();
            let mut optimized_compiler = compiler;
            optimized_compiler.version = best_version;
            compilers.push(optimized_compiler);
            break; // Solo necesitamos un compilador
        }
    }
    
    compilers
}

pub fn get_cpp_status_summary() -> String {
    let compilers = get_available_cpp_compilers();
    
    if compilers.is_empty() {
        return "❌ No se encontraron compiladores C++\n\n💡 Instala uno de los siguientes:\n   - Windows: MinGW-w64 (https://www.mingw-w64.org/)\n   - Linux: sudo apt install build-essential\n   - macOS: xcode-select --install".to_string();
    }

    let mut summary = String::from("✅ Compilador C++ encontrado:\n\n");
    
    for compiler in &compilers {
        summary.push_str(&format!("🔧 Compilador: {}\n", compiler.info.name));
        summary.push_str(&format!("📦 Versión: {}\n", compiler.info.version));
        summary.push_str(&format!("🎯 Estándar: {}\n", compiler.version.as_str()));
        summary.push_str(&format!("📍 Ruta: {}\n", 
            compiler.info.path.as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or("N/A".to_string())));
        
        if let Some(path) = &compiler.info.path {
            summary.push_str(&format!("⚡ Comando: {}\n", 
                compiler.get_compile_command("source.cpp", "output.exe")));
        }
        
        summary.push_str("\n");
    }
    
    summary.push_str("🚀 El compilador C++ está listo para usar!\n");
    summary
}
