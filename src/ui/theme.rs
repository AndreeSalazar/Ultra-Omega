// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Temas y Colores Profesionales
// Diseño moderno con jerarquía visual clara y consistente
// ═══════════════════════════════════════════════════════════════════════════════

use eframe::egui::Color32;

/// Tema de colores profesional para Ultra-Omega
pub struct UltraOmegaTheme {
    // Colores primarios - Base de la interfaz
    pub background_primary: Color32,      // Fondo principal
    pub background_secondary: Color32,    // Fondo secundario (paneles)
    pub background_tertiary: Color32,      // Fondo terciario (elementos activos)
    
    // Colores de superficie
    pub surface_primary: Color32,         // Superficies principales
    pub surface_secondary: Color32,       // Superficies secundarias
    pub surface_elevated: Color32,       // Superficies elevadas (cards, dialogs)
    
    // Colores de texto
    pub text_primary: Color32,            // Texto principal
    pub text_secondary: Color32,         // Texto secundario
    pub text_muted: Color32,             // Texto deshabilitado/marcador de posición
    pub text_inverse: Color32,           // Texto sobre fondos oscuros
    
    // Colores de acento
    pub accent_primary: Color32,         // Acento principal (azul)
    pub accent_secondary: Color32,       // Acento secundario (púrpura)
    pub accent_success: Color32,         // Éxito (verde)
    pub accent_warning: Color32,         // Advertencia (ámbar)
    pub accent_error: Color32,           // Error (rojo)
    pub accent_info: Color32,            // Información (cyan)
    
    // Colores de borde y divisores
    pub border_primary: Color32,          // Bordes principales
    pub border_secondary: Color32,        // Bordes secundarios
    pub border_focus: Color32,            // Bordes de enfoque
    pub border_subtle: Color32,           // Bordes sutiles
    
    // Colores para nodos por lenguaje
    pub node_rust: Color32,              // Nodos Rust
    pub node_cpp: Color32,               // Nodos C++
    pub node_python: Color32,            // Nodos Python
    pub node_java: Color32,              // Nodos Java
    pub node_asm: Color32,               // Nodos Assembly
    pub node_text: Color32,              // Nodos Texto
    pub node_folder: Color32,            // Nodos Carpeta
    pub node_auto: Color32,              // Nodos Auto
    
    // Colores para pins
    pub pin_input: Color32,              // Pins de entrada
    pub pin_output: Color32,             // Pins de salida
    pub pin_connected: Color32,          // Pins conectados
    pub pin_hover: Color32,              // Pins al pasar el mouse
    
    // Colores para terminal
    pub terminal_bg: Color32,            // Fondo de terminal
    pub terminal_text: Color32,          // Texto de terminal
    pub terminal_selection: Color32,     // Selección de terminal
    
    // Sombras y efectos
    pub shadow_light: Color32,          // Sombra ligera
    pub shadow_medium: Color32,          // Sombra media
    pub shadow_strong: Color32,          // Sombra fuerte
}

impl Default for UltraOmegaTheme {
    fn default() -> Self {
        Self {
            // Esquema de colores oscuro profesional inspirado en VS Code + Blender
            background_primary: Color32::from_rgb(18, 18, 20),        // Fondo principal muy oscuro
            background_secondary: Color32::from_rgb(24, 26, 28),      // Paneles laterales
            background_tertiary: Color32::from_rgb(32, 34, 37),      // Elementos activos
            
            surface_primary: Color32::from_rgb(37, 39, 43),          // Superficies principales
            surface_secondary: Color32::from_rgb(43, 45, 49),        // Superficies secundarias
            surface_elevated: Color32::from_rgb(49, 51, 55),        // Cards y dialogs
            
            text_primary: Color32::from_rgb(230, 230, 230),         // Texto principal
            text_secondary: Color32::from_rgb(180, 180, 180),        // Texto secundario
            text_muted: Color32::from_rgb(120, 120, 120),          // Texto deshabilitado
            text_inverse: Color32::from_rgb(255, 255, 255),         // Texto sobre fondos oscuros
            
            // Acentos modernos y distinguibles
            accent_primary: Color32::from_rgb(97, 175, 239),        // Azul principal
            accent_secondary: Color32::from_rgb(156, 136, 255),     // Púrpura secundario
            accent_success: Color32::from_rgb(76, 175, 80),         // Verde éxito
            accent_warning: Color32::from_rgb(255, 185, 0),         // Ámbar advertencia
            accent_error: Color32::from_rgb(244, 67, 54),           // Rojo error
            accent_info: Color32::from_rgb(0, 188, 212),             // Cyan información
            
            // Bordes con diferentes niveles de visibilidad
            border_primary: Color32::from_rgb(60, 60, 65),          // Bordes principales
            border_secondary: Color32::from_rgb(45, 45, 50),        // Bordes secundarios
            border_focus: Color32::from_rgb(97, 175, 239),          // Bordes de enfoque (mismo que acento)
            border_subtle: Color32::from_rgb(30, 30, 35),           // Bordes sutiles
            
            // Colores de nodos distintivos pero armoniosos
            node_rust: Color32::from_rgb(255, 140, 100),           // Naranja Rust
            node_cpp: Color32::from_rgb(0, 89, 153),               // Azul profundo C++
            node_python: Color32::from_rgb(55, 118, 171),          // Azul Python
            node_java: Color32::from_rgb(237, 139, 0),             // Naranja Java
            node_asm: Color32::from_rgb(255, 220, 100),            // Amarillo Assembly
            node_text: Color32::from_rgb(200, 200, 150),          // Verde oliva Texto
            node_folder: Color32::from_rgb(255, 200, 87),          // Amarillo dorado Carpetas
            node_auto: Color32::from_rgb(180, 180, 180),           // Gris Auto
            
            // Pins con buena visibilidad y contraste
            pin_input: Color32::from_rgb(97, 175, 239),             // Azul entrada
            pin_output: Color32::from_rgb(156, 136, 255),          // Púrpura salida
            pin_connected: Color32::from_rgb(76, 175, 80),         // Verde conectado
            pin_hover: Color32::from_rgb(255, 255, 255),           // Blanco hover
            
            // Terminal con buen contraste
            terminal_bg: Color32::from_rgb(12, 12, 14),             // Fondo terminal muy oscuro
            terminal_text: Color32::from_rgb(200, 200, 200),       // Texto terminal
            terminal_selection: Color32::from_rgb(97, 175, 239),    // Selección terminal
            
            // Sombras con diferentes intensidades
            shadow_light: Color32::from_rgba_unmultiplied(0, 0, 0, 20),    // Sombra ligera
            shadow_medium: Color32::from_rgba_unmultiplied(0, 0, 0, 40),   // Sombra media
            shadow_strong: Color32::from_rgba_unmultiplied(0, 0, 0, 60),   // Sombra fuerte
        }
    }
}

impl UltraOmegaTheme {
    /// Obtener color para un lenguaje de nodo específico
    pub fn node_language_color(&self, language: &crate::core::node_graph::NodeLanguage) -> Color32 {
        use crate::core::node_graph::NodeLanguage;
        match language {
            NodeLanguage::Rust => self.node_rust,
            NodeLanguage::Cpp => self.node_cpp,
            NodeLanguage::Python => self.node_python,
            NodeLanguage::Java => self.node_java,
            NodeLanguage::Asm => self.node_asm,
            NodeLanguage::Text => self.node_text,
            NodeLanguage::Auto => self.node_auto,
        }
    }
    
    /// Obtener color de fondo para editor según lenguaje
    pub fn editor_background(&self, language: &crate::core::node_graph::NodeLanguage) -> Color32 {
        match language {
            crate::core::node_graph::NodeLanguage::Rust => Color32::from_rgb(30, 30, 35),
            crate::core::node_graph::NodeLanguage::Cpp => Color32::from_rgb(25, 35, 45),
            crate::core::node_graph::NodeLanguage::Python => Color32::from_rgb(25, 30, 40),
            crate::core::node_graph::NodeLanguage::Java => Color32::from_rgb(30, 30, 35),
            crate::core::node_graph::NodeLanguage::Asm => Color32::from_rgb(35, 30, 25),
            crate::core::node_graph::NodeLanguage::Text => Color32::from_rgb(30, 30, 30),
            crate::core::node_graph::NodeLanguage::Auto => Color32::from_rgb(28, 28, 28),
        }
    }
    
    /// Obtener color de texto para editor según lenguaje
    pub fn editor_text(&self, language: &crate::core::node_graph::NodeLanguage) -> Color32 {
        match language {
            crate::core::node_graph::NodeLanguage::Rust => Color32::from_rgb(240, 240, 240),
            crate::core::node_graph::NodeLanguage::Cpp => Color32::from_rgb(220, 220, 240),
            crate::core::node_graph::NodeLanguage::Python => Color32::from_rgb(230, 230, 250),
            crate::core::node_graph::NodeLanguage::Java => Color32::from_rgb(240, 240, 240),
            crate::core::node_graph::NodeLanguage::Asm => Color32::from_rgb(250, 240, 230),
            crate::core::node_graph::NodeLanguage::Text => Color32::from_rgb(220, 220, 220),
            crate::core::node_graph::NodeLanguage::Auto => Color32::from_rgb(230, 230, 230),
        }
    }
}

/// Instancia global del tema
pub static THEME: std::sync::LazyLock<UltraOmegaTheme> = std::sync::LazyLock::new(UltraOmegaTheme::default);

/// Funciones de utilidad para aplicar el tema
pub mod utils {
    use super::*;
    use eframe::egui::{Stroke, Rounding, Margin, Vec2};
    
    /// Crear un stroke con el color y grosor del tema
    pub fn theme_stroke(width: f32, color: Color32) -> Stroke {
        Stroke::new(width, color)
    }
    
    /// Crear rounding con radio consistente
    pub fn theme_rounding(radius: f32) -> Rounding {
        Rounding::same(radius)
    }
    
    /// Crear margin con valores consistentes
    pub fn theme_margin(all: f32) -> Margin {
        Margin::same(all)
    }
    
    /// Crear margin asimétrico
    pub fn theme_margin_xy(x: f32, y: f32) -> Margin {
        Margin::symmetric(x, y)
    }
    
    /// Crear margin completo
    pub fn theme_margin_full(top: f32, right: f32, bottom: f32, left: f32) -> Margin {
        Margin { top, right, bottom, left }
    }
    
    /// Interpolar entre dos colores
    pub fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);
        Color32::from_rgba_premultiplied(
            (a.r() as f32 * (1.0 - t) + b.r() as f32 * t) as u8,
            (a.g() as f32 * (1.0 - t) + b.g() as f32 * t) as u8,
            (a.b() as f32 * (1.0 - t) + b.b() as f32 * t) as u8,
            (a.a() as f32 * (1.0 - t) + b.a() as f32 * t) as u8,
        )
    }
    
    /// Oscurecer un color
    pub fn darken_color(color: Color32, amount: f32) -> Color32 {
        lerp_color(color, Color32::BLACK, amount)
    }
    
    /// Aclarar un color
    pub fn lighten_color(color: Color32, amount: f32) -> Color32 {
        lerp_color(color, Color32::WHITE, amount)
    }
}
