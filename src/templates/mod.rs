// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega v2.0
// Enfoque 100% en Rust
// ═══════════════════════════════════════════════════════════════════════════

use crate::core::node_graph::NodeLanguage;

// ══════════════════════════════════════════
// RUST
// ══════════════════════════════════════════
pub mod rust {
    // Básico
    pub const HELLO: &str = include_str!("rust/hello_world.rs");
    pub const VARIABLES: &str = include_str!("rust/variables.rs");
    pub const FUNCTIONS: &str = include_str!("rust/functions.rs");
    pub const CONTROL_FLOW: &str = include_str!("rust/control_flow.rs");
    pub const COLLECTIONS: &str = include_str!("rust/collections.rs");
    // Intermedio
    pub const STRUCTS: &str = include_str!("rust/structs.rs");
    pub const ENUMS: &str = include_str!("rust/enums.rs");
    pub const ERROR_HANDLING: &str = include_str!("rust/error_handling.rs");
    pub const MODULES: &str = include_str!("rust/modules.rs");
    pub const CLOSURES: &str = include_str!("rust/closures.rs");
    // Avanzado
    pub const OWNERSHIP: &str = include_str!("rust/ownership.rs");
    pub const TRAITS: &str = include_str!("rust/traits.rs");
    pub const GENERICS: &str = include_str!("rust/generics.rs");
    pub const LIFETIMES: &str = include_str!("rust/lifetimes.rs");
    pub const ASYNC: &str = include_str!("rust/async.rs");
    pub const MACROS: &str = include_str!("rust/macros.rs");
    pub const UNSAFE: &str = include_str!("rust/unsafe.rs");
    pub const CONCURRENCY: &str = include_str!("rust/concurrency.rs");
    // Librerías
    pub const LIB_UTILS: &str = include_str!("rust/lib_utils.rs");
    pub const LIB_MATH: &str = include_str!("rust/lib_math.rs");
    pub const LIB_IO: &str = include_str!("rust/lib_io.rs");
    pub const LIB_ERROR: &str = include_str!("rust/lib_error.rs");
}

// ══════════════════════════════════════════
// Estructura de Template para el menú
// ══════════════════════════════════════════
#[derive(Clone)]
pub struct Template {
    pub name: &'static str,
    pub code: &'static str,
    pub category: &'static str,
    pub subcategory: &'static str,
    pub color: (u8, u8, u8),
    pub icon: &'static str,
    pub language: NodeLanguage,
}

// Todos los templates disponibles
pub fn all_templates() -> Vec<Template> {
    vec![
        // ══════════════════════════════════════════════════════════════
        // RUST
        // ══════════════════════════════════════════════════════════════
        Template { name: "Hola Mundo", code: rust::HELLO, category: "Rust", subcategory: "Básico", color: (0xde, 0x39, 0x00), icon: "🦀", language: NodeLanguage::Rust },
        Template { name: "Variables y Mutabilidad", code: rust::VARIABLES, category: "Rust", subcategory: "Básico", color: (0xde, 0x39, 0x00), icon: "📦", language: NodeLanguage::Rust },
        Template { name: "Funciones", code: rust::FUNCTIONS, category: "Rust", subcategory: "Básico", color: (0xde, 0x39, 0x00), icon: "⚡", language: NodeLanguage::Rust },
        Template { name: "Control de Flujo", code: rust::CONTROL_FLOW, category: "Rust", subcategory: "Básico", color: (0xde, 0x39, 0x00), icon: "🔄", language: NodeLanguage::Rust },
        Template { name: "Colecciones", code: rust::COLLECTIONS, category: "Rust", subcategory: "Básico", color: (0xde, 0x39, 0x00), icon: "📚", language: NodeLanguage::Rust },
        Template { name: "Structs y Enums", code: rust::STRUCTS, category: "Rust", subcategory: "Intermedio", color: (0xde, 0x39, 0x00), icon: "🏗️", language: NodeLanguage::Rust },
        Template { name: "Enums Avanzados", code: rust::ENUMS, category: "Rust", subcategory: "Intermedio", color: (0xde, 0x39, 0x00), icon: "📋", language: NodeLanguage::Rust },
        Template { name: "Manejo de Errores", code: rust::ERROR_HANDLING, category: "Rust", subcategory: "Intermedio", color: (0xde, 0x39, 0x00), icon: "⚠️", language: NodeLanguage::Rust },
        Template { name: "Módulos", code: rust::MODULES, category: "Rust", subcategory: "Intermedio", color: (0xde, 0x39, 0x00), icon: "📁", language: NodeLanguage::Rust },
        Template { name: "Closures", code: rust::CLOSURES, category: "Rust", subcategory: "Intermedio", color: (0xde, 0x39, 0x00), icon: "🔗", language: NodeLanguage::Rust },
        Template { name: "Ownership y Borrowing", code: rust::OWNERSHIP, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "🔒", language: NodeLanguage::Rust },
        Template { name: "Traits", code: rust::TRAITS, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "🎭", language: NodeLanguage::Rust },
        Template { name: "Genéricos", code: rust::GENERICS, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "🔀", language: NodeLanguage::Rust },
        Template { name: "Lifetimes", code: rust::LIFETIMES, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "⏱️", language: NodeLanguage::Rust },
        Template { name: "Async/Await", code: rust::ASYNC, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "⚡", language: NodeLanguage::Rust },
        Template { name: "Macros", code: rust::MACROS, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "🔧", language: NodeLanguage::Rust },
        Template { name: "Unsafe Rust", code: rust::UNSAFE, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "⚠️", language: NodeLanguage::Rust },
        Template { name: "Concurrencia", code: rust::CONCURRENCY, category: "Rust", subcategory: "Avanzado", color: (0xde, 0x39, 0x00), icon: "🔀", language: NodeLanguage::Rust },
        Template { name: "📚 Utils (Utilidades)", code: rust::LIB_UTILS, category: "Rust", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "🛠️", language: NodeLanguage::Rust },
        Template { name: "📚 Math (Matemáticas)", code: rust::LIB_MATH, category: "Rust", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "🔢", language: NodeLanguage::Rust },
        Template { name: "📚 IO (Entrada/Salida)", code: rust::LIB_IO, category: "Rust", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "📂", language: NodeLanguage::Rust },
        Template { name: "📚 Error (Manejo de Errores)", code: rust::LIB_ERROR, category: "Rust", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "⚠️", language: NodeLanguage::Rust },
    ]
}

// Compatibilidad con código anterior
#[allow(unused_imports)]
pub use rust::HELLO as RUST_HELLO;
