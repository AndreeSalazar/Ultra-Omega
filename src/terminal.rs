use std::process::Command;
use std::path::{Path, PathBuf};

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
    pub active_tab: TerminalTab,
    pub visible: bool,
    pub pinned: bool,
    pub hide_timer: f32,
}

#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum TerminalTab {
    #[default]
    Nasm,
    C,
    Cpp,
    Rust,
    Mojo,
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self {
            asm_output: String::new(),
            c_output: String::new(),
            cpp_output: String::new(),
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
    Nasm,
    C,
    Cpp,
    Rust,
    Mojo,
}

impl TerminalManager {
    pub fn run_code(&mut self, code: &str, lang: Language, workspace_path: Option<&PathBuf>) {
        self.visible = true;
        self.hide_timer = 10.0; // Show for 10 seconds
        
        let (output_buffer, tab) = match lang {
            Language::Nasm => (&mut self.asm_output, TerminalTab::Nasm),
            Language::C => (&mut self.c_output, TerminalTab::C),
            Language::Cpp => (&mut self.cpp_output, TerminalTab::Cpp),
            Language::Rust => (&mut self.rust_output, TerminalTab::Rust),
            Language::Mojo => {
                // Mojo usa el buffer de Rust por ahora (o se puede crear uno específico)
                (&mut self.rust_output, TerminalTab::Mojo)
            },
        };
        
        self.active_tab = tab;
        output_buffer.clear();
        output_buffer.push_str(">>> Iniciando compilación...\n");

        // Determinar el directorio de trabajo
        let work_dir = workspace_path
            .map(|p| p.clone())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        let exe_file = format!("program{}", EXE_EXTENSION);
        let exe_path = work_dir.join(&exe_file);
        let exe_file_str = exe_file.as_str();

        match lang {
            Language::Nasm => Self::compile_nasm(code, &work_dir, exe_file_str, output_buffer),
            Language::C => Self::compile_c(code, &work_dir, exe_file_str, output_buffer),
            Language::Cpp => Self::compile_cpp(code, &work_dir, exe_file_str, output_buffer),
            Language::Rust => Self::compile_rust(code, &work_dir, exe_file_str, output_buffer),
            Language::Mojo => Self::compile_mojo(code, &work_dir, exe_file_str, output_buffer),
        }

        // Run if compiled
        if exe_path.exists() {
            output_buffer.push_str(">>> Ejecutando...\n\n");
            match Command::new(&exe_path)
                .current_dir(&work_dir)
                .output()
            {
                Ok(run_out) => {
                    output_buffer.push_str("--- SALIDA DEL PROGRAMA ---\n");
                    output_buffer.push_str(&String::from_utf8_lossy(&run_out.stdout));
                    output_buffer.push_str(&String::from_utf8_lossy(&run_out.stderr));
                    output_buffer.push_str("\n---------------------------\n");
                    
                    // Analizar el código de salida
                    let exit_code = run_out.status.code();
                    if let Some(code) = exit_code {
                        if code == 139 || code == -11 {
                            // SIGSEGV (segmentation fault)
                            output_buffer.push_str(&format!("Exit code: signal: 11 (SIGSEGV) (core dumped)\n"));
                            output_buffer.push_str("\n⚠️  SEGMENTATION FAULT detectado!\n");
                            output_buffer.push_str("   Posibles causas:\n");
                            #[cfg(not(target_os = "windows"))]
                            {
                                output_buffer.push_str("   1. Código escrito para Windows ejecutándose en Linux\n");
                                output_buffer.push_str("      - Windows usa: mov rcx, arg (primer argumento)\n");
                                output_buffer.push_str("      - Linux usa:   mov rdi, arg (primer argumento)\n");
                                output_buffer.push_str("   2. Pila mal alineada (debe ser múltiplo de 16 bytes)\n");
                                output_buffer.push_str("   3. Acceso a memoria inválida\n\n");
                                output_buffer.push_str("   Solución: Adapta el código para Linux:\n");
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
                            output_buffer.push_str(&format!("Exit code: {}\n", code));
                        } else {
                            output_buffer.push_str(&format!("Exit code: {}\n", run_out.status));
                        }
                    } else {
                        // Proceso terminado por señal
                        output_buffer.push_str(&format!("Exit code: {}\n", run_out.status));
                        if run_out.status.to_string().contains("signal") {
                            output_buffer.push_str("\n⚠️  El programa fue terminado por una señal del sistema.\n");
                        }
                    }
                }
                Err(e) => {
                    output_buffer.push_str(&format!("Error ejecutando programa: {}\n", e));
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
        let cmd_output = Command::new("gcc")
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

        let exe_path = work_dir.join(exe_file);
        let cmd_output = Command::new("g++")
            .current_dir(work_dir)
            .args(&[temp_file.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "G++", &exe_path, output);
    }

    fn compile_rust(code: &str, work_dir: &Path, exe_file: &str, output: &mut String) {
        let temp_file = work_dir.join("temp.rs");
        if let Err(e) = std::fs::write(&temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let exe_path = work_dir.join(exe_file);
        let cmd_output = Command::new("rustc")
            .current_dir(work_dir)
            .args(&[temp_file.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "Rustc", &exe_path, output);
    }

    fn handle_compile_output(result: std::io::Result<std::process::Output>, name: &str, exe_file: &Path, output: &mut String) {
        match result {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if !stderr.is_empty() {
                    output.push_str(&format!("{} Log:\n", name));
                    output.push_str(&stderr);
                }
                if out.status.success() {
                    output.push_str(&format!(">>> {}: Compilación exitosa.\n", name));
                } else {
                    output.push_str(&format!(">>> {}: Error de compilación.\n", name));
                    let _ = std::fs::remove_file(exe_file);
                }
            }
            Err(e) => {
                output.push_str(&format!("Error invocando {}: {}\n", name, e));
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

        // Verificar si NASM está instalado y funciona
        let nasm_check = Command::new("nasm").arg("-v").output();
        match nasm_check {
            Ok(out) => {
                let version = String::from_utf8_lossy(&out.stdout);
                if !version.is_empty() {
                    output.push_str(&format!(">>> NASM detectado: {}\n", version.trim()));
                }
            },
            Err(e) => {
                output.push_str(&format!(">>> Error: NASM no está instalado o no está en PATH.\n"));
                output.push_str(&format!(">>> Error detallado: {}\n", e));
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
                }
                return;
            }
        }

        // NASM - formato según OS
        output.push_str(&format!(">>> Compilando con NASM (formato: {})...\n", NASM_FORMAT));
        match Command::new("nasm")
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

        // Link with GCC - argumentos según OS
        #[cfg(target_os = "windows")]
        let link_args = vec![temp_obj.file_name().unwrap().to_str().unwrap(), "-o", exe_file];
        
        #[cfg(not(target_os = "windows"))]
        {
            // Para Linux, usar -no-pie para evitar problemas con Position Independent Executables
            // y asegurar compatibilidad con código NASM estándar
            output.push_str(">>> Linkeando con GCC...\n");
            let link_args = vec![
                temp_obj.file_name().unwrap().to_str().unwrap(),
                "-o",
                exe_file,
                "-no-pie",  // Importante para código NASM en Linux
            ];
            
            match Command::new("gcc")
                .current_dir(work_dir)
                .args(&link_args)
                .output()
            {
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    
                    if !stdout.is_empty() {
                        output.push_str("GCC Output:\n");
                        output.push_str(&stdout);
                    }
                    
                    if !stderr.is_empty() {
                        output.push_str("Linker Log:\n");
                        output.push_str(&stderr);
                    }
                    
                    if out.status.success() {
                        output.push_str(">>> Linker: Éxito.\n");
                    } else {
                        output.push_str(">>> Linker: Error (intentando fallback sin -no-pie)...\n");
                        // Intentar sin -no-pie como fallback
                        let link_args_fallback = vec![
                            temp_obj.file_name().unwrap().to_str().unwrap(),
                            "-o",
                            exe_file,
                        ];
                        match Command::new("gcc")
                            .current_dir(work_dir)
                            .args(&link_args_fallback)
                            .output()
                        {
                            Ok(out2) => {
                                let stderr2 = String::from_utf8_lossy(&out2.stderr);
                                if !stderr2.is_empty() {
                                    output.push_str(&format!("Fallback Log: {}\n", stderr2));
                                }
                                if out2.status.success() {
                                    output.push_str(">>> Linker (fallback): Éxito.\n");
                                } else {
                                    output.push_str(">>> Linker (fallback): Error.\n");
                                    output.push_str(&format!(">>> Exit code: {}\n", out2.status.code().unwrap_or(-1)));
                                    let _ = std::fs::remove_file(&exe_path);
                                }
                            }
                            Err(e) => {
                                output.push_str(&format!(">>> Error en fallback: {}\n", e));
                                let _ = std::fs::remove_file(&exe_path);
                            }
                        }
                    }
                }
                Err(e) => {
                    output.push_str(&format!(">>> Error ejecutando GCC: {}\n", e));
                    output.push_str(">>> Verifica que GCC esté instalado: gcc --version\n");
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            match Command::new("gcc")
                .current_dir(work_dir)
                .args(&link_args)
                .output()
            {
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if !stderr.is_empty() {
                        output.push_str("Linker Log:\n");
                        output.push_str(&stderr);
                    }
                    if out.status.success() {
                        output.push_str(">>> Linker: Éxito.\n");
                    } else {
                        output.push_str(">>> Linker: Error.\n");
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        if !stdout.is_empty() {
                            output.push_str(&format!("STDOUT: {}\n", stdout));
                        }
                        let _ = std::fs::remove_file(&exe_path);
                    }
                }
                Err(e) => {
                    output.push_str(&format!("Error linkeando: {}\n", e));
                }
            }
        }
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

