/* ═══════════════════════════════════════════════════════════════════════════
 * RUST UTILS - Utilidades generales
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: lib_utils.rs
 * Descripción: Funciones utilidades generales reutilizables para Rust
 * 
 * USO: Este módulo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a utils con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════
// LOGGING Y DEBUG
// ═══════════════════════════════════════════════════════════════════════════

/// Nivel de log
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Logger simple
pub struct Logger {
    pub min_level: LogLevel,
}

impl Logger {
    pub fn new(min_level: LogLevel) -> Self {
        Logger { min_level }
    }
    
    pub fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.min_level as u8 {
            println!("[{}] {}", level, message);
        }
    }
    
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }
    
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

// Logger global por defecto
pub static mut GLOBAL_LOGGER: Option<Logger> = None;

pub fn init_logger(min_level: LogLevel) {
    unsafe {
        GLOBAL_LOGGER = Some(Logger::new(min_level));
    }
}

pub fn log(level: LogLevel, message: &str) {
    unsafe {
        if let Some(ref logger) = GLOBAL_LOGGER {
            logger.log(level, message);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE TIEMPO
// ═══════════════════════════════════════════════════════════════════════════

/// Obtener timestamp actual en segundos
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

/// Obtener timestamp actual en milisegundos
pub fn get_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

/// Medir tiempo de ejecución de una función
pub fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = SystemTime::now();
    let result = f();
    let elapsed = start.elapsed().unwrap_or(Duration::from_secs(0));
    (result, elapsed)
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE FORMATO
// ═══════════════════════════════════════════════════════════════════════════

/// Formatear bytes como tamaño legible
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Formatear duración como tiempo legible
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if secs > 0 {
        format!("{}s {}ms", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILIDADES DE STRING
// ═══════════════════════════════════════════════════════════════════════════

/// Capitalizar primera letra
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Repetir string n veces
pub fn repeat_string(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Verificar si string contiene solo dígitos
pub fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

