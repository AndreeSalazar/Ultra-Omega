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
            // ── Fondos cálidos estilo Houdini (no azulados) ──
            ink_black:      Color::from_rgb(22, 19, 17),       // #161311 - tinta cálida
            ink_deep:       Color::from_rgb(30, 26, 23),       // #1E1A17 - laca oscura
            ink_medium:     Color::from_rgb(42, 37, 32),       // #2A2520 - piedra cálida
            porcelain:      Color::from_rgb(226, 218, 199),    // #E2DAC7 - porcelana cálida
            porcelain_warm: Color::from_rgb(214, 203, 181),    // #D6CBB5 - arroz y seda
            silk_cream:     Color::from_rgb(194, 181, 153),    // #C2B599 - seda envejecida

            // ── Superficies de jade, obsidiana y pizarra ──
            jade_dark:   Color::from_rgb(22, 38, 32),          // #162620 - jade oscuro cálido
            jade_medium: Color::from_rgb(37, 72, 61),          // #25483D - jade pulido
            jade_light:  Color::from_rgb(78, 125, 96),         // #4E7D60 - jade imperial
            obsidian:    Color::from_rgb(24, 21, 19),          // #181513 - obsidiana cálida
            slate:       Color::from_rgb(38, 34, 29),          // #26221D - pizarra cálida

            // ── Texto legible sobre superficies oscuras ──
            text_primary:   Color::from_rgb(232, 223, 205),    // #E8DFCD - marfil
            text_secondary: Color::from_rgb(190, 177, 151),    // #BEB197 - seda tenue
            text_muted:     Color::from_rgb(124, 115, 99),     // #7C7363 - pincel seco
            text_gold:      Color::from_rgb(224, 178, 65),     // #E0B241 - dorado imperial
            text_jade:      Color::from_rgb(128, 184, 144),    // #80B890 - jade claro

            // ── Acentos culturales: rojo chino profundo, oro, jade ──
            vermillion:    Color::from_rgb(118, 38, 28),       // #76261C - rojo chino oscuro elegante
            imperial_gold: Color::from_rgb(178, 142, 52),      // #B28E34 - oro apagado
            jade_green:    Color::from_rgb(62, 110, 78),       // #3E6E4E - jade oscuro
            indigo:        Color::from_rgb(34, 52, 80),        // #223450 - añil nocturno
            plum:          Color::from_rgb(96, 42, 62),        // #602A3E - ciruela oscura
            copper:        Color::from_rgb(120, 75, 38),       // #784B26 - cobre antiguo apagado

            // ── Bordes tipo trazo de pincel ──
            border_primary:   Color::from_rgb(62, 55, 45),     // #3E372D
            border_secondary: Color::from_rgb(40, 36, 32),     // #282420
            border_focus:     Color::from_rgb(218, 171, 54),   // #DAAB36 - oro
            border_subtle:    Color::from_rgb(28, 25, 22),     // #1C1916
            border_gold:      Color::from_rgb(175, 132, 40),   // #AF8428

            // ── Nodos: cuerpos muy oscuros y elegantes, headers apagados ──
            node_rust:       Color::from_rgb(118, 38, 28),     // rojo chino oscuro
            node_rust_body:  Color::from_rgb(24, 20, 19),      // laca muy oscura
            node_text:       Color::from_rgb(120, 75, 38),     // cobre apagado
            node_text_body:  Color::from_rgb(22, 20, 18),      // arcilla profunda
            node_auto:       Color::from_rgb(62, 110, 78),     // jade oscuro
            node_auto_body:  Color::from_rgb(18, 26, 22),      // jade nocturno oscuro
            node_folder:     Color::from_rgb(178, 142, 52),    // oro apagado
            node_folder_body:Color::from_rgb(26, 23, 16),      // brocado profundo

            // ── Pins como perlas lacadas ──
            pin_input:     Color::from_rgb(128, 184, 144),     // jade claro
            pin_output:    Color::from_rgb(155, 30, 20),       // rojo chino profundo
            pin_connected: Color::from_rgb(218, 171, 54),      // oro imperial
            pin_hover:     Color::from_rgb(226, 218, 199),     // porcelana

            // ── Terminal ──
            terminal_bg:      Color::from_rgb(18, 16, 14),     // tinta cálida oscura
            terminal_text:    Color::from_rgb(194, 181, 153),  // seda clara
            terminal_selection: Color::from_rgb(74, 150, 105), // jade

            // ── Grid sutil de papel de caligrafía cálido ──
            grid_line: Color::from_rgb(30, 27, 23),            // #1E1B17
            grid_axis: Color::from_rgb(52, 46, 38),            // #342E26
            grid_dot:  Color::from_rgb(42, 38, 33),            // #2A2621

            // ── Conexiones tipo tinta antigua (cobre apagado) ──
            link_default: Color::from_rgb(120, 75, 38),       // cobre antiguo
            link_active:  Color::from_rgb(118, 38, 28),        // rojo chino oscuro
            link_hover:   Color::from_rgb(178, 142, 52),       // oro apagado
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
