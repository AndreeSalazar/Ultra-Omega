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

    // Base wire (thick, dark outline for visibility)
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(5.0 * zoom, Color32::BLACK.gamma_multiply(0.5)),
    }));

    // Main colored wire
    painter.add(Shape::CubicBezier(CubicBezierShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(3.0 * zoom, color),
    }));

    // Animated Pulse (Unreal Style)
    if dist > 10.0 {
        let pulse_speed = 0.5; // Loops per second
        let t = (time * pulse_speed).rem_euclid(1.0) as f32;
        
        // Draw a glowing pulse traveling along the wire
        let pulse_pos = sample_cubic_bezier(points, t);
        let pulse_radius = 4.0 * zoom;
        
        painter.circle_filled(
            pulse_pos,
            pulse_radius,
            Color32::WHITE.gamma_multiply(0.9),
        );
        
        // Optional: Second faint pulse for trail effect
        let t2 = (t - 0.05).max(0.0);
        if t2 > 0.0 {
             let trail_pos = sample_cubic_bezier(points, t2);
             painter.circle_filled(
                trail_pos,
                pulse_radius * 0.7,
                Color32::WHITE.gamma_multiply(0.4),
            );
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

