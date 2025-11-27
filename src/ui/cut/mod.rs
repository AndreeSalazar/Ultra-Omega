use eframe::egui::{Color32, Painter, Pos2, Stroke};

pub struct CutTool {
    pub active: bool,
    pub points: Vec<Pos2>, // Polilínea para dibujo libre
}

impl Default for CutTool {
    fn default() -> Self {
        Self {
            active: false,
            points: Vec::new(),
        }
    }
}

impl CutTool {
    pub fn draw_cut_line(&self, painter: &Painter, time: f64) {
        if self.points.len() < 2 {
            return;
        }

        // Calcular longitud total de la línea para animación
        let total_length: f32 = self.points
            .windows(2)
            .map(|w| w[0].distance(w[1]))
            .sum();
        
        // Animación: pulso que viaja a lo largo de la línea
        let pulse_speed = 1.5; // Velocidad del pulso (ciclos por segundo)
        let pulse_t = (time * pulse_speed).rem_euclid(1.0) as f32;
        let pulse_position = pulse_t * total_length;
        
        // Calcular posición del pulso en la línea
        let mut accumulated_length = 0.0;
        let mut pulse_point: Option<Pos2> = None;
        
        for i in 0..(self.points.len() - 1) {
            let start = self.points[i];
            let end = self.points[i + 1];
            let segment_length = start.distance(end);
            
            if accumulated_length <= pulse_position && pulse_position < accumulated_length + segment_length {
                // El pulso está en este segmento
                let local_t = (pulse_position - accumulated_length) / segment_length;
                pulse_point = Some(start.lerp(end, local_t));
                break;
            }
            
            accumulated_length += segment_length;
        }

        // Efecto neón VERDE: dibujar múltiples capas para crear glow
        for i in 0..(self.points.len() - 1) {
            let start = self.points[i];
            let end = self.points[i + 1];
            
            // Calcular distancia desde el pulso para efecto de brillo
            let segment_center = start.lerp(end, 0.5);
            let pulse_glow = if let Some(pulse_pos) = pulse_point {
                let dist_to_pulse = segment_center.distance(pulse_pos);
                // Efecto de brillo que se desvanece con la distancia
                (1.0 - (dist_to_pulse / 50.0).min(1.0)).max(0.0)
            } else {
                0.0
            };
            
            // Capa 1: Glow exterior (muy suave, grande)
            painter.line_segment(
                [start, end],
                Stroke::new(8.0, Color32::from_rgba_unmultiplied(50, 255, 50, 30)),
            );
            
            // Capa 2: Glow medio (suave)
            painter.line_segment(
                [start, end],
                Stroke::new(6.0, Color32::from_rgba_unmultiplied(80, 255, 80, 60)),
            );
            
            // Capa 3: Glow interno (más visible)
            painter.line_segment(
                [start, end],
                Stroke::new(4.0, Color32::from_rgba_unmultiplied(120, 255, 120, 120)),
            );
            
            // Capa 4: Línea principal neón VERDE (brillante con animación)
            let bright_green = Color32::from_rgb(
                (100.0 + pulse_glow * 155.0) as u8,
                255,
                (150.0 + pulse_glow * 105.0) as u8,
            );
            painter.line_segment(
                [start, end],
                Stroke::new(2.5, bright_green),
            );
            
            // Capa 5: Línea central brillante (highlight animado)
            let highlight_intensity = (pulse_glow * 255.0) as u8;
            painter.line_segment(
                [start, end],
                Stroke::new(1.0, Color32::from_rgb(highlight_intensity, 255, highlight_intensity)),
            );
        }
        
        // Punto inicial con efecto neón VERDE
        if let Some(&start) = self.points.first() {
            // Glow exterior
            painter.circle_filled(start, 10.0, Color32::from_rgba_unmultiplied(50, 255, 50, 40));
            painter.circle_filled(start, 8.0, Color32::from_rgba_unmultiplied(80, 255, 80, 80));
            painter.circle_filled(start, 6.0, Color32::from_rgba_unmultiplied(120, 255, 120, 150));
            // Círculo principal
            painter.circle_filled(start, 5.0, Color32::from_rgb(100, 255, 150));
            // Highlight central
            painter.circle_filled(start, 2.5, Color32::from_rgb(200, 255, 200));
        }
        
        // Punto final con efecto neón VERDE más intenso (posición actual) - animado
        if let Some(&end) = self.points.last() {
            // Pulso animado en el punto final
            let pulse_alpha = ((time * 2.0).sin() * 0.5 + 0.5) as f32;
            let pulse_size = 6.0 + (time * 3.0).sin() as f32 * 2.0;
            
            // Glow exterior animado (más grande)
            painter.circle_filled(end, 12.0, Color32::from_rgba_unmultiplied(50, 255, 50, (50.0 * pulse_alpha) as u8));
            painter.circle_filled(end, 10.0, Color32::from_rgba_unmultiplied(80, 255, 80, (100.0 * pulse_alpha) as u8));
            painter.circle_filled(end, 8.0, Color32::from_rgba_unmultiplied(120, 255, 120, (180.0 * pulse_alpha) as u8));
            // Círculo principal animado
            painter.circle_filled(end, pulse_size, Color32::from_rgb(150, 255, 200));
            // Borde brillante animado
            painter.circle_stroke(end, pulse_size, Stroke::new(2.0, Color32::from_rgb(200, 255, 200)));
            // Highlight central
            painter.circle_filled(end, 3.0, Color32::from_rgb(220, 255, 220));
        }
        
        // Puntos intermedios con efecto neón VERDE sutil
        for i in 1..(self.points.len() - 1) {
            let point = self.points[i];
            // Glow sutil
            painter.circle_filled(point, 4.0, Color32::from_rgba_unmultiplied(100, 255, 100, 80));
            // Punto principal
            painter.circle_filled(point, 2.5, Color32::from_rgb(120, 255, 180));
            // Highlight
            painter.circle_filled(point, 1.0, Color32::from_rgb(180, 255, 200));
        }
        
        // Dibujar pulso animado que viaja a lo largo de la línea
        if let Some(pulse_pos) = pulse_point {
            // Pulso brillante que viaja
            let pulse_radius = 8.0 + (time * 4.0).sin() as f32 * 3.0;
            painter.circle_filled(pulse_pos, pulse_radius, Color32::from_rgba_unmultiplied(200, 255, 200, 200));
            painter.circle_filled(pulse_pos, pulse_radius * 0.6, Color32::from_rgb(150, 255, 200));
            painter.circle_filled(pulse_pos, pulse_radius * 0.3, Color32::from_rgb(255, 255, 255));
        }
    }

    pub fn check_intersection_with_bezier(&self, bezier_points: [Pos2; 4]) -> bool {
        if self.points.len() < 2 {
            return false;
        }

        // Verificar intersección entre cada segmento de la polilínea y la curva Bézier
        // Muestreamos la curva y verificamos si algún segmento de la línea la cruza
        const SAMPLES: usize = 20;
        
        for i in 0..(self.points.len() - 1) {
            let line_start = self.points[i];
            let line_end = self.points[i + 1];
            
            for j in 0..SAMPLES {
                let t1 = j as f32 / SAMPLES as f32;
                let t2 = (j + 1) as f32 / SAMPLES as f32;
                
                let p1 = sample_cubic_bezier(bezier_points, t1);
                let p2 = sample_cubic_bezier(bezier_points, t2);
                
                // Verificar si el segmento de la curva intersecta con el segmento de la línea de corte
                if line_segment_intersect(line_start, line_end, p1, p2) {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn add_point(&mut self, point: Pos2) {
        // Agregar punto solo si está suficientemente lejos del último punto
        // Esto previene demasiados puntos muy cercanos
        const MIN_DISTANCE: f32 = 3.0;
        
        if let Some(&last_point) = self.points.last() {
            if point.distance(last_point) < MIN_DISTANCE {
                return; // Punto muy cercano, no agregar
            }
        }
        
        self.points.push(point);
    }
    
    pub fn clear(&mut self) {
        self.points.clear();
    }
    
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
    
    pub fn has_points(&self) -> bool {
        !self.points.is_empty()
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

