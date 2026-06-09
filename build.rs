use shaderc::{Compiler, CompileOptions, ShaderKind};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let compiler = Compiler::new().expect("Failed to create shaderc compiler");
    let mut options = CompileOptions::new().expect("Failed to create compile options");
    options.set_target_env(shaderc::TargetEnv::Vulkan, 0);

    let out_dir = env::var("OUT_DIR").unwrap();
    let shader_dir = Path::new("shaders");

    // Crear directorio de shaders si no existe
    if !shader_dir.exists() {
        fs::create_dir_all(shader_dir).unwrap();
    }

    for entry in fs::read_dir(shader_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str());

        if ext == Some("vert") || ext == Some("frag") {
            let kind = if ext == Some("vert") {
                ShaderKind::Vertex
            } else {
                ShaderKind::Fragment
            };

            let source = fs::read_to_string(&path).unwrap();
            let result = compiler
                .compile_into_spirv(&source, kind, path.to_str().unwrap(), "main", Some(&options))
                .unwrap_or_else(|e| panic!("Shader compilation failed for {:?}: {}", path, e));

            let shader_name = path.file_name().unwrap().to_string_lossy();
            let out_path = Path::new(&out_dir).join(format!("{}.spv", shader_name));
            
            fs::write(&out_path, result.as_binary_u8()).unwrap();
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
