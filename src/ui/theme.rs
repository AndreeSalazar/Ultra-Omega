// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Paleta Elegante Estilo Chino
// Inspirada en caligrafia, porcelana, jade y seda tradicional
// ═══════════════════════════════════════════════════════════════════════════════

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

    pub fn mix(self, other: Color, t: f32) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub fn dim(self, factor: f32) -> Color {
        Color {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a,
        }
    }
}

pub struct UltraOmegaTheme {
    // ── Tinta y Porcelana (fondos) ──
    pub ink_black: Color,
    pub ink_deep: Color,
    pub ink_medium: Color,
    pub porcelain: Color,
    pub porcelain_warm: Color,
    pub silk_cream: Color,

    // ── Piedras preciosas (superficies) ──
    pub jade_dark: Color,
    pub jade_medium: Color,
    pub jade_light: Color,
    pub obsidian: Color,
    pub slate: Color,

    // ── Texto (tinta sobre porcelana) ──
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_gold: Color,
    pub text_jade: Color,

    // ── Acentos imperiales ──
    pub vermillion: Color,
    pub imperial_gold: Color,
    pub jade_green: Color,
    pub indigo: Color,
    pub plum: Color,
    pub copper: Color,

    // ── Bordes (trazos de pincel) ──
    pub border_primary: Color,
    pub border_secondary: Color,
    pub border_focus: Color,
    pub border_subtle: Color,
    pub border_gold: Color,

    // ── Nodos por lenguaje ──
    pub node_rust: Color,
    pub node_rust_body: Color,
    pub node_text: Color,
    pub node_text_body: Color,
    pub node_auto: Color,
    pub node_auto_body: Color,
    pub node_folder: Color,
    pub node_folder_body: Color,

    // ── Pins (perlas) ──
    pub pin_input: Color,
    pub pin_output: Color,
    pub pin_connected: Color,
    pub pin_hover: Color,

    // ── Terminal ──
    pub terminal_bg: Color,
    pub terminal_text: Color,
    pub terminal_selection: Color,

    // ── Grid (cuaderno de caligrafia) ──
    pub grid_line: Color,
    pub grid_axis: Color,
    pub grid_dot: Color,

    // ── Conexiones (tinta) ──
    pub link_default: Color,
    pub link_active: Color,
    pub link_hover: Color,
}

impl Default for UltraOmegaTheme {
    fn default() -> Self {
        Self {
            // ── Tinta y Porcelana ──
            ink_black:      Color::from_rgb(22, 18, 16),      // #161210 - tinta negra cálida
            ink_deep:       Color::from_rgb(30, 26, 23),      // #1E1A17
            ink_medium:     Color::from_rgb(42, 37, 32),      // #2A2520
            porcelain:      Color::from_rgb(240, 237, 229),    // #F0EDE5 - porcelana
            porcelain_warm: Color::from_rgb(232, 228, 216),    // #E8E4D8
            silk_cream:     Color::from_rgb(220, 212, 195),    // #DCD4C3

            // ── Piedras preciosas ──
            jade_dark:   Color::from_rgb(35, 42, 35),          // #232A23
            jade_medium: Color::from_rgb(50, 58, 48),          // #323A30
            jade_light:  Color::from_rgb(65, 75, 60),          // #414B3C
            obsidian:    Color::from_rgb(28, 25, 24),          // #1C1918
            slate:       Color::from_rgb(48, 44, 40),          // #302C28

            // ── Texto ──
            text_primary:   Color::from_rgb(235, 230, 218),    // #EBE6DA - marfil
            text_secondary: Color::from_rgb(195, 185, 168),    // #C3B9A8
            text_muted:     Color::from_rgb(140, 132, 118),    // #8C8476
            text_gold:      Color::from_rgb(212, 168, 67),     // #D4A843 - dorado imperial
            text_jade:      Color::from_rgb(120, 175, 120),    // #78AF78 - jade

            // ── Acentos imperiales ──
            vermillion:    Color::from_rgb(194, 59, 34),       // #C23B22 - rojo vermillon
            imperial_gold: Color::from_rgb(212, 168, 67),      // #D4A843 - oro imperial
            jade_green:    Color::from_rgb(91, 140, 90),       // #5B8C5A - jade verde
            indigo:        Color::from_rgb(46, 64, 87),        // #2E4057 - indigo
            plum:          Color::from_rgb(139, 64, 73),       // #8B4049 - ciruela
            copper:        Color::from_rgb(168, 112, 62),      // #A8703E - cobre

            // ── Bordes ──
            border_primary:   Color::from_rgb(70, 63, 55),     // #463F37
            border_secondary: Color::from_rgb(55, 50, 44),     // #37322C
            border_focus:     Color::from_rgb(212, 168, 67),   // #D4A843 - oro
            border_subtle:    Color::from_rgb(38, 34, 30),     // #26221E
            border_gold:      Color::from_rgb(180, 142, 55),   // #B48E37

            // ── Nodos ──
            node_rust:       Color::from_rgb(194, 59, 34),     // vermillon para Rust
            node_rust_body:  Color::from_rgb(45, 32, 28),      // marron oscuro cálido
            node_text:       Color::from_rgb(168, 112, 62),    // cobre para texto
            node_text_body:  Color::from_rgb(38, 34, 28),      // marron profundo
            node_auto:       Color::from_rgb(91, 140, 90),     // jade para auto
            node_auto_body:  Color::from_rgb(32, 38, 30),      // verde oscuro
            node_folder:     Color::from_rgb(212, 168, 67),    // oro para carpetas
            node_folder_body:Color::from_rgb(42, 38, 28),      // dorado oscuro

            // ── Pins (perlas) ──
            pin_input:     Color::from_rgb(120, 175, 120),     // jade claro
            pin_output:    Color::from_rgb(194, 59, 34),       // vermillon
            pin_connected: Color::from_rgb(212, 168, 67),      // oro imperial
            pin_hover:     Color::from_rgb(240, 237, 229),     // porcelana

            // ── Terminal ──
            terminal_bg:      Color::from_rgb(18, 16, 14),     // tinta pura
            terminal_text:    Color::from_rgb(210, 200, 180),   // seda clara
            terminal_selection: Color::from_rgb(91, 140, 90),   // jade

            // ── Grid ──
            grid_line: Color::from_rgb(42, 38, 33),            // #2A2621
            grid_axis: Color::from_rgb(70, 63, 55),            // #463F37
            grid_dot:  Color::from_rgb(55, 50, 44),            // #37322C

            // ── Conexiones ──
            link_default: Color::from_rgb(168, 112, 62),       // cobre
            link_active:  Color::from_rgb(194, 59, 34),        // vermillon
            link_hover:   Color::from_rgb(212, 168, 67),       // oro
        }
    }
}

impl UltraOmegaTheme {
    pub fn node_language_color(&self, language: &crate::core::node_graph::NodeLanguage) -> Color {
        use crate::core::node_graph::NodeLanguage;
        match language {
            NodeLanguage::Rust => self.vermillion,
            NodeLanguage::Text => self.copper,
            NodeLanguage::Auto => self.jade_green,
        }
    }
}

pub static THEME: std::sync::LazyLock<UltraOmegaTheme> = std::sync::LazyLock::new(UltraOmegaTheme::default);
