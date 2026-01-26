// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Language Support Module
// Soporte especializado para los 5 lenguajes principales
// ═══════════════════════════════════════════════════════════════════════════════

pub mod cpp;
pub mod rust;
pub mod asm;
pub mod java;
pub mod python;

// Re-exportar tipos principales
pub use cpp::{CppCompiler, CppVersion, get_available_cpp_compilers, get_cpp_status_summary};
