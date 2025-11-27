use eframe::egui::{Color32, Painter, Pos2, Stroke};

pub struct CutTool {
    pub active: bool,
    pub line_start: Option<Pos2>,
    pub line_end: Option<Pos2>,
}

impl Default for CutTool {
    fn default() -> Self {
        Self {
            active: false,
            line_start: None,
            line_end: None,
        }
    }
}

impl CutTool {
    pub fn draw_cut_line(&self, painter: &Painter) {
        if let (Some(start), Some(end)) = (self.line_start, self.line_end) {
            // Línea de corte estilo Houdini (roja con patrón)
            painter.line_segment(
                [start, end],
                Stroke::new(2.0, Color32::from_rgb(255, 100, 100)),
            );
            
            // Círculos en los extremos para mejor visibilidad
            painter.circle_filled(start, 4.0, Color32::from_rgb(255, 100, 100));
            painter.circle_filled(end, 4.0, Color32::from_rgb(255, 100, 100));
            
            // Línea punteada para efecto visual
            draw_dashed_line(painter, start, end, Color32::from_rgb(255, 150, 150), 1.0, 4.0);
        }
    }

    pub fn check_intersection_with_bezier(&self, bezier_points: [Pos2; 4]) -> bool {
        if let (Some(line_start), Some(line_end)) = (self.line_start, self.line_end) {
            // Verificar intersección entre línea recta y curva Bézier
            // Muestreamos la curva y verificamos si la línea la cruza
            const SAMPLES: usize = 20;
            for i in 0..SAMPLES {
                let t1 = i as f32 / SAMPLES as f32;
                let t2 = (i + 1) as f32 / SAMPLES as f32;
                
                let p1 = sample_cubic_bezier(bezier_points, t1);
                let p2 = sample_cubic_bezier(bezier_points, t2);
                
                // Verificar si el segmento de la curva intersecta con la línea de corte
                if line_segment_intersect(line_start, line_end, p1, p2) {
                    return true;
                }
            }
        }
        false
    }
}

fn draw_dashed_line(painter: &Painter, start: Pos2, end: Pos2, color: Color32, width: f32, dash_length: f32) {
    let dir = (end - start).normalized();
    let total_length = start.distance(end);
    let mut current_pos = start;
    let mut draw = true;
    
    while current_pos.distance(start) < total_length {
        let next_pos = if current_pos.distance(start) + dash_length > total_length {
            end
        } else {
            current_pos + dir * dash_length
        };
        
        if draw {
            painter.line_segment([current_pos, next_pos], Stroke::new(width, color));
        }
        
        current_pos = next_pos;
        draw = !draw;
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

fn line_segment_intersect(a1: Pos2, a2: Pos2, b1: Pos2, b2: Pos2) -> bool {
    // Algoritmo de intersección de segmentos de línea
    let d = (a2.x - a1.x) * (b2.y - b1.y) - (a2.y - a1.y) * (b2.x - b1.x);
    
    if d.abs() < 1e-6 {
        return false; // Líneas paralelas
    }
    
    let t = ((b1.x - a1.x) * (b2.y - b1.y) - (b1.y - a1.y) * (b2.x - b1.x)) / d;
    let u = ((b1.x - a1.x) * (a2.y - a1.y) - (b1.y - a1.y) * (a2.x - a1.x)) / d;
    
    t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
}

