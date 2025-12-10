use eframe::egui::{self, Align2, Color32, FontId, Painter, Pos2, Rect, Stroke, Vec2, Visuals};
use eframe::egui::epaint::{RectShape, Shape, TextureId};
use crate::core::node_graph::{Node, Pin};

/// Verificar si un nodo es un nodo carpeta
fn is_folder_node(node: &Node) -> bool {
    node.title.starts_with("📁 ") && node.subnetwork_graph.is_some()
}

// Shared constants should ideally be in a config, but duplicating for now to keep UI self-contained
// or we can update node_graph to export them.
pub const HEADER_HEIGHT: f32 = 36.0;
pub const PIN_SPACING: f32 = 26.0;
pub const NODE_WIDTH: f32 = 240.0; // Slightly wider for "Professional" look
pub const CONTENT_PADDING: f32 = 14.0;
const PIN_RADIUS: f32 = 7.0; // Más grande para mejor visibilidad
const PIN_TEXT_GAP: f32 = 10.0;

pub fn draw_node(
    painter: &Painter,
    node: &Node,
    rect: Rect, // Pre-calculated screen rect
    zoom: f32,
    selected: bool,
    is_inherited: bool,
    _visuals: &Visuals,
    connected_pins: &std::collections::HashSet<crate::core::node_graph::PinId>, // Pins que están conectados
) {
    let rounding = egui::Rounding::same(12.0 * zoom);
    let is_folder = is_folder_node(node);

    // ═══════════════════════════════════════════════════════════════════
    // 🆕 RENDERIZADO ESPECIAL PARA NODOS CARPETA
    // ═══════════════════════════════════════════════════════════════════
    if is_folder {
        draw_folder_node_custom(painter, node, rect, zoom, selected, is_inherited, connected_pins);
        return;
    }

    // 0. Selection Highlight (Outer Glow)
    if selected {
        let glow_rect = rect.expand(4.0 * zoom);
        painter.rect_stroke(
            glow_rect,
            rounding,
            Stroke::new(2.0 * zoom, Color32::from_rgb(255, 200, 50)), // Golden highlight
        );
    }

    // 0.5. Inheritance Highlight (Green Glow)
    if is_inherited {
        let glow_rect = rect.expand(6.0 * zoom);
        painter.rect_stroke(
            glow_rect,
            rounding,
            Stroke::new(3.0 * zoom, Color32::from_rgb(89, 185, 89)), // Green highlight for inheritance
        );
    }

    // 1. Node Shadow (Soft, professional)
    // painter.rect_shadow is not a standard egui painter method in this version, using add(Shape) with shadow
    let shadow = eframe::egui::epaint::Shadow {
        offset: Vec2::ZERO,
        blur: 12.0 * zoom,
        spread: 4.0 * zoom,
        color: Color32::from_black_alpha(80),
    };
    painter.add(shadow.tessellate(rect, rounding));

    // 2. Header
    let header_height = HEADER_HEIGHT * zoom;
    let header_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_height));
    let header_rounding = egui::Rounding {
        nw: rounding.nw,
        ne: rounding.ne,
        sw: 0.0,
        se: 0.0,
    };

    // Header Background with slight gradient simulation (top lighter)
    painter.add(Shape::Rect(RectShape {
        rect: header_rect,
        rounding: header_rounding,
        fill: node.color,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));
    
    // Header Gloss/Highlight (Top line)
    painter.line_segment(
        [header_rect.min + Vec2::new(0.0, 1.0), header_rect.right_top() + Vec2::new(0.0, 1.0)],
        Stroke::new(1.0 * zoom, Color32::WHITE.gamma_multiply(0.3)),
    );

    // 3. Body Background (Dark, matte)
    let body_rect = Rect::from_min_max(
        Pos2::new(rect.min.x, rect.min.y + header_height),
        rect.max,
    );
    let body_rounding = egui::Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: rounding.sw,
        se: rounding.se,
    };
    
    let body_color = Color32::from_rgb(20, 20, 25).gamma_multiply(0.95);
    painter.add(Shape::Rect(RectShape {
        rect: body_rect,
        rounding: body_rounding,
        fill: body_color,
        stroke: Stroke::new(1.0 * zoom, Color32::from_gray(40)), // Subtle border
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));

    // 4. Title Text
    let text_zoom = zoom.clamp(0.5, 1.25);
    let title_font = FontId::proportional(18.0 * text_zoom);
    
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 INDICADOR VISUAL PARA SUBNETWORKS
    // ═══════════════════════════════════════════════════════════════════
    let is_subnetwork = node.subnetwork_graph.is_some();
    let title_with_icon = if is_subnetwork {
        format!("📁 {}", node.title)
    } else {
        node.title.clone()
    };
    
    // Shadow for text
    painter.text(
        header_rect.left_center() + Vec2::new(13.0 * zoom, 1.0 * zoom),
        Align2::LEFT_CENTER,
        &title_with_icon,
        title_font.clone(),
        Color32::BLACK.gamma_multiply(0.5),
    );
    painter.text(
        header_rect.left_center() + Vec2::new(12.0 * zoom, 0.0),
        Align2::LEFT_CENTER,
        &title_with_icon,
        title_font,
        Color32::WHITE,
    );

    // 5. Pins
    let pin_font = FontId::proportional(14.0 * text_zoom);
    
    for (i, pin) in node.inputs.iter().enumerate() {
        let y = rect.min.y + header_height + PIN_SPACING * zoom * (i as f32 + 0.5);
        let pos = Pos2::new(rect.min.x + CONTENT_PADDING * zoom, y);
        let is_connected = connected_pins.contains(&pin.id);
        draw_pin(painter, pin, pos, Align2::LEFT_CENTER, 1.0, zoom, &pin_font, is_connected);
    }

    for (i, pin) in node.outputs.iter().enumerate() {
        let y = rect.min.y + header_height + PIN_SPACING * zoom * (i as f32 + 0.5);
        let pos = Pos2::new(rect.max.x - CONTENT_PADDING * zoom, y);
        let is_connected = connected_pins.contains(&pin.id);
        draw_pin(painter, pin, pos, Align2::RIGHT_CENTER, -1.0, zoom, &pin_font, is_connected);
    }
}

/// Renderizado personalizado para Nodos Carpeta - Estilo Windows 11 Elegante
fn draw_folder_node_custom(
    painter: &Painter,
    node: &Node,
    rect: Rect,
    zoom: f32,
    selected: bool,
    _is_inherited: bool,
    _connected_pins: &std::collections::HashSet<crate::core::node_graph::PinId>,
) {
    // Bordes más redondeados estilo Windows 11
    let rounding = egui::Rounding::same(16.0 * zoom);
    
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 DETECTAR SI ES CARPETA HEREDABLE
    // ═══════════════════════════════════════════════════════════════════
    let is_inheritable = node.title.contains("(Heredable)");
    
    // ═══════════════════════════════════════════════════════════════════
    // 1. SOMBRA PROFESIONAL (Estilo Windows 11 - suave y elegante)
    // ═══════════════════════════════════════════════════════════════════
    let shadow_color = if is_inheritable {
        Color32::from_rgba_unmultiplied(150, 100, 255, 60) // Sombra púrpura para heredables
    } else {
        Color32::from_black_alpha(50) // Sombra normal
    };
    
    let shadow = eframe::egui::epaint::Shadow {
        offset: Vec2::new(0.0, 4.0 * zoom),
        blur: 20.0 * zoom,
        spread: 0.0,
        color: shadow_color,
    };
    painter.add(shadow.tessellate(rect, rounding));
    
    // ═══════════════════════════════════════════════════════════════════
    // 2. GLOW SUTIL (Solo cuando está seleccionado, más profesional)
    // ═══════════════════════════════════════════════════════════════════
    if selected {
        if is_inheritable {
            // Glow púrpura/magenta para carpetas heredables
            painter.rect_filled(
                rect.expand(6.0 * zoom),
                rounding,
                Color32::from_rgba_unmultiplied(180, 120, 255, 25),
            );
            painter.rect_filled(
                rect.expand(3.0 * zoom),
                rounding,
                Color32::from_rgba_unmultiplied(200, 140, 255, 40),
            );
        } else {
            // Glow dorado para carpetas normales
            painter.rect_filled(
                rect.expand(6.0 * zoom),
                rounding,
                Color32::from_rgba_unmultiplied(255, 215, 100, 20),
            );
            painter.rect_filled(
                rect.expand(3.0 * zoom),
                rounding,
                Color32::from_rgba_unmultiplied(255, 215, 100, 35),
            );
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════
    // 3. BORDE SUAVE Y ELEGANTE (Estilo Windows 11)
    // ═══════════════════════════════════════════════════════════════════
    let border_color = if is_inheritable {
        // Bordes púrpura/magenta únicos para carpetas heredables
        if selected {
            Color32::from_rgb(220, 160, 255) // Púrpura brillante cuando seleccionado
        } else {
            Color32::from_rgb(180, 130, 240) // Púrpura suave base
        }
    } else {
        // Bordes dorados para carpetas normales
        if selected {
            Color32::from_rgb(255, 220, 120) // Dorado suave cuando seleccionado
        } else {
            Color32::from_rgb(240, 190, 90) // Dorado más sutil
        }
    };
    
    // Borde delgado y elegante (no grueso)
    painter.rect_stroke(
        rect,
        rounding,
        Stroke::new(1.5 * zoom, border_color),
    );
    
    // ═══════════════════════════════════════════════════════════════════
    // 4. HEADER CON GRADIENTE SUAVE (Estilo Windows 11)
    // ═══════════════════════════════════════════════════════════════════
    let header_height = HEADER_HEIGHT * zoom;
    let header_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_height));
    let header_rounding = egui::Rounding {
        nw: rounding.nw,
        ne: rounding.ne,
        sw: 0.0,
        se: 0.0,
    };
    
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 COLORES ÚNICOS PARA CARPETAS HEREDABLES
    // ═══════════════════════════════════════════════════════════════════
    let (header_bg_base, header_bg_top) = if is_inheritable {
        // Esquema de colores púrpura/magenta único para heredables
        if selected {
            (
                Color32::from_rgb(200, 150, 255), // Púrpura brillante base
                Color32::from_rgb(230, 180, 255), // Púrpura claro superior
            )
        } else {
            (
                Color32::from_rgb(180, 130, 240), // Púrpura base suave
                Color32::from_rgb(210, 160, 250), // Púrpura claro superior
            )
        }
    } else {
        // Esquema de colores dorado para carpetas normales
        if selected {
            (
                Color32::from_rgb(250, 210, 110), // Dorado suave cuando seleccionado
                Color32::from_rgb(255, 230, 140), // Gradiente superior más claro
            )
        } else {
            (
                Color32::from_rgb(240, 195, 95), // Dorado base más sutil
                Color32::from_rgb(250, 215, 115), // Gradiente superior suave
            )
        }
    };
    
    // Base del header
    painter.add(Shape::Rect(RectShape {
        rect: header_rect,
        rounding: header_rounding,
        fill: header_bg_base,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));
    
    // Gradiente superior suave (más sutil que antes)
    let gradient_height = header_height * 0.35;
    let gradient_rect = Rect::from_min_size(
        header_rect.min,
        Vec2::new(header_rect.width(), gradient_height)
    );
    painter.add(Shape::Rect(RectShape {
        rect: gradient_rect,
        rounding: header_rounding,
        fill: header_bg_top,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));
    
    // Línea de brillo superior muy sutil
    painter.line_segment(
        [
            Pos2::new(header_rect.min.x + rounding.nw + 2.0 * zoom, header_rect.min.y + 1.0 * zoom),
            Pos2::new(header_rect.max.x - rounding.ne - 2.0 * zoom, header_rect.min.y + 1.0 * zoom),
        ],
        Stroke::new(1.0 * zoom, Color32::from_white_alpha(60)),
    );
    
    // Separador sutil entre header y body
    painter.line_segment(
        [
            Pos2::new(header_rect.min.x, header_rect.max.y - 0.5 * zoom),
            Pos2::new(header_rect.max.x, header_rect.max.y - 0.5 * zoom),
        ],
        Stroke::new(1.0 * zoom, Color32::from_black_alpha(40)),
    );
    
    // ═══════════════════════════════════════════════════════════════════
    // 5. ICONO Y TÍTULO ELEGANTES (Estilo Windows 11 - un solo icono centrado)
    // ═══════════════════════════════════════════════════════════════════
    let text_zoom = zoom.clamp(0.5, 1.25);
    let title_font = FontId::proportional(16.0 * text_zoom);
    
    // Extraer título sin el emoji 📁 y mostrar el lenguaje si es heredable
    let mut clean_title = node.title.strip_prefix("📁 ").unwrap_or(&node.title).to_string();
    
    // Si es heredable y tiene lenguaje específico, asegurar que se muestre en el título
    if is_inheritable && !matches!(node.language, crate::core::node_graph::NodeLanguage::Auto | crate::core::node_graph::NodeLanguage::Text) {
        let lang_name = match node.language {
            crate::core::node_graph::NodeLanguage::Rust => "Rust",
            crate::core::node_graph::NodeLanguage::C => "C",
            crate::core::node_graph::NodeLanguage::Cpp => "C++",
            crate::core::node_graph::NodeLanguage::Python => "Python",
            crate::core::node_graph::NodeLanguage::Java => "Java",
            crate::core::node_graph::NodeLanguage::Zig => "Zig",
            crate::core::node_graph::NodeLanguage::Asm => "Assembly",
            _ => "",
        };
        
        if !lang_name.is_empty() && !clean_title.contains(&format!("[{}]", lang_name)) {
            // Agregar el lenguaje al título si no está ya presente
            if clean_title.contains("(Heredable)") {
                clean_title = clean_title.replace("(Heredable)", &format!("(Heredable) [{}]", lang_name));
            } else {
                clean_title = format!("{} [{}]", clean_title, lang_name);
            }
        }
    }
    
    // Un solo icono más grande y centrado (estilo Windows 11)
    let icon_size = 24.0 * zoom;
    let icon_y = header_rect.center().y;
    let icon_x = header_rect.min.x + 16.0 * zoom;
    
    // ═══════════════════════════════════════════════════════════════════
    // 🆕 COLORES DE TEXTO ÚNICOS PARA HEREDABLES
    // ═══════════════════════════════════════════════════════════════════
    let (icon_color, text_color) = if is_inheritable {
        // Colores oscuros púrpura para contraste con fondo púrpura
        (
            Color32::from_rgb(80, 50, 120), // Púrpura oscuro para icono
            Color32::from_rgb(90, 60, 130), // Púrpura oscuro para texto
        )
    } else {
        // Colores marrón oscuro para carpetas normales
        (
            Color32::from_rgb(60, 45, 20), // Marrón oscuro elegante
            Color32::from_rgb(70, 50, 25), // Marrón oscuro para texto
        )
    };
    
    // Sombra sutil del icono
    painter.text(
        Pos2::new(icon_x + 0.5 * zoom, icon_y + 0.5 * zoom),
        Align2::LEFT_CENTER,
        "📁",
        FontId::proportional(icon_size),
        Color32::from_rgba_unmultiplied(0, 0, 0, 40),
    );
    
    // Icono principal
    painter.text(
        Pos2::new(icon_x, icon_y),
        Align2::LEFT_CENTER,
        "📁",
        FontId::proportional(icon_size),
        icon_color,
    );
    
    // Título con sombra sutil (usar referencia para evitar move)
    let title_x = icon_x + icon_size + 12.0 * zoom;
    let clean_title_ref = &clean_title;
    painter.text(
        Pos2::new(title_x + 0.5 * zoom, icon_y + 0.5 * zoom),
        Align2::LEFT_CENTER,
        clean_title_ref,
        title_font.clone(),
        Color32::from_rgba_unmultiplied(0, 0, 0, 50),
    );
    painter.text(
        Pos2::new(title_x, icon_y),
        Align2::LEFT_CENTER,
        clean_title_ref,
        title_font,
        text_color,
    );
    
    // ═══════════════════════════════════════════════════════════════════
    // 6. BODY ELEGANTE (Fondo suave y profesional)
    // ═══════════════════════════════════════════════════════════════════
    let body_rect = Rect::from_min_max(
        Pos2::new(rect.min.x, rect.min.y + header_height),
        rect.max,
    );
    let body_rounding = egui::Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: rounding.sw,
        se: rounding.se,
    };
    
    // Fondo más claro y elegante (no tan oscuro)
    let body_color = Color32::from_rgb(28, 28, 35);
    painter.add(Shape::Rect(RectShape {
        rect: body_rect,
        rounding: body_rounding,
        fill: body_color,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));
    
    // Efecto de profundidad muy sutil
    let depth_shadow = Rect::from_min_size(
        body_rect.min,
        Vec2::new(body_rect.width(), 3.0 * zoom)
    );
    painter.add(Shape::Rect(RectShape {
        rect: depth_shadow,
        rounding: body_rounding,
        fill: Color32::from_black_alpha(30),
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));
    
    // Indicador visual minimalista
    if let Some(_subgraph) = &node.subnetwork_graph {
        let indicator_text = "Contenedor de nodos";
        let indicator_font = FontId::proportional(11.0 * text_zoom);
        
        // Texto con sombra sutil
        painter.text(
            body_rect.center() + Vec2::new(0.5, 0.5) * zoom,
            Align2::CENTER_CENTER,
            indicator_text,
            indicator_font.clone(),
            Color32::from_black_alpha(100),
        );
        
        // Texto principal más claro
        painter.text(
            body_rect.center(),
            Align2::CENTER_CENTER,
            indicator_text,
            indicator_font,
            Color32::from_rgb(140, 140, 155),
        );
    }
}

fn draw_pin(
    painter: &Painter,
    pin: &Pin,
    center: Pos2,
    align: Align2,
    direction: f32,
    zoom: f32,
    font: &FontId,
    is_connected: bool,
) {
    let radius = PIN_RADIUS * zoom;
    
    // Área de hover más grande (invisible pero ayuda con el hit test visual)
    let hover_radius = radius * 2.5;
    painter.circle_filled(center, hover_radius, Color32::from_black_alpha(0));
    
    if is_connected {
        // PIN CONECTADO: Efecto de llenado con neón
        // Glow exterior animado
        painter.circle_filled(center, radius * 2.0, Color32::from_rgba_unmultiplied(100, 200, 255, 40));
        painter.circle_filled(center, radius * 1.6, Color32::from_rgba_unmultiplied(100, 200, 255, 80));
        
        // Borde exterior brillante
        painter.circle_stroke(center, radius * 1.4, Stroke::new(2.0 * zoom, Color32::from_rgb(100, 200, 255)));
        
        // Pin lleno con color del conector
        painter.circle_filled(center, radius * 1.1, Color32::from_rgb(100, 200, 255));
        
        // Highlight central brillante
        painter.circle_filled(center, radius * 0.5, Color32::from_rgb(150, 220, 255));
        
        // Borde interno para definición
        painter.circle_stroke(center, radius * 1.1, Stroke::new(1.5 * zoom, Color32::from_rgb(150, 220, 255)));
    } else {
        // PIN DESCONECTADO: Diseño mejorado con más personalidad
        // Glow sutil exterior
        painter.circle_filled(center, radius * 1.8, Color32::from_rgba_unmultiplied(150, 150, 150, 20));
        
        // Anillo exterior elegante
        painter.circle_stroke(center, radius * 1.4, Stroke::new(1.5 * zoom, Color32::from_rgb(120, 120, 120)));
        
        // Pin Rim principal (más grueso y visible)
        painter.circle_stroke(center, radius, Stroke::new(3.0 * zoom, Color32::from_rgb(180, 180, 180)));
        
        // Pin Hole (centro oscuro con profundidad)
        painter.circle_filled(center, radius * 0.65, Color32::from_rgb(25, 25, 30));
        
        // Highlight sutil en el borde superior
        let highlight_pos = center + Vec2::new(0.0, -radius * 0.3);
        painter.circle_filled(highlight_pos, radius * 0.3, Color32::from_rgba_unmultiplied(255, 255, 255, 60));
        
        // Borde interno para definición
        painter.circle_stroke(center, radius * 0.65, Stroke::new(1.0 * zoom, Color32::from_rgb(60, 60, 60)));
    }

    // Label con mejor contraste
    let text_pos = center + Vec2::new(direction * (radius * 1.8 + PIN_TEXT_GAP * zoom), 0.0);
    let label_color = if is_connected {
        Color32::from_rgb(150, 220, 255) // Color más brillante cuando está conectado
    } else {
        Color32::from_rgb(212, 212, 212)
    };
    painter.text(
        text_pos,
        align,
        &pin.label,
        font.clone(),
        label_color,
    );
}

