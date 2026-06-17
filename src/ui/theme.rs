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
            // ── Fondos cálidos estilo Houdini ──
            ink_black:      Color::from_rgb(26, 23, 21),      // #1A1715 - car温暖 oscuro
            ink_deep:       Color::from_rgb(34, 30, 27),      // #221E1B - marrón cálido
            ink_medium:     Color::from_rgb(48, 43, 38),      // #302B26 - marrón medio
            porcelain:      Color::from_rgb(228, 222, 210),    // #E4DED2 - porcelana cálida
            porcelain_warm: Color::from_rgb(218, 210, 196),    // #DAD2C4
            silk_cream:     Color::from_rgb(200, 192, 175),    // #C8C0AF

            // ── Superficies ──
            jade_dark:   Color::from_rgb(38, 42, 36),          // #262A24
            jade_medium: Color::from_rgb(52, 56, 48),          // #343830
            jade_light:  Color::from_rgb(68, 74, 62),          // #444A3E
            obsidian:    Color::from_rgb(30, 27, 25),          // #1E1B19
            slate:       Color::from_rgb(52, 47, 42),          // #342F2A

            // ── Texto (legible y cálido) ──
            text_primary:   Color::from_rgb(225, 218, 204),    // #E1DACC - marfil cálido
            text_secondary: Color::from_rgb(185, 175, 158),    // #B9AF9E
            text_muted:     Color::from_rgb(130, 122, 108),    // #827A6C
            text_gold:      Color::from_rgb(218, 172, 68),     // #DAAC44 - dorado imperial
            text_jade:      Color::from_rgb(125, 170, 115),    // #7DAA73 - jade

            // ── Acentos imperiales (rojo y dorado) ──
            vermillion:    Color::from_rgb(185, 55, 32),       // #B93720 - rojo vermillón profundo
            imperial_gold: Color::from_rgb(205, 165, 62),      // #CDA53E - oro imperial
            jade_green:    Color::from_rgb(85, 135, 82),       // #558752 - jade verde
            indigo:        Color::from_rgb(42, 58, 78),        // #2A3A4E - indigo profundo
            plum:          Color::from_rgb(130, 58, 65),       // #823A41 - ciruela
            copper:        Color::from_rgb(160, 108, 58),      // #A06C3A - cobre

            // ── Bordes ──
            border_primary:   Color::from_rgb(65, 58, 50),     // #413A32
            border_secondary: Color::from_rgb(50, 45, 40),     // #322D28
            border_focus:     Color::from_rgb(205, 165, 62),   // #CDA53E - oro
            border_subtle:    Color::from_rgb(36, 32, 28),     // #24201C
            border_gold:      Color::from_rgb(170, 135, 50),   // #AA8732

            // ── Nodos (mejor contraste para texto) ──
            node_rust:       Color::from_rgb(185, 55, 32),     // vermillón profundo
            node_rust_body:  Color::from_rgb(55, 42, 36),      // marrón cálido claro (legible)
            node_text:       Color::from_rgb(160, 108, 58),    // cobre
            node_text_body:  Color::from_rgb(48, 40, 34),      // marrón medio
            node_auto:       Color::from_rgb(85, 135, 82),     // jade
            node_auto_body:  Color::from_rgb(40, 48, 36),      // verde cálido oscuro
            node_folder:     Color::from_rgb(205, 165, 62),    // oro
            node_folder_body:Color::from_rgb(50, 45, 34),      // dorado oscuro

            // ── Pins (perlas) ──
            pin_input:     Color::from_rgb(125, 170, 115),     // jade claro
            pin_output:    Color::from_rgb(185, 55, 32),       // vermillón
            pin_connected: Color::from_rgb(205, 165, 62),      // oro imperial
            pin_hover:     Color::from_rgb(228, 222, 210),     // porcelana

            // ── Terminal ──
            terminal_bg:      Color::from_rgb(22, 20, 18),     // car温暖 oscuro
            terminal_text:    Color::from_rgb(200, 192, 175),   // seda clara
            terminal_selection: Color::from_rgb(85, 135, 82),   // jade

            // ── Grid (sutil y cálido) ──
            grid_line: Color::from_rgb(40, 36, 31),            // #28241F
            grid_axis: Color::from_rgb(65, 58, 50),            // #413A32
            grid_dot:  Color::from_rgb(50, 45, 40),            // #322D28

            // ── Conexiones ──
            link_default: Color::from_rgb(160, 108, 58),       // cobre
            link_active:  Color::from_rgb(185, 55, 32),        // vermillón
            link_hover:   Color::from_rgb(205, 165, 62),       // oro
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
