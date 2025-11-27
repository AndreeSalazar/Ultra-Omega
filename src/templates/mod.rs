// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega
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
        // ══════════════════════════════════════════
        // ASM - Básico
        // ══════════════════════════════════════════
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
            name: "Suma",
            code: asm::SUM,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "➕",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Bucle",
            code: asm::LOOP,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "↻",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Condicional",
            code: asm::CONDITIONAL,
            category: "Assembler",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "🔀",
            language: NodeLanguage::Asm,
        },
        // ASM - Intermedio
        Template {
            name: "Variables",
            code: asm::VARIABLES,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📦",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Funciones",
            code: asm::FUNCTIONS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Strings",
            code: asm::STRINGS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Arrays",
            code: asm::ARRAYS,
            category: "Assembler",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📊",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════
        // C - Básico
        // ══════════════════════════════════════════
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
            name: "Variables",
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
        // C - Intermedio
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
            name: "Structs",
            code: c::STRUCTS,
            category: "C",
            subcategory: "Intermedio",
            color: (0x00, 0x59, 0x9C),
            icon: "🏗️",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════
        // C++ - Básico
        // ══════════════════════════════════════════
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
            name: "Variables",
            code: cpp::VARIABLES,
            category: "C++",
            subcategory: "Básico",
            color: (0x00, 0x44, 0x82),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        // C++ - Intermedio
        Template {
            name: "Clases (OOP)",
            code: cpp::CLASSES,
            category: "C++",
            subcategory: "Intermedio",
            color: (0x00, 0x44, 0x82),
            icon: "🏛️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "STL",
            code: cpp::STL,
            category: "C++",
            subcategory: "Avanzado",
            color: (0x00, 0x44, 0x82),
            icon: "📚",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "C++ Moderno",
            code: cpp::MODERN,
            category: "C++",
            subcategory: "Avanzado",
            color: (0x00, 0x44, 0x82),
            icon: "🚀",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════
        // Rust - Básico
        // ══════════════════════════════════════════
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
            name: "Variables",
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
        // Rust - Intermedio
        Template {
            name: "Structs y Enums",
            code: rust::STRUCTS,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "🏗️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Ownership",
            code: rust::OWNERSHIP,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🔒",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Traits",
            code: rust::TRAITS,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🎭",
            language: NodeLanguage::Rust,
        },
    ]
}

// Compatibilidad con código anterior (marcados con allow unused)
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
