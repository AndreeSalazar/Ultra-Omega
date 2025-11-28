// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega
// Organizados por jerarquía: Básico → Intermedio → Avanzado
// ═══════════════════════════════════════════════════════════════════════════

use crate::node_graph::NodeLanguage;

// ══════════════════════════════════════════
// ASSEMBLER (NASM x64)
// ══════════════════════════════════════════
pub mod asm {
    // Básicos
    pub const HELLO: &str = include_str!("asm/hello_world.asm");
    pub const SUM: &str = include_str!("asm/sum.asm");
    pub const LOOP: &str = include_str!("asm/loop.asm");
    pub const CONDITIONAL: &str = include_str!("asm/conditional.asm");
    
    // Intermedio
    pub const VARIABLES: &str = include_str!("asm/variables.asm");
    pub const FUNCTIONS: &str = include_str!("asm/functions.asm");
    pub const STRINGS: &str = include_str!("asm/strings.asm");
    pub const ARRAYS: &str = include_str!("asm/arrays.asm");
    
    // Librerías independientes (para herencia)
    pub const LIB_PRINT: &str = include_str!("asm/lib_print.asm");
    pub const LIB_MATH: &str = include_str!("asm/lib_math.asm");
    pub const LIB_STRING: &str = include_str!("asm/lib_string.asm");
    pub const LIB_MEMORY: &str = include_str!("asm/lib_memory.asm");
    pub const LIB_IO: &str = include_str!("asm/lib_io.asm");
}

// ══════════════════════════════════════════
// C
// ══════════════════════════════════════════
pub mod c {
    // Básicos
    pub const HELLO: &str = include_str!("c/hello_world.c");
    pub const VARIABLES: &str = include_str!("c/variables.c");
    pub const FUNCTIONS: &str = include_str!("c/functions.c");
    
    // Intermedio
    pub const ARRAYS: &str = include_str!("c/arrays.c");
    pub const STRUCTS: &str = include_str!("c/structs.c");
    
    // Librerías independientes (para herencia)
    pub const LIB_UTILS: &str = include_str!("c/lib_utils.c");
    pub const LIB_STRING: &str = include_str!("c/lib_string.c");
    pub const LIB_MEMORY: &str = include_str!("c/lib_memory.c");
    pub const LIB_IO: &str = include_str!("c/lib_io.c");
}

// ══════════════════════════════════════════
// C++
// ══════════════════════════════════════════
pub mod cpp {
    // Básicos
    pub const HELLO: &str = include_str!("cpp/hello_world.cpp");
    pub const VARIABLES: &str = include_str!("cpp/variables.cpp");
    
    // Intermedio/Avanzado
    pub const CLASSES: &str = include_str!("cpp/classes.cpp");
    pub const STL: &str = include_str!("cpp/stl.cpp");
    pub const MODERN: &str = include_str!("cpp/modern.cpp");
}

// ══════════════════════════════════════════
// RUST
// ══════════════════════════════════════════
pub mod rust {
    // Básicos
    pub const HELLO: &str = include_str!("rust/hello_world.rs");
    pub const VARIABLES: &str = include_str!("rust/variables.rs");
    pub const FUNCTIONS: &str = include_str!("rust/functions.rs");
    
    // Intermedio/Avanzado
    pub const STRUCTS: &str = include_str!("rust/structs.rs");
    pub const OWNERSHIP: &str = include_str!("rust/ownership.rs");
    pub const TRAITS: &str = include_str!("rust/traits.rs");
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
    pub color: (u8, u8, u8),  // RGB
    pub icon: &'static str,
    pub language: NodeLanguage,
}

// Todos los templates en un vector para búsqueda
pub fn all_templates() -> Vec<Template> {
    vec![
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: asm::HELLO,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "⏵",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Suma Básica",
            code: asm::SUM,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "➕",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Bucle Simple",
            code: asm::LOOP,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "↻",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Condicional If/Else",
            code: asm::CONDITIONAL,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "🔀",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Variables y Datos",
            code: asm::VARIABLES,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📦",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Funciones y Llamadas",
            code: asm::FUNCTIONS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Manejo de Strings",
            code: asm::STRINGS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Arrays y Memoria",
            code: asm::ARRAYS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📊",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - LIBRERÍAS (Componentes independientes)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Lib: Impresión",
            code: asm::LIB_PRINT,
            category: "Assembler",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🖨️",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Matemáticas",
            code: asm::LIB_MATH,
            category: "Assembler",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🔢",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Strings",
            code: asm::LIB_STRING,
            category: "Assembler",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Memoria",
            code: asm::LIB_MEMORY,
            category: "Assembler",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "💾",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Entrada/Salida",
            code: asm::LIB_IO,
            category: "Assembler",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⌨️",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: c::HELLO,
            category: "C",
            subcategory: "Básico",
            color: (0x00, 0x59, 0x9C),
            icon: "⏵",
            language: NodeLanguage::C,
        },
        Template {
            name: "Variables y Tipos",
            code: c::VARIABLES,
            category: "C",
            subcategory: "Básico",
            color: (0x00, 0x59, 0x9C),
            icon: "📦",
            language: NodeLanguage::C,
        },
        Template {
            name: "Funciones",
            code: c::FUNCTIONS,
            category: "C",
            subcategory: "Básico",
            color: (0x00, 0x59, 0x9C),
            icon: "⚡",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Arrays",
            code: c::ARRAYS,
            category: "C",
            subcategory: "Intermedio",
            color: (0x00, 0x59, 0x9C),
            icon: "📊",
            language: NodeLanguage::C,
        },
        Template {
            name: "Estructuras",
            code: c::STRUCTS,
            category: "C",
            subcategory: "Intermedio",
            color: (0x00, 0x59, 0x9C),
            icon: "🏗️",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C - LIBRERÍAS (Componentes independientes)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Lib: Utilidades",
            code: c::LIB_UTILS,
            category: "C",
            subcategory: "Librerías",
            color: (0x00, 0x40, 0x70),
            icon: "🔧",
            language: NodeLanguage::C,
        },
        Template {
            name: "📚 Lib: Strings",
            code: c::LIB_STRING,
            category: "C",
            subcategory: "Librerías",
            color: (0x00, 0x40, 0x70),
            icon: "📝",
            language: NodeLanguage::C,
        },
        Template {
            name: "📚 Lib: Memoria",
            code: c::LIB_MEMORY,
            category: "C",
            subcategory: "Librerías",
            color: (0x00, 0x40, 0x70),
            icon: "💾",
            language: NodeLanguage::C,
        },
        Template {
            name: "📚 Lib: Entrada/Salida",
            code: c::LIB_IO,
            category: "C",
            subcategory: "Librerías",
            color: (0x00, 0x40, 0x70),
            icon: "⌨️",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C++ - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: cpp::HELLO,
            category: "C++",
            subcategory: "Básico",
            color: (0x00, 0x44, 0x82),
            icon: "⏵",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "Variables Modernas",
            code: cpp::VARIABLES,
            category: "C++",
            subcategory: "Básico",
            color: (0x00, 0x44, 0x82),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C++ - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Clases y OOP",
            code: cpp::CLASSES,
            category: "C++",
            subcategory: "Intermedio",
            color: (0x00, 0x44, 0x82),
            icon: "🏛️",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // C++ - AVANZADO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "STL Containers",
            code: cpp::STL,
            category: "C++",
            subcategory: "Avanzado",
            color: (0x00, 0x44, 0x82),
            icon: "📚",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "C++ Moderno (C++17/20)",
            code: cpp::MODERN,
            category: "C++",
            subcategory: "Avanzado",
            color: (0x00, 0x44, 0x82),
            icon: "🚀",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // RUST - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: rust::HELLO,
            category: "Rust",
            subcategory: "Básico",
            color: (0xde, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Variables y Mutabilidad",
            code: rust::VARIABLES,
            category: "Rust",
            subcategory: "Básico",
            color: (0xde, 0x39, 0x00),
            icon: "📦",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Funciones",
            code: rust::FUNCTIONS,
            category: "Rust",
            subcategory: "Básico",
            color: (0xde, 0x39, 0x00),
            icon: "⚡",
            language: NodeLanguage::Rust,
        },
        
        // ══════════════════════════════════════════════════════════════
        // RUST - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Structs y Enums",
            code: rust::STRUCTS,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "🏗️",
            language: NodeLanguage::Rust,
        },
        
        // ══════════════════════════════════════════════════════════════
        // RUST - AVANZADO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Ownership y Borrowing",
            code: rust::OWNERSHIP,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🔒",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Traits y Generics",
            code: rust::TRAITS,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🎭",
            language: NodeLanguage::Rust,
        },
    ]
}

// Compatibilidad con código anterior
#[allow(unused_imports)]
pub use asm::HELLO as ASM_HELLO;
#[allow(unused_imports)]
pub use asm::SUM as ASM_SUM;
#[allow(unused_imports)]
pub use asm::LOOP as ASM_LOOP;
#[allow(unused_imports)]
pub use asm::CONDITIONAL as ASM_CONDITIONAL;
#[allow(unused_imports)]
pub use c::HELLO as C_HELLO;
#[allow(unused_imports)]
pub use cpp::HELLO as CPP_HELLO;
#[allow(unused_imports)]
pub use rust::HELLO as RUST_HELLO;
