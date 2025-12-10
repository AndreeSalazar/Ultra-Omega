// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega
// Organizados por jerarquía: Básico → Intermedio → Avanzado
// Incluye FastOS: Templates completos para crear un sistema operativo
// ═══════════════════════════════════════════════════════════════════════════

use crate::core::node_graph::NodeLanguage;

// ══════════════════════════════════════════
// ASSEMBLER (NASM x64) - WINDOWS
// ══════════════════════════════════════════
pub mod asm_windows {
    // Básicos
    pub const HELLO: &str = include_str!("asm-windows/hello_world.asm");
    pub const SUM: &str = include_str!("asm-windows/sum.asm");
    pub const LOOP: &str = include_str!("asm-windows/loop.asm");
    pub const CONDITIONAL: &str = include_str!("asm-windows/conditional.asm");
    
    // Intermedio
    pub const VARIABLES: &str = include_str!("asm-windows/variables.asm");
    pub const FUNCTIONS: &str = include_str!("asm-windows/functions.asm");
    pub const STRINGS: &str = include_str!("asm-windows/strings.asm");
    pub const ARRAYS: &str = include_str!("asm-windows/arrays.asm");
    
    // Librerías independientes (para herencia)
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
    // Básicos
    pub const HELLO: &str = include_str!("asm-linux/hello_world.asm");
    pub const SUM: &str = include_str!("asm-linux/sum.asm");
    pub const LOOP: &str = include_str!("asm-linux/loop.asm");
    pub const CONDITIONAL: &str = include_str!("asm-linux/conditional.asm");
    
    // Intermedio
    pub const VARIABLES: &str = include_str!("asm-linux/variables.asm");
    pub const FUNCTIONS: &str = include_str!("asm-linux/functions.asm");
    pub const STRINGS: &str = include_str!("asm-linux/strings.asm");
    pub const ARRAYS: &str = include_str!("asm-linux/arrays.asm");
    
    // Librerías independientes (para herencia)
    pub const LIB_PRINT: &str = include_str!("asm-linux/lib_print.asm");
    pub const LIB_MATH: &str = include_str!("asm-linux/lib_math.asm");
    pub const LIB_STRING: &str = include_str!("asm-linux/lib_string.asm");
    pub const LIB_MEMORY: &str = include_str!("asm-linux/lib_memory.asm");
    pub const LIB_IO: &str = include_str!("asm-linux/lib_io.asm");
}

// Compatibilidad: mantener asm apuntando a asm_windows por defecto
#[allow(deprecated)]
pub mod asm {
    pub use super::asm_windows::*;
}

// ══════════════════════════════════════════
// FASTOS - Sistema Operativo Completo
// ══════════════════════════════════════════
pub mod fastos {
    // Bootloader
    pub const BOOT_SECTOR: &str = include_str!("fastos/boot_sector.asm");
    pub const STAGE2: &str = include_str!("fastos/bootloader_stage2.asm");
    
    // Kernel ASM
    pub const KERNEL_ENTRY: &str = include_str!("fastos/kernel_entry.asm");
    
    // Kernel C
    pub const KERNEL_MAIN: &str = include_str!("fastos/kernel_main.c");
    pub const KERNEL_H: &str = include_str!("fastos/kernel.h");
    
    // Drivers
    pub const VGA_DRIVER: &str = include_str!("fastos/vga_driver.c");
    pub const VGA_H: &str = include_str!("fastos/vga.h");
    pub const KEYBOARD_DRIVER: &str = include_str!("fastos/keyboard_driver.c");
    pub const KEYBOARD_H: &str = include_str!("fastos/keyboard.h");
    pub const TIMER: &str = include_str!("fastos/timer.c");
    pub const TIMER_H: &str = include_str!("fastos/timer.h");
    
    // Sistema
    pub const IDT: &str = include_str!("fastos/idt.c");
    pub const IDT_H: &str = include_str!("fastos/idt.h");
    pub const MEMORY: &str = include_str!("fastos/memory.c");
    pub const MEMORY_H: &str = include_str!("fastos/memory.h");
    pub const SHELL: &str = include_str!("fastos/shell.c");
    pub const SHELL_H: &str = include_str!("fastos/shell.h");
    
    // Utilidades
    pub const STRING: &str = include_str!("fastos/string.c");
    pub const STRING_H: &str = include_str!("fastos/string.h");
    pub const TYPES_H: &str = include_str!("fastos/types.h");
    pub const PORTS_H: &str = include_str!("fastos/ports.h");
    
    // Build
    pub const LINKER: &str = include_str!("fastos/linker.ld");
    pub const MAKEFILE: &str = include_str!("fastos/Makefile");
    pub const BUILD_BAT: &str = include_str!("fastos/build.bat");
    pub const README: &str = include_str!("fastos/README.md");
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
// ══════════════════════════════════════════
// PYTHON 3.12 - Lenguaje de programación interpretado de alto nivel
// ══════════════════════════════════════════
pub mod python {
    // Básicos
    pub const HELLO_WORLD: &str = include_str!("python/hello_world.py");
    pub const VARIABLES: &str = include_str!("python/variables.py");
    pub const CONDITIONALS: &str = include_str!("python/conditionals.py");
    pub const LOOPS: &str = include_str!("python/loops.py");
    pub const FUNCTIONS: &str = include_str!("python/functions.py");
    pub const LISTS_DICTS: &str = include_str!("python/lists_dicts.py");
}

// JAVA 25 - Lenguaje orientado a objetos multiplataforma
// ══════════════════════════════════════════
pub mod java {
    // Básicos
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
    
    // Java 25 - Características más recientes
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

pub mod rust {
    // ═══════════════════════════════════════
    // BÁSICO
    // ═══════════════════════════════════════
    pub const HELLO: &str = include_str!("rust/hello_world.rs");
    pub const VARIABLES: &str = include_str!("rust/variables.rs");
    pub const FUNCTIONS: &str = include_str!("rust/functions.rs");
    pub const CONTROL_FLOW: &str = include_str!("rust/control_flow.rs");
    pub const COLLECTIONS: &str = include_str!("rust/collections.rs");
    
    // ═══════════════════════════════════════
    // INTERMEDIO
    // ═══════════════════════════════════════
    pub const STRUCTS: &str = include_str!("rust/structs.rs");
    pub const ENUMS: &str = include_str!("rust/enums.rs");
    pub const ERROR_HANDLING: &str = include_str!("rust/error_handling.rs");
    pub const MODULES: &str = include_str!("rust/modules.rs");
    pub const CLOSURES: &str = include_str!("rust/closures.rs");
    
    // ═══════════════════════════════════════
    // AVANZADO
    // ═══════════════════════════════════════
    pub const OWNERSHIP: &str = include_str!("rust/ownership.rs");
    pub const TRAITS: &str = include_str!("rust/traits.rs");
    pub const GENERICS: &str = include_str!("rust/generics.rs");
    pub const LIFETIMES: &str = include_str!("rust/lifetimes.rs");
    pub const ASYNC: &str = include_str!("rust/async.rs");
    pub const MACROS: &str = include_str!("rust/macros.rs");
    pub const UNSAFE: &str = include_str!("rust/unsafe.rs");
    pub const CONCURRENCY: &str = include_str!("rust/concurrency.rs");
    
    // ═══════════════════════════════════════
    // LIBRERÍAS (Funciones independientes reutilizables)
    // ═══════════════════════════════════════
    pub const LIB_UTILS: &str = include_str!("rust/lib_utils.rs");
    pub const LIB_MATH: &str = include_str!("rust/lib_math.rs");
    pub const LIB_IO: &str = include_str!("rust/lib_io.rs");
    pub const LIB_ERROR: &str = include_str!("rust/lib_error.rs");
}

// ══════════════════════════════════════════
// ZIG
// Lenguaje de sistemas moderno y seguro
// ══════════════════════════════════════════
pub mod zig {
    // ═══════════════════════════════════════
    // BÁSICO
    // ═══════════════════════════════════════
    pub const HELLO: &str = include_str!("zig/hello_world.zig");
    pub const VARIABLES: &str = include_str!("zig/variables.zig");
    pub const FUNCTIONS: &str = include_str!("zig/functions.zig");
}

// ══════════════════════════════════════════
// FASTOS ASM+RUST+ZIG (Sistema Operativo Multi-Lenguaje)
// ══════════════════════════════════════════
pub mod fastos_asm_rust_zig {
    // ═══════════════════════════════════════
    // ASM (NASM x86_64) - Bootloader y bajo nivel
    // ═══════════════════════════════════════
    pub mod asm {
        pub const BOOT_UEFI: &str = include_str!("fastos_asm_rust_zig/asm/boot_uefi.asm");
        pub const KERNEL_ENTRY: &str = include_str!("fastos_asm_rust_zig/asm/kernel_entry.asm");
        pub const INTERRUPTS: &str = include_str!("fastos_asm_rust_zig/asm/interrupts.asm");
        pub const MEMORY_LOW: &str = include_str!("fastos_asm_rust_zig/asm/memory_low.asm");
    }
    
    // ═══════════════════════════════════════
    // RUST (Kernel) - Seguridad y drivers
    // ═══════════════════════════════════════
    pub mod rust {
        pub const KERNEL_MAIN: &str = include_str!("fastos_asm_rust_zig/rust/kernel_main.rs");
        pub const PORTS: &str = include_str!("fastos_asm_rust_zig/rust/ports.rs");
        pub const INTERRUPTS: &str = include_str!("fastos_asm_rust_zig/rust/interrupts.rs");
        pub const MEMORY: &str = include_str!("fastos_asm_rust_zig/rust/memory.rs");
        pub const DRIVERS: &str = include_str!("fastos_asm_rust_zig/rust/drivers.rs");
        pub const FFI: &str = include_str!("fastos_asm_rust_zig/rust/ffi.rs");
    }
    
    // ═══════════════════════════════════════
    // ZIG (Sistema) - Performance y simplicidad
    // ═══════════════════════════════════════
    pub mod zig {
        pub const SYSTEM: &str = include_str!("fastos_asm_rust_zig/zig/system.zig");
        pub const ALLOCATOR: &str = include_str!("fastos_asm_rust_zig/zig/allocator.zig");
        pub const FS: &str = include_str!("fastos_asm_rust_zig/zig/filesystem.zig");
        pub const SCHEDULER: &str = include_str!("fastos_asm_rust_zig/zig/scheduler.zig");
    }
    
    // ═══════════════════════════════════════
    // INTEGRATION (Build & Config)
    // ═══════════════════════════════════════
    pub mod integration {
        pub const CARGO_TOML: &str = include_str!("fastos_asm_rust_zig/integration/Cargo.toml");
        pub const BUILD_SH: &str = include_str!("fastos_asm_rust_zig/integration/build.sh");
        pub const BUILD_BAT: &str = include_str!("fastos_asm_rust_zig/integration/build.bat");
        pub const LINKER_LD: &str = include_str!("fastos_asm_rust_zig/integration/linker.ld");
        pub const README: &str = include_str!("fastos_asm_rust_zig/integration/README.md");
    }
    
    // README principal
    pub const README: &str = include_str!("fastos_asm_rust_zig/README.md");
    pub const POTENCIAL: &str = include_str!("fastos_asm_rust_zig/POTENCIAL.md");
}

// ══════════════════════════════════════════
// VULKAN API (C++)
// ══════════════════════════════════════════
pub mod vulkan {
    // Tipos base
    pub const TYPES_H: &str = include_str!("vulkan/vulkan_types.h");
    
    // Inicialización
    pub const INSTANCE: &str = include_str!("vulkan/instance.cpp");
    pub const DEVICE: &str = include_str!("vulkan/device.cpp");
    pub const SWAPCHAIN: &str = include_str!("vulkan/swapchain.cpp");
    
    // Pipeline
    pub const PIPELINE: &str = include_str!("vulkan/pipeline.cpp");
    pub const SHADER_VERT: &str = include_str!("vulkan/shader.vert");
    pub const SHADER_FRAG: &str = include_str!("vulkan/shader.frag");
    
    // Recursos
    pub const BUFFERS: &str = include_str!("vulkan/buffers.cpp");
    pub const TEXTURE: &str = include_str!("vulkan/texture.cpp");
    
    // Comandos y Sync
    pub const COMMANDS: &str = include_str!("vulkan/commands.cpp");
    pub const SYNC: &str = include_str!("vulkan/sync.cpp");
    
    // Renderizado
    pub const RENDER_LOOP: &str = include_str!("vulkan/render_loop.cpp");
    pub const MAIN: &str = include_str!("vulkan/main.cpp");
    
    // Librerías (helpers y utilidades reutilizables)
    pub const HELPERS: &str = include_str!("vulkan/helpers.cpp");
    pub const RESOURCE_MANAGER: &str = include_str!("vulkan/resource_manager.cpp");
    pub const WINDOW_MANAGER: &str = include_str!("vulkan/window_manager.cpp");
    pub const SYNC_MANAGER: &str = include_str!("vulkan/sync_manager.cpp");
    
    // Build
    pub const CMAKE: &str = include_str!("vulkan/CMakeLists.txt");
    pub const README: &str = include_str!("vulkan/README.md");
}

// ══════════════════════════════════════════
// DIRECTX12 API (C++)
// ══════════════════════════════════════════
pub mod directx12 {
    // Tipos base
    pub const TYPES_H: &str = include_str!("directx12/directx12_types.h");
    
    // Inicialización
    pub const ADAPTER: &str = include_str!("directx12/adapter.cpp");
    pub const DEVICE: &str = include_str!("directx12/device.cpp");
    pub const SWAPCHAIN: &str = include_str!("directx12/swapchain.cpp");
    
    // Comandos
    pub const COMMAND_ALLOCATOR: &str = include_str!("directx12/command_allocator.cpp");
    pub const COMMAND_LIST: &str = include_str!("directx12/command_list.cpp");
    
    // Pipeline
    pub const ROOT_SIGNATURE: &str = include_str!("directx12/root_signature.cpp");
    pub const PIPELINE_STATE: &str = include_str!("directx12/pipeline_state.cpp");
    pub const SHADER_HLSL: &str = include_str!("directx12/shader.hlsl");
    
    // Recursos
    pub const BUFFERS: &str = include_str!("directx12/buffers.cpp");
    
    // Renderizado
    pub const RENDER_LOOP: &str = include_str!("directx12/render_loop.cpp");
    pub const MAIN: &str = include_str!("directx12/main.cpp");
    
    // Librerías (helpers y utilidades reutilizables)
    pub const HELPERS: &str = include_str!("directx12/helpers.cpp");
    pub const RESOURCE_MANAGER: &str = include_str!("directx12/resource_manager.cpp");
    pub const WINDOW_MANAGER: &str = include_str!("directx12/window_manager.cpp");
    pub const SYNC_MANAGER: &str = include_str!("directx12/sync_manager.cpp");
    
    // Build
    pub const CMAKE: &str = include_str!("directx12/CMakeLists.txt");
    pub const README: &str = include_str!("directx12/README.md");
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
        // ASM/NASM - WINDOWS - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: asm_windows::HELLO,
            category: "Assembler (Windows)",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "⏵",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Suma Básica",
            code: asm_windows::SUM,
            category: "Assembler (Windows)",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "➕",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Bucle Simple",
            code: asm_windows::LOOP,
            category: "Assembler (Windows)",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "↻",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Condicional If/Else",
            code: asm_windows::CONDITIONAL,
            category: "Assembler (Windows)",
            subcategory: "Básico",
            color: (0xff, 0x47, 0x00),
            icon: "🔀",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - WINDOWS - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Variables y Datos",
            code: asm_windows::VARIABLES,
            category: "Assembler (Windows)",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📦",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Funciones y Llamadas",
            code: asm_windows::FUNCTIONS,
            category: "Assembler (Windows)",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Manejo de Strings",
            code: asm_windows::STRINGS,
            category: "Assembler (Windows)",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Arrays y Memoria",
            code: asm_windows::ARRAYS,
            category: "Assembler (Windows)",
            subcategory: "Intermedio",
            color: (0xff, 0x47, 0x00),
            icon: "📊",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - WINDOWS - LIBRERÍAS (Componentes independientes)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Lib: Impresión",
            code: asm_windows::LIB_PRINT,
            category: "Assembler (Windows)",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🖨️",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Matemáticas",
            code: asm_windows::LIB_MATH,
            category: "Assembler (Windows)",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🔢",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Strings",
            code: asm_windows::LIB_STRING,
            category: "Assembler (Windows)",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Memoria",
            code: asm_windows::LIB_MEMORY,
            category: "Assembler (Windows)",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "💾",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Entrada/Salida",
            code: asm_windows::LIB_IO,
            category: "Assembler (Windows)",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⌨️",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - LINUX - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: asm_linux::HELLO,
            category: "Assembler (Linux)",
            subcategory: "Básico",
            color: (0x00, 0xaa, 0xff),
            icon: "⏵",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Suma Básica",
            code: asm_linux::SUM,
            category: "Assembler (Linux)",
            subcategory: "Básico",
            color: (0x00, 0xaa, 0xff),
            icon: "➕",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Bucle Simple",
            code: asm_linux::LOOP,
            category: "Assembler (Linux)",
            subcategory: "Básico",
            color: (0x00, 0xaa, 0xff),
            icon: "↻",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Condicional If/Else",
            code: asm_linux::CONDITIONAL,
            category: "Assembler (Linux)",
            subcategory: "Básico",
            color: (0x00, 0xaa, 0xff),
            icon: "🔀",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - LINUX - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Variables y Datos",
            code: asm_linux::VARIABLES,
            category: "Assembler (Linux)",
            subcategory: "Intermedio",
            color: (0x00, 0xaa, 0xff),
            icon: "📦",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Funciones y Llamadas",
            code: asm_linux::FUNCTIONS,
            category: "Assembler (Linux)",
            subcategory: "Intermedio",
            color: (0x00, 0xaa, 0xff),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Manejo de Strings",
            code: asm_linux::STRINGS,
            category: "Assembler (Linux)",
            subcategory: "Intermedio",
            color: (0x00, 0xaa, 0xff),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Arrays y Memoria",
            code: asm_linux::ARRAYS,
            category: "Assembler (Linux)",
            subcategory: "Intermedio",
            color: (0x00, 0xaa, 0xff),
            icon: "📊",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ASM/NASM - LINUX - LIBRERÍAS (Componentes independientes)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Lib: Impresión",
            code: asm_linux::LIB_PRINT,
            category: "Assembler (Linux)",
            subcategory: "Librerías",
            color: (0x00, 0x80, 0xcc),
            icon: "🖨️",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Matemáticas",
            code: asm_linux::LIB_MATH,
            category: "Assembler (Linux)",
            subcategory: "Librerías",
            color: (0x00, 0x80, 0xcc),
            icon: "🔢",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Strings",
            code: asm_linux::LIB_STRING,
            category: "Assembler (Linux)",
            subcategory: "Librerías",
            color: (0x00, 0x80, 0xcc),
            icon: "📝",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Memoria",
            code: asm_linux::LIB_MEMORY,
            category: "Assembler (Linux)",
            subcategory: "Librerías",
            color: (0x00, 0x80, 0xcc),
            icon: "💾",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "📚 Lib: Entrada/Salida",
            code: asm_linux::LIB_IO,
            category: "Assembler (Linux)",
            subcategory: "Librerías",
            color: (0x00, 0x80, 0xcc),
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
        Template {
            name: "Control de Flujo",
            code: rust::CONTROL_FLOW,
            category: "Rust",
            subcategory: "Básico",
            color: (0xde, 0x39, 0x00),
            icon: "🔄",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Colecciones",
            code: rust::COLLECTIONS,
            category: "Rust",
            subcategory: "Básico",
            color: (0xde, 0x39, 0x00),
            icon: "📚",
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
        Template {
            name: "Enums Avanzados",
            code: rust::ENUMS,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "📋",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Manejo de Errores",
            code: rust::ERROR_HANDLING,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "⚠️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Módulos",
            code: rust::MODULES,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "📁",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Closures",
            code: rust::CLOSURES,
            category: "Rust",
            subcategory: "Intermedio",
            color: (0xde, 0x39, 0x00),
            icon: "🔗",
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
            name: "Traits",
            code: rust::TRAITS,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🎭",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Genéricos",
            code: rust::GENERICS,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🔀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Lifetimes",
            code: rust::LIFETIMES,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "⏱️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Async/Await",
            code: rust::ASYNC,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "⚡",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Macros",
            code: rust::MACROS,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🔧",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Unsafe Rust",
            code: rust::UNSAFE,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "⚠️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Concurrencia",
            code: rust::CONCURRENCY,
            category: "Rust",
            subcategory: "Avanzado",
            color: (0xde, 0x39, 0x00),
            icon: "🔀",
            language: NodeLanguage::Rust,
        },
        
        // ══════════════════════════════════════════════════════════════
        // RUST - LIBRERÍAS (Funciones independientes reutilizables)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Utils (Utilidades)",
            code: rust::LIB_UTILS,
            category: "Rust",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🛠️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "📚 Math (Matemáticas)",
            code: rust::LIB_MATH,
            category: "Rust",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🔢",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "📚 IO (Entrada/Salida)",
            code: rust::LIB_IO,
            category: "Rust",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📂",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "📚 Error (Manejo de Errores)",
            code: rust::LIB_ERROR,
            category: "Rust",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⚠️",
            language: NodeLanguage::Rust,
        },
        
        // ══════════════════════════════════════════════════════════════
        // ZIG - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: zig::HELLO,
            category: "Zig",
            subcategory: "Básico",
            color: (0xf0, 0xaa, 0x00),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        Template {
            name: "Variables y Tipos",
            code: zig::VARIABLES,
            category: "Zig",
            subcategory: "Básico",
            color: (0xf0, 0xaa, 0x00),
            icon: "📦",
            language: NodeLanguage::Zig,
        },
        Template {
            name: "Funciones",
            code: zig::FUNCTIONS,
            category: "Zig",
            subcategory: "Básico",
            color: (0xf0, 0xaa, 0x00),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        
        // ══════════════════════════════════════════════════════════════
        // JAVA 25 - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: java::HELLO,
            category: "Java",
            subcategory: "Básico",
            color: (0xed, 0x8b, 0x00),
            icon: "☕",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Variables y Tipos",
            code: java::VARIABLES,
            category: "Java",
            subcategory: "Básico",
            color: (0xed, 0x8b, 0x00),
            icon: "📦",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Clases y Objetos",
            code: java::CLASSES,
            category: "Java",
            subcategory: "Básico",
            color: (0xed, 0x8b, 0x00),
            icon: "🏛️",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Métodos y Funciones",
            code: java::METHODS,
            category: "Java",
            subcategory: "Básico",
            color: (0xed, 0x8b, 0x00),
            icon: "⚡",
            language: NodeLanguage::Java,
        },
        
        // ══════════════════════════════════════════════════════════════
        // JAVA 25 - INTERMEDIO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Colecciones",
            code: java::COLLECTIONS,
            category: "Java",
            subcategory: "Intermedio",
            color: (0xed, 0x8b, 0x00),
            icon: "📚",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Genéricos",
            code: java::GENERICS,
            category: "Java",
            subcategory: "Intermedio",
            color: (0xed, 0x8b, 0x00),
            icon: "🔀",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Excepciones",
            code: java::EXCEPTIONS,
            category: "Java",
            subcategory: "Intermedio",
            color: (0xed, 0x8b, 0x00),
            icon: "⚠️",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Interfaces",
            code: java::INTERFACES,
            category: "Java",
            subcategory: "Intermedio",
            color: (0xed, 0x8b, 0x00),
            icon: "🔌",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Herencia",
            code: java::INHERITANCE,
            category: "Java",
            subcategory: "Intermedio",
            color: (0xed, 0x8b, 0x00),
            icon: "🔗",
            language: NodeLanguage::Java,
        },
        
        // ══════════════════════════════════════════════════════════════
        // JAVA 25 - AVANZADO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Streams API",
            code: java::STREAMS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🌊",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Lambdas",
            code: java::LAMBDAS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "λ",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Concurrencia",
            code: java::CONCURRENCY,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "⚙️",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Reflection",
            code: java::REFLECTION,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🔮",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Anotaciones",
            code: java::ANNOTATIONS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🏷️",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Records",
            code: java::RECORDS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "📋",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Sealed Classes",
            code: java::SEALED_CLASSES,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🔒",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Pattern Matching",
            code: java::PATTERN_MATCHING,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🔍",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Text Blocks",
            code: java::TEXT_BLOCKS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "📄",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Virtual Threads",
            code: java::VIRTUAL_THREADS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🧵",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Switch Expressions",
            code: java::SWITCH_EXPRESSIONS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🔀",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Record Patterns",
            code: java::RECORD_PATTERNS,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🎯",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Foreign Memory API",
            code: java::FOREIGN_MEMORY,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "💾",
            language: NodeLanguage::Java,
        },
        Template {
            name: "Structured Concurrency",
            code: java::STRUCTURED_CONCURRENCY,
            category: "Java",
            subcategory: "Avanzado",
            color: (0xed, 0x8b, 0x00),
            icon: "🔗",
            language: NodeLanguage::Java,
        },
        
        // ══════════════════════════════════════════════════════════════
        // JAVA 25 - LIBRERÍAS (Funciones independientes reutilizables)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Utils (Utilidades)",
            code: java::LIB_UTILS,
            category: "Java",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🛠️",
            language: NodeLanguage::Java,
        },
        Template {
            name: "📚 Collections (Colecciones)",
            code: java::LIB_COLLECTIONS,
            category: "Java",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📚",
            language: NodeLanguage::Java,
        },
        Template {
            name: "📚 I/O (Entrada/Salida)",
            code: java::LIB_IO,
            category: "Java",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📁",
            language: NodeLanguage::Java,
        },
        Template {
            name: "📚 Async (Asíncrono)",
            code: java::LIB_ASYNC,
            category: "Java",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⚡",
            language: NodeLanguage::Java,
        },
        
        // ══════════════════════════════════════════════════════════════
        // PYTHON 3.12 - BÁSICO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "Hola Mundo",
            code: python::HELLO_WORLD,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB), // Python blue oficial
            icon: "🐍",
            language: NodeLanguage::Python,
        },
        Template {
            name: "Variables y Tipos",
            code: python::VARIABLES,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB),
            icon: "📦",
            language: NodeLanguage::Python,
        },
        Template {
            name: "Condicionales",
            code: python::CONDITIONALS,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB),
            icon: "🔀",
            language: NodeLanguage::Python,
        },
        Template {
            name: "Bucles",
            code: python::LOOPS,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB),
            icon: "↻",
            language: NodeLanguage::Python,
        },
        Template {
            name: "Funciones",
            code: python::FUNCTIONS,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB),
            icon: "⚡",
            language: NodeLanguage::Python,
        },
        Template {
            name: "Listas y Diccionarios",
            code: python::LISTS_DICTS,
            category: "Python",
            subcategory: "Básico",
            color: (0x37, 0x76, 0xAB),
            icon: "📚",
            language: NodeLanguage::Python,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - TIPOS BASE
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Vulkan Types",
            code: vulkan::TYPES_H,
            category: "Vulkan",
            subcategory: "Base",
            color: (0xac, 0x14, 0x2c),
            icon: "📋",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - INICIALIZACIÓN
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Instance",
            code: vulkan::INSTANCE,
            category: "Vulkan",
            subcategory: "Inicialización",
            color: (0xac, 0x14, 0x2c),
            icon: "🔌",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Device",
            code: vulkan::DEVICE,
            category: "Vulkan",
            subcategory: "Inicialización",
            color: (0xac, 0x14, 0x2c),
            icon: "🖥️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Swapchain",
            code: vulkan::SWAPCHAIN,
            category: "Vulkan",
            subcategory: "Inicialización",
            color: (0xac, 0x14, 0x2c),
            icon: "🔄",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - PIPELINE
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Graphics Pipeline",
            code: vulkan::PIPELINE,
            category: "Vulkan",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "🔧",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Vertex Shader",
            code: vulkan::SHADER_VERT,
            category: "Vulkan",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "📐",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Fragment Shader",
            code: vulkan::SHADER_FRAG,
            category: "Vulkan",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "🎨",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - RECURSOS
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Buffers",
            code: vulkan::BUFFERS,
            category: "Vulkan",
            subcategory: "Recursos",
            color: (0x00, 0x80, 0x80),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Texture",
            code: vulkan::TEXTURE,
            category: "Vulkan",
            subcategory: "Recursos",
            color: (0x00, 0x80, 0x80),
            icon: "🖼️",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - COMANDOS Y SINCRONIZACIÓN
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Commands",
            code: vulkan::COMMANDS,
            category: "Vulkan",
            subcategory: "Ejecución",
            color: (0xff, 0x8c, 0x00),
            icon: "📋",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Sync Objects",
            code: vulkan::SYNC,
            category: "Vulkan",
            subcategory: "Ejecución",
            color: (0xff, 0x8c, 0x00),
            icon: "⏱️",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - LIBRERÍAS (Funciones independientes reutilizables)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Helpers (Utilidades)",
            code: vulkan::HELPERS,
            category: "Vulkan",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🛠️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Resource Manager",
            code: vulkan::RESOURCE_MANAGER,
            category: "Vulkan",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Window Manager",
            code: vulkan::WINDOW_MANAGER,
            category: "Vulkan",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🪟",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Sync Manager",
            code: vulkan::SYNC_MANAGER,
            category: "Vulkan",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⏱️",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - RENDERIZADO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 Render Loop",
            code: vulkan::RENDER_LOOP,
            category: "Vulkan",
            subcategory: "Renderizado",
            color: (0x00, 0xbf, 0xff),
            icon: "↻",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "🎮 Main (Completo)",
            code: vulkan::MAIN,
            category: "Vulkan",
            subcategory: "Renderizado",
            color: (0xff, 0xd7, 0x00),
            icon: "🎮",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // VULKAN - BUILD
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🎮 CMakeLists",
            code: vulkan::CMAKE,
            category: "Vulkan",
            subcategory: "Build",
            color: (0x06, 0x4f, 0x8c),
            icon: "🛠️",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🎮 README",
            code: vulkan::README,
            category: "Vulkan",
            subcategory: "Build",
            color: (0x06, 0x4f, 0x8c),
            icon: "📖",
            language: NodeLanguage::Text,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS ASM+RUST+ZIG - BOOTLOADER
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Bootloader UEFI (ASM)",
            code: fastos_asm_rust_zig::asm::BOOT_UEFI,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "ASM (Bootloader)",
            color: (0xFF, 0x44, 0x00),
            icon: "💿",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🚀 Kernel Entry (ASM)",
            code: fastos_asm_rust_zig::asm::KERNEL_ENTRY,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "ASM (Bootloader)",
            color: (0xFF, 0x44, 0x00),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🚀 Interrupts (ASM)",
            code: fastos_asm_rust_zig::asm::INTERRUPTS,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "ASM (Bootloader)",
            color: (0xFF, 0x44, 0x00),
            icon: "🔴",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🚀 Memory Low Level (ASM)",
            code: fastos_asm_rust_zig::asm::MEMORY_LOW,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "ASM (Bootloader)",
            color: (0xFF, 0x44, 0x00),
            icon: "💾",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS ASM+RUST+ZIG - RUST (KERNEL)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🦀 Kernel Main (Rust)",
            code: fastos_asm_rust_zig::rust::KERNEL_MAIN,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🦀 Port I/O (Rust)",
            code: fastos_asm_rust_zig::rust::PORTS,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🦀 Interrupts System (Rust)",
            code: fastos_asm_rust_zig::rust::INTERRUPTS,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🦀 Memory Management (Rust)",
            code: fastos_asm_rust_zig::rust::MEMORY,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🦀 Drivers (Rust)",
            code: fastos_asm_rust_zig::rust::DRIVERS,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🦀 FFI Interface (Rust)",
            code: fastos_asm_rust_zig::rust::FFI,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS ASM+RUST+ZIG - ZIG (SISTEMA)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "⚡ Sistema (Zig)",
            code: fastos_asm_rust_zig::zig::SYSTEM,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Zig (Sistema)",
            color: (0xF7, 0xA4, 0x1D),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        Template {
            name: "⚡ Allocator (Zig)",
            code: fastos_asm_rust_zig::zig::ALLOCATOR,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Zig (Sistema)",
            color: (0xF7, 0xA4, 0x1D),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        Template {
            name: "⚡ Filesystem (Zig)",
            code: fastos_asm_rust_zig::zig::FS,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Zig (Sistema)",
            color: (0xF7, 0xA4, 0x1D),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        Template {
            name: "⚡ Scheduler (Zig)",
            code: fastos_asm_rust_zig::zig::SCHEDULER,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Zig (Sistema)",
            color: (0xF7, 0xA4, 0x1D),
            icon: "⚡",
            language: NodeLanguage::Zig,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS ASM+RUST+ZIG - INTEGRATION
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔗 Cargo.toml",
            code: fastos_asm_rust_zig::integration::CARGO_TOML,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "⚙️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "🔗 Linker Script",
            code: fastos_asm_rust_zig::integration::LINKER_LD,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔗",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🔗 Build Script (Linux/Mac)",
            code: fastos_asm_rust_zig::integration::BUILD_SH,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔧",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🔗 Build Script (Windows)",
            code: fastos_asm_rust_zig::integration::BUILD_BAT,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔧",
            language: NodeLanguage::Text,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS ASM+RUST+ZIG - DOCUMENTACIÓN
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📖 README",
            code: fastos_asm_rust_zig::README,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Docs",
            color: (0x88, 0x88, 0x88),
            icon: "📖",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🔥 POTENCIAL.md",
            code: fastos_asm_rust_zig::POTENCIAL,
            category: "FastOS ASM+Rust+Zig",
            subcategory: "Docs",
            color: (0xFF, 0x44, 0x00),
            icon: "🔥",
            language: NodeLanguage::Text,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - TIPOS BASE
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 DirectX12 Types",
            code: directx12::TYPES_H,
            category: "DirectX12",
            subcategory: "Base",
            color: (0x00, 0x7a, 0xcc),
            icon: "📋",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - INICIALIZACIÓN
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 Adapter (GPU)",
            code: directx12::ADAPTER,
            category: "DirectX12",
            subcategory: "Inicialización",
            color: (0x00, 0x7a, 0xcc),
            icon: "🎮",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Device",
            code: directx12::DEVICE,
            category: "DirectX12",
            subcategory: "Inicialización",
            color: (0x00, 0x7a, 0xcc),
            icon: "🖥️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Swapchain",
            code: directx12::SWAPCHAIN,
            category: "DirectX12",
            subcategory: "Inicialización",
            color: (0x00, 0x7a, 0xcc),
            icon: "🔄",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - COMANDOS
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 Command Allocator",
            code: directx12::COMMAND_ALLOCATOR,
            category: "DirectX12",
            subcategory: "Comandos",
            color: (0x00, 0x9a, 0xff),
            icon: "⚙️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Command List",
            code: directx12::COMMAND_LIST,
            category: "DirectX12",
            subcategory: "Comandos",
            color: (0x00, 0x9a, 0xff),
            icon: "📋",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - PIPELINE
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 Root Signature",
            code: directx12::ROOT_SIGNATURE,
            category: "DirectX12",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "🔧",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Pipeline State",
            code: directx12::PIPELINE_STATE,
            category: "DirectX12",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "📐",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Shader (HLSL)",
            code: directx12::SHADER_HLSL,
            category: "DirectX12",
            subcategory: "Pipeline",
            color: (0x8b, 0x00, 0x8b),
            icon: "🎨",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - RECURSOS
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 Buffers",
            code: directx12::BUFFERS,
            category: "DirectX12",
            subcategory: "Recursos",
            color: (0x00, 0x80, 0x80),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - LIBRERÍAS (Funciones independientes reutilizables)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "📚 Helpers (Utilidades)",
            code: directx12::HELPERS,
            category: "DirectX12",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🛠️",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Resource Manager",
            code: directx12::RESOURCE_MANAGER,
            category: "DirectX12",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "📦",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Window Manager",
            code: directx12::WINDOW_MANAGER,
            category: "DirectX12",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "🪟",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "📚 Sync Manager",
            code: directx12::SYNC_MANAGER,
            category: "DirectX12",
            subcategory: "Librerías",
            color: (0x80, 0x40, 0x00),
            icon: "⏱️",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - RENDERIZADO
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 Render Loop",
            code: directx12::RENDER_LOOP,
            category: "DirectX12",
            subcategory: "Renderizado",
            color: (0x00, 0xbf, 0xff),
            icon: "↻",
            language: NodeLanguage::Cpp,
        },
        Template {
            name: "💎 Main (Completo)",
            code: directx12::MAIN,
            category: "DirectX12",
            subcategory: "Renderizado",
            color: (0xff, 0xd7, 0x00),
            icon: "💎",
            language: NodeLanguage::Cpp,
        },
        
        // ══════════════════════════════════════════════════════════════
        // DIRECTX12 - BUILD
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "💎 CMakeLists",
            code: directx12::CMAKE,
            category: "DirectX12",
            subcategory: "Build",
            color: (0x06, 0x4f, 0x8c),
            icon: "🛠️",
            language: NodeLanguage::Text,
        },
        Template {
            name: "💎 README",
            code: directx12::README,
            category: "DirectX12",
            subcategory: "Build",
            color: (0x06, 0x4f, 0x8c),
            icon: "📖",
            language: NodeLanguage::Text,
        },
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
pub use c::HELLO as C_HELLO;
#[allow(unused_imports)]
pub use cpp::HELLO as CPP_HELLO;
#[allow(unused_imports)]
pub use rust::HELLO as RUST_HELLO;
