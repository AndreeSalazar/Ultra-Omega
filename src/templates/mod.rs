// ═══════════════════════════════════════════════════════════════════════════
// Templates de código para Ultra Omega
// Organizados por jerarquía: Básico → Intermedio → Avanzado
// Incluye FastOS: Templates completos para crear un sistema operativo
// Incluye Binary: Templates binarios/hexadecimales para ejecución directa
// ═══════════════════════════════════════════════════════════════════════════

use crate::node_graph::NodeLanguage;

// ══════════════════════════════════════════
// BINARY TEMPLATES (CPU x86_64 / GPU SPIR-V)
// Templates ejecutables en código máquina directo
// ══════════════════════════════════════════
pub mod binary;

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
}

// ══════════════════════════════════════════
// FASTOS 64-BIT (UEFI + Long Mode)
// ══════════════════════════════════════════
pub mod fastos64 {
    // Bootloader UEFI
    pub const BOOT_UEFI: &str = include_str!("fastos64/boot_uefi.asm");
    pub const KERNEL_ENTRY64: &str = include_str!("fastos64/kernel_entry64.asm");
    pub const BOOT_MULTIBOOT2: &str = include_str!("fastos64/boot_multiboot2.asm");
    
    // Kernel
    pub const KERNEL_MAIN64: &str = include_str!("fastos64/kernel_main64.c");
    pub const TYPES64_H: &str = include_str!("fastos64/types64.h");
    pub const PORTS64_H: &str = include_str!("fastos64/ports64.h");
    
    // Gráficos
    pub const FRAMEBUFFER_H: &str = include_str!("fastos64/framebuffer.h");
    pub const FRAMEBUFFER_C: &str = include_str!("fastos64/framebuffer.c");
    pub const FONT8X16_H: &str = include_str!("fastos64/font8x16.h");
    
    // Sistema
    pub const IDT64_H: &str = include_str!("fastos64/idt64.h");
    pub const MEMORY64_H: &str = include_str!("fastos64/memory64.h");
    pub const KEYBOARD64_H: &str = include_str!("fastos64/keyboard64.h");
    pub const SHELL64_H: &str = include_str!("fastos64/shell64.h");
    
    // PCI (para detección de GPU)
    pub const PCI_H: &str = include_str!("fastos64/pci.h");
    pub const PCI_C: &str = include_str!("fastos64/pci.c");
    
    // GPU NVIDIA (RTX 3060)
    pub const GPU_NVIDIA_H: &str = include_str!("fastos64/gpu_nvidia.h");
    pub const GPU_NVIDIA_C: &str = include_str!("fastos64/gpu_nvidia.c");
    
    // Vulkan-FastOS (Capa de abstracción)
    pub const VULKAN_FASTOS_H: &str = include_str!("fastos64/vulkan_fastos.h");
    pub const VULKAN_FASTOS_C: &str = include_str!("fastos64/vulkan_fastos.c");
    
    // Vulkan Nativo (para GPU Passthrough)
    pub const VULKAN_NATIVE_H: &str = include_str!("fastos64/vulkan_native.h");
    pub const VULKAN_NATIVE_C: &str = include_str!("fastos64/vulkan_native.c");
    
    // Desktop (Window Manager estilo Windows 11)
    pub const WINDOW_C: &str = include_str!("fastos64/desktop/window.c");
    pub const TASKBAR_C: &str = include_str!("fastos64/desktop/taskbar.c");
    pub const GRAPHICS_C: &str = include_str!("fastos64/desktop/graphics.c");
    pub const MOUSE_C: &str = include_str!("fastos64/desktop/mouse.c");
    
    // Build System
    pub const LINKER64_LD: &str = include_str!("fastos64/linker64.ld");
    pub const MAKEFILE: &str = include_str!("fastos64/Makefile");
    pub const BUILD64_BAT: &str = include_str!("fastos64/build64.bat");
    pub const BUILD_SIMPLE_BAT: &str = include_str!("fastos64/build_simple.bat");
    
    // Documentación
    pub const README: &str = include_str!("fastos64/README.md");
    pub const VFIO_PASSTHROUGH: &str = include_str!("fastos64/vfio_passthrough.md");
    
    // FastOS XP Edition (Desktop completo estilo Windows XP)
    pub const FASTOS_XP: &str = include_str!("fastos64/fastos_xp.asm");
}

// ══════════════════════════════════════════
// FASTOS 64-BIT RUST (ASM + Rust)
// Sistema operativo combinando NASM y Rust
// ══════════════════════════════════════════
pub mod fastos64_rust {
    // ═══════════════════════════════════════
    // ASM (NASM x86_64)
    // ═══════════════════════════════════════
    pub mod asm {
        pub const BOOT_UEFI: &str = include_str!("fastos64_rust/asm/boot_uefi.asm");
        pub const KERNEL_ENTRY: &str = include_str!("fastos64_rust/asm/kernel_entry.asm");
        pub const INTERRUPTS: &str = include_str!("fastos64_rust/asm/interrupts.asm");
        pub const MEMORY: &str = include_str!("fastos64_rust/asm/memory.asm");
    }
    
    // ═══════════════════════════════════════
    // RUST (Kernel)
    // ═══════════════════════════════════════
    pub mod rust {
        pub const KERNEL_MAIN: &str = include_str!("fastos64_rust/rust/kernel_main.rs");
        pub const PORTS: &str = include_str!("fastos64_rust/rust/ports.rs");
        pub const INTERRUPTS: &str = include_str!("fastos64_rust/rust/interrupts.rs");
        pub const MEMORY: &str = include_str!("fastos64_rust/rust/memory.rs");
        pub const DRIVERS: &str = include_str!("fastos64_rust/rust/drivers.rs");
        pub const FFI: &str = include_str!("fastos64_rust/rust/ffi.rs");
        pub const LIB: &str = include_str!("fastos64_rust/rust/lib.rs");
    }
    
    // ═══════════════════════════════════════
    // INTEGRATION (Build & Config)
    // ═══════════════════════════════════════
    pub mod integration {
        pub const CARGO_TOML: &str = include_str!("fastos64_rust/integration/Cargo.toml");
        pub const LINKER_LD: &str = include_str!("fastos64_rust/integration/linker.ld");
        pub const BUILD_SH: &str = include_str!("fastos64_rust/integration/build.sh");
        pub const BUILD_BAT: &str = include_str!("fastos64_rust/integration/build.bat");
        pub const EXAMPLE: &str = include_str!("fastos64_rust/integration/example_integration.rs");
        pub const README: &str = include_str!("fastos64_rust/integration/README.md");
    }
    
    // README principal
    pub const README: &str = include_str!("fastos64_rust/README.md");
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
    
    // Build
    pub const CMAKE: &str = include_str!("vulkan/CMakeLists.txt");
    pub const README: &str = include_str!("vulkan/README.md");
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
        // FASTOS - BOOTLOADER (ASM)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 Boot Sector",
            code: fastos::BOOT_SECTOR,
            category: "FastOS",
            subcategory: "Bootloader",
            color: (0xff, 0x00, 0x00),
            icon: "💿",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🔥 Stage 2 Bootloader",
            code: fastos::STAGE2,
            category: "FastOS",
            subcategory: "Bootloader",
            color: (0xff, 0x00, 0x00),
            icon: "🚀",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🔥 Kernel Entry",
            code: fastos::KERNEL_ENTRY,
            category: "FastOS",
            subcategory: "Bootloader",
            color: (0xff, 0x00, 0x00),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS - KERNEL (C)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 Kernel Main",
            code: fastos::KERNEL_MAIN,
            category: "FastOS",
            subcategory: "Kernel",
            color: (0xff, 0x44, 0x00),
            icon: "🧠",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Kernel Header",
            code: fastos::KERNEL_H,
            category: "FastOS",
            subcategory: "Kernel",
            color: (0xff, 0x44, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS - DRIVERS (C)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 VGA Driver",
            code: fastos::VGA_DRIVER,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "🖥️",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 VGA Header",
            code: fastos::VGA_H,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Keyboard Driver",
            code: fastos::KEYBOARD_DRIVER,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "⌨️",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Keyboard Header",
            code: fastos::KEYBOARD_H,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Timer Driver",
            code: fastos::TIMER,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "⏱️",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Timer Header",
            code: fastos::TIMER_H,
            category: "FastOS",
            subcategory: "Drivers",
            color: (0x00, 0xaa, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS - SISTEMA (C)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 IDT (Interrupciones)",
            code: fastos::IDT,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "⚡",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 IDT Header",
            code: fastos::IDT_H,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Memory Manager",
            code: fastos::MEMORY,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "💾",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Memory Header",
            code: fastos::MEMORY_H,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Shell",
            code: fastos::SHELL,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "💻",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Shell Header",
            code: fastos::SHELL_H,
            category: "FastOS",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xaa),
            icon: "📋",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS - UTILIDADES (C)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 String Library",
            code: fastos::STRING,
            category: "FastOS",
            subcategory: "Utilidades",
            color: (0x00, 0x88, 0xcc),
            icon: "📝",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 String Header",
            code: fastos::STRING_H,
            category: "FastOS",
            subcategory: "Utilidades",
            color: (0x00, 0x88, 0xcc),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Types Header",
            code: fastos::TYPES_H,
            category: "FastOS",
            subcategory: "Utilidades",
            color: (0x00, 0x88, 0xcc),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🔥 Ports Header",
            code: fastos::PORTS_H,
            category: "FastOS",
            subcategory: "Utilidades",
            color: (0x00, 0x88, 0xcc),
            icon: "📋",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS - BUILD SYSTEM
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🔥 Linker Script",
            code: fastos::LINKER,
            category: "FastOS",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "🔗",
            language: NodeLanguage::Asm,  // Para resaltado
        },
        Template {
            name: "🔥 Makefile",
            code: fastos::MAKEFILE,
            category: "FastOS",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "🛠️",
            language: NodeLanguage::C,  // Para resaltado
        },
        Template {
            name: "🔥 README",
            code: fastos::README,
            category: "FastOS",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "📖",
            language: NodeLanguage::C,  // Para resaltado
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
        // FASTOS 64-BIT - BOOTLOADER
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Boot UEFI",
            code: fastos64::BOOT_UEFI,
            category: "FastOS 64-bit",
            subcategory: "Bootloader",
            color: (0x00, 0xd4, 0xff),
            icon: "💿",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "🚀 Kernel Entry 64",
            code: fastos64::KERNEL_ENTRY64,
            category: "FastOS 64-bit",
            subcategory: "Bootloader",
            color: (0x00, 0xd4, 0xff),
            icon: "⚡",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - KERNEL
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Kernel Main 64",
            code: fastos64::KERNEL_MAIN64,
            category: "FastOS 64-bit",
            subcategory: "Kernel",
            color: (0xff, 0x88, 0x00),
            icon: "🧠",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Types 64-bit",
            code: fastos64::TYPES64_H,
            category: "FastOS 64-bit",
            subcategory: "Kernel",
            color: (0xff, 0x88, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Ports 64-bit",
            code: fastos64::PORTS64_H,
            category: "FastOS 64-bit",
            subcategory: "Kernel",
            color: (0xff, 0x88, 0x00),
            icon: "📋",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - GRÁFICOS
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Framebuffer Header",
            code: fastos64::FRAMEBUFFER_H,
            category: "FastOS 64-bit",
            subcategory: "Gráficos",
            color: (0x00, 0xff, 0x88),
            icon: "🖥️",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Framebuffer Driver",
            code: fastos64::FRAMEBUFFER_C,
            category: "FastOS 64-bit",
            subcategory: "Gráficos",
            color: (0x00, 0xff, 0x88),
            icon: "🖥️",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - SISTEMA
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 IDT 64-bit",
            code: fastos64::IDT64_H,
            category: "FastOS 64-bit",
            subcategory: "Sistema",
            color: (0xaa, 0x00, 0xff),
            icon: "⚡",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - PCI (Para GPU)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 PCI Header",
            code: fastos64::PCI_H,
            category: "FastOS 64-bit",
            subcategory: "PCI/GPU",
            color: (0xff, 0x00, 0x88),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 PCI Driver",
            code: fastos64::PCI_C,
            category: "FastOS 64-bit",
            subcategory: "PCI/GPU",
            color: (0xff, 0x00, 0x88),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - GPU NVIDIA (RTX 3060)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 GPU NVIDIA Header",
            code: fastos64::GPU_NVIDIA_H,
            category: "FastOS 64-bit",
            subcategory: "GPU/Vulkan",
            color: (0x76, 0xb9, 0x00),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 GPU NVIDIA Driver",
            code: fastos64::GPU_NVIDIA_C,
            category: "FastOS 64-bit",
            subcategory: "GPU/Vulkan",
            color: (0x76, 0xb9, 0x00),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Vulkan-FastOS Header",
            code: fastos64::VULKAN_FASTOS_H,
            category: "FastOS 64-bit",
            subcategory: "GPU/Vulkan",
            color: (0xac, 0x14, 0x2c),
            icon: "🔥",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Vulkan-FastOS Impl",
            code: fastos64::VULKAN_FASTOS_C,
            category: "FastOS 64-bit",
            subcategory: "GPU/Vulkan",
            color: (0xac, 0x14, 0x2c),
            icon: "🔥",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - VULKAN NATIVO (GPU Passthrough)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Vulkan Native Header",
            code: fastos64::VULKAN_NATIVE_H,
            category: "FastOS 64-bit",
            subcategory: "Vulkan Native",
            color: (0xac, 0x14, 0x2c),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Vulkan Native Impl",
            code: fastos64::VULKAN_NATIVE_C,
            category: "FastOS 64-bit",
            subcategory: "Vulkan Native",
            color: (0xac, 0x14, 0x2c),
            icon: "🎮",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - DESKTOP (Window Manager)
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Window Manager",
            code: fastos64::WINDOW_C,
            category: "FastOS 64-bit",
            subcategory: "Desktop",
            color: (0x00, 0x78, 0xd4),
            icon: "🪟",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Taskbar",
            code: fastos64::TASKBAR_C,
            category: "FastOS 64-bit",
            subcategory: "Desktop",
            color: (0x00, 0x78, 0xd4),
            icon: "📊",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Graphics Primitives",
            code: fastos64::GRAPHICS_C,
            category: "FastOS 64-bit",
            subcategory: "Desktop",
            color: (0x00, 0x78, 0xd4),
            icon: "🎨",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Mouse Driver",
            code: fastos64::MOUSE_C,
            category: "FastOS 64-bit",
            subcategory: "Desktop",
            color: (0x00, 0x78, 0xd4),
            icon: "🖱️",
            language: NodeLanguage::C,
        },
        Template {
            name: "🚀 Font 8x16",
            code: fastos64::FONT8X16_H,
            category: "FastOS 64-bit",
            subcategory: "Desktop",
            color: (0x00, 0x78, 0xd4),
            icon: "🔤",
            language: NodeLanguage::C,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - BUILD SYSTEM
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 Linker Script 64",
            code: fastos64::LINKER64_LD,
            category: "FastOS 64-bit",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "🔗",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🚀 Makefile",
            code: fastos64::MAKEFILE,
            category: "FastOS 64-bit",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "🛠️",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🚀 Build Script (Full)",
            code: fastos64::BUILD64_BAT,
            category: "FastOS 64-bit",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "⚙️",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🚀 Build Script (Simple)",
            code: fastos64::BUILD_SIMPLE_BAT,
            category: "FastOS 64-bit",
            subcategory: "Build",
            color: (0x88, 0x88, 0x00),
            icon: "⚙️",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🚀 Boot Multiboot2",
            code: fastos64::BOOT_MULTIBOOT2,
            category: "FastOS 64-bit",
            subcategory: "Bootloader",
            color: (0x00, 0xd4, 0xff),
            icon: "💿",
            language: NodeLanguage::Asm,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS 64-BIT - DOCUMENTACIÓN
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🚀 README",
            code: fastos64::README,
            category: "FastOS 64-bit",
            subcategory: "Docs",
            color: (0x88, 0x88, 0x88),
            icon: "📖",
            language: NodeLanguage::Text,
        },
        Template {
            name: "🚀 VFIO GPU Passthrough",
            code: fastos64::VFIO_PASSTHROUGH,
            category: "FastOS 64-bit",
            subcategory: "Docs",
            color: (0x76, 0xb9, 0x00),
            icon: "🎮",
            language: NodeLanguage::Text,
        },
        
        // ══════════════════════════════════════════════════════════════
        // FASTOS XP EDITION - Desktop completo
        // ══════════════════════════════════════════════════════════════
        Template {
            name: "🖥️ FastOS XP Edition",
            code: fastos64::FASTOS_XP,
            category: "FastOS 64-bit",
            subcategory: "Desktop XP",
            color: (0x00, 0x78, 0xd4),
            icon: "🖥️",
            language: NodeLanguage::Asm,
        },
        
        // ═══════════════════════════════════════════════════════════════════════════════
        // FASTOS 64-BIT RUST (ASM + Rust)
        // ═══════════════════════════════════════════════════════════════════════════════
        
        // ───────────────────────────────────────────────────────────────────────────────
        // ASM (NASM)
        // ───────────────────────────────────────────────────────────────────────────────
        Template {
            name: "Bootloader UEFI (ASM)",
            code: fastos64_rust::asm::BOOT_UEFI,
            category: "FastOS 64-bit Rust",
            subcategory: "ASM (NASM)",
            color: (0xFF, 0x44, 0x00),
            icon: "🔴",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Kernel Entry (ASM)",
            code: fastos64_rust::asm::KERNEL_ENTRY,
            category: "FastOS 64-bit Rust",
            subcategory: "ASM (NASM)",
            color: (0xFF, 0x44, 0x00),
            icon: "🔴",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Interrupts (ASM)",
            code: fastos64_rust::asm::INTERRUPTS,
            category: "FastOS 64-bit Rust",
            subcategory: "ASM (NASM)",
            color: (0xFF, 0x44, 0x00),
            icon: "🔴",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Memory Functions (ASM)",
            code: fastos64_rust::asm::MEMORY,
            category: "FastOS 64-bit Rust",
            subcategory: "ASM (NASM)",
            color: (0xFF, 0x44, 0x00),
            icon: "🔴",
            language: NodeLanguage::Asm,
        },
        
        // ───────────────────────────────────────────────────────────────────────────────
        // RUST (Kernel)
        // ───────────────────────────────────────────────────────────────────────────────
        Template {
            name: "Kernel Main (Rust)",
            code: fastos64_rust::rust::KERNEL_MAIN,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Port I/O (Rust)",
            code: fastos64_rust::rust::PORTS,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Interrupts System (Rust)",
            code: fastos64_rust::rust::INTERRUPTS,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Memory Management (Rust)",
            code: fastos64_rust::rust::MEMORY,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Drivers (Rust)",
            code: fastos64_rust::rust::DRIVERS,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "FFI Interface (Rust)",
            code: fastos64_rust::rust::FFI,
            category: "FastOS 64-bit Rust",
            subcategory: "Rust (Kernel)",
            color: (0xDE, 0x39, 0x00),
            icon: "🦀",
            language: NodeLanguage::Rust,
        },
        
        // ───────────────────────────────────────────────────────────────────────────────
        // INTEGRATION (Build & Config)
        // ───────────────────────────────────────────────────────────────────────────────
        Template {
            name: "Cargo.toml",
            code: fastos64_rust::integration::CARGO_TOML,
            category: "FastOS 64-bit Rust",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "⚙️",
            language: NodeLanguage::Rust,
        },
        Template {
            name: "Linker Script",
            code: fastos64_rust::integration::LINKER_LD,
            category: "FastOS 64-bit Rust",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔗",
            language: NodeLanguage::Asm,
        },
        Template {
            name: "Build Script (Linux/Mac)",
            code: fastos64_rust::integration::BUILD_SH,
            category: "FastOS 64-bit Rust",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔧",
            language: NodeLanguage::Text,
        },
        Template {
            name: "Build Script (Windows)",
            code: fastos64_rust::integration::BUILD_BAT,
            category: "FastOS 64-bit Rust",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "🔧",
            language: NodeLanguage::Text,
        },
        Template {
            name: "Integration Example",
            code: fastos64_rust::integration::EXAMPLE,
            category: "FastOS 64-bit Rust",
            subcategory: "Integration",
            color: (0x88, 0x88, 0x00),
            icon: "📝",
            language: NodeLanguage::Rust,
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
