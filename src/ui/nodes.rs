use eframe::egui::{self, Align2, Color32, FontId, Painter, Pos2, Rect, Stroke, Vec2, Visuals};
use eframe::egui::epaint::{RectShape, Shape, TextureId};
use crate::node_graph::{Node, Pin};

// Shared constants should ideally be in a config, but duplicating for now to keep UI self-contained
// or we can update node_graph to export them.
pub const HEADER_HEIGHT: f32 = 36.0;
pub const PIN_SPACING: f32 = 26.0;
pub const NODE_WIDTH: f32 = 240.0; // Slightly wider for "Professional" look
pub const CONTENT_PADDING: f32 = 14.0;
const PIN_RADIUS: f32 = 5.0;
const PIN_TEXT_GAP: f32 = 10.0;

pub fn draw_node(
    painter: &Painter,
    node: &Node,
    rect: Rect, // Pre-calculated screen rect
    zoom: f32,
    selected: bool,
    _visuals: &Visuals,
) {
    let rounding = egui::Rounding::same(12.0 * zoom);

    // 0. Selection Highlight (Outer Glow)
    if selected {
        let glow_rect = rect.expand(4.0 * zoom);
        painter.rect_stroke(
            glow_rect,
            rounding,
            Stroke::new(2.0 * zoom, Color32::from_rgb(255, 200, 50)), // Golden highlight
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
    // Shadow for text
    painter.text(
        header_rect.left_center() + Vec2::new(13.0 * zoom, 1.0 * zoom),
        Align2::LEFT_CENTER,
        &node.title,
        title_font.clone(),
        Color32::BLACK.gamma_multiply(0.5),
    );
    painter.text(
        header_rect.left_center() + Vec2::new(12.0 * zoom, 0.0),
        Align2::LEFT_CENTER,
        &node.title,
        title_font,
        Color32::WHITE,
    );

    // 5. Pins
    let pin_font = FontId::proportional(14.0 * text_zoom);
    
    for (i, pin) in node.inputs.iter().enumerate() {
        let y = rect.min.y + header_height + PIN_SPACING * zoom * (i as f32 + 0.5);
        let pos = Pos2::new(rect.min.x + CONTENT_PADDING * zoom, y);
        draw_pin(painter, pin, pos, Align2::LEFT_CENTER, 1.0, zoom, &pin_font);
    }

    for (i, pin) in node.outputs.iter().enumerate() {
        let y = rect.min.y + header_height + PIN_SPACING * zoom * (i as f32 + 0.5);
        let pos = Pos2::new(rect.max.x - CONTENT_PADDING * zoom, y);
        draw_pin(painter, pin, pos, Align2::RIGHT_CENTER, -1.0, zoom, &pin_font);
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
) {
    let radius = PIN_RADIUS * zoom;
    
    // Pin Hole (Darker center)
    painter.circle_filled(center, radius, Color32::BLACK);
    
    // Pin Rim (Colored or Grey)
    painter.circle_stroke(center, radius, Stroke::new(2.0 * zoom, Color32::from_gray(150)));

    // Label
    let text_pos = center + Vec2::new(direction * (radius + PIN_TEXT_GAP * zoom), 0.0);
    painter.text(
        text_pos,
        align,
        &pin.label,
        font.clone(),
        Color32::from_gray(220), // Off-white text
    );
}

