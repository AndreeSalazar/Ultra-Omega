use eframe::egui::{Color32, Painter, Pos2, Shape, Stroke, Vec2};
use eframe::egui::epaint::CubicBezierShape;

pub fn draw_connection(
    painter: &Painter,
    start: Pos2,
    end: Pos2,
    color: Color32,
    zoom: f32,
    time: f64,
) {
    let dist = start.distance(end);
    let control_dist = dist.min(100.0 * zoom) * 0.5;
    
    // Curve control points (horizontal flow)
    let points = [
        start,
        start + Vec2::new(control_dist, 0.0),
        end - Vec2::new(control_dist, 0.0),
        end,
    ];

    // Efecto neón mejorado: múltiples capas para glow
    // Capa 1: Glow exterior (muy suave, grande)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(8.0 * zoom, Color32::from_rgba_unmultiplied(
            color.r(), color.g(), color.b(), 30
        )),
    }));

    // Capa 2: Glow medio (suave)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(6.0 * zoom, Color32::from_rgba_unmultiplied(
            color.r(), color.g(), color.b(), 60
        )),
    }));

    // Capa 3: Glow interno (más visible)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(4.5 * zoom, Color32::from_rgba_unmultiplied(
            color.r(), color.g(), color.b(), 100
        )),
    }));

    // Capa 4: Base wire (outline para definición)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(4.0 * zoom, Color32::BLACK.gamma_multiply(0.4)),
    }));

    // Capa 5: Main colored wire (brillante)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(3.5 * zoom, color),
    }));

    // Capa 6: Highlight central (brillante)
    let highlight_color = Color32::from_rgb(
        (color.r() as f32 * 0.7 + 255.0 * 0.3) as u8,
        (color.g() as f32 * 0.7 + 255.0 * 0.3) as u8,
        (color.b() as f32 * 0.7 + 255.0 * 0.3) as u8,
    );
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(1.5 * zoom, highlight_color),
    }));

    // Animated Pulse mejorado (Unreal Style)
    if dist > 10.0 {
        let pulse_speed = 0.6; // Loops per second
        let t = (time * pulse_speed).rem_euclid(1.0) as f32;
        
        // Draw a glowing pulse traveling along the wire
        let pulse_pos = sample_cubic_bezier(points, t);
        let pulse_radius = 5.0 * zoom;
        
        // Pulso con glow
        painter.circle_filled(
            pulse_pos,
            pulse_radius * 1.5,
            Color32::from_rgba_unmultiplied(255, 255, 255, 80),
        );
        painter.circle_filled(
            pulse_pos,
            pulse_radius,
            Color32::WHITE.gamma_multiply(0.95),
        );
        painter.circle_filled(
            pulse_pos,
            pulse_radius * 0.5,
            highlight_color,
        );
        
        // Trail effect mejorado
        for i in 1..=3 {
            let t_trail = (t - 0.03 * i as f32).max(0.0);
            if t_trail > 0.0 {
                let trail_pos = sample_cubic_bezier(points, t_trail);
                let trail_alpha = (1.0 - i as f32 * 0.3).max(0.0);
                painter.circle_filled(
                    trail_pos,
                    pulse_radius * (0.8 - i as f32 * 0.15),
                    Color32::from_rgba_unmultiplied(255, 255, 255, (100.0 * trail_alpha) as u8),
                );
            }
        }
    }
}

fn sample_cubic_bezier(points: [Pos2; 4], t: f32) -> Pos2 {
    let t1 = 1.0 - t;
    let t1_2 = t1 * t1;
    let t1_3 = t1_2 * t1;
    let t2 = t * t;
    let t3 = t2 * t;

    let p0 = points[0].to_vec2();
    let p1 = points[1].to_vec2();
    let p2 = points[2].to_vec2();
    let p3 = points[3].to_vec2();

    (p0 * t1_3 + p1 * 3.0 * t1_2 * t + p2 * 3.0 * t1 * t2 + p3 * t3).to_pos2()
}

