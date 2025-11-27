use std::process::Command;
use std::path::{Path, PathBuf};

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
        };
        
        self.active_tab = tab;
        output_buffer.clear();
        output_buffer.push_str(">>> Iniciando compilación...\n");

        // Determinar el directorio de trabajo
        let work_dir = workspace_path
            .map(|p| p.clone())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        let exe_file = "program.exe";
        let exe_path = work_dir.join(exe_file);

        match lang {
            Language::Nasm => Self::compile_nasm(code, &work_dir, exe_file, output_buffer),
            Language::C => Self::compile_c(code, &work_dir, exe_file, output_buffer),
            Language::Cpp => Self::compile_cpp(code, &work_dir, exe_file, output_buffer),
            Language::Rust => Self::compile_rust(code, &work_dir, exe_file, output_buffer),
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
                    output_buffer.push_str(&format!("Exit code: {}\n", run_out.status));
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
        let temp_asm = work_dir.join("temp.asm");
        let temp_obj = work_dir.join("temp.obj");
        let exe_path = work_dir.join(exe_file);

        if let Err(e) = std::fs::write(&temp_asm, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        // NASM
        match Command::new("nasm")
            .current_dir(work_dir)
            .args(&["-f", "win64", temp_asm.file_name().unwrap().to_str().unwrap(), "-o", temp_obj.file_name().unwrap().to_str().unwrap()])
            .output()
        {
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                if !stderr.is_empty() {
                    output.push_str("NASM Log:\n");
                    output.push_str(&stderr);
                }
                if !out.status.success() {
                    output.push_str(">>> NASM: Error de ensamblado.\n");
                    return;
                }
            }
            Err(e) => {
                output.push_str(&format!("Error ejecutando NASM: {}\n", e));
                return;
            }
        }

        // Link with GCC
        match Command::new("gcc")
            .current_dir(work_dir)
            .args(&[temp_obj.file_name().unwrap().to_str().unwrap(), "-o", exe_file])
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
                    let _ = std::fs::remove_file(&exe_path);
                }
            }
            Err(e) => {
                output.push_str(&format!("Error linkeando: {}\n", e));
            }
        }
    }
}

