use std::process::Command;
use std::path::Path;

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
    pub fn run_code(&mut self, code: &str, lang: Language) {
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

        let exe_file = "program.exe";
        let exe_path = std::env::current_dir().unwrap_or_default().join(exe_file);

        match lang {
            Language::Nasm => Self::compile_nasm(code, exe_file, output_buffer),
            Language::C => Self::compile_c(code, exe_file, output_buffer),
            Language::Cpp => Self::compile_cpp(code, exe_file, output_buffer),
            Language::Rust => Self::compile_rust(code, exe_file, output_buffer),
        }

        // Run if compiled
        if Path::new(exe_file).exists() {
            output_buffer.push_str(">>> Ejecutando...\n\n");
            match Command::new(exe_path).output() {
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

    fn compile_c(code: &str, exe_file: &str, output: &mut String) {
        let temp_file = "temp.c";
        if let Err(e) = std::fs::write(temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let cmd_output = Command::new("gcc")
            .args(&[temp_file, "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "GCC", exe_file, output);
    }

    fn compile_cpp(code: &str, exe_file: &str, output: &mut String) {
        let temp_file = "temp.cpp";
        if let Err(e) = std::fs::write(temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let cmd_output = Command::new("g++")
            .args(&[temp_file, "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "G++", exe_file, output);
    }

    fn compile_rust(code: &str, exe_file: &str, output: &mut String) {
        let temp_file = "temp.rs";
        if let Err(e) = std::fs::write(temp_file, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        let cmd_output = Command::new("rustc")
            .args(&[temp_file, "-o", exe_file])
            .output();

        Self::handle_compile_output(cmd_output, "Rustc", exe_file, output);
    }

    fn handle_compile_output(result: std::io::Result<std::process::Output>, name: &str, exe_file: &str, output: &mut String) {
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

    fn compile_nasm(code: &str, exe_file: &str, output: &mut String) {
        let temp_asm = "temp.asm";
        let temp_obj = "temp.obj";

        if let Err(e) = std::fs::write(temp_asm, code) {
            output.push_str(&format!("Error guardando archivo: {}\n", e));
            return;
        }

        // NASM
        match Command::new("nasm")
            .args(&["-f", "win64", temp_asm, "-o", temp_obj])
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
            .args(&[temp_obj, "-o", exe_file])
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
                    let _ = std::fs::remove_file(exe_file);
                }
            }
            Err(e) => {
                output.push_str(&format!("Error linkeando: {}\n", e));
            }
        }
    }
}

