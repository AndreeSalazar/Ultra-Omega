// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Nodos para Mapa Semántico
// Diseño vertical mejorado: Entrada arriba ↑ → Salida abajo ↓
// Estilo profesional y cómodo para trabajo complejo
// ═══════════════════════════════════════════════════════════════════════════════

use eframe::egui::{self, Align2, Color32, FontId, Painter, Pos2, Rect, Stroke, Vec2, Visuals};
use eframe::egui::epaint::{RectShape, Shape, TextureId};
use crate::core::node_graph::{Node, Pin, PinId};
use std::collections::HashSet;

// ══════════════════════════════════════════
// CONSTANTES MEJORADAS PARA NODOS SEMÁNTICOS
// ══════════════════════════════════════════

/// Altura del header del nodo (más espacioso)
pub const HEADER_HEIGHT: f32 = 38.0;

/// Ancho del nodo semántico (balanceado)
pub const NODE_WIDTH: f32 = 180.0;

/// Altura mínima del body (más cómodo)
pub const BODY_HEIGHT: f32 = 32.0;

/// Radio del pin (más grande y visible)
pub const PIN_RADIUS: f32 = 9.0;

/// Padding del contenido
pub const CONTENT_PADDING: f32 = 16.0;

/// Offset vertical de los pins desde el borde del nodo
pub const PIN_OFFSET: f32 = 0.0;

/// Rounding de las esquinas
pub const NODE_ROUNDING: f32 = 14.0;

// ══════════════════════════════════════════
// COLORES PERSONALIZADOS
// ══════════════════════════════════════════

/// Color de fondo del body (oscuro profesional)
const BODY_BG_COLOR: Color32 = Color32::from_rgb(22, 22, 28);

/// Color del borde del body
const BODY_BORDER_COLOR: Color32 = Color32::from_rgb(45, 45, 55);

/// Color de texto principal
const TEXT_PRIMARY: Color32 = Color32::from_rgb(240, 240, 245);

/// Color de texto secundario (labels)
const TEXT_SECONDARY: Color32 = Color32::from_rgb(180, 180, 190);

/// Color de pin conectado
const PIN_CONNECTED_COLOR: Color32 = Color32::from_rgb(80, 200, 255);

/// Color de pin conectado (highlight)
const PIN_CONNECTED_HIGHLIGHT: Color32 = Color32::from_rgb(140, 220, 255);

/// Color de pin desconectado
const PIN_DISCONNECTED_COLOR: Color32 = Color32::from_rgb(140, 140, 150);

/// Color del hueco del pin
const PIN_HOLE_COLOR: Color32 = Color32::from_rgb(25, 25, 32);

// ══════════════════════════════════════════
// CÁLCULO DE DIMENSIONES
// ══════════════════════════════════════════

/// Calcula el rectángulo del nodo en modo semántico
pub fn calculate_node_rect(node: &Node, zoom: f32) -> Rect {
    let width = NODE_WIDTH * zoom;
    let header = HEADER_HEIGHT * zoom;
    let body = BODY_HEIGHT * zoom;
    
    let height = header + body;
    
    Rect::from_min_size(
        node.position,
        Vec2::new(width, height),
    )
}

/// Obtiene la posición del pin de entrada (arriba del nodo)
pub fn get_input_pin_position(node_rect: Rect, pin_index: usize, total_pins: usize, zoom: f32) -> Pos2 {
    if total_pins == 0 { return node_rect.center_top(); }
    
    let usable_width = node_rect.width() - CONTENT_PADDING * 2.0 * zoom;
    
    let spacing = if total_pins > 1 {
        usable_width / (total_pins as f32)
    } else {
        0.0
    };
    
    let start_x = if total_pins > 1 {
        node_rect.min.x + CONTENT_PADDING * zoom + spacing / 2.0
    } else {
        node_rect.center().x
    };
    
    Pos2::new(
        start_x + (pin_index as f32) * spacing,
        node_rect.min.y - PIN_OFFSET * zoom,
    )
}

/// Obtiene la posición del pin de salida (abajo del nodo)
pub fn get_output_pin_position(node_rect: Rect, pin_index: usize, total_pins: usize, zoom: f32) -> Pos2 {
    if total_pins == 0 { return node_rect.center_bottom(); }
    
    let usable_width = node_rect.width() - CONTENT_PADDING * 2.0 * zoom;
    
    let spacing = if total_pins > 1 {
        usable_width / (total_pins as f32)
    } else {
        0.0
    };
    
    let start_x = if total_pins > 1 {
        node_rect.min.x + CONTENT_PADDING * zoom + spacing / 2.0
    } else {
        node_rect.center().x
    };
    
    Pos2::new(
        start_x + (pin_index as f32) * spacing,
        node_rect.max.y + PIN_OFFSET * zoom,
    )
}

// ══════════════════════════════════════════
// RENDERIZADO DE NODO SEMÁNTICO MEJORADO
// ══════════════════════════════════════════

/// Dibuja un nodo en estilo mapa semántico (vertical) - VERSIÓN MEJORADA
pub fn draw_semantic_node(
    painter: &Painter,
    node: &Node,
    rect: Rect,
    zoom: f32,
    selected: bool,
    is_inherited: bool,
    _visuals: &Visuals,
    connected_pins: &HashSet<PinId>,
) {
    let rounding = egui::Rounding::same(NODE_ROUNDING * zoom);
    let text_zoom = zoom.clamp(0.7, 1.3);

    // ═══════════════════════════════════════════════════════════════════
    // 0. GLOW DE SELECCIÓN / HERENCIA (Efecto exterior suave)
    // ═══════════════════════════════════════════════════════════════════
    
    if selected {
        // Glow dorado suave con múltiples capas
        let glow_color_1 = Color32::from_rgba_unmultiplied(255, 200, 50, 15);
        let glow_color_2 = Color32::from_rgba_unmultiplied(255, 200, 50, 30);
        let glow_color_3 = Color32::from_rgba_unmultiplied(255, 200, 50, 50);
        
        painter.rect_filled(rect.expand(12.0 * zoom), rounding, glow_color_1);
        painter.rect_filled(rect.expand(8.0 * zoom), rounding, glow_color_2);
        painter.rect_filled(rect.expand(4.0 * zoom), rounding, glow_color_3);
        
        // Borde de selección nítido
        painter.rect_stroke(
            rect.expand(2.0 * zoom), 
            rounding, 
            Stroke::new(2.5 * zoom, Color32::from_rgb(255, 210, 80))
        );
    }

    if is_inherited {
        // Glow verde para herencia
        let glow_color = Color32::from_rgba_unmultiplied(80, 200, 120, 40);
        painter.rect_filled(rect.expand(8.0 * zoom), rounding, glow_color);
        painter.rect_stroke(
            rect.expand(3.0 * zoom), 
            rounding, 
            Stroke::new(2.5 * zoom, Color32::from_rgb(80, 200, 120))
        );
    }

    // ═══════════════════════════════════════════════════════════════════
    // 1. SOMBRA PROFESIONAL (Múltiples capas para profundidad)
    // ═══════════════════════════════════════════════════════════════════
    
    // Sombra difusa grande
    let shadow_large = eframe::egui::epaint::Shadow {
        offset: Vec2::new(0.0, 4.0 * zoom),
        blur: 16.0 * zoom,
        spread: 0.0,
        color: Color32::from_black_alpha(60),
    };
    painter.add(shadow_large.tessellate(rect, rounding));
    
    // Sombra más cercana y nítida
    let shadow_close = eframe::egui::epaint::Shadow {
        offset: Vec2::new(0.0, 2.0 * zoom),
        blur: 6.0 * zoom,
        spread: 1.0 * zoom,
        color: Color32::from_black_alpha(40),
    };
    painter.add(shadow_close.tessellate(rect, rounding));

    // ═══════════════════════════════════════════════════════════════════
    // 2. HEADER MEJORADO (Con gradiente simulado y estilo premium)
    // ═══════════════════════════════════════════════════════════════════
    
    let header_height = HEADER_HEIGHT * zoom;
    let header_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_height));
    let header_rounding = egui::Rounding {
        nw: rounding.nw,
        ne: rounding.ne,
        sw: 0.0,
        se: 0.0,
    };

    // Fondo del header con color del nodo
    painter.add(Shape::Rect(RectShape {
        rect: header_rect,
        rounding: header_rounding,
        fill: node.color,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));

    // Gradiente superior (brillo)
    let gradient_top = Rect::from_min_size(
        header_rect.min,
        Vec2::new(header_rect.width(), header_height * 0.4)
    );
    painter.add(Shape::Rect(RectShape {
        rect: gradient_top,
        rounding: header_rounding,
        fill: Color32::from_white_alpha(25),
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));

    // Línea de brillo superior elegante
    let highlight_y = header_rect.min.y + 1.5 * zoom;
    painter.line_segment(
        [
            Pos2::new(header_rect.min.x + rounding.nw + 2.0 * zoom, highlight_y),
            Pos2::new(header_rect.max.x - rounding.ne - 2.0 * zoom, highlight_y),
        ],
        Stroke::new(1.0 * zoom, Color32::from_white_alpha(80)),
    );

    // Línea de separación inferior del header
    let separator_y = header_rect.max.y - 0.5 * zoom;
    painter.line_segment(
        [
            Pos2::new(header_rect.min.x, separator_y),
            Pos2::new(header_rect.max.x, separator_y),
        ],
        Stroke::new(1.0 * zoom, Color32::from_black_alpha(60)),
    );

    // ═══════════════════════════════════════════════════════════════════
    // 3. BODY MEJORADO (Fondo oscuro elegante)
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

    // Fondo del body
    painter.add(Shape::Rect(RectShape {
        rect: body_rect,
        rounding: body_rounding,
        fill: BODY_BG_COLOR,
        stroke: Stroke::NONE,
        fill_texture_id: TextureId::default(),
        uv: Rect::ZERO,
    }));

    // Borde exterior completo del nodo
    painter.rect_stroke(
        rect,
        rounding,
        Stroke::new(1.2 * zoom, BODY_BORDER_COLOR),
    );

    // ═══════════════════════════════════════════════════════════════════
    // 4. TÍTULO MEJORADO (Más legible y centrado)
    // ═══════════════════════════════════════════════════════════════════
    
    let title_font = FontId::proportional(15.0 * text_zoom);
    let title_pos = header_rect.center();
    
    // Sombra del título para contraste
    painter.text(
        title_pos + Vec2::new(1.0, 1.0) * zoom,
        Align2::CENTER_CENTER,
        &node.title,
        title_font.clone(),
        Color32::from_black_alpha(180),
    );
    
    // Título principal
    painter.text(
        title_pos,
        Align2::CENTER_CENTER,
        &node.title,
        title_font,
        TEXT_PRIMARY,
    );

    // ═══════════════════════════════════════════════════════════════════
    // 5. PINS DE ENTRADA (ARRIBA ↑) - ESTILO MEJORADO
    // ═══════════════════════════════════════════════════════════════════
    
    let pin_font = FontId::proportional(12.0 * text_zoom);
    let total_inputs = node.inputs.len();
    
    for (i, pin) in node.inputs.iter().enumerate() {
        let pos = get_input_pin_position(rect, i, total_inputs, zoom);
        let is_connected = connected_pins.contains(&pin.id);
        draw_premium_pin(painter, pin, pos, true, zoom, &pin_font, is_connected, node.color);
    }

    // ═══════════════════════════════════════════════════════════════════
    // 6. PINS DE SALIDA (ABAJO ↓) - ESTILO MEJORADO
    // ═══════════════════════════════════════════════════════════════════
    
    let total_outputs = node.outputs.len();
    
    for (i, pin) in node.outputs.iter().enumerate() {
        let pos = get_output_pin_position(rect, i, total_outputs, zoom);
        let is_connected = connected_pins.contains(&pin.id);
        draw_premium_pin(painter, pin, pos, false, zoom, &pin_font, is_connected, node.color);
    }
}

// ══════════════════════════════════════════
// RENDERIZADO DE PIN PREMIUM
// ══════════════════════════════════════════

/// Dibuja un pin con estilo premium mejorado
fn draw_premium_pin(
    painter: &Painter,
    pin: &Pin,
    center: Pos2,
    is_input: bool,
    zoom: f32,
    font: &FontId,
    is_connected: bool,
    node_color: Color32,
) {
    let radius = PIN_RADIUS * zoom;
    
    // ═══════════════════════════════════════════════════════════════════
    // PIN CONECTADO - Estilo brillante y activo
    // ═══════════════════════════════════════════════════════════════════
    
    if is_connected {
        // Glow exterior grande (efecto neón suave)
        painter.circle_filled(
            center, 
            radius * 2.5, 
            Color32::from_rgba_unmultiplied(
                PIN_CONNECTED_COLOR.r(), 
                PIN_CONNECTED_COLOR.g(), 
                PIN_CONNECTED_COLOR.b(), 
                20
            )
        );
        
        // Glow medio
        painter.circle_filled(
            center, 
            radius * 1.9, 
            Color32::from_rgba_unmultiplied(
                PIN_CONNECTED_COLOR.r(), 
                PIN_CONNECTED_COLOR.g(), 
                PIN_CONNECTED_COLOR.b(), 
                40
            )
        );
        
        // Anillo exterior brillante
        painter.circle_stroke(
            center, 
            radius * 1.4, 
            Stroke::new(2.0 * zoom, PIN_CONNECTED_COLOR)
        );
        
        // Pin principal lleno
        painter.circle_filled(center, radius * 1.1, PIN_CONNECTED_COLOR);
        
        // Highlight superior (efecto 3D)
        let highlight_pos = center + Vec2::new(0.0, -radius * 0.3);
        painter.circle_filled(
            highlight_pos, 
            radius * 0.45, 
            PIN_CONNECTED_HIGHLIGHT
        );
        
        // Borde definido
        painter.circle_stroke(
            center, 
            radius * 1.1, 
            Stroke::new(1.0 * zoom, PIN_CONNECTED_HIGHLIGHT)
        );
    } 
    // ═══════════════════════════════════════════════════════════════════
    // PIN DESCONECTADO - Estilo elegante y sutil
    // ═══════════════════════════════════════════════════════════════════
    else {
        // Glow sutil exterior
        painter.circle_filled(
            center, 
            radius * 2.0, 
            Color32::from_rgba_unmultiplied(150, 150, 160, 12)
        );
        
        // Anillo exterior decorativo
        painter.circle_stroke(
            center, 
            radius * 1.5, 
            Stroke::new(1.0 * zoom, Color32::from_rgb(70, 70, 80))
        );
        
        // Anillo principal grueso
        painter.circle_stroke(
            center, 
            radius * 1.05, 
            Stroke::new(3.5 * zoom, PIN_DISCONNECTED_COLOR)
        );
        
        // Centro oscuro (hueco del conector)
        painter.circle_filled(center, radius * 0.6, PIN_HOLE_COLOR);
        
        // Highlight superior sutil (efecto 3D)
        let highlight_offset = if is_input { -radius * 0.35 } else { -radius * 0.35 };
        painter.circle_filled(
            center + Vec2::new(0.0, highlight_offset),
            radius * 0.28,
            Color32::from_rgba_unmultiplied(255, 255, 255, 35),
        );
        
        // Borde interno del hueco
        painter.circle_stroke(
            center, 
            radius * 0.6, 
            Stroke::new(1.0 * zoom, Color32::from_rgb(40, 40, 48))
        );
    }

    // ═══════════════════════════════════════════════════════════════════
    // LABEL DEL PIN - Tipografía mejorada
    // ═══════════════════════════════════════════════════════════════════
    
    let label_distance = radius * 2.8 + 6.0 * zoom;
    let label_offset = if is_input {
        Vec2::new(0.0, -label_distance)
    } else {
        Vec2::new(0.0, label_distance)
    };
    
    let text_pos = center + label_offset;
    
    // Color del label según estado
    let label_color = if is_connected {
        PIN_CONNECTED_HIGHLIGHT
    } else {
        TEXT_SECONDARY
    };
    
    // Sombra del texto para mejor legibilidad
    painter.text(
        text_pos + Vec2::new(0.5, 0.5) * zoom,
        Align2::CENTER_CENTER,
        &pin.label,
        font.clone(),
        Color32::from_black_alpha(150),
    );
    
    // Texto principal
    painter.text(
        text_pos,
        Align2::CENTER_CENTER,
        &pin.label,
        font.clone(),
        label_color,
    );
}

// ══════════════════════════════════════════
// CONECTORES MEJORADOS PARA MAPA SEMÁNTICO
// ══════════════════════════════════════════

/// Dibuja un conector curvo vertical con estilo premium
pub fn draw_semantic_connector(
    painter: &Painter,
    from: Pos2,
    to: Pos2,
    _color: Color32, // Ignorar color personalizado, usar blanco siempre
    zoom: f32,
    is_highlighted: bool,
) {
    let thickness = if is_highlighted { 3.5 } else { 2.5 } * zoom;
    
    // Calcular puntos de control para curva Bezier suave
    let distance_y = (to.y - from.y).abs();
    let control_offset = (distance_y * 0.45).max(40.0 * zoom);
    
    let ctrl1 = Pos2::new(from.x, from.y + control_offset);
    let ctrl2 = Pos2::new(to.x, to.y - control_offset);
    
    // Color blanco para todas las conexiones sobre fondo negro
    let wire_color = Color32::from_rgb(255, 255, 255); // Blanco puro
    let glow_color_soft = Color32::from_rgba_unmultiplied(255, 255, 255, 40);  // Glow suave
    let glow_color_medium = Color32::from_rgba_unmultiplied(255, 255, 255, 80); // Glow medio
    let glow_color_bright = Color32::from_rgba_unmultiplied(255, 255, 255, 120); // Glow brillante
    
    // ═══════════════════════════════════════════════════════════════════
    // GLOW DEL CONECTOR (Efecto neón blanco sobre fondo negro)
    // ═══════════════════════════════════════════════════════════════════
    
    if is_highlighted {
        // Glow exterior grande (más visible cuando está resaltado)
        draw_bezier_curve(painter, from, ctrl1, ctrl2, to, glow_color_soft, thickness * 5.0);
        
        // Glow medio
        draw_bezier_curve(painter, from, ctrl1, ctrl2, to, glow_color_medium, thickness * 3.0);
        
        // Glow brillante
        draw_bezier_curve(painter, from, ctrl1, ctrl2, to, glow_color_bright, thickness * 2.0);
    } else {
        // Glow sutil para conexiones normales
        draw_bezier_curve(painter, from, ctrl1, ctrl2, to, glow_color_soft, thickness * 3.0);
        draw_bezier_curve(painter, from, ctrl1, ctrl2, to, glow_color_medium, thickness * 2.0);
    }
    
    // ═══════════════════════════════════════════════════════════════════
    // LÍNEA PRINCIPAL (Blanco)
    // ═══════════════════════════════════════════════════════════════════
    
    draw_bezier_curve(painter, from, ctrl1, ctrl2, to, wire_color, thickness);
    
    // ═══════════════════════════════════════════════════════════════════
    // HIGHLIGHT (Brillo lateral para efecto 3D - Blanco)
    // ═══════════════════════════════════════════════════════════════════
    
    let highlight_color = Color32::from_rgb(255, 255, 255); // Blanco puro
    let offset = Vec2::new(-0.8 * zoom, 0.0);
    draw_bezier_curve(
        painter, 
        from + offset, 
        ctrl1 + offset, 
        ctrl2 + offset, 
        to + offset, 
        highlight_color, 
        1.5 * zoom
    );
}

/// Dibuja una curva Bezier cúbica con alta calidad
fn draw_bezier_curve(
    painter: &Painter,
    p0: Pos2,
    p1: Pos2,
    p2: Pos2,
    p3: Pos2,
    color: Color32,
    thickness: f32,
) {
    const SEGMENTS: usize = 40; // Más segmentos = curva más suave
    let mut points = Vec::with_capacity(SEGMENTS + 1);
    
    for i in 0..=SEGMENTS {
        let t = i as f32 / SEGMENTS as f32;
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        let x = mt3 * p0.x + 3.0 * mt2 * t * p1.x + 3.0 * mt * t2 * p2.x + t3 * p3.x;
        let y = mt3 * p0.y + 3.0 * mt2 * t * p1.y + 3.0 * mt * t2 * p2.y + t3 * p3.y;
        
        points.push(Pos2::new(x, y));
    }
    
    // Dibujar con líneas suaves
    for window in points.windows(2) {
        painter.line_segment(
            [window[0], window[1]], 
            Stroke::new(thickness, color)
        );
    }
}

// ══════════════════════════════════════════
// UTILIDADES
// ══════════════════════════════════════════

/// Calcula el rectángulo total de un nodo incluyendo los pins
pub fn get_node_bounds_with_pins(node_rect: Rect, node: &Node, zoom: f32) -> Rect {
    let pin_extend = PIN_RADIUS * 3.0 * zoom;
    let label_extend = 25.0 * zoom;
    
    let top_extend = if !node.inputs.is_empty() { pin_extend + label_extend } else { 0.0 };
    let bottom_extend = if !node.outputs.is_empty() { pin_extend + label_extend } else { 0.0 };
    
    Rect::from_min_max(
        node_rect.min - Vec2::new(0.0, top_extend),
        node_rect.max + Vec2::new(0.0, bottom_extend),
    )
}

/// Verifica si un punto está sobre un pin de entrada
pub fn hit_test_input_pin(node_rect: Rect, node: &Node, point: Pos2, zoom: f32) -> Option<usize> {
    let hit_radius = PIN_RADIUS * 2.5 * zoom;
    
    for (i, _pin) in node.inputs.iter().enumerate() {
        let pin_pos = get_input_pin_position(node_rect, i, node.inputs.len(), zoom);
        if (point - pin_pos).length() < hit_radius {
            return Some(i);
        }
    }
    None
}

/// Verifica si un punto está sobre un pin de salida
pub fn hit_test_output_pin(node_rect: Rect, node: &Node, point: Pos2, zoom: f32) -> Option<usize> {
    let hit_radius = PIN_RADIUS * 2.5 * zoom;
    
    for (i, _pin) in node.outputs.iter().enumerate() {
        let pin_pos = get_output_pin_position(node_rect, i, node.outputs.len(), zoom);
        if (point - pin_pos).length() < hit_radius {
            return Some(i);
        }
    }
    None
}
