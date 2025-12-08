/* ═══════════════════════════════════════════════════════════════════════════
 * RUST ERROR - Utilidades de manejo de errores
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: lib_error.rs
 * Descripción: Funciones de manejo de errores reutilizables para Rust
 * 
 * USO: Este módulo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a error con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

use std::fmt;
use std::error::Error as StdError;

// ═══════════════════════════════════════════════════════════════════════════
// ERROR PERSONALIZADO
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct AppError {
    pub message: String,
    pub code: u32,
}

impl AppError {
    pub fn new(message: &str, code: u32) -> Self {
        AppError {
            message: message.to_string(),
            code,
        }
    }
    
    pub fn from_string(message: String, code: u32) -> Self {
        AppError { message, code }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error [{}]: {}", self.code, self.message)
    }
}

impl StdError for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RESULT TYPE HELPER
// ═══════════════════════════════════════════════════════════════════════════

pub type AppResult<T> = Result<T, AppError>;

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES DE CONVERSIÓN
// ═══════════════════════════════════════════════════════════════════════════

/// Convertir error genérico a AppError
pub fn to_app_error<E: StdError>(error: E, code: u32) -> AppError {
    AppError::new(&error.to_string(), code)
}

/// Convertir Option a Result con mensaje
pub fn option_to_result<T>(opt: Option<T>, message: &str, code: u32) -> AppResult<T> {
    opt.ok_or_else(|| AppError::new(message, code))
}

// ═══════════════════════════════════════════════════════════════════════════
// VERIFICACIÓN Y VALIDACIÓN
// ═══════════════════════════════════════════════════════════════════════════

/// Verificar condición y retornar error si falla
pub fn require(condition: bool, message: &str, code: u32) -> AppResult<()> {
    if condition {
        Ok(())
    } else {
        Err(AppError::new(message, code))
    }
}

/// Verificar que valor esté en rango
pub fn require_range<T: PartialOrd>(
    value: T,
    min: T,
    max: T,
    var_name: &str,
    code: u32,
) -> AppResult<T> {
    if value >= min && value <= max {
        Ok(value)
    } else {
        Err(AppError::new(
            &format!("{} debe estar entre {:?} y {:?}", var_name, min, max),
            code,
        ))
    }
}

/// Verificar que string no esté vacío
pub fn require_non_empty(s: &str, var_name: &str, code: u32) -> AppResult<&str> {
    if s.is_empty() {
        Err(AppError::new(
            &format!("{} no puede estar vacío", var_name),
            code,
        ))
    } else {
        Ok(s)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// LOGGING DE ERRORES
// ═══════════════════════════════════════════════════════════════════════════

/// Log error de forma amigable
pub fn log_error(error: &AppError) {
    eprintln!("❌ Error: {}", error);
}

/// Log error con contexto adicional
pub fn log_error_with_context(error: &AppError, context: &str) {
    eprintln!("❌ Error en {}: {}", context, error);
}

/// Ejecutar función y capturar/loggear errores
pub fn catch_and_log<F, T>(f: F, context: &str) -> Option<T>
where
    F: FnOnce() -> AppResult<T>,
{
    match f() {
        Ok(result) => Some(result),
        Err(error) => {
            log_error_with_context(&error, context);
            None
        }
    }
}

