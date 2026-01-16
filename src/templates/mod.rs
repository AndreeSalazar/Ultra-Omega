// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega
// Lenguajes soportados: Rust, ASM, Java, Python
// ═══════════════════════════════════════════════════════════════════════════

use crate::core::node_graph::NodeLanguage;

// ══════════════════════════════════════════
// ASSEMBLER (NASM x64) - WINDOWS
// ══════════════════════════════════════════
pub mod asm_windows {
    pub const HELLO: &str = include_str!("asm-windows/hello_world.asm");
    pub const SUM: &str = include_str!("asm-windows/sum.asm");
    pub const LOOP: &str = include_str!("asm-windows/loop.asm");
    pub const CONDITIONAL: &str = include_str!("asm-windows/conditional.asm");
    pub const VARIABLES: &str = include_str!("asm-windows/variables.asm");
    pub const FUNCTIONS: &str = include_str!("asm-windows/functions.asm");
    pub const STRINGS: &str = include_str!("asm-windows/strings.asm");
    pub const ARRAYS: &str = include_str!("asm-windows/arrays.asm");
    pub const LIB_PRINT: &str = include_str!("asm-windows/lib_print.asm");
    pub const LIB_MATH: &str = include_str!("asm-windows/lib_math.asm");
    pub const LIB_STRING: &str = include_str!("asm-windows/lib_string.asm");
    pub const LIB_MEMORY: &str = include_str!("asm-windows/lib_memory.asm");
    pub const LIB_IO: &str = include_str!("asm-windows/lib_io.asm");
}

// ══════════════════════════════════════════
// ASSEMBLER (NASM x64) - LINUX
// ══════════════════════════════════════════
pub mod asm_linux {
    pub const HELLO: &str = include_str!("asm-linux/hello_world.asm");
    pub const SUM: &str = include_str!("asm-linux/sum.asm");
    pub const LOOP: &str = include_str!("asm-linux/loop.asm");
    pub const CONDITIONAL: &str = include_str!("asm-linux/conditional.asm");
    pub const VARIABLES: &str = include_str!("asm-linux/variables.asm");
    pub const FUNCTIONS: &str = include_str!("asm-linux/functions.asm");
    pub const STRINGS: &str = include_str!("asm-linux/strings.asm");
    pub const ARRAYS: &str = include_str!("asm-linux/arrays.asm");
    pub const LIB_PRINT: &str = include_str!("asm-linux/lib_print.asm");
    pub const LIB_MATH: &str = include_str!("asm-linux/lib_math.asm");
    pub const LIB_STRING: &str = include_str!("asm-linux/lib_string.asm");
    pub const LIB_MEMORY: &str = include_str!("asm-linux/lib_memory.asm");
    pub const LIB_IO: &str = include_str!("asm-linux/lib_io.asm");
}

// Compatibilidad: asm apunta a asm_windows por defecto
#[allow(deprecated)]
pub mod asm {
    pub use super::asm_windows::*;
}

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
// JAVA 25
// ══════════════════════════════════════════
pub mod java {
    // Básico
    pub const HELLO: &str = include_str!("java/hello_world.java");
    pub const VARIABLES: &str = include_str!("java/variables.java");
    pub const CLASSES: &str = include_str!("java/classes.java");
    pub const METHODS: &str = include_str!("java/methods.java");
    // Intermedio
    pub const COLLECTIONS: &str = include_str!("java/collections.java");
    pub const GENERICS: &str = include_str!("java/generics.java");
    pub const EXCEPTIONS: &str = include_str!("java/exceptions.java");
    pub const INTERFACES: &str = include_str!("java/interfaces.java");
    pub const INHERITANCE: &str = include_str!("java/inheritance.java");
    // Avanzado
    pub const STREAMS: &str = include_str!("java/streams.java");
    pub const LAMBDAS: &str = include_str!("java/lambdas.java");
    pub const CONCURRENCY: &str = include_str!("java/concurrency.java");
    pub const REFLECTION: &str = include_str!("java/reflection.java");
    pub const ANNOTATIONS: &str = include_str!("java/annotations.java");
    pub const RECORDS: &str = include_str!("java/records.java");
    pub const SEALED_CLASSES: &str = include_str!("java/sealed_classes.java");
    pub const PATTERN_MATCHING: &str = include_str!("java/pattern_matching.java");
    pub const TEXT_BLOCKS: &str = include_str!("java/text_blocks.java");
    pub const VIRTUAL_THREADS: &str = include_str!("java/virtual_threads.java");
    pub const SWITCH_EXPRESSIONS: &str = include_str!("java/switch_expressions.java");
    pub const RECORD_PATTERNS: &str = include_str!("java/record_patterns.java");
    pub const FOREIGN_MEMORY: &str = include_str!("java/foreign_memory.java");
    pub const STRUCTURED_CONCURRENCY: &str = include_str!("java/structured_concurrency.java");
    // Librerías
    pub const LIB_UTILS: &str = include_str!("java/lib_utils.java");
    pub const LIB_COLLECTIONS: &str = include_str!("java/lib_collections.java");
    pub const LIB_IO: &str = include_str!("java/lib_io.java");
    pub const LIB_ASYNC: &str = include_str!("java/lib_async.java");
}

// ══════════════════════════════════════════
// PYTHON 3.12
// ══════════════════════════════════════════
pub mod python {
    pub const HELLO_WORLD: &str = include_str!("python/hello_world.py");
    pub const VARIABLES: &str = include_str!("python/variables.py");
    pub const CONDITIONALS: &str = include_str!("python/conditionals.py");
    pub const LOOPS: &str = include_str!("python/loops.py");
    pub const FUNCTIONS: &str = include_str!("python/functions.py");
    pub const LISTS_DICTS: &str = include_str!("python/lists_dicts.py");
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
        // ASM/NASM - WINDOWS
        // ══════════════════════════════════════════════════════════════
        Template { name: "Hola Mundo", code: asm_windows::HELLO, category: "Assembler (Windows)", subcategory: "Básico", color: (0xff, 0x47, 0x00), icon: "⏵", language: NodeLanguage::Asm },
        Template { name: "Suma Básica", code: asm_windows::SUM, category: "Assembler (Windows)", subcategory: "Básico", color: (0xff, 0x47, 0x00), icon: "➕", language: NodeLanguage::Asm },
        Template { name: "Bucle Simple", code: asm_windows::LOOP, category: "Assembler (Windows)", subcategory: "Básico", color: (0xff, 0x47, 0x00), icon: "↻", language: NodeLanguage::Asm },
        Template { name: "Condicional If/Else", code: asm_windows::CONDITIONAL, category: "Assembler (Windows)", subcategory: "Básico", color: (0xff, 0x47, 0x00), icon: "🔀", language: NodeLanguage::Asm },
        Template { name: "Variables y Datos", code: asm_windows::VARIABLES, category: "Assembler (Windows)", subcategory: "Intermedio", color: (0xff, 0x47, 0x00), icon: "📦", language: NodeLanguage::Asm },
        Template { name: "Funciones y Llamadas", code: asm_windows::FUNCTIONS, category: "Assembler (Windows)", subcategory: "Intermedio", color: (0xff, 0x47, 0x00), icon: "⚡", language: NodeLanguage::Asm },
        Template { name: "Manejo de Strings", code: asm_windows::STRINGS, category: "Assembler (Windows)", subcategory: "Intermedio", color: (0xff, 0x47, 0x00), icon: "📝", language: NodeLanguage::Asm },
        Template { name: "Arrays y Memoria", code: asm_windows::ARRAYS, category: "Assembler (Windows)", subcategory: "Intermedio", color: (0xff, 0x47, 0x00), icon: "📊", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Impresión", code: asm_windows::LIB_PRINT, category: "Assembler (Windows)", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "🖨️", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Matemáticas", code: asm_windows::LIB_MATH, category: "Assembler (Windows)", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "🔢", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Strings", code: asm_windows::LIB_STRING, category: "Assembler (Windows)", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "📝", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Memoria", code: asm_windows::LIB_MEMORY, category: "Assembler (Windows)", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "💾", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Entrada/Salida", code: asm_windows::LIB_IO, category: "Assembler (Windows)", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "⌨️", language: NodeLanguage::Asm },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - LINUX
        // ══════════════════════════════════════════════════════════════
        Template { name: "Hola Mundo", code: asm_linux::HELLO, category: "Assembler (Linux)", subcategory: "Básico", color: (0x00, 0xaa, 0xff), icon: "⏵", language: NodeLanguage::Asm },
        Template { name: "Suma Básica", code: asm_linux::SUM, category: "Assembler (Linux)", subcategory: "Básico", color: (0x00, 0xaa, 0xff), icon: "➕", language: NodeLanguage::Asm },
        Template { name: "Bucle Simple", code: asm_linux::LOOP, category: "Assembler (Linux)", subcategory: "Básico", color: (0x00, 0xaa, 0xff), icon: "↻", language: NodeLanguage::Asm },
        Template { name: "Condicional If/Else", code: asm_linux::CONDITIONAL, category: "Assembler (Linux)", subcategory: "Básico", color: (0x00, 0xaa, 0xff), icon: "🔀", language: NodeLanguage::Asm },
        Template { name: "Variables y Datos", code: asm_linux::VARIABLES, category: "Assembler (Linux)", subcategory: "Intermedio", color: (0x00, 0xaa, 0xff), icon: "📦", language: NodeLanguage::Asm },
        Template { name: "Funciones y Llamadas", code: asm_linux::FUNCTIONS, category: "Assembler (Linux)", subcategory: "Intermedio", color: (0x00, 0xaa, 0xff), icon: "⚡", language: NodeLanguage::Asm },
        Template { name: "Manejo de Strings", code: asm_linux::STRINGS, category: "Assembler (Linux)", subcategory: "Intermedio", color: (0x00, 0xaa, 0xff), icon: "📝", language: NodeLanguage::Asm },
        Template { name: "Arrays y Memoria", code: asm_linux::ARRAYS, category: "Assembler (Linux)", subcategory: "Intermedio", color: (0x00, 0xaa, 0xff), icon: "📊", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Impresión", code: asm_linux::LIB_PRINT, category: "Assembler (Linux)", subcategory: "Librerías", color: (0x00, 0x80, 0xcc), icon: "🖨️", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Matemáticas", code: asm_linux::LIB_MATH, category: "Assembler (Linux)", subcategory: "Librerías", color: (0x00, 0x80, 0xcc), icon: "🔢", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Strings", code: asm_linux::LIB_STRING, category: "Assembler (Linux)", subcategory: "Librerías", color: (0x00, 0x80, 0xcc), icon: "📝", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Memoria", code: asm_linux::LIB_MEMORY, category: "Assembler (Linux)", subcategory: "Librerías", color: (0x00, 0x80, 0xcc), icon: "💾", language: NodeLanguage::Asm },
        Template { name: "📚 Lib: Entrada/Salida", code: asm_linux::LIB_IO, category: "Assembler (Linux)", subcategory: "Librerías", color: (0x00, 0x80, 0xcc), icon: "⌨️", language: NodeLanguage::Asm },
        
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
        
        // ══════════════════════════════════════════════════════════════
        // JAVA 25
        // ══════════════════════════════════════════════════════════════
        Template { name: "Hola Mundo", code: java::HELLO, category: "Java", subcategory: "Básico", color: (0xed, 0x8b, 0x00), icon: "☕", language: NodeLanguage::Java },
        Template { name: "Variables y Tipos", code: java::VARIABLES, category: "Java", subcategory: "Básico", color: (0xed, 0x8b, 0x00), icon: "📦", language: NodeLanguage::Java },
        Template { name: "Clases y Objetos", code: java::CLASSES, category: "Java", subcategory: "Básico", color: (0xed, 0x8b, 0x00), icon: "🏛️", language: NodeLanguage::Java },
        Template { name: "Métodos y Funciones", code: java::METHODS, category: "Java", subcategory: "Básico", color: (0xed, 0x8b, 0x00), icon: "⚡", language: NodeLanguage::Java },
        Template { name: "Colecciones", code: java::COLLECTIONS, category: "Java", subcategory: "Intermedio", color: (0xed, 0x8b, 0x00), icon: "📚", language: NodeLanguage::Java },
        Template { name: "Genéricos", code: java::GENERICS, category: "Java", subcategory: "Intermedio", color: (0xed, 0x8b, 0x00), icon: "🔀", language: NodeLanguage::Java },
        Template { name: "Excepciones", code: java::EXCEPTIONS, category: "Java", subcategory: "Intermedio", color: (0xed, 0x8b, 0x00), icon: "⚠️", language: NodeLanguage::Java },
        Template { name: "Interfaces", code: java::INTERFACES, category: "Java", subcategory: "Intermedio", color: (0xed, 0x8b, 0x00), icon: "🔌", language: NodeLanguage::Java },
        Template { name: "Herencia", code: java::INHERITANCE, category: "Java", subcategory: "Intermedio", color: (0xed, 0x8b, 0x00), icon: "🔗", language: NodeLanguage::Java },
        Template { name: "Streams API", code: java::STREAMS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🌊", language: NodeLanguage::Java },
        Template { name: "Lambdas", code: java::LAMBDAS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "λ", language: NodeLanguage::Java },
        Template { name: "Concurrencia", code: java::CONCURRENCY, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "⚙️", language: NodeLanguage::Java },
        Template { name: "Reflection", code: java::REFLECTION, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🔮", language: NodeLanguage::Java },
        Template { name: "Anotaciones", code: java::ANNOTATIONS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🏷️", language: NodeLanguage::Java },
        Template { name: "Records", code: java::RECORDS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "📋", language: NodeLanguage::Java },
        Template { name: "Sealed Classes", code: java::SEALED_CLASSES, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🔒", language: NodeLanguage::Java },
        Template { name: "Pattern Matching", code: java::PATTERN_MATCHING, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🔍", language: NodeLanguage::Java },
        Template { name: "Text Blocks", code: java::TEXT_BLOCKS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "📄", language: NodeLanguage::Java },
        Template { name: "Virtual Threads", code: java::VIRTUAL_THREADS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🧵", language: NodeLanguage::Java },
        Template { name: "Switch Expressions", code: java::SWITCH_EXPRESSIONS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🔀", language: NodeLanguage::Java },
        Template { name: "Record Patterns", code: java::RECORD_PATTERNS, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🎯", language: NodeLanguage::Java },
        Template { name: "Foreign Memory API", code: java::FOREIGN_MEMORY, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "💾", language: NodeLanguage::Java },
        Template { name: "Structured Concurrency", code: java::STRUCTURED_CONCURRENCY, category: "Java", subcategory: "Avanzado", color: (0xed, 0x8b, 0x00), icon: "🔗", language: NodeLanguage::Java },
        Template { name: "📚 Utils (Utilidades)", code: java::LIB_UTILS, category: "Java", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "🛠️", language: NodeLanguage::Java },
        Template { name: "📚 Collections (Colecciones)", code: java::LIB_COLLECTIONS, category: "Java", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "📚", language: NodeLanguage::Java },
        Template { name: "📚 I/O (Entrada/Salida)", code: java::LIB_IO, category: "Java", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "📁", language: NodeLanguage::Java },
        Template { name: "📚 Async (Asíncrono)", code: java::LIB_ASYNC, category: "Java", subcategory: "Librerías", color: (0x80, 0x40, 0x00), icon: "⚡", language: NodeLanguage::Java },
        
        // ══════════════════════════════════════════════════════════════
        // PYTHON 3.12
        // ══════════════════════════════════════════════════════════════
        Template { name: "Hola Mundo", code: python::HELLO_WORLD, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "🐍", language: NodeLanguage::Python },
        Template { name: "Variables y Tipos", code: python::VARIABLES, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "📦", language: NodeLanguage::Python },
        Template { name: "Condicionales", code: python::CONDITIONALS, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "🔀", language: NodeLanguage::Python },
        Template { name: "Bucles", code: python::LOOPS, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "↻", language: NodeLanguage::Python },
        Template { name: "Funciones", code: python::FUNCTIONS, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "⚡", language: NodeLanguage::Python },
        Template { name: "Listas y Diccionarios", code: python::LISTS_DICTS, category: "Python", subcategory: "Básico", color: (0x37, 0x76, 0xAB), icon: "📚", language: NodeLanguage::Python },
    ]
}

// Compatibilidad con código anterior
#[allow(unused_imports)]
pub use asm_windows::HELLO as ASM_HELLO;
#[allow(unused_imports)]
pub use asm_windows::SUM as ASM_SUM;
#[allow(unused_imports)]
pub use asm_windows::LOOP as ASM_LOOP;
#[allow(unused_imports)]
pub use asm_windows::CONDITIONAL as ASM_CONDITIONAL;
#[allow(unused_imports)]
pub use rust::HELLO as RUST_HELLO;
