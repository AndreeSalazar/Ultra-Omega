// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Temas y Colores Profesionales
// Diseño moderno con jerarquía visual clara y consistente
// Versión simplificada para Vulkan backend
// ═══════════════════════════════════════════════════════════════════════════════

/// Estructura de color RGBA simple para Vulkan
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
    
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// Tema de colores profesional para Ultra-Omega
pub struct UltraOmegaTheme {
    // Colores primarios - Base de la interfaz
    pub background_primary: Color,
    pub background_secondary: Color,
    pub background_tertiary: Color,
    
    // Colores de superficie
    pub surface_primary: Color,
    pub surface_secondary: Color,
    pub surface_elevated: Color,
    
    // Colores de texto
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_inverse: Color,
    
    // Colores de acento
    pub accent_primary: Color,
    pub accent_secondary: Color,
    pub accent_success: Color,
    pub accent_warning: Color,
    pub accent_error: Color,
    pub accent_info: Color,
    
    // Colores de borde y divisores
    pub border_primary: Color,
    pub border_secondary: Color,
    pub border_focus: Color,
    pub border_subtle: Color,
    
    // Colores para nodos por lenguaje
    pub node_rust: Color,
    pub node_cpp: Color,
    pub node_python: Color,
    pub node_java: Color,
    pub node_asm: Color,
    pub node_text: Color,
    pub node_folder: Color,
    pub node_auto: Color,
    
    // Colores para pins
    pub pin_input: Color,
    pub pin_output: Color,
    pub pin_connected: Color,
    pub pin_hover: Color,
    
    // Colores para terminal
    pub terminal_bg: Color,
    pub terminal_text: Color,
    pub terminal_selection: Color,
}

impl Default for UltraOmegaTheme {
    fn default() -> Self {
        Self {
            // Esquema de colores oscuro profesional inspirado en VS Code + Blender
            background_primary: Color::from_rgb(18, 18, 20),
            background_secondary: Color::from_rgb(24, 26, 28),
            background_tertiary: Color::from_rgb(32, 34, 37),
            
            surface_primary: Color::from_rgb(37, 39, 43),
            surface_secondary: Color::from_rgb(43, 45, 49),
            surface_elevated: Color::from_rgb(49, 51, 55),
            
            text_primary: Color::from_rgb(230, 230, 230),
            text_secondary: Color::from_rgb(180, 180, 180),
            text_muted: Color::from_rgb(120, 120, 120),
            text_inverse: Color::from_rgb(255, 255, 255),
            
            // Acentos modernos y distinguibles
            accent_primary: Color::from_rgb(97, 175, 239),
            accent_secondary: Color::from_rgb(156, 136, 255),
            accent_success: Color::from_rgb(76, 175, 80),
            accent_warning: Color::from_rgb(255, 185, 0),
            accent_error: Color::from_rgb(244, 67, 54),
            accent_info: Color::from_rgb(0, 188, 212),
            
            // Bordes con diferentes niveles de visibilidad
            border_primary: Color::from_rgb(60, 60, 65),
            border_secondary: Color::from_rgb(45, 45, 50),
            border_focus: Color::from_rgb(97, 175, 239),
            border_subtle: Color::from_rgb(30, 30, 35),
            
            // Colores de nodos distintivos pero armoniosos
            node_rust: Color::from_rgb(255, 140, 100),
            node_cpp: Color::from_rgb(0, 89, 153),
            node_python: Color::from_rgb(55, 118, 171),
            node_java: Color::from_rgb(237, 139, 0),
            node_asm: Color::from_rgb(255, 220, 100),
            node_text: Color::from_rgb(200, 200, 150),
            node_folder: Color::from_rgb(255, 200, 87),
            node_auto: Color::from_rgb(180, 180, 180),
            
            // Pins con buena visibilidad y contraste
            pin_input: Color::from_rgb(97, 175, 239),
            pin_output: Color::from_rgb(156, 136, 255),
            pin_connected: Color::from_rgb(76, 175, 80),
            pin_hover: Color::from_rgb(255, 255, 255),
            
            // Terminal con buen contraste
            terminal_bg: Color::from_rgb(12, 12, 14),
            terminal_text: Color::from_rgb(200, 200, 200),
            terminal_selection: Color::from_rgb(97, 175, 239),
        }
    }
}

impl UltraOmegaTheme {
    /// Obtener color para un lenguaje de nodo específico
    pub fn node_language_color(&self, language: &crate::core::node_graph::NodeLanguage) -> Color {
        use crate::core::node_graph::NodeLanguage;
        match language {
            NodeLanguage::Rust => self.node_rust,
            NodeLanguage::Text => self.node_text,
            NodeLanguage::Auto => self.node_auto,
        }
    }
}

/// Instancia global del tema
pub static THEME: std::sync::LazyLock<UltraOmegaTheme> = std::sync::LazyLock::new(UltraOmegaTheme::default);
