use std::process::Command;
use std::path::{Path, PathBuf};
use crate::compilation::compiler_detector::deep_search_executable;

#[cfg(target_os = "windows")]
const EXE_EXTENSION: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const EXE_EXTENSION: &str = "";

pub struct TerminalManager {
    pub rust_output: String,
    pub active_tab: TerminalTab,
    pub visible: bool,
    pub pinned: bool,
    pub hide_timer: f32,
}

#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum TerminalTab {
    #[default]
    Rust,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self {
            rust_output: String::new(),
            active_tab: TerminalTab::default(),
            visible: false,
            pinned: false,
            hide_timer: 0.0,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Language {
    Rust,
}

impl TerminalManager {
    /// Buscar un comando de compilador (con búsqueda profunda)
    fn find_compiler_cmd(cmd: &str, _output: &mut String) -> Option<String> {
        use crate::compilation::compiler_detector::find_executable;
        
        // Primero intentar en PATH
        if let Some(path) = find_executable(cmd) {
            match Command::new(&path).arg("--version").output()
                .or_else(|_| Command::new(&path).arg("-v").output()) {
                Ok(out) => {
                    if out.status.success() || out.status.code().is_some() {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
                Err(_) => {}
            }
        }
        
        // Si no funciona, búsqueda profunda
        if let Some(path) = deep_search_executable(cmd) {
            match Command::new(&path).arg("--version").output()
                .or_else(|_| Command::new(&path).arg("-v").output()) {
                Ok(out) => {
                    if out.status.success() || out.status.code().is_some() {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
                Err(_) => {}
            }
        }
        
        None
    }
    
    pub fn run_code(&mut self, code: &str, lang: Language, workspace_path: Option<&PathBuf>) {
        self.visible = true;
        self.hide_timer = 10.0;
        
        let output_buffer = &mut self.rust_output;
        self.active_tab = TerminalTab::Rust;
        output_buffer.clear();
        
        // Header visual mejorado
        output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
        output_buffer.push_str("║              🚀 INICIANDO PROCESO DE COMPILACIÓN RUST        ║\n");
        output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
        
        // Determinar el directorio de trabajo
        let work_dir = workspace_path
            .map(|p| p.clone())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        output_buffer.push_str(&format!("📁 Directorio: {}\n\n", work_dir.display()));

        let exe_file = format!("program{}", EXE_EXTENSION);
        let exe_path = work_dir.join(&exe_file);
        let exe_file_str = exe_file.as_str();

        Self::compile_rust(code, &work_dir, exe_file_str, output_buffer);

        // Run if compiled
        if exe_path.exists() {
            // Obtener tamaño del ejecutable
            let exe_size = if let Ok(metadata) = std::fs::metadata(&exe_path) {
                metadata.len()
            } else {
                0
            };
            
            // Separador visual antes de ejecución
            output_buffer.push_str("\n");
            output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
            output_buffer.push_str("║                    ▶️  EJECUTANDO PROGRAMA                    ║\n");
            output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
            
            output_buffer.push_str(&format!("📦 Ejecutable: {}\n", exe_file));
            output_buffer.push_str(&format!("📏 Tamaño: {} bytes ({:.2} KB)\n", exe_size, exe_size as f64 / 1024.0));
            output_buffer.push_str("⏳ Ejecutando...\n\n");
            
            let start_time = std::time::Instant::now();
            let mut cmd = Command::new(&exe_path);
            cmd.current_dir(&work_dir);
            
            match cmd.output() {
                Ok(run_out) => {
                    let elapsed = start_time.elapsed();
                    
                    let stdout_str = String::from_utf8_lossy(&run_out.stdout);
                    let stderr_str = String::from_utf8_lossy(&run_out.stderr);
                    
                    output_buffer.push_str("\n");
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    output_buffer.push_str("║                  📊 RESULTADOS DEL PROGRAMA                    ║\n");
                    output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                    
                    if !stdout_str.is_empty() {
                        output_buffer.push_str("📤 SALIDA ESTÁNDAR (Resultado del programa):\n");
                        output_buffer.push_str("═══════════════════════════════════════════════════════════════\n");
                        for line in stdout_str.lines() {
                            output_buffer.push_str(&format!("  {}\n", line));
                        }
                        output_buffer.push_str("═══════════════════════════════════════════════════════════════\n\n");
                    }
                    
                    if !stderr_str.is_empty() {
                        output_buffer.push_str("⚠️  ERRORES/ADVERTENCIAS:\n");
                        output_buffer.push_str("─────────────────────────────────────────────────────────────\n");
                        for line in stderr_str.lines() {
                            output_buffer.push_str(&format!("  {}\n", line));
                        }
                        output_buffer.push_str("─────────────────────────────────────────────────────────────\n\n");
                    }
                    
                    let exit_code = run_out.status.code();
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    if let Some(code) = exit_code {
                        if code != 0 {
                            output_buffer.push_str(&format!("║  ❌ Estado: Error (código de salida: {})                  ║\n", code));
                        } else {
                            output_buffer.push_str("║  ✅ Estado: Ejecución exitosa                              ║\n");
                        }
                    } else {
                        output_buffer.push_str("║  ⚠️  Estado: Terminado por señal del sistema              ║\n");
                    }
                    output_buffer.push_str(&format!("║  ⏱️  Tiempo de ejecución: {:.2} ms                        ║\n", elapsed.as_millis()));
                    output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
                }
                Err(e) => {
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    output_buffer.push_str("║                    ❌ ERROR DE EJECUCIÓN                      ║\n");
                    output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                    output_buffer.push_str(&format!("Detalle: {}\n\n", e));
                    output_buffer.push_str("💡 Verifica que el ejecutable tenga permisos de ejecución.\n");
                }
            }
        }
    }

    fn compile_rust(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.rs");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let exe_path = work_dir.join(exe_file);
        
        // Buscar Rust con búsqueda profunda
        let rust_path = Self::find_compiler_cmd("rustc", output);
        if rust_path.is_none() {
            output.push_str(">>> Error: Rust no está instalado o no está en PATH.\n");
            output.push_str(">>> Instala Rust desde: https://rustup.rs/\n");
            return;
        }
        
        let rustc_cmd = rust_path.unwrap();
        output.push_str(&format!(">>> Rust detectado: {}\n", rustc_cmd));
        
        // Obtener versión
        match Command::new(&rustc_cmd).arg("--version").output() {
            Ok(out) => {
                let version = String::from_utf8_lossy(&out.stdout);
                if !version.trim().is_empty() {
                    output.push_str(&format!(">>> Versión: {}\n", version.trim()));
                }
            }
            Err(_) => {}
        }
        
        let cmd_output = Command::new(&rustc_cmd)
            .current_dir(work_dir)
            .args(&[temp_file.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "Rustc", &exe_path, output);
    }

    fn handle_compile_output(result: std::io::Result<std::process::Output>, name: &str, exe_file: &Path, output: &mut String) {
        match result {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                
                if !stdout.is_empty() {
                    output.push_str(&format!("{} Salida:\n", name));
                    output.push_str(&stdout);
                    if !stdout.ends_with('\n') {
                        output.push_str("\n");
                    }
                }
                
                if !stderr.is_empty() {
                    output.push_str(&format!("{} Errores/Advertencias:\n", name));
                    output.push_str(&stderr);
                    if !stderr.ends_with('\n') {
                        output.push_str("\n");
                    }
                }
                
                if out.status.success() {
                    output.push_str(&format!("✅ {}: Compilación exitosa.\n", name));
                } else {
                    output.push_str(&format!("❌ {}: Error de compilación.\n", name));
                    if let Some(code) = out.status.code() {
                        output.push_str(&format!(">>> Código de salida: {}\n", code));
                    }
                    
                    let _ = std::fs::remove_file(exe_file);
                }
            }
            Err(e) => {
                output.push_str(&format!(">>> Error invocando {}: {}\n", name, e));
                output.push_str(&format!(">>> Detalle: El ejecutable no se pudo ejecutar. Verifica que:\n"));
                output.push_str(&format!(">>>   1. El compilador esté instalado correctamente\n"));
                output.push_str(&format!(">>>   2. El compilador esté en el PATH o en una ubicación conocida\n"));
                output.push_str(&format!(">>>   3. Tengas permisos para ejecutar el compilador\n"));
            }
        }
    }
}
