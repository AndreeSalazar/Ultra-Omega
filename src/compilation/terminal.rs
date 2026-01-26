use crate::compilation::cpp_linker::{CppLinkerManager, compile_cpp_auto};
use std::process::Command;
use std::path::{Path, PathBuf};
use crate::expressions::channels::ChannelValue;

#[cfg(target_os = "windows")]
const EXE_EXTENSION: &str = ".exe";
#[cfg(not(target_os = "windows"))]
const EXE_EXTENSION: &str = "";

#[cfg(target_os = "windows")]
const OBJ_EXTENSION: &str = ".obj";
#[cfg(not(target_os = "windows"))]
const OBJ_EXTENSION: &str = ".o";

#[cfg(target_os = "windows")]
const NASM_FORMAT: &str = "win64";
#[cfg(not(target_os = "windows"))]
const NASM_FORMAT: &str = "elf64";

pub struct TerminalManager {
    pub asm_output: String,
    pub c_output: String,
    pub cpp_output: String,
    pub rust_output: String,
    pub zig_output: String,
    pub java_output: String,
    pub python_output: String,
    pub active_tab: TerminalTab,
    pub visible: bool,
    pub pinned: bool,
    pub hide_timer: f32,
    // Para Java: almacenar el nombre de la clase principal compilada
    pub java_main_class: Option<String>,
}

#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum TerminalTab {
    #[default]
    Nasm,
    Rust,
    Java,
    Python,
    Cpp,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self {
            asm_output: String::new(),
            c_output: String::new(),
            cpp_output: String::new(),
            rust_output: String::new(),
            zig_output: String::new(),
            java_output: String::new(),
            python_output: String::new(),
            active_tab: TerminalTab::default(),
            visible: false,
            pinned: false,
            hide_timer: 0.0,
            java_main_class: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Language {
    Nasm,
    Rust,
    Java,
    Python,
    Cpp,
}

impl TerminalManager {
    /// Buscar un comando de compilador (con búsqueda profunda)
    fn find_compiler_cmd(cmd: &str, _output: &mut String) -> Option<String> {
        use crate::compilation::compiler_detector::{find_executable, deep_search_executable};
        
        // Primero intentar en PATH
        if let Some(path) = find_executable(cmd) {
            // Verificar que funciona
            match Command::new(&path).arg("--version").output()
                .or_else(|_| Command::new(&path).arg("-v").output())
                .or_else(|_| {
                    // Para NASM
                    if cmd == "nasm" {
                        Command::new(&path).arg("-v").output()
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No version flag"))
                    }
                }) {
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
            // Verificar que funciona
            match Command::new(&path).arg("--version").output()
                .or_else(|_| Command::new(&path).arg("-v").output())
                .or_else(|_| {
                    if cmd == "nasm" {
                        Command::new(&path).arg("-v").output()
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No version flag"))
                    }
                }) {
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
        self.hide_timer = 10.0; // Show for 10 seconds
        
        let (output_buffer, tab) = match lang {
            Language::Nasm => (&mut self.asm_output, TerminalTab::Nasm),
            Language::Rust => (&mut self.rust_output, TerminalTab::Rust),
            Language::Java => (&mut self.java_output, TerminalTab::Java),
            Language::Python => (&mut self.python_output, TerminalTab::Python),
            Language::Cpp => (&mut self.cpp_output, TerminalTab::Cpp),
        };
        
        self.active_tab = tab;
        output_buffer.clear();
        
        // Header visual mejorado
        output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
        output_buffer.push_str("║              🚀 INICIANDO PROCESO DE COMPILACIÓN             ║\n");
        output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
        
        // Información del lenguaje
        let lang_name = match lang {
            Language::Nasm => "NASM (Assembly)",
            Language::Rust => "Rust",
            Language::Java => "Java 25",
            Language::Python => "Python 3.12",
            Language::Cpp => "C++ (11/14/17)",
        };
        output_buffer.push_str(&format!("📝 Lenguaje: {}\n", lang_name));
        
        // Determinar el directorio de trabajo
        let work_dir = workspace_path
            .map(|p| p.clone())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        output_buffer.push_str(&format!("📁 Directorio: {}\n\n", work_dir.display()));

        let exe_file = format!("program{}", EXE_EXTENSION);
        let exe_path = work_dir.join(&exe_file);
        let exe_file_str = exe_file.as_str();

        match lang {
            Language::Nasm => Self::compile_nasm(code, &work_dir, exe_file_str, output_buffer),
            Language::Rust => Self::compile_rust(code, &work_dir, exe_file_str, output_buffer),
            Language::Java => {
                if let Some(main_class) = Self::compile_java(code, &work_dir, output_buffer) {
                    self.java_main_class = Some(main_class.clone());
                    
                    // Separador visual antes de ejecución
                    output_buffer.push_str("\n");
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    output_buffer.push_str("║                    ▶️  EJECUTANDO PROGRAMA                    ║\n");
                    output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                    
                    output_buffer.push_str(&format!("🔍 Clase principal: {}\n", main_class));
                    output_buffer.push_str("⏳ Ejecutando...\n\n");
                    
                    let java_path = Self::find_compiler_cmd("java", output_buffer);
                    if let Some(java_cmd) = java_path {
                        // Medir tiempo de ejecución
                        let start_time = std::time::Instant::now();
                        
                        // Configurar variables de entorno para UTF-8
                        let mut cmd = Command::new(&java_cmd);
                        cmd.current_dir(&work_dir);
                        cmd.arg(&main_class);
                        
                        // En Windows, configurar codificación UTF-8
                        #[cfg(target_os = "windows")]
                        {
                            cmd.env("JAVA_TOOL_OPTIONS", "-Dfile.encoding=UTF-8");
                            cmd.env("PYTHONIOENCODING", "utf-8");
                        }
                        
                        match cmd.output() {
                            Ok(run_out) => {
                                let elapsed = start_time.elapsed();
                                
                                // Decodificar correctamente como UTF-8
                                let stdout_str = String::from_utf8_lossy(&run_out.stdout);
                                let stderr_str = String::from_utf8_lossy(&run_out.stderr);
                                
                                // Sección de resultados mejorada con más espacio y claridad
                                output_buffer.push_str("\n");
                                output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                                output_buffer.push_str("║                  📊 RESULTADOS DEL PROGRAMA                    ║\n");
                                output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                                
                                if !stdout_str.is_empty() {
                                    output_buffer.push_str("📤 SALIDA ESTÁNDAR (Resultado del programa):\n");
                                    output_buffer.push_str("═══════════════════════════════════════════════════════════════\n");
                                    output_buffer.push_str("\n");
                                    // Mostrar cada línea de salida de forma destacada con mejor formato
                                    let lines: Vec<&str> = stdout_str.lines().collect();
                                    for (i, line) in lines.iter().enumerate() {
                                        // Agregar indentación y formato mejorado
                                        if line.trim().is_empty() {
                                            output_buffer.push_str("\n");
                                        } else {
                                            output_buffer.push_str(&format!("  {}\n", line));
                                        }
                                    }
                                    // Si hay una línea sin salto al final del stdout_str original
                                    if !stdout_str.ends_with('\n') && !stdout_str.is_empty() && lines.is_empty() {
                                        // Si no hay líneas procesadas, usar el texto completo
                                        output_buffer.push_str(&format!("  {}\n", stdout_str));
                                    }
                                    output_buffer.push_str("\n");
                                    output_buffer.push_str("═══════════════════════════════════════════════════════════════\n");
                                    output_buffer.push_str("\n");
                                }
                                
                                if !stderr_str.is_empty() {
                                    output_buffer.push_str("⚠️  ERRORES/ADVERTENCIAS:\n");
                                    output_buffer.push_str("─────────────────────────────────────────────────────────────\n");
                                    output_buffer.push_str("\n");
                                    for line in stderr_str.lines() {
                                        output_buffer.push_str(&format!("  {}\n", line));
                                    }
                                    if !stderr_str.ends_with('\n') && !stderr_str.is_empty() {
                                        let last_line = stderr_str.lines().last().unwrap_or("");
                                        if !last_line.is_empty() {
                                            output_buffer.push_str(&format!("  {}\n", last_line));
                                        }
                                    }
                                    output_buffer.push_str("\n");
                                    output_buffer.push_str("─────────────────────────────────────────────────────────────\n");
                                    output_buffer.push_str("\n");
                                }
                                
                                // Estado final mejorado
                                output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                                if let Some(exit_code) = run_out.status.code() {
                                    if exit_code == 0 {
                                        output_buffer.push_str("║  ✅ Estado: Ejecución exitosa                              ║\n");
                                    } else {
                                        output_buffer.push_str(&format!("║  ❌ Estado: Error (código de salida: {})                  ║\n", exit_code));
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
                                output_buffer.push_str("💡 Sugerencias:\n");
                                output_buffer.push_str(&format!("   1. Verifica que la clase '{}' existe y está compilada\n", main_class));
                                output_buffer.push_str("   2. Asegúrate de tener el JRE (Java Runtime Environment) instalado\n");
                                output_buffer.push_str("   3. Verifica que 'java' esté en tu PATH\n");
                            }
                        }
                    } else {
                        output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                        output_buffer.push_str("║                    ❌ ERROR: JAVA NO ENCONTRADO                 ║\n");
                        output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                        output_buffer.push_str("No se encontró 'java' en PATH.\n");
                        output_buffer.push_str("Asegúrate de tener el JDK instalado.\n");
                    }
                }
                return;
            }
            Language::Python => {
                // Por ahora ejecutar sin contexto (sin ChannelManager)
                // En el futuro, se puede pasar contexto desde app.rs
                Self::run_python(code, &work_dir, output_buffer);
                return;
            }
            Language::Cpp => {
                Self::compile_cpp(code, &work_dir, exe_file_str, output_buffer);
                return;
            }
        }

        // Run if compiled (para Java se maneja por separado)
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
            
            // En Windows, configurar codificación UTF-8
            #[cfg(target_os = "windows")]
            {
                cmd.env("PYTHONIOENCODING", "utf-8");
            }
            
            match cmd.output() {
                Ok(run_out) => {
                    let elapsed = start_time.elapsed();
                    
                    // Decodificar correctamente como UTF-8
                    let stdout_str = String::from_utf8_lossy(&run_out.stdout);
                    let stderr_str = String::from_utf8_lossy(&run_out.stderr);
                    
                    // Sección de resultados mejorada con más espacio y claridad
                    output_buffer.push_str("\n");
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    output_buffer.push_str("║                  📊 RESULTADOS DEL PROGRAMA                    ║\n");
                    output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                    
                    if !stdout_str.is_empty() {
                        output_buffer.push_str("📤 SALIDA ESTÁNDAR (Resultado del programa):\n");
                        output_buffer.push_str("═══════════════════════════════════════════════════════════════\n");
                        output_buffer.push_str("\n");
                        // Mostrar cada línea de salida de forma destacada
                        for line in stdout_str.lines() {
                            output_buffer.push_str(&format!("  {}\n", line));
                        }
                        // Si hay una línea sin salto al final
                        if !stdout_str.ends_with('\n') && !stdout_str.is_empty() {
                            let last_line = stdout_str.lines().last().unwrap_or("");
                            if !last_line.is_empty() {
                                output_buffer.push_str(&format!("  {}\n", last_line));
                            }
                        }
                        output_buffer.push_str("\n");
                        output_buffer.push_str("═══════════════════════════════════════════════════════════════\n");
                        output_buffer.push_str("\n");
                    }
                    
                    if !stderr_str.is_empty() {
                        output_buffer.push_str("⚠️  ERRORES/ADVERTENCIAS:\n");
                        output_buffer.push_str("─────────────────────────────────────────────────────────────\n");
                        output_buffer.push_str("\n");
                        for line in stderr_str.lines() {
                            output_buffer.push_str(&format!("  {}\n", line));
                        }
                        if !stderr_str.ends_with('\n') && !stderr_str.is_empty() {
                            let last_line = stderr_str.lines().last().unwrap_or("");
                            if !last_line.is_empty() {
                                output_buffer.push_str(&format!("  {}\n", last_line));
                            }
                        }
                        output_buffer.push_str("\n");
                        output_buffer.push_str("─────────────────────────────────────────────────────────────\n");
                        output_buffer.push_str("\n");
                    }
                    
                    // Analizar el código de salida
                    let exit_code = run_out.status.code();
                    output_buffer.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                    if let Some(code) = exit_code {
                        if code == 139 || code == -11 {
                            // SIGSEGV (segmentation fault)
                            output_buffer.push_str("║  ❌ Estado: SEGMENTATION FAULT detectado!                ║\n");
                            output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                            output_buffer.push_str("🔍 Posibles causas:\n");
                            #[cfg(not(target_os = "windows"))]
                            {
                                output_buffer.push_str("   1. Código escrito para Windows ejecutándose en Linux\n");
                                output_buffer.push_str("      - Windows usa: mov rcx, arg (primer argumento)\n");
                                output_buffer.push_str("      - Linux usa:   mov rdi, arg (primer argumento)\n");
                                output_buffer.push_str("   2. Pila mal alineada (debe ser múltiplo de 16 bytes)\n");
                                output_buffer.push_str("   3. Acceso a memoria inválida\n\n");
                                output_buffer.push_str("💡 Solución: Adapta el código para Linux:\n");
                                output_buffer.push_str("   - Cambia 'mov rcx' por 'mov rdi' para primer argumento\n");
                                output_buffer.push_str("   - Cambia 'mov rdx' por 'mov rsi' para segundo argumento\n");
                                output_buffer.push_str("   - Asegura alineación de pila (sub rsp, 8 o múltiplo de 16)\n");
                            }
                            #[cfg(target_os = "windows")]
                            {
                                output_buffer.push_str("   1. Acceso a memoria inválida\n");
                                output_buffer.push_str("   2. Pila mal alineada\n");
                                output_buffer.push_str("   3. Argumentos incorrectos en llamadas a funciones\n");
                            }
                        } else if code != 0 {
                            output_buffer.push_str(&format!("║  ❌ Estado: Error (código de salida: {})                  ║\n", code));
                            output_buffer.push_str(&format!("║  ⏱️  Tiempo de ejecución: {:.2} ms                        ║\n", elapsed.as_millis()));
                            output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
                        } else {
                            output_buffer.push_str("║  ✅ Estado: Ejecución exitosa                              ║\n");
                            output_buffer.push_str(&format!("║  ⏱️  Tiempo de ejecución: {:.2} ms                        ║\n", elapsed.as_millis()));
                            output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
                        }
                    } else {
                        // Proceso terminado por señal
                        output_buffer.push_str("║  ⚠️  Estado: Terminado por señal del sistema              ║\n");
                        output_buffer.push_str(&format!("║  ⏱️  Tiempo de ejecución: {:.2} ms                        ║\n", elapsed.as_millis()));
                        output_buffer.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
                    }
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

    fn compile_c(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.c");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let exe_path = work_dir.join(exe_file);
        
        // Buscar compilador C disponible
        let gcc_path = Self::find_compiler_cmd("gcc", output);
        if gcc_path.is_none() {
            output.push_str(">>> Error: No se encontró GCC.\n");
            output.push_str(">>> Instala MinGW-w64 (Windows) o gcc (Linux/Mac)\n");
            return;
        }
        
        let gcc_cmd = gcc_path.unwrap();
        output.push_str(&format!(">>> Usando compilador: {}\n", gcc_cmd));
        
        let cmd_output = Command::new(&gcc_cmd)
            .current_dir(work_dir)
            .args(&[temp_file.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "GCC", &exe_path, output);
    }

    fn compile_cpp(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.cpp");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        // Usar el nuevo sistema de linkers C++
        output.push_str("🔷 Usando sistema avanzado de linkers C++...\n");
        
        match compile_cpp_auto(&temp_file, exe_file, work_dir, output) {
            Ok(exe_path) => {
                output.push_str(&format!("✅ Compilación exitosa: {}\n", exe_path.display()));
                output.push_str("🚀 Ejecutable listo para correr\n");
            }
            Err(e) => {
                output.push_str(&format!("❌ Error de compilación: {}\n", e));
                output.push_str("💡 Verifica que tengas un compilador C++ instalado\n");
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
            #[cfg(target_os = "windows")]
            {
                output.push_str(">>>   Ejecuta: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n");
                output.push_str(">>>   O descarga el instalador desde rustup.rs\n");
            }
            #[cfg(not(target_os = "windows"))]
            {
                output.push_str(">>>   Ejecuta: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n");
            }
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
                // Decodificar correctamente como UTF-8 para soportar caracteres especiales
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                
                // Mostrar stdout si hay contenido
                if !stdout.is_empty() {
                    output.push_str(&format!("{} Salida:\n", name));
                    output.push_str(&stdout);
                    if !stdout.ends_with('\n') {
                        output.push_str("\n");
                    }
                }
                
                // Mostrar stderr si hay contenido
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
                    
                    // Si no hay stderr pero falló, mostrar más información
                    if stderr.is_empty() && stdout.is_empty() {
                        output.push_str(">>> (No se recibió salida del compilador. Verifica que el compilador esté correctamente instalado)\n");
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

    fn compile_nasm(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let exe_file_str = exe_file; // Mantener como referencia
        let temp_asm = work_dir.join("temp.asm");
        let temp_obj = work_dir.join(format!("temp{}", OBJ_EXTENSION));
        let exe_path = work_dir.join(exe_file);

        // Convertir código Windows a Linux automáticamente
        #[cfg(not(target_os = "windows"))]
        let code_to_compile = {
            let code_lower = code.to_lowercase();
            let is_windows_code = code_lower.contains("mov rcx") || 
                                  code_lower.contains("mov ecx") ||
                                  (code_lower.contains("win64") && !code_lower.contains("elf64"));
            
            if is_windows_code {
                output.push_str("⚠️  Código Windows detectado - Convirtiendo a Linux automáticamente...\n");
                output.push_str("   Cambios aplicados:\n");
                output.push_str("   - mov rcx → mov rdi (primer argumento)\n");
                output.push_str("   - mov rdx → mov rsi (segundo argumento)\n");
                output.push_str("   - Ajuste de alineación de pila\n\n");
                
                // Conversión automática
                let mut converted = code.to_string();
                
                // Reemplazar convenciones de llamadas
                // Windows: rcx, rdx, r8, r9
                // Linux:   rdi, rsi, rdx, rcx
                
                // Primer argumento: rcx → rdi (solo en contexto de llamadas)
                converted = converted.replace("mov rcx,", "mov rdi,");
                converted = converted.replace("mov ecx,", "mov edi,");
                
                // Segundo argumento: rdx → rsi (solo si no es el tercero)
                // Nota: Esto es más complejo, por ahora solo convertimos casos comunes
                if converted.contains("mov rdx,") && !converted.contains("mov rsi,") {
                    // Solo si no hay rsi ya, reemplazar el primer rdx
                    let lines: Vec<&str> = converted.lines().collect();
                    let mut new_lines = Vec::new();
                    let mut rdx_found = false;
                    for line in lines {
                        if !rdx_found && line.contains("mov rdx,") && line.contains("msg") {
                            new_lines.push(line.replace("mov rdx,", "mov rsi,"));
                            rdx_found = true;
                        } else {
                            new_lines.push(line.to_string());
                        }
                    }
                    converted = new_lines.join("\n");
                }
                
                // Ajustar alineación de pila (Windows usa 40, Linux usa 8 o múltiplo de 16)
                // Windows: sub rsp, 40 (shadow space 32 + alineación 8)
                // Linux:   sub rsp, 8 (solo alineación)
                converted = converted.replace("sub rsp, 40", "sub rsp, 8");
                converted = converted.replace("add rsp, 40", "add rsp, 8");
                
                // Asegurar que RAX esté en 0 antes de call printf (requerido en Linux para variadic functions)
                let lines: Vec<&str> = converted.lines().collect();
                let mut new_lines = Vec::new();
                
                for (i, line) in lines.iter().enumerate() {
                    let line_lower = line.to_lowercase();
                    let line_lower_trimmed = line_lower.trim();
                    
                    // Si encontramos call printf, verificar si hay xor rax antes
                    if line_lower_trimmed.contains("call printf") {
                        // Buscar hacia atrás (últimas 5 líneas) para ver si hay xor rax/eax
                        let mut has_xor_rax = false;
                        for j in (0..i).rev().take(5) {
                            let prev_line = lines[j].to_lowercase();
                            if prev_line.contains("xor rax") || prev_line.contains("xor eax") {
                                has_xor_rax = true;
                                break;
                            }
                        }
                        
                        // Si no hay xor rax y hay mov rdi antes, agregarlo
                        if !has_xor_rax {
                            // Verificar si hay mov rdi en las líneas anteriores
                            let mut has_mov_rdi = false;
                            for j in (0..i).rev().take(10) {
                                if lines[j].to_lowercase().contains("mov rdi") {
                                    has_mov_rdi = true;
                                    break;
                                }
                            }
                            
                            if has_mov_rdi {
                                // Agregar xor rax, rax antes del call
                                new_lines.push("    xor rax, rax".to_string());
                            }
                        }
                    }
                    
                    new_lines.push(line.to_string());
                }
                
                converted = new_lines.join("\n");
                
                // Cambiar comentarios de Windows a Linux
                converted = converted.replace("Windows x64", "Linux x64");
                converted = converted.replace("(Windows)", "(Linux)");
                converted = converted.replace("RCX (primer argumento)", "RDI (primer argumento en Linux)");
                
                converted
            } else {
                code.to_string()
            }
        };
        
        #[cfg(target_os = "windows")]
        let code_to_compile = code.to_string();

        if let Err(e) = std::fs::write(&temp_asm, &code_to_compile) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        // Buscar NASM con búsqueda profunda
        let nasm_path = Self::find_compiler_cmd("nasm", output);
        if nasm_path.is_none() {
            output.push_str(">>> Error: NASM no está instalado o no está en PATH.\n");
            output.push_str(">>> Instala NASM con:\n");
            #[cfg(target_os = "linux")]
            {
                output.push_str(">>>   sudo apt-get install nasm  (Debian/Ubuntu)\n");
                output.push_str(">>>   sudo dnf install nasm       (Fedora/RHEL)\n");
                output.push_str(">>>   sudo pacman -S nasm        (Arch)\n");
            }
            #[cfg(target_os = "macos")]
            {
                output.push_str(">>>   brew install nasm\n");
            }
            #[cfg(target_os = "windows")]
            {
                output.push_str(">>>   Descarga desde: https://nasm.us\n");
                output.push_str(">>>   Extrae en C:\\NASM y agrega a PATH\n");
            }
            return;
        }
        
        let nasm_cmd = nasm_path.unwrap();
        output.push_str(&format!(">>> NASM detectado: {}\n", nasm_cmd));
        
        // Obtener versión
        match Command::new(&nasm_cmd).arg("-v").output() {
            Ok(out) => {
                let version = String::from_utf8_lossy(&out.stdout);
                if !version.trim().is_empty() {
                    output.push_str(&format!(">>> Versión: {}\n", version.trim()));
                }
            }
            Err(_) => {}
        }

        // NASM - formato según OS
        output.push_str(&format!(">>> Compilando con NASM (formato: {})...\n", NASM_FORMAT));
        match Command::new(&nasm_cmd)
            .current_dir(work_dir)
            .args(&["-f", NASM_FORMAT, temp_asm.file_name().unwrap().to_str().unwrap(), "-o", temp_obj.file_name().unwrap().to_str().unwrap()])
            .output()
        {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                
                if !stdout.is_empty() {
                    output.push_str("NASM Output:\n");
                    output.push_str(&stdout);
                }
                
                if !stderr.is_empty() {
                    output.push_str("NASM Log:\n");
                    output.push_str(&stderr);
                }
                
                if out.status.success() {
                    output.push_str(">>> NASM: Compilación exitosa.\n");
                } else {
                    output.push_str(">>> NASM: Error de ensamblado.\n");
                    output.push_str(&format!(">>> Exit code: {}\n", out.status.code().unwrap_or(-1)));
                    return;
                }
            }
            Err(e) => {
                output.push_str(&format!(">>> Error ejecutando NASM: {}\n", e));
                output.push_str(">>> Asegúrate de que NASM esté instalado y en tu PATH.\n");
                output.push_str(">>> Verifica con: nasm -v\n");
                return;
            }
        }
        
        // Verificar que el archivo objeto se creó
        if !temp_obj.exists() {
            output.push_str(&format!(">>> Error: El archivo objeto no se creó: {:?}\n", temp_obj));
            return;
        }

        // DIAGNÓSTICO: Verificar si el código NASM tiene 'global main'
        let code_lower = code.to_lowercase();
        let has_global_main = code_lower.contains("global main") || code_lower.contains("global _main");
        if !has_global_main {
            output.push_str(">>> ⚠️ ADVERTENCIA: El código NASM no parece tener 'global main' exportado.\n");
            output.push_str(">>>    Esto puede causar errores de linkeo.\n");
            output.push_str(">>>    Asegúrate de tener una línea como: 'global main' o 'global _main'\n\n");
        } else {
            output.push_str(">>> ✅ Verificado: 'global main' encontrado en el código.\n\n");
        }

        // Linkear automáticamente usando el auto-linker
        output.push_str(">>> Linkeando automáticamente...\n");
        match crate::compilation::auto_linker::auto_link(&temp_obj, exe_file, work_dir, output) {
            Ok(_) => {
                output.push_str(">>> ✅ Linkeo completado exitosamente.\n");
            }
            Err(e) => {
                output.push_str(&format!(">>> ❌ Error en linkeo: {}\n", e));
                output.push_str("\n>>> 💡 INTENTO MANUAL - Ejecuta este comando en tu terminal para ver el error completo:\n");
                
                // Generar comando manual para que el usuario lo pruebe
                #[cfg(target_os = "windows")]
                {
                    // Intentar detectar qué linker estaría disponible
                    if let Some(gcc_path) = Self::find_compiler_cmd("gcc", output) {
                        output.push_str(&format!(">>>    {} {} -o {}\n", 
                            gcc_path,
                            temp_obj.file_name().unwrap().to_str().unwrap(),
                            exe_file));
                        output.push_str(&format!(">>>    O si eso falla, intenta:\n"));
                        output.push_str(&format!(">>>    {} {} -o {} -m64 -Wl,--subsystem,console -lmsvcrt\n", 
                            gcc_path,
                            temp_obj.file_name().unwrap().to_str().unwrap(),
                            exe_file));
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    if let Some(gcc_path) = Self::find_compiler_cmd("gcc", output) {
                        output.push_str(&format!(">>>    {} {} -o {}\n", 
                            gcc_path,
                            temp_obj.file_name().unwrap().to_str().unwrap(),
                            exe_file));
                    }
                }
                
                let _ = std::fs::remove_file(&exe_path);
            }
        }
    }

    fn compile_zig(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.zig");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let exe_path = work_dir.join(exe_file);
        
        // Buscar Zig con búsqueda profunda
        let zig_path = Self::find_compiler_cmd("zig", output);
        if zig_path.is_none() {
            output.push_str(">>> Error: Zig no está instalado o no está en PATH.\n");
            output.push_str(">>> Instala Zig desde: https://ziglang.org/download/\n");
            #[cfg(target_os = "linux")]
            {
                output.push_str(">>>   O desde repositorio: sudo apt install zig\n");
            }
            #[cfg(target_os = "windows")]
            {
                output.push_str(">>>   Descarga desde: https://ziglang.org/download/\n");
                output.push_str(">>>   Extrae en C:\\Zig y agrega a PATH\n");
                output.push_str(">>>   O con chocolatey: choco install zig\n");
            }
            #[cfg(target_os = "macos")]
            {
                output.push_str(">>>   O con Homebrew: brew install zig\n");
            }
            return;
        }
        
        let zig_cmd = zig_path.unwrap();
        output.push_str(&format!(">>> Zig detectado: {}\n", zig_cmd));
        
        // Zig puede compilar directamente desde archivo fuente
        // zig run temp.zig  - ejecuta directamente
        // zig build-exe temp.zig -o program - compila a ejecutable
        let cmd_output = Command::new(&zig_cmd)
            .current_dir(work_dir)
            .args(&["build-exe", temp_file.file_name().unwrap().to_str().unwrap(), "-fno-strip", "-O", "Debug", "--name", exe_file])
            .output();

        match cmd_output {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                
                if !stdout.is_empty() {
                    output.push_str("Zig Output:\n");
                    output.push_str(&stdout);
                }
                
                if !stderr.is_empty() {
                    output.push_str("Zig Log:\n");
                    output.push_str(&stderr);
                }
                
                if out.status.success() {
                    output.push_str(">>> Zig: Compilación exitosa.\n");
                } else {
                    output.push_str(">>> Zig: Error de compilación.\n");
                    let _ = std::fs::remove_file(&exe_path);
                }
            }
            Err(e) => {
                output.push_str(&format!("Error ejecutando Zig: {}\n", e));
                output.push_str(">>> Zig no está instalado o no está en PATH.\n");
                output.push_str(">>> Instala Zig desde: https://ziglang.org/download/\n");
                #[cfg(target_os = "linux")]
                {
                    output.push_str(">>>   O desde repositorio: sudo apt install zig\n");
                }
                #[cfg(target_os = "windows")]
                {
                    output.push_str(">>>   Descarga desde: https://ziglang.org/download/\n");
                    output.push_str(">>>   O con chocolatey: choco install zig\n");
                }
                #[cfg(target_os = "macos")]
                {
                    output.push_str(">>>   O con Homebrew: brew install zig\n");
                }
            }
        }
    }

    fn compile_java(code: &str, work_dir: &Path, output: &mut String) -> Option<String> {
        let temp_file = work_dir.join("Main.java");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return None;
        }
        
        // Buscar javac
        let javac_path = Self::find_compiler_cmd("javac", output);
        if javac_path.is_none() {
            output.push_str(">>> Error: No se encontró javac (Java Compiler).\n");
            output.push_str(">>> Instala el JDK (Java Development Kit):\n");
            #[cfg(target_os = "windows")]
            {
                output.push_str(">>>   Descarga desde: https://adoptium.net/\n");
                output.push_str(">>>   O con chocolatey: choco install temurin\n");
            }
            #[cfg(target_os = "linux")]
            {
                output.push_str(">>>   sudo apt install openjdk-25-jdk\n");
                output.push_str(">>>   O desde: https://adoptium.net/\n");
            }
            #[cfg(target_os = "macos")]
            {
                output.push_str(">>>   brew install --cask temurin\n");
                output.push_str(">>>   O desde: https://adoptium.net/\n");
            }
            return None;
        }
        
        let javac_cmd = javac_path.unwrap();
        output.push_str(&format!(">>> Usando javac: {}\n", javac_cmd));
        
        // Compilar con Java 25 (usando --release 25 o --source 25 --target 25)
        let cmd_output = Command::new(&javac_cmd)
            .current_dir(work_dir)
            .args(&["--source", "25", "--target", "25", "--enable-preview", temp_file.file_name().unwrap().to_str().unwrap()])
            .output();
        
        match cmd_output {
            Ok(out) => {
                // Decodificar correctamente como UTF-8 para soportar caracteres especiales
                let stderr = String::from_utf8_lossy(&out.stderr);
                let stdout = String::from_utf8_lossy(&out.stdout);
                
                if !stdout.is_empty() {
                    output.push_str("javac Salida:\n");
                    output.push_str(&stdout);
                    if !stdout.ends_with('\n') {
                        output.push_str("\n");
                    }
                }
                
                if !stderr.is_empty() {
                    output.push_str("javac Errores/Advertencias:\n");
                    output.push_str(&stderr);
                    if !stderr.ends_with('\n') {
                        output.push_str("\n");
                    }
                }
                
                if out.status.success() {
                    output.push_str("✅ Java: Compilación exitosa.\n");
                    
                    // Extraer el nombre de la clase principal del código
                    let main_class = Self::extract_java_main_class(code);
                    if let Some(class_name) = main_class {
                        return Some(class_name);
                    } else {
                        // Por defecto usar "Main"
                        output.push_str("⚠️  Advertencia: No se pudo determinar la clase principal, usando 'Main'\n");
                        return Some("Main".to_string());
                    }
                } else {
                    output.push_str("❌ Java: Error de compilación.\n");
                    if let Some(code) = out.status.code() {
                        output.push_str(&format!(">>> Código de salida: {}\n", code));
                    }
                    return None;
                }
            }
            Err(e) => {
                output.push_str(&format!("❌ Error ejecutando javac: {}\n", e));
                output.push_str(">>> javac no está instalado o no está en PATH.\n");
                return None;
            }
        }
    }
    
    // Función helper para extraer el nombre de la clase principal de Java
    fn extract_java_main_class(code: &str) -> Option<String> {
        // Buscar "public class" seguido del nombre
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("public class") {
                // Extraer el nombre de la clase
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 && parts[0] == "public" && parts[1] == "class" {
                    let class_name = parts[2];
                    // Remover llaves si están en la misma línea
                    let class_name = class_name.trim_end_matches('{').trim();
                    return Some(class_name.to_string());
                }
            }
        }
        None
    }

    /// Resolver código completo para Python: heredado + expresiones ch()
    fn resolve_python_code(
        code: &str,
        inherited_code: Option<&str>,
    ) -> String {
        let mut resolved = String::new();
        
        // 1. Agregar código heredado al inicio si existe
        if let Some(inherited) = inherited_code {
            resolved.push_str("# ════════════════════════════════════════════════════════════\n");
            resolved.push_str("# 🔗 CÓDIGO HEREDADO (del nodo padre)\n");
            resolved.push_str("# ════════════════════════════════════════════════════════════\n");
            resolved.push_str(inherited);
            resolved.push_str("\n\n");
        }
        
        // 2. Resolver expresiones ch() en el código actual (sin contexto por ahora)
        let code_with_ch_resolved = Self::resolve_python_ch_with_context(code, None, None);
        
        // 3. Agregar el código actual
        resolved.push_str("# ════════════════════════════════════════════════════════════\n");
        resolved.push_str("# 📝 CÓDIGO ACTUAL (este nodo)\n");
        resolved.push_str("# ════════════════════════════════════════════════════════════\n");
        resolved.push_str(&code_with_ch_resolved);
        
        resolved
    }

    /// Resolver expresiones ch() con acceso a ChannelManager
    fn resolve_python_ch_with_context(
        code: &str,
        channel_manager: Option<&crate::expressions::channels::ChannelManager>,
        current_node_id: Option<crate::core::node_graph::NodeId>,
    ) -> String {
        let mut resolved_code = code.to_string();
        let mut i = 0;
        
        // Buscar patrones ch("...") o ch('...') y resolverlos
        while i < resolved_code.len() {
            // Buscar "ch("
            if let Some(ch_pos) = resolved_code[i..].find("ch(") {
                let start = i + ch_pos;
                let after_ch = start + 2; // Después de "ch"
                
                if after_ch < resolved_code.len() && resolved_code.as_bytes()[after_ch] == b'(' {
                    // Buscar comillas
                    let quote_pos = resolved_code[after_ch + 1..]
                        .find(|c| c == '"' || c == '\'');
                    
                    if let Some(quote_offset) = quote_pos {
                        let quote_char = resolved_code.as_bytes()[after_ch + 1 + quote_offset] as char;
                        let node_name_start = after_ch + 2 + quote_offset;
                        
                        // Buscar la comilla de cierre
                        if let Some(end_quote_pos) = resolved_code[node_name_start..]
                            .find(quote_char) {
                            let node_name_end = node_name_start + end_quote_pos;
                            let node_name = &resolved_code[node_name_start..node_name_end];
                            
                            // Buscar el paréntesis de cierre
                            if node_name_end + 1 < resolved_code.len() 
                                && resolved_code.as_bytes()[node_name_end + 1] == b')' {
                                let expr_end = node_name_end + 2;
                                let full_expr = &resolved_code[start..expr_end];
                                
                                // Intentar resolver desde ChannelManager
                                let replacement = if let Some(cm) = channel_manager {
                                    if let Some(code_value) = cm.get_node_code(node_name) {
                                        // Encontró código del nodo - inyectarlo como código Python
                                        format!(
                                            "\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n# 📦 Código heredado de nodo: '{}'\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{}\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                                            node_name, code_value
                                        )
                                    } else if let Some(ChannelValue::Code(code)) = cm.get_channel(node_name) {
                                        format!(
                                            "\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n# 📦 Código del canal: '{}'\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{}\n# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                                            node_name, code
                                        )
                                    } else {
                                        format!(
                                            "# ⚠️ Error: No se encontró código para nodo '{}' (ch('{}'))",
                                            node_name, node_name
                                        )
                                    }
                                } else {
                                    format!(
                                        "# 🔗 Referencia a nodo: '{}' (ch('{}'))\n# 💡 Conecta este nodo al nodo '{}' para heredar su código",
                                        node_name, node_name, node_name
                                    )
                                };
                                
                                resolved_code.replace_range(start..expr_end, &replacement);
                                i = start + replacement.len();
                                continue;
                            }
                        }
                    }
                }
                i = start + 3;
            } else {
                break;
            }
        }
        
        resolved_code
    }

    /// Ejecutar Python con contexto completo (ChannelManager + herencia)
    pub fn run_python_with_context(
        code: &str,
        work_dir: &Path,
        output: &mut String,
        channel_manager: Option<&crate::expressions::channels::ChannelManager>,
        inherited_code: Option<&str>,
        current_node_id: Option<crate::core::node_graph::NodeId>,
    ) {
        // Resolver código completo (herencia + ch())
        let mut resolved = String::new();
        
        // 1. Agregar código heredado al inicio si existe
        if let Some(inherited) = inherited_code {
            resolved.push_str("# ════════════════════════════════════════════════════════════\n");
            resolved.push_str("# 🔗 CÓDIGO HEREDADO (del nodo padre)\n");
            resolved.push_str("# ════════════════════════════════════════════════════════════\n");
            resolved.push_str(inherited);
            resolved.push_str("\n\n");
        }
        
        // 2. Resolver expresiones ch() en el código actual con contexto
        let code_with_ch_resolved = Self::resolve_python_ch_with_context(
            code,
            channel_manager,
            current_node_id,
        );
        
        // 3. Agregar el código actual
        resolved.push_str("# ════════════════════════════════════════════════════════════\n");
        resolved.push_str("# 📝 CÓDIGO ACTUAL (este nodo)\n");
        resolved.push_str("# ════════════════════════════════════════════════════════════\n");
        resolved.push_str(&code_with_ch_resolved);
        
        // Ejecutar el código resuelto
        Self::run_python(&resolved, work_dir, output);
    }

    fn run_python(code: &str, work_dir: &Path, output: &mut String) {
        
        let temp_file = work_dir.join("temp.py");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("❌ Error guardando archivo: {}\n", e));
            return;
        }
        
        // Buscar Python: intentar python3 primero (Linux/Mac), luego python (Windows)
        let python_cmd = Self::find_compiler_cmd("python3", output)
            .or_else(|| Self::find_compiler_cmd("python", output));
        
        if python_cmd.is_none() {
            output.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
            output.push_str("║                    ❌ ERROR: PYTHON NO ENCONTRADO              ║\n");
            output.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
            output.push_str("Python 3.12 no está instalado o no está en PATH.\n\n");
            output.push_str("💡 Instalación:\n");
            #[cfg(target_os = "windows")]
            {
                output.push_str("   1. Descarga desde: https://www.python.org/downloads/\n");
                output.push_str("   2. Durante la instalación, marca 'Add python.exe to PATH'\n");
                output.push_str("   3. O con chocolatey: choco install python312\n");
            }
            #[cfg(target_os = "linux")]
            {
                output.push_str("   sudo apt update && sudo apt install python3.12\n");
                output.push_str("   O desde: https://www.python.org/downloads/\n");
            }
            #[cfg(target_os = "macos")]
            {
                output.push_str("   brew install python@3.12\n");
                output.push_str("   O desde: https://www.python.org/downloads/\n");
            }
            return;
        }
        
        let python_path = python_cmd.unwrap();
        output.push_str(&format!(">>> Usando Python: {}\n", python_path));
        
        // Verificar versión de Python
        if let Ok(version_out) = Command::new(&python_path).arg("--version").output() {
            let version_str = String::from_utf8_lossy(&version_out.stdout);
            output.push_str(&format!(">>> Versión: {}\n\n", version_str.trim()));
        }
        
        // Separador visual antes de ejecución
        output.push_str("\n");
        output.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    ▶️  EJECUTANDO PROGRAMA                    ║\n");
        output.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
        
        output.push_str(&format!("📝 Archivo: {}\n", temp_file.file_name().unwrap().to_str().unwrap()));
        output.push_str("⏳ Ejecutando...\n\n");
        
        // Medir tiempo de ejecución
        let start_time = std::time::Instant::now();
        
        // Ejecutar Python con -u (unbuffered) para salida inmediata
        let mut cmd = Command::new(&python_path);
        cmd.current_dir(work_dir);
        cmd.arg("-u"); // Unbuffered output
        cmd.arg(temp_file.file_name().unwrap().to_str().unwrap());
        
        // Configurar variables de entorno para UTF-8
        #[cfg(target_os = "windows")]
        {
            cmd.env("PYTHONIOENCODING", "utf-8");
            cmd.env("PYTHONUTF8", "1");
        }
        
        match cmd.output() {
            Ok(run_out) => {
                let elapsed = start_time.elapsed();
                
                // Decodificar correctamente como UTF-8
                let stdout_str = String::from_utf8_lossy(&run_out.stdout);
                let stderr_str = String::from_utf8_lossy(&run_out.stderr);
                
                // Sección de resultados mejorada
                output.push_str("\n");
                output.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                output.push_str("║                  📊 RESULTADOS DEL PROGRAMA                    ║\n");
                output.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                
                if !stdout_str.is_empty() {
                    output.push_str("📤 SALIDA ESTÁNDAR (Resultado del programa):\n");
                    output.push_str("═══════════════════════════════════════════════════════════════\n");
                    output.push_str("\n");
                    // Mostrar cada línea de salida de forma destacada
                    let lines: Vec<&str> = stdout_str.lines().collect();
                    for line in lines.iter() {
                        if line.trim().is_empty() {
                            output.push_str("\n");
                        } else {
                            output.push_str(&format!("  {}\n", line));
                        }
                    }
                    // Si hay una línea sin salto al final
                    if !stdout_str.ends_with('\n') && !stdout_str.is_empty() && lines.is_empty() {
                        output.push_str(&format!("  {}\n", stdout_str));
                    }
                    output.push_str("\n");
                    output.push_str("═══════════════════════════════════════════════════════════════\n");
                    output.push_str("\n");
                }
                
                if !stderr_str.is_empty() {
                    output.push_str("⚠️  ERRORES/ADVERTENCIAS:\n");
                    output.push_str("─────────────────────────────────────────────────────────────\n");
                    output.push_str("\n");
                    for line in stderr_str.lines() {
                        output.push_str(&format!("  {}\n", line));
                    }
                    if !stderr_str.ends_with('\n') && !stderr_str.is_empty() {
                        let last_line = stderr_str.lines().last().unwrap_or("");
                        if !last_line.is_empty() {
                            output.push_str(&format!("  {}\n", last_line));
                        }
                    }
                    output.push_str("\n");
                    output.push_str("─────────────────────────────────────────────────────────────\n");
                    output.push_str("\n");
                }
                
                // Estado final
                output.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                if let Some(exit_code) = run_out.status.code() {
                    if exit_code == 0 {
                        output.push_str("║  ✅ Estado: Ejecución exitosa                              ║\n");
                    } else {
                        output.push_str(&format!("║  ❌ Estado: Error (código de salida: {})                  ║\n", exit_code));
                    }
                } else {
                    output.push_str("║  ⚠️  Estado: Terminado por señal del sistema              ║\n");
                }
                output.push_str(&format!("║  ⏱️  Tiempo de ejecución: {:.2} ms                        ║\n", elapsed.as_millis()));
                output.push_str("╚═══════════════════════════════════════════════════════════════╝\n");
            }
            Err(e) => {
                output.push_str("╔═══════════════════════════════════════════════════════════════╗\n");
                output.push_str("║                    ❌ ERROR DE EJECUCIÓN                      ║\n");
                output.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
                output.push_str(&format!("Detalle: {}\n\n", e));
                output.push_str("💡 Sugerencias:\n");
                output.push_str("   1. Verifica que Python 3.12 esté instalado correctamente\n");
                output.push_str("   2. Asegúrate de que 'python3' o 'python' esté en tu PATH\n");
                output.push_str("   3. Verifica que el archivo Python tenga sintaxis correcta\n");
            }
        }
        
        // Limpiar archivo temporal (opcional, puede dejarse para debugging)
        // let _ = std::fs::remove_file(&temp_file);
    }

    fn compile_mojo(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.mojo");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let exe_path = work_dir.join(exe_file);
        
        // Mojo compila directamente a ejecutable
        // En producción, esto usaría el compilador Mojo real
        let cmd_output = Command::new("mojo")
            .current_dir(work_dir)
            .args(&["build", temp_file.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
            .output();

        match cmd_output {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if !stderr.is_empty() {
                    output.push_str("Mojo Log:\n");
                    output.push_str(&stderr);
                }
                if out.status.success() {
                    output.push_str(">>> Mojo: Compilación exitosa.\n");
                } else {
                    output.push_str(">>> Mojo: Error de compilación.\n");
                    let _ = std::fs::remove_file(&exe_path);
                }
            }
            Err(e) => {
                output.push_str(&format!("Error ejecutando Mojo: {}\n", e));
                output.push_str("Nota: Mojo SDK debe estar instalado y en PATH.\n");
                output.push_str("Visita: https://docs.modular.com/mojo/get-started\n");
            }
        }
    }
}

