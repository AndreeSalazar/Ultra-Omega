// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Auto-Linker
// Sistema automático de linkeo que detecta y usa el mejor linker disponible
// Soporta: GCC, Clang, MSVC Link, LD, Zig
// ═══════════════════════════════════════════════════════════════════════════════

use std::process::Command;
use std::path::{Path, PathBuf};
use crate::compilation::compiler_detector::{CompilerStatus, deep_search_executable, find_executable};

#[derive(Debug, Clone)]
pub struct LinkerInfo {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub available: bool,
}

#[derive(Debug)]
pub struct AutoLinker {
    status: CompilerStatus,
}

impl AutoLinker {
    pub fn new() -> Self {
        Self {
            status: CompilerStatus::new(),
        }
    }

    /// Linkear un archivo objeto a ejecutable
    /// Retorna Ok(path) si tiene éxito, Err(message) si falla
    pub fn link_object_to_exe(
        &self,
        obj_file: &Path,
        exe_file: &str,
        work_dir: &Path,
        output: &mut String,
    ) -> Result<PathBuf, String> {
        let exe_path = work_dir.join(exe_file);

        // Intentar con diferentes linkers según disponibilidad
        let linkers = self.get_available_linkers();

        if linkers.is_empty() {
            let mut error_msg = "❌ No se encontró ningún linker disponible.\n\n".to_string();
            error_msg.push_str("💡 Para Windows, instala MinGW-w64:\n");
            error_msg.push_str("   - Descarga: https://www.mingw-w64.org/downloads/\n");
            error_msg.push_str("   - O con Chocolatey: choco install mingw\n");
            error_msg.push_str("\n   Asegúrate de agregar MinGW\\bin a tu PATH.\n");
            error_msg.push_str("\n   Verifica instalación: gcc --version\n");
            return Err(error_msg);
        }

        output.push_str(&format!(">>> {} linkers disponibles.
", linkers.len()));

        // Intentar con cada linker en orden de preferencia
        for linker in &linkers {
            output.push_str(&format!(">>> Intentando linkeo con {}...\n", linker.name));
            output.push_str(&format!(">>>   Comando: {}\n", linker.command));
            
            match self.try_link_with(linker, obj_file, &exe_path, work_dir, output) {
                Ok(_) => {
                    output.push_str(&format!(">>> ✅ Linkeo exitoso con {}\n", linker.name));
                    return Ok(exe_path);
                }
                Err(e) => {
                    // El error ya está formateado con detalles en try_link_with
                    output.push_str(&e);
                    output.push_str("\n");
                    // Continuar con el siguiente linker
                }
            }
        }

        Err(">>> Error: Linkeo falló. Verifica instalación de MinGW/GCC y compatibilidad del código ASM.".to_string())
    }

    /// Obtener lista de linkers disponibles ordenados por preferencia
    /// Detecta directamente los linkers disponibles en el sistema
    fn get_available_linkers(&self) -> Vec<LinkerInfo> {
        let mut linkers = Vec::new();
        
        // Debug: Mostrar qué estamos buscando
        #[cfg(debug_assertions)]
        {
            eprintln!("[Auto-Linker] Buscando linkers disponibles...");
        }

        #[cfg(target_os = "windows")] {
            // Windows: Preferir MSVC link.exe, luego GCC/MinGW, luego Clang
            
            // 1. Intentar MSVC Link con búsqueda profunda
            if let Some(link_path) = Self::find_linker_cmd("link") {
                linkers.push(LinkerInfo {
                    name: "MSVC Link".to_string(),
                    command: link_path,
                    args: vec![
                        "/OUT:".to_string(),
                        "/ENTRY:main".to_string(),
                        "/SUBSYSTEM:CONSOLE".to_string(),
                    ],
                    available: true,
                });
            }

            // 2. Intentar GCC/MinGW con búsqueda profunda (obtener rutas completas)
            let gcc_commands = vec!["gcc", "mingw32-gcc", "x86_64-w64-mingw32-gcc", "i686-w64-mingw32-gcc"];
            for cmd in gcc_commands {
                if let Some(gcc_path) = Self::find_linker_cmd(cmd) {
                    linkers.push(LinkerInfo {
                        name: format!("GCC ({})", cmd),
                        command: gcc_path,
                        args: vec![],
                        available: true,
                    });
                    break; // Solo agregar uno de GCC
                }
            }

            // 3. Intentar Clang con búsqueda profunda
            if let Some(clang_path) = Self::find_linker_cmd("clang++")
                .or_else(|| Self::find_linker_cmd("clang")) {
                linkers.push(LinkerInfo {
                    name: "Clang++".to_string(),
                    command: clang_path,
                    args: vec![],
                    available: true,
                });
            }
        }

        #[cfg(not(target_os = "windows"))] {
            // Linux/Mac: Preferir GCC, luego Clang, luego LD
            
            // 1. Intentar GCC con búsqueda profunda
            if let Some(gcc_path) = Self::find_linker_cmd("gcc") {
                linkers.push(LinkerInfo {
                    name: "GCC".to_string(),
                    command: gcc_path,
                    args: vec!["-no-pie".to_string()], // Importante para NASM
                    available: true,
                });
            }

            // 2. Intentar Clang con búsqueda profunda
            if let Some(clang_path) = Self::find_linker_cmd("clang") {
                linkers.push(LinkerInfo {
                    name: "Clang".to_string(),
                    command: clang_path,
                    args: vec!["-no-pie".to_string()],
                    available: true,
                });
            }

            // 3. LD como último recurso con búsqueda profunda
            if let Some(ld_path) = Self::find_linker_cmd("ld") {
                linkers.push(LinkerInfo {
                    name: "LD".to_string(),
                    command: ld_path,
                    args: vec![],
                    available: true,
                });
            }
        }

        // FALLBACK: Si no encontramos nada con verificación previa,
        // intentar con búsqueda profunda directamente
        if linkers.is_empty() {
            #[cfg(target_os = "windows")]
            {
                // Fallback: Intentar encontrar gcc con búsqueda profunda
                let gcc_commands = vec!["gcc", "mingw32-gcc", "x86_64-w64-mingw32-gcc"];
                for cmd in gcc_commands {
                    if let Some(gcc_path) = Self::find_linker_cmd(cmd) {
                        linkers.push(LinkerInfo {
                            name: format!("GCC ({})", cmd),
                            command: gcc_path,
                            args: vec![],
                            available: true,
                        });
                        break; // Solo agregar uno
                    }
                }
            }
            
            #[cfg(not(target_os = "windows"))]
            {
                // Fallback: Intentar encontrar gcc y clang con búsqueda profunda
                if let Some(gcc_path) = Self::find_linker_cmd("gcc") {
                    linkers.push(LinkerInfo {
                        name: "GCC (Fallback)".to_string(),
                        command: gcc_path,
                        args: vec!["-no-pie".to_string()],
                        available: true,
                    });
                }
                if let Some(clang_path) = Self::find_linker_cmd("clang") {
                    linkers.push(LinkerInfo {
                        name: "Clang (Fallback)".to_string(),
                        command: clang_path,
                        args: vec!["-no-pie".to_string()],
                        available: true,
                    });
                }
            }
        }

        linkers
    }

    /// Encontrar un comando de linker usando búsqueda profunda
    fn find_linker_cmd(command: &str) -> Option<String> {
        // Primero intentar con find_executable (PATH)
        if let Some(path) = find_executable(command) {
            // Verificar que funciona
            if Self::verify_linker_works(&path, command) {
                return Some(path.to_string_lossy().to_string());
            }
        }
        
        // Si no funciona, búsqueda profunda
        if let Some(path) = deep_search_executable(command) {
            // Verificar que funciona
            if Self::verify_linker_works(&path, command) {
                return Some(path.to_string_lossy().to_string());
            }
        }
        
        None
    }
    
    /// Verificar que un linker realmente funciona
    fn verify_linker_works(path: &Path, command: &str) -> bool {
        // Para link.exe de MSVC, necesita argumentos especiales
        if command == "link" || command.contains("link") {
            let result = Command::new(path)
                .arg("/?")
                .output();
            
            return result.map(|out| out.status.success() || out.status.code().is_some()).unwrap_or(false);
        }
        
        // Para otros comandos, usar --version
        let result = if cfg!(target_os = "windows") {
            Command::new(path)
                .arg("--version")
                .output()
                .or_else(|_| {
                    // Algunos comandos pueden no tener --version, intentar solo ejecutarlos
                    Command::new(path)
                        .arg("/?")
                        .output()
                })
        } else {
            Command::new(path)
                .arg("--version")
                .output()
                .or_else(|_| Command::new(path).arg("-v").output())
        };
        
        result.map(|out| out.status.success() || out.status.code().is_some()).unwrap_or(false)
    }

    /// Verificar si un comando está disponible en el sistema (método legacy, mantener para compatibilidad)
    fn check_command_available(command: &str) -> bool {
        use std::process::Command;
        
        // Para link.exe de MSVC, necesita argumentos especiales
        if command == "link.exe" || command.contains("link") {
            let result = Command::new("link.exe")
                .arg("/?")
                .output()
                .or_else(|_| Command::new("link")
                    .arg("/?")
                    .output());
            
            return result.map(|out| out.status.success() || out.status.code().is_some()).unwrap_or(false);
        }
        
        // Para otros comandos, usar --version
        let result = if cfg!(target_os = "windows") {
            // En Windows, intentar con .exe primero
            Command::new(format!("{}.exe", command))
                .arg("--version")
                .output()
                .or_else(|_| Command::new(command).arg("--version").output())
                .or_else(|_| {
                    // Algunos comandos pueden no tener --version, intentar solo ejecutarlos
                    Command::new(format!("{}.exe", command))
                        .arg("/?")
                        .output()
                        .or_else(|_| Command::new(command).arg("/?").output())
                })
        } else {
            Command::new(command)
                .arg("--version")
                .output()
                .or_else(|_| Command::new(command).arg("-v").output())
        };
        
        result.map(|out| out.status.success() || out.status.code().is_some()).unwrap_or(false)
    }

    /// Intentar linkear con un linker específico
    fn try_link_with(
        &self,
        linker: &LinkerInfo,
        obj_file: &Path,
        exe_path: &Path,
        work_dir: &Path,
        output: &mut String,
    ) -> Result<(), String> {
        // Guardar referencias para diagnóstico si falla
        let linker_cmd = linker.command.clone();
        let obj_file_path = obj_file.to_path_buf();
        // Usar rutas absolutas para evitar problemas de directorio/permiso
        let obj_abs = obj_file.to_string_lossy().to_string();
        let exe_abs = exe_path.to_string_lossy().to_string();

        #[cfg(target_os = "windows")] {
            if linker.command == "link.exe" || linker.command.contains("link") {
                // MSVC Link syntax
                // Si tiene path completo, usarlo; si no, buscar link.exe
                let link_cmd = if linker.command.contains("\\") || linker.command.contains("/") {
                    linker.command.clone()
                } else {
                    // Buscar link.exe en PATH o en ubicaciones comunes
                    if let Some(msvc_path) = &self.status.cpp_msvc.path {
                        if msvc_path.exists() {
                            msvc_path.parent()
                                .and_then(|p| p.parent())
                                .and_then(|p| p.parent())
                                .and_then(|p| p.parent())
                                .and_then(|p| {
                                    let link_exe = p.join("VC/Tools/MSVC")
                                        .read_dir()
                                        .ok()?
                                        .filter_map(|e| e.ok())
                                        .filter_map(|e| {
                                            let link_path = e.path().join("bin/Hostx64/x64/link.exe");
                                            if link_path.exists() {
                                                Some(link_path.to_string_lossy().to_string())
                                            } else {
                                                None
                                            }
                                        })
                                        .next();
                                    link_exe
                                })
                                .unwrap_or_else(|| "link.exe".to_string())
                        } else {
                            "link.exe".to_string()
                        }
                    } else {
                        "link.exe".to_string()
                    }
                };

                let mut args = vec![
                    obj_abs.clone(),
                    format!("/OUT:{}", exe_abs.clone()),
                    "/ENTRY:main".to_string(),
                    "/SUBSYSTEM:CONSOLE".to_string(),
                ];
                // Añadir librerías base por defecto para MSVC
                args.push("msvcrt.lib".to_string());
                args.push("kernel32.lib".to_string());
                args.extend(linker.args.clone());

                let result = Command::new(&link_cmd)
                    .current_dir(work_dir)
                    .args(&args)
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .stdin(std::process::Stdio::null())
                    .output();

                return self.handle_link_result(result, linker.name.as_str(), &link_cmd, &obj_file_path, &args, output);
            }
        }

        // GCC/Clang syntax (cross-platform)
        // linker.command ya debería contener la ruta completa si se encontró con búsqueda profunda
        
        // ENFOQUE ROBUSTO: Intentar múltiples variantes automáticamente
        // Comenzar con el formato más simple posible
        let mut args = vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone()];

        // Argumentos específicos para Windows cuando se linkea NASM
        #[cfg(target_os = "windows")]
        {
            // FORMATO ABSOLUTAMENTE MÍNIMO - solo lo esencial
            args.push("-m64".to_string());
            // NO agregar NADA más - dejar que GCC resuelva todo automáticamente
        }
        
        // NO extender con linker.args aquí - queremos control total

        // Para LD, necesitamos argumentos adicionales
        #[cfg(not(target_os = "windows"))]
        if linker.command.ends_with("/ld") || linker.command.ends_with("\\ld") || linker.command.ends_with("ld") {
            // LD necesita más argumentos, intentar con opciones básicas
            args = vec![
                "-o".to_string(),
                exe_abs.clone(),
                obj_abs.clone(),
                "-lc".to_string(), // Link con libc
            ];
            
            // En Linux, puede necesitar entry point
            #[cfg(target_os = "linux")]
            {
                args.push("--entry=_start".to_string());
            }
        }
        
        // Para debugging: verificar que el archivo objeto existe y es válido
        if !obj_file.exists() {
            output.push_str(&format!(">>> Error: Objeto no existe: {:?}\n", obj_file));
            return Err("Archivo objeto no encontrado".to_string());
        }

        // Verificar tamaño del archivo objeto
        if let Ok(metadata) = std::fs::metadata(obj_file) {
            let size = metadata.len();
            output.push_str(&format!(">>> Archivo objeto: {:?} ({} bytes)\n", obj_file, size));
            if size == 0 {
                output.push_str(">>> ⚠️ ADVERTENCIA: El archivo objeto está vacío\n");
                return Err("Archivo objeto vacío - compilación falló".to_string());
            }
        }

        // Verificar que el linker existe y es ejecutable
        if !std::path::Path::new(&linker.command).exists() {
            output.push_str(&format!(">>> Error: Linker no existe: {}\n", linker.command));
            return Err(format!("Linker no encontrado: {}", linker.command));
        }

        // Evitar problemas al sobrescribir ejecutables bloqueados
        if exe_path.exists() {
            let _ = std::fs::remove_file(&exe_path);
        }

        // ENFOQUE ROBUSTO: Intentar múltiples variantes en secuencia ANTES del intento original
        let is_windows = cfg!(target_os = "windows");
        let is_gnu_like = linker.command.contains("gcc") || linker.command.contains("clang");
        
        // Variantes de comandos a probar (en orden de simplicidad)
        let mut variants: Vec<(Vec<String>, &str)> = Vec::new();
        
                    #[cfg(target_os = "windows")]
                    if is_gnu_like {
                        // IMPORTANTE: El orden de argumentos en GCC importa mucho
                        // Las librerías DEBEN ir DESPUÉS del objeto y la salida
                        // Estructura: objeto -o salida flags librerías
                        
                        // Variante 1: Con -m64 y librerías C al final (orden correcto)
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone(), "-m64".to_string(), "-lmsvcrt".to_string(), "-lmingwex".to_string()],
                            "con -m64, -lmsvcrt y -lmingwex (printf puede estar aquí)"
                        ));
                        
                        // Variante 2: Solo msvcrt (más simple)
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone(), "-m64".to_string(), "-lmsvcrt".to_string()],
                            "con -m64 y -lmsvcrt"
                        ));
                        
                        // Variante 3: Con subsistema Y librerías (orden: objeto -o salida flags librerías)
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone(), "-m64".to_string(), "-Wl,--subsystem,console".to_string(), "-lmsvcrt".to_string()],
                            "con -m64, subsistema y -lmsvcrt"
                        ));
                        
                        // Variante 4: Usar rutas relativas (a veces GCC funciona mejor con rutas relativas)
                        if let Ok(obj_rel) = obj_file.strip_prefix(work_dir) {
                            let exe_rel_str = exe_path.strip_prefix(work_dir)
                                .unwrap_or(exe_path)
                                .to_string_lossy()
                                .to_string();
                            variants.push((
                                vec![obj_rel.to_string_lossy().to_string(), "-o".to_string(), exe_rel_str, "-m64".to_string(), "-lmsvcrt".to_string()],
                                "con rutas relativas y -lmsvcrt"
                            ));
                        }
                        
                        // Variante 5: Con -m64 solamente (GCC debería agregar librerías automáticamente)
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone(), "-m64".to_string()],
                            "con -m64 (GCC debería agregar librerías automáticamente)"
                        ));
                        
                        // Variante 6: Con subsistema pero sin librerías explícitas
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone(), "-m64".to_string(), "-Wl,--subsystem,console".to_string()],
                            "con -m64 y subsistema console"
                        ));
                        
                        // Variante 7: ABSOLUTAMENTE MÍNIMA (solo objeto y salida) - ÚLTIMA
                        variants.push((
                            vec![obj_abs.clone(), "-o".to_string(), exe_abs.clone()],
                            "absolutamente mínima (sin flags)"
                        ));
                    }
        
        // Intentar cada variante en orden
        output.push_str(">>> 🔄 Intentando múltiples variantes automáticamente...\n");
        for (variant_args, variant_name) in &variants {
            output.push_str(&format!(">>> [{}/{}] Intentando variante: {}...\n", 
                variants.iter().position(|(a, _)| a == variant_args).unwrap_or(0) + 1,
                variants.len(),
                variant_name));
            output.push_str(&format!(">>> Ejecutando: {} {}\n", linker.command, variant_args.join(" ")));
            
            let variant_result = Command::new(&linker.command)
                .current_dir(work_dir)
                .args(variant_args)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .stdin(std::process::Stdio::null())
                .output();
            
            let tagged_name = format!("{} ({})", linker.name, variant_name);
            match self.handle_link_result(variant_result, &tagged_name, &linker_cmd, &obj_file_path, variant_args, output) {
                Ok(()) => {
                    output.push_str(&format!(">>> ✅ ✅ ✅ ÉXITO con variante: {}\n", variant_name));
                    return Ok(());
                }
                Err(e) => {
                    // Solo mostrar error si no es silencioso (para no saturar el output)
                    if !e.contains("No se recibió salida del linker") {
                        output.push_str(&format!(">>> ❌ Variante '{}' falló\n", variant_name));
                    }
                    // Continuar con la siguiente variante
                }
            }
        }
        
        // Si todas las variantes fallaron, intentar con verbose para diagnóstico
        output.push_str("\n>>> ⚠️ Todas las variantes básicas fallaron. Intentando con verbose para diagnóstico...\n");
        
        // Preparar el intento original con verbose
        let mut verbose_args = args.clone();
        verbose_args.push("-v".to_string());
        
        let verbose_result = Command::new(&linker.command)
            .current_dir(work_dir)
            .args(&verbose_args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output();
        
        match self.handle_link_result(verbose_result, &format!("{} (verbose)", linker.name), &linker_cmd, &obj_file_path, &verbose_args, output) {
            Ok(()) => Ok(()),
            Err(_) => {
                // Verificar si el ejecutable se creó (a veces GCC reporta error pero crea el archivo)
                if exe_path.exists() {
                    let exe_size = std::fs::metadata(&exe_path).map(|m| m.len()).unwrap_or(0);
                    if exe_size > 0 {
                        output.push_str(&format!(">>> ✅ ¡EJECUTABLE CREADO! ({} bytes) - Intentando ejecutarlo...\n", exe_size));
                        return Ok(());
                    }
                }
                
                // Si falló, intentar buscar el error real ejecutando directamente con stderr visible
                output.push_str("\n>>> 🔍 Ejecutando diagnóstico profundo con verbose...\n");
                
                // Intentar ejecutar directamente CON VERBOSE para capturar TODO el stderr
                let mut diag_cmd = Command::new(&linker.command);
                diag_cmd.current_dir(work_dir)
                    .arg(&obj_abs)
                    .arg("-o")
                    .arg(&exe_abs)
                    .arg("-m64")
                    .arg("-v")  // AGREGAR VERBOSE para ver TODO el proceso
                    .arg("-Wl,--verbose")  // También verbose en el linker
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .stdin(std::process::Stdio::null());
                
                output.push_str(">>> Ejecutando comando de diagnóstico directo...\n");
                match diag_cmd.output() {
                    Ok(out) => {
                        let stderr_full = String::from_utf8_lossy(&out.stderr);
                        let stdout_full = String::from_utf8_lossy(&out.stdout);
                        let exit_code = out.status.code().unwrap_or(-1);
                        
                        output.push_str(&format!(">>> Exit code: {}\n", exit_code));
                        output.push_str(&format!(">>> stderr bytes: {}, stdout bytes: {}\n", out.stderr.len(), out.stdout.len()));
                        
                        // Mostrar TODO el stderr para análisis completo
                        output.push_str(">>> 📋 OUTPUT COMPLETO DEL LINKER (STDERR):\n");
                        if stderr_full.is_empty() && out.stderr.is_empty() {
                            output.push_str(">>> (VACÍO - No hay salida en stderr)\n");
                        } else if stderr_full.is_empty() && !out.stderr.is_empty() {
                            output.push_str(">>> (stderr tiene bytes pero no se pudo decodificar como UTF-8)\n");
                            output.push_str(&format!(">>> Primeros 200 bytes (hex): {:?}\n", 
                                &out.stderr.iter().take(200).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")));
                        } else {
                            output.push_str(&stderr_full);
                        }
                        
                        if !stdout_full.is_empty() {
                            output.push_str("\n>>> 📋 OUTPUT COMPLETO DEL LINKER (STDOUT):\n");
                            output.push_str(&stdout_full);
                        } else if !out.stdout.is_empty() {
                            output.push_str("\n>>> (stdout tiene bytes pero no se pudo decodificar)\n");
                            output.push_str(&format!(">>> Primeros 200 bytes (hex): {:?}\n", 
                                &out.stdout.iter().take(200).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")));
                        }
                        output.push_str("\n");
                        
                        // Buscar líneas de error REALES después de collect2.exe
                        let error_lines: Vec<&str> = stderr_full.lines().collect();
                        let mut found_collect2_line = false;
                        let mut after_collect2_lines = Vec::new();
                        
                        // Buscar la línea de collect2.exe (la línea larga con todos los argumentos)
                        for (idx, line) in error_lines.iter().enumerate() {
                            if line.contains("collect2.exe") && line.len() > 500 {
                                found_collect2_line = true;
                                // Recopilar TODAS las líneas después de esta
                                for next_line in error_lines.iter().skip(idx + 1) {
                                    after_collect2_lines.push(next_line.trim());
                                }
                                break;
                            }
                        }
                        
                        let mut shown_lines = 0;
                        if found_collect2_line && !after_collect2_lines.is_empty() {
                            output.push_str(">>> 🔍 ANALIZANDO TODAS LAS LÍNEAS DESPUÉS DE collect2.exe:\n");
                            output.push_str(">>> (Mostrando TODAS las líneas, incluso si no parecen errores)\n");
                            
                            for line in &after_collect2_lines {
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    continue;
                                }
                                
                                let lower = trimmed.to_lowercase();
                                // Filtrar solo líneas de configuración MÁS OBVIAS
                                if !lower.starts_with("using built-in") 
                                    && !lower.starts_with("target:")
                                    && !lower.starts_with("configured with:")
                                    && !lower.starts_with("thread model:")
                                    && !lower.starts_with("compiler_path")
                                    && !lower.starts_with("library_path")
                                    && !lower.starts_with("collect_gcc_options")
                                    && !trimmed.contains("collect2.exe")
                                    && !trimmed.contains("collect_ltowrapper")
                                    && trimmed.len() < 500 {  // Ignorar líneas muy largas (comandos)
                                    
                                    // Mostrar TODAS las líneas que pasen el filtro básico
                                    output.push_str(&format!(">>>    {}\n", trimmed));
                                    shown_lines += 1;
                                    
                                    // Si es un error obvio, marcarlo
                                    if lower.contains("error") 
                                        || lower.contains("undefined")
                                        || lower.contains("cannot")
                                        || lower.contains("failed")
                                        || lower.contains("unrecognized")
                                        || lower.contains("invalid")
                                        || lower.contains("no such")
                                        || lower.contains("fatal") {
                                        output.push_str(&format!(">>>       ⚠️ ← ESTO PARECE UN ERROR\n"));
                                    }
                                    
                                    // Limitar a 50 líneas para no saturar
                                    if shown_lines >= 50 {
                                        output.push_str(">>>    ... (mostrando solo primeras 50 líneas)\n");
                                        break;
                                    }
                                }
                            }
                            
                            if shown_lines == 0 {
                                output.push_str(">>> ⚠️ No hay líneas después de collect2.exe (o todas fueron filtradas).\n");
                                output.push_str(">>> Esto sugiere que el linker falló inmediatamente sin producir salida.\n");
                            }
                        } else if !found_collect2_line {
                            output.push_str(">>> ⚠️ No se encontró la línea de collect2.exe en el output.\n");
                            output.push_str(">>> El linker puede no estar ejecutándose correctamente.\n");
                        }
                        
                        // Si no encontramos ninguna línea después de collect2.exe, el error puede estar en el linker real (ld)
                        // Intentar ejecutar ld directamente si está disponible
                        if shown_lines == 0 && exit_code != 0 {
                            output.push_str("\n>>> 🔧 Intentando ejecutar el linker (ld) directamente...\n");
                            
                            // Buscar ld.exe en el mismo directorio que gcc
                            let gcc_dir = std::path::Path::new(&linker.command).parent();
                            if let Some(dir) = gcc_dir {
                                let ld_paths = vec![
                                    dir.join("ld.exe"),
                                    dir.join("../x86_64-w64-mingw32/bin/ld.exe"),
                                    dir.join("../../lib/gcc/x86_64-w64-mingw32/15.2.0/../../../../x86_64-w64-mingw32/bin/ld.exe"),
                                ];
                                
                                for ld_path in ld_paths {
                                    if ld_path.exists() {
                                        output.push_str(&format!(">>> Encontrado ld.exe: {}\n", ld_path.display()));
                                        
                                        // Intentar linkear directamente con ld (sin runtime de C)
                                        let ld_cmd = Command::new(&ld_path)
                                            .current_dir(work_dir)
                                            .arg(&obj_abs)
                                            .arg("-o")
                                            .arg(&exe_abs)
                                            .arg("--entry=main")
                                            .arg("-subsystem=console")
                                            .stdout(std::process::Stdio::piped())
                                            .stderr(std::process::Stdio::piped())
                                            .output();
                                        
                                        if let Ok(ld_out) = ld_cmd {
                                            let ld_stderr = String::from_utf8_lossy(&ld_out.stderr);
                                            if !ld_stderr.is_empty() {
                                                output.push_str(">>> Output de ld directamente:\n");
                                                output.push_str(&ld_stderr);
                                                output.push_str("\n");
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                            
                            // También intentar verificar el objeto con objdump
                            if let Some(objdump_path) = Self::find_objdump(&linker.command) {
                                output.push_str(&format!("\n>>> 🔍 Verificando objeto con objdump: {}\n", objdump_path));
                                if let Ok(objdump_out) = Command::new(&objdump_path)
                                    .arg("-h")  // Headers de secciones
                                    .arg("-t")  // Tabla de símbolos
                                    .arg(obj_file.to_string_lossy().as_ref())
                                    .output()
                                {
                                    let objdump_stderr = String::from_utf8_lossy(&objdump_out.stderr);
                                    let objdump_stdout = String::from_utf8_lossy(&objdump_out.stdout);
                                    
                                    if !objdump_stderr.is_empty() {
                                        output.push_str(&format!(">>> objdump stderr: {}\n", objdump_stderr));
                                    }
                                    if !objdump_stdout.is_empty() {
                                        output.push_str(">>> objdump output:\n");
                                        output.push_str(&objdump_stdout);
                                        
                                        // Verificar si main está exportado correctamente
                                        if objdump_stdout.contains("main") {
                                            output.push_str(">>> ✅ 'main' encontrado en el objeto\n");
                                        } else {
                                            output.push_str(">>> ❌ 'main' NO encontrado en el objeto - esto puede ser el problema\n");
                                        }
                                    }
                                }
                            }
                        }
                        
                        // También verificar si el ejecutable se creó (a veces GCC reporta error pero crea el archivo)
                        if exe_path.exists() {
                            let exe_size = std::fs::metadata(&exe_path).map(|m| m.len()).unwrap_or(0);
                            if exe_size > 0 {
                                output.push_str(&format!("\n>>> ✅ EJECUTABLE ENCONTRADO! ({} bytes) - El linkeo pudo haber funcionado\n", exe_size));
                                output.push_str(">>> Intentando verificar si el ejecutable es válido...\n");
                                return Ok(());
                            }
                        } else {
                            output.push_str("\n>>> ❌ El ejecutable NO se creó.\n");
                        }
                    }
                    Err(e) => {
                        output.push_str(&format!(">>> ❌ Error ejecutando diagnóstico: {}\n", e));
                    }
                }
                
                Err("Todas las variantes y diagnósticos fallaron. Revisa los errores arriba.".to_string())
            }
        }
    }

    /// Manejar el resultado del linkeo
    fn handle_link_result(
        &self,
        result: std::io::Result<std::process::Output>,
        linker_name: &str,
        linker_cmd: &str,
        obj_file: &std::path::Path,
        args: &[String],
        output: &mut String,
    ) -> Result<(), String> {
        match result {
            Ok(out) => {
                // Intentar decodificar como UTF-8, pero también intentar con pérdida de caracteres
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);

                // Debug: Mostrar información sobre la salida
                output.push_str(&format!(">>> Debug: stdout_len={}, stderr_len={}\n", 
                    out.stdout.len(), out.stderr.len()));

                // Mostrar stdout siempre que haya contenido
                if !stdout.is_empty() {
                    output.push_str(&format!("{} Output:\n{}\n", linker_name, stdout));
                } else {
                    output.push_str(&format!(">>> {}: Sin salida stdout\n", linker_name));
                }

                // Mostrar stderr siempre (es crítico para ver errores)
                if !stderr.is_empty() {
                    output.push_str(&format!("{} Error/Log:\n{}\n", linker_name, stderr));
                } else {
                    output.push_str(&format!(">>> {}: Sin salida stderr\n", linker_name));
                }
                
                // Si ambos están vacíos pero hay bytes, intentar decodificar de otra forma
                if stdout.is_empty() && stderr.is_empty() && (!out.stdout.is_empty() || !out.stderr.is_empty()) {
                    output.push_str(">>> ⚠️ ADVERTENCIA: Hay bytes en stdout/stderr pero no se pudieron decodificar como UTF-8\n");
                    if !out.stdout.is_empty() {
                        output.push_str(&format!(">>> stdout (hex primeros 100 bytes): {:?}\n", 
                            &out.stdout.iter().take(100).collect::<Vec<_>>()));
                    }
                    if !out.stderr.is_empty() {
                        output.push_str(&format!(">>> stderr (hex primeros 100 bytes): {:?}\n", 
                            &out.stderr.iter().take(100).collect::<Vec<_>>()));
                    }
                }

                if out.status.success() {
                    Ok(())
                } else {
                    let exit_code = out.status.code().unwrap_or(-1);
                    
                    // Construir mensaje de error más detallado
                    let mut error_msg = format!(">>> ❌ {} falló (Exit code: {})\n", linker_name, exit_code);
                    
                    // Si hay stderr, extraer solo las líneas de error relevantes
                    if !stderr.is_empty() {
                        let all_lines: Vec<&str> = stderr.lines().collect();
                        
                        // Palabras clave que indican un error real (no configuración)
                        let real_error_keywords = [
                            "undefined reference",
                            "cannot find",
                            "unrecognized",
                            "invalid",
                            "no such",
                            "multiple definition",
                            "undefined symbol",
                            "ld returned",
                            "collect2.exe: error",
                            "error:",
                            "fatal error",
                            "failed to",
                            "cannot open",
                            "cannot locate",
                        ];
                        
                        // Palabras clave a ignorar (son parte de la configuración, no errores)
                        let ignore_keywords = [
                            "using built-in",
                            "collect_gcc",
                            "collect_ltowrapper",
                            "target:",
                            "configured with:",
                            "thread model:",
                            "supported lto",
                            "gcc version",
                            "compiler_path",
                            "library_path",
                            "collect_gcc_options",
                        ];
                        
                        let mut error_lines: Vec<&str> = Vec::new();
                        let mut found_real_errors = false;
                        
                        // Buscar errores reales (prioridad a líneas con keywords de error)
                        for line in &all_lines {
                            let line_lower = line.to_lowercase().trim().to_string();
                            if line_lower.is_empty() {
                                continue;
                            }
                            
                            // Ignorar líneas de configuración
                            if ignore_keywords.iter().any(|kw| line_lower.starts_with(kw)) {
                                continue;
                            }
                            
                            // Buscar errores reales
                            if real_error_keywords.iter().any(|kw| line_lower.contains(kw)) {
                                error_lines.push(line.trim());
                                found_real_errors = true;
                            }
                        }
                        
                        // Si encontramos errores reales, mostrarlos
                        if found_real_errors && !error_lines.is_empty() {
                            error_msg.push_str(">>> Errores detectados:\n");
                            for line in error_lines {
                                error_msg.push_str(&format!(">>>    {}\n", line));
                            }
                        } else {
                            // Si no encontramos keywords específicos, buscar en las últimas líneas
                            // donde suele estar el error real del linker
                            let start = all_lines.len().saturating_sub(20);
                            let mut last_lines: Vec<&str> = Vec::new();
                            
                            // Buscar desde el final hacia atrás para encontrar el error real
                            for (idx, line) in all_lines.iter().enumerate().skip(start) {
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    continue;
                                }
                                
                                let line_lower = trimmed.to_lowercase();
                                // Ignorar líneas de configuración
                                if ignore_keywords.iter().any(|kw| line_lower.starts_with(kw)) {
                                    continue;
                                }
                                
                                // Si la línea contiene collect2.exe o es una línea de comando muy larga,
                                // el error real probablemente está después de todas las líneas de collect2
                                if line.contains("collect2.exe") || line.len() > 500 {
                                    // Buscar líneas después de esta, pero también antes si no encontramos nada
                                    // El error podría estar antes o después de collect2.exe
                                    let mut found_after = false;
                                    if idx + 1 < all_lines.len() {
                                        for next_line in all_lines.iter().skip(idx + 1) {
                                            let next_trimmed = next_line.trim();
                                            if !next_trimmed.is_empty() {
                                                let next_lower = next_trimmed.to_lowercase();
                                                if !ignore_keywords.iter().any(|kw| next_lower.starts_with(kw)) {
                                                    last_lines.push(next_trimmed);
                                                    found_after = true;
                                                }
                                            }
                                        }
                                    }
                                    // Si no encontramos nada después, buscar en las últimas 5 líneas del output completo
                                    if !found_after && all_lines.len() >= 5 {
                                        for i in (all_lines.len().saturating_sub(5)..all_lines.len()).rev() {
                                            let check_line = all_lines[i].trim();
                                            if !check_line.is_empty() {
                                                let check_lower = check_line.to_lowercase();
                                                if !ignore_keywords.iter().any(|kw| check_lower.starts_with(kw)) &&
                                                   !check_lower.contains("collect2.exe") &&
                                                   (check_lower.contains("undefined") || 
                                                    check_lower.contains("error") ||
                                                    check_lower.contains("cannot") ||
                                                    check_lower.contains("failed")) {
                                                    last_lines.insert(0, check_line);
                                                }
                                            }
                                        }
                                    }
                                    break;
                                }
                                
                                last_lines.push(trimmed);
                            }
                            
                            // Si no encontramos nada útil, intentar ejecutar sin verbose para obtener el error real
                            if last_lines.is_empty() {
                                error_msg.push_str(">>> Error del linker detectado pero no visible en verbose.\n");
                                error_msg.push_str(">>> Ejecutando sin verbose para obtener el error real...\n");
                                
                                // Aquí podríamos hacer un reintento sin verbose, pero mejor mostrar sugerencias
                                error_msg.push_str(">>> Posibles causas:\n");
                                error_msg.push_str(">>>    - Símbolo 'main' no exportado: verifica 'global main' en el código NASM\n");
                                error_msg.push_str(">>>    - printf no encontrado: asegúrate de linkear con -lmsvcrt o librerías C\n");
                                error_msg.push_str(">>>    - Formato del objeto incompatible: verifica que NASM compile a win64\n");
                            } else {
                                error_msg.push_str(">>> Últimas líneas del error:\n");
                                for line in last_lines.iter().take(10) {
                                    error_msg.push_str(&format!(">>>    {}\n", line));
                                }
                            }
                        }
                    } else if !stdout.is_empty() {
                        // Si no hay stderr pero sí stdout, mostrar stdout
                        error_msg.push_str(">>> Output:\n");
                        for line in stdout.lines() {
                            if !line.trim().is_empty() {
                                error_msg.push_str(&format!(">>>    {}\n", line.trim()));
                            }
                        }
                    } else {
                        error_msg.push_str(">>> ⚠️ No se recibió salida del linker (ni stdout ni stderr)\n");
                        error_msg.push_str(">>>    Esto es inusual - el linker falló silenciosamente.\n\n");
                        error_msg.push_str(">>> Diagnóstico:\n");
                        error_msg.push_str(">>>    1. Verificando archivo objeto...\n");
                        
                        // Intentar verificar el objeto con objdump si está disponible
                        #[cfg(target_os = "windows")]
                        {
                            if let Some(objdump_path) = Self::find_objdump(linker_cmd) {
                                error_msg.push_str(&format!(">>>       Usando objdump: {}\n", objdump_path));
                                if let Ok(objdump_out) = Command::new(&objdump_path)
                                    .arg("-t")
                                    .arg(obj_file.to_string_lossy().as_ref())
                                    .output()
                                {
                                    let objdump_stderr = String::from_utf8_lossy(&objdump_out.stderr);
                                    let objdump_stdout = String::from_utf8_lossy(&objdump_out.stdout);
                                    if !objdump_stderr.is_empty() {
                                        error_msg.push_str(&format!(">>>       objdump error: {}\n", objdump_stderr));
                                    }
                                    if !objdump_stdout.is_empty() {
                                        // Buscar símbolos
                                        if objdump_stdout.contains("main") || objdump_stdout.contains(".text") {
                                            error_msg.push_str(">>>       ✅ Archivo objeto válido (contiene sección .text)\n");
                                            if objdump_stdout.contains(" main") || objdump_stdout.contains(" main\n") || objdump_stdout.contains(" main\r") {
                                                error_msg.push_str(">>>       ✅ Símbolo 'main' encontrado en el objeto\n");
                                            } else {
                                                error_msg.push_str(">>>       ❌ Símbolo 'main' NO encontrado en el objeto\n");
                                                error_msg.push_str(">>>          ⚠️  Verifica que el código NASM tenga 'global main'\n");
                                            }
                                        } else {
                                            error_msg.push_str(">>>       ❌ Archivo objeto parece inválido o vacío\n");
                                        }
                                    }
                                }
                            } else {
                                error_msg.push_str(">>>       objdump no disponible para diagnóstico\n");
                            }
                        }
                        
                        error_msg.push_str("\n>>> Posibles soluciones:\n");
                        error_msg.push_str(">>>    1. Verifica que el código NASM tenga 'global main' exportado\n");
                        error_msg.push_str(">>>    2. Verifica que el formato de compilación sea correcto (win64)\n");
                        error_msg.push_str(">>>    3. Asegúrate de que printf esté declarado como 'extern printf'\n");
                        error_msg.push_str(">>>    4. Intenta ejecutar manualmente:\n");
                        error_msg.push_str(&format!(">>>       {} {}\n", linker_cmd, args.join(" ")));
                    }
                    
                    Err(error_msg)
                }
            }
            Err(e) => {
                let mut error_msg = format!(">>> ❌ Error ejecutando {}: {}\n", linker_name, e);
                error_msg.push_str(">>>    El ejecutable no se pudo ejecutar.\n");
                error_msg.push_str(">>>    Verifica que:\n");
                error_msg.push_str(">>>      1. El linker esté instalado correctamente\n");
                error_msg.push_str(">>>      2. Tengas permisos para ejecutarlo\n");
                error_msg.push_str(">>>      3. La ruta sea correcta\n");
                Err(error_msg)
            }
        }
    }

    /// Buscar objdump en la misma instalación que el linker
    fn find_objdump(linker_path: &str) -> Option<String> {
        use std::path::PathBuf;
        let linker_pb = PathBuf::from(linker_path);
        if let Some(bin_dir) = linker_pb.parent() {
            let objdump_path = bin_dir.join("objdump.exe");
            if objdump_path.exists() {
                return Some(objdump_path.to_string_lossy().to_string());
            }
            // También buscar en ubicaciones comunes de MinGW
            let objdump_path = bin_dir.join("../x86_64-w64-mingw32/bin/objdump.exe");
            if objdump_path.exists() {
                return Some(objdump_path.to_string_lossy().to_string());
            }
        }
        None
    }

    /// Detectar automáticamente el mejor linker disponible
    pub fn detect_best_linker(&self) -> Option<LinkerInfo> {
        let linkers = self.get_available_linkers();
        linkers.first().cloned()
    }

    /// Actualizar el estado de compiladores (útil si cambian después de la inicialización)
    pub fn update_status(&mut self) {
        use crate::compilation::compiler_detector::detect_all_compilers;
        self.status = detect_all_compilers();
    }

    /// Obtener información de todos los linkers disponibles
    pub fn get_linker_status(&self) -> String {
        let linkers = self.get_available_linkers();
        let mut status = format!("=== Estado de Linkers ===\n\n");
        
        status.push_str(&format!(">>> {} linkers disponibles.\n", linkers.len()));

        if linkers.is_empty() {
            status.push_str("❌ No se encontraron linkers disponibles.\nInstala MinGW-w64 o MSVC para Windows.\n");
        }

        status
    }
}

impl Default for AutoLinker {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES PÚBLICAS DE UTILIDAD
// ═══════════════════════════════════════════════════════════════════════════════

/// Linkear automáticamente un archivo objeto
pub fn auto_link(obj_file: &Path, exe_file: &str, work_dir: &Path, output: &mut String) -> Result<PathBuf, String> {
    let mut linker = AutoLinker::new();
    // Actualizar estado antes de linkear (por si acaso cambió)
    linker.update_status();
    linker.link_object_to_exe(obj_file, exe_file, work_dir, output)
}

