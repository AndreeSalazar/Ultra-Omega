/* ═══════════════════════════════════════════════════════════════════════════
 * RUST IO - Utilidades de entrada/salida
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: lib_io.rs
 * Descripción: Funciones de I/O reutilizables para Rust
 * 
 * USO: Este módulo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a io con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════
// LECTURA DE ARCHIVOS
// ═══════════════════════════════════════════════════════════════════════════

/// Leer archivo completo como String
pub fn read_file_string(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Leer archivo completo como bytes
pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(path)
}

/// Verificar si archivo existe
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Obtener tamaño de archivo
pub fn file_size(path: &str) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

// ═══════════════════════════════════════════════════════════════════════════
// ESCRITURA DE ARCHIVOS
// ═══════════════════════════════════════════════════════════════════════════

/// Escribir string a archivo
pub fn write_file_string(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Escribir bytes a archivo
pub fn write_file_bytes(path: &str, content: &[u8]) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Crear directorio (con padres si no existen)
pub fn create_dir_all(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path)
}

/// Eliminar archivo
pub fn remove_file(path: &str) -> Result<(), io::Error> {
    fs::remove_file(path)
}

/// Eliminar directorio (vacío)
pub fn remove_dir(path: &str) -> Result<(), io::Error> {
    fs::remove_dir(path)
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE PATH
// ═══════════════════════════════════════════════════════════════════════════

/// Obtener extensión de archivo
pub fn get_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Obtener nombre de archivo (sin path)
pub fn get_filename(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

/// Obtener directorio padre
pub fn get_parent(path: &str) -> Option<String> {
    Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}

/// Unir paths
pub fn join_paths(path1: &str, path2: &str) -> String {
    Path::new(path1)
        .join(path2)
        .to_string_lossy()
        .to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// ENTRADA ESTÁNDAR
// ═══════════════════════════════════════════════════════════════════════════

/// Leer línea desde stdin
pub fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

/// Escribir a stdout
pub fn print(text: &str) {
    print!("{}", text);
    io::stdout().flush().ok();
}

/// Escribir línea a stdout
pub fn println(text: &str) {
    println!("{}", text);
}

