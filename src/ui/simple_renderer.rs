// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Renderizado con Softbuffer
// Renderizado 2D ultra simple y directo para node graphs
// ═══════════════════════════════════════════════════════════════════════════════

use eframe::egui::{Color32, Vec2, Rect, Pos2};
use crate::ui::theme::THEME;

/// Renderizador ultra simple con softbuffer
pub struct SimpleRenderer {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl SimpleRenderer {
    /// Crear nuevo renderizador simple
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            pixels: vec![0; 800 * 600],
        }
    }

    /// Inicializar con dimensiones
    pub fn initialize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.pixels = vec![0; (width * height) as usize];
    }

    /// Obtener referencia a los pixels
    pub fn get_pixels(&self) -> &[u32] {
        &self.pixels
    }

    /// Limpiar con un color
    pub fn clear(&mut self, color: Color32) {
        let color_value = color_to_argb(color);
        self.pixels.fill(color_value);
    }

    /// Dibujar un pixel
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color_to_argb(color);
        }
    }

    /// Dibujar una línea (algoritmo de Bresenham)
    pub fn draw_line(&mut self, start: Pos2, end: Pos2, color: Color32) {
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.draw_pixel(x as u32, y as u32, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Dibujar un rectángulo
    pub fn draw_rect(&mut self, rect: Rect, color: Color32) {
        let min_x = rect.min.x as u32;
        let min_y = rect.min.y as u32;
        let max_x = rect.max.x as u32;
        let max_y = rect.max.y as u32;

        for y in min_y..=max_y.min(self.height - 1) {
            for x in min_x..=max_x.min(self.width - 1) {
                self.draw_pixel(x, y, color);
            }
        }
    }

    /// Dibujar un rectángulo con borde
    pub fn draw_rect_border(&mut self, rect: Rect, fill_color: Color32, border_color: Color32) {
        // Rellenar
        self.draw_rect(rect, fill_color);
        
        // Borde superior
        self.draw_line(rect.min, Pos2::new(rect.max.x, rect.min.y), border_color);
        // Borde inferior
        self.draw_line(Pos2::new(rect.min.x, rect.max.y), rect.max, border_color);
        // Borde izquierdo
        self.draw_line(rect.min, Pos2::new(rect.min.x, rect.max.y), border_color);
        // Borde derecho
        self.draw_line(Pos2::new(rect.max.x, rect.min.y), rect.max, border_color);
    }

    /// Dibujar un círculo (algoritmo simple)
    pub fn draw_circle(&mut self, center: Pos2, radius: f32, color: Color32) {
        let cx = center.x;
        let cy = center.y;
        let r2 = radius * radius;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                if dx * dx + dy * dy <= r2 {
                    self.draw_pixel(x, y, color);
                }
            }
        }
    }

    /// Dibujar un círculo con borde
    pub fn draw_circle_border(&mut self, center: Pos2, radius: f32, fill_color: Color32, border_color: Color32, border_width: f32) {
        // Rellenar
        self.draw_circle(center, radius, fill_color);
        
        // Borde (dibujando círculos concéntricos)
        for i in 0..(border_width as i32) {
            let r = radius + i as f32;
            let r_inner = radius - i as f32 - 1.0;
            if r_inner > 0.0 {
                self.draw_circle_outline(center, r, border_color);
                self.draw_circle_outline(center, r_inner, fill_color);
            } else {
                self.draw_circle_outline(center, r, border_color);
            }
        }
    }

    /// Dibujar solo el contorno del círculo
    fn draw_circle_outline(&mut self, center: Pos2, radius: f32, color: Color32) {
        let cx = center.x;
        let cy = center.y;
        let r2 = radius * radius;

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist2 = dx * dx + dy * dy;
                
                // Dibujar solo si está cerca del perímetro
                if (dist2 - r2).abs() < radius * 0.1 {
                    self.draw_pixel(x, y, color);
                }
            }
        }
    }

    /// Dibujar texto simple (caracteres básicos)
    pub fn draw_text(&mut self, text: &str, pos: Pos2, color: Color32) {
        // Implementación muy básica de texto (solo para demostración)
        let mut x = pos.x as u32;
        let mut y = pos.y as u32;
        
        for ch in text.chars() {
            self.draw_char(ch, x, y, color);
            x += 8; // Espaciado simple entre caracteres
        }
    }

    /// Dibujar un carácter simple (implementación básica)
    fn draw_char(&mut self, ch: char, x: u32, y: u32, color: Color32) {
        // Caracteres simples para demostración
        match ch {
            'A' => {
                // Dibujar una 'A' simple
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 8) as f32), color);
            }
            'B' => {
                // Dibujar una 'B' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 6) as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 6) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 6) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 6) as f32, y as f32), Pos2::new((x + 6) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new((x + 6) as f32, (y + 8) as f32), Pos2::new((x + 6) as f32, (y + 16) as f32), color);
            }
            'C' => {
                // Dibujar una 'C' simple
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 8) as f32), 7.0, color);
                // Borrar parte derecha
                for py in y..=(y + 16) {
                    for px in (x + 6)..=(x + 8) {
                        self.draw_pixel(px, py, Color32::BLACK);
                    }
                }
            }
            'D' => {
                // Dibujar una 'D' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 8) as f32), 7.0, color);
                // Rellenar lado izquierdo
                for py in y..=(y + 16) {
                    for px in x..=(x + 2) {
                        self.draw_pixel(px, py, color);
                    }
                }
            }
            'E' => {
                // Dibujar una 'E' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 6) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            'F' => {
                // Dibujar una 'F' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 6) as f32, (y + 8) as f32), color);
            }
            'G' => {
                // Dibujar una 'G' simple
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 8) as f32), 7.0, color);
                // Línea horizontal
                self.draw_line(Pos2::new((x + 4) as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 12) as f32), color);
            }
            'H' => {
                // Dibujar una 'H' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 8) as f32), color);
            }
            'I' => {
                // Dibujar una 'I' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, y as f32), Pos2::new((x + 4) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            'K' => {
                // Dibujar una 'K' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            'L' => {
                // Dibujar una 'L' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            'M' => {
                // Dibujar una 'M' simple
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new(x as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 4) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            'N' => {
                // Dibujar una 'N' simple
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new(x as f32, y as f32), color);
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, y as f32), color);
            }
            'O' => {
                // Dibujar una 'O' simple
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 8) as f32), 7.0, color);
            }
            'P' => {
                // Dibujar una 'P' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 4) as f32), 4.0, color);
                // Rellenar lado izquierdo
                for py in y..=(y + 8) {
                    for px in x..=(x + 2) {
                        self.draw_pixel(px, py, color);
                    }
                }
            }
            'R' => {
                // Dibujar una 'R' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 4) as f32), 4.0, color);
                self.draw_line(Pos2::new((x + 4) as f32, (y + 8) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                // Rellenar lado izquierdo
                for py in y..=(y + 8) {
                    for px in x..=(x + 2) {
                        self.draw_pixel(px, py, color);
                    }
                }
            }
            'S' => {
                // Dibujar una 'S' simple
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 4) as f32), 4.0, color);
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 12) as f32), 4.0, color);
                // Borrar partes no deseadas
                for py in y..=(y + 4) {
                    for px in (x + 4)..=(x + 8) {
                        self.draw_pixel(px, py, Color32::BLACK);
                    }
                }
                for py in (y + 12)..=(y + 16) {
                    for px in x..=(x + 4) {
                        self.draw_pixel(px, py, Color32::BLACK);
                    }
                }
            }
            'T' => {
                // Dibujar una 'T' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, y as f32), Pos2::new((x + 4) as f32, (y + 16) as f32), color);
            }
            'U' => {
                // Dibujar una 'U' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new(x as f32, (y + 12) as f32), color);
                self.draw_circle_outline(Pos2::new((x + 4) as f32, (y + 12) as f32), 4.0, color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new((x + 8) as f32, (y + 12) as f32), color);
            }
            'V' => {
                // Dibujar una 'V' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 4) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, y as f32), color);
            }
            'W' => {
                // Dibujar una 'W' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 2) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 2) as f32, (y + 16) as f32), Pos2::new((x + 4) as f32, y as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, y as f32), Pos2::new((x + 6) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 6) as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, y as f32), color);
            }
            'X' => {
                // Dibujar una 'X' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
            }
            'Y' => {
                // Dibujar una 'Y' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 4) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new((x + 4) as f32, (y + 8) as f32), color);
                self.draw_line(Pos2::new((x + 4) as f32, (y + 8) as f32), Pos2::new((x + 4) as f32, (y + 16) as f32), color);
            }
            'Z' => {
                // Dibujar una 'Z' simple
                self.draw_line(Pos2::new(x as f32, y as f32), Pos2::new((x + 8) as f32, y as f32), color);
                self.draw_line(Pos2::new((x + 8) as f32, y as f32), Pos2::new(x as f32, (y + 16) as f32), color);
                self.draw_line(Pos2::new(x as f32, (y + 16) as f32), Pos2::new((x + 8) as f32, (y + 16) as f32), color);
            }
            ' ' => {
                // Espacio - no dibujar nada
            }
            _ => {
                // Caracter no soportado - dibujar un cuadrado
                self.draw_rect(Rect::from_min_size(Pos2::new(x as f32, y as f32), Vec2::new(6.0, 8.0)), color);
            }
        }
    }

    /// Dibujar un nodo completo
    pub fn draw_node(&mut self, pos: Pos2, size: Vec2, title: &str, color: Color32) {
        // Fondo del nodo
        let node_rect = Rect::from_min_size(pos, size);
        self.draw_rect(node_rect, color);
        
        // Borde
        self.draw_rect_border(node_rect, color, THEME.border_primary);
        
        // Título
        self.draw_text(title, Pos2::new(pos.x + 5.0, pos.y + 5.0), THEME.text_primary);
    }

    /// Dibujar una conexión curva entre nodos
    pub fn draw_connection(&mut self, start: Pos2, end: Pos2, color: Color32) {
        // Conexión simple con línea recta por ahora
        self.draw_line(start, end, color);
        
        // Dibujar puntos en los extremos
        self.draw_circle_border(start, 4.0, color, THEME.border_primary, 1.0);
        self.draw_circle_border(end, 4.0, color, THEME.border_primary, 1.0);
    }
}

impl Default for SimpleRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convertir color egui a formato ARGB (0xAARRGGBB)
fn color_to_argb(color: Color32) -> u32 {
    ((color.a() as u32) << 24) |
    ((color.r() as u32) << 16) |
    ((color.g() as u32) << 8) |
    (color.b() as u32)
}

/// Demostración de capacidades del renderizador simple
pub fn demo_simple_rendering() -> SimpleRenderer {
    let mut renderer = SimpleRenderer::new();
    renderer.initialize(800, 600);
    
    // Limpiar con fondo oscuro
    renderer.clear(THEME.background_primary);
    
    // Demostrar formas básicas
    renderer.draw_circle_border(Pos2::new(100.0, 100.0), 50.0, THEME.accent_primary, THEME.border_primary, 2.0);
    renderer.draw_rect_border(Rect::from_min_size(Pos2::new(200.0, 50.0), Vec2::new(100.0, 100.0)), THEME.node_rust, THEME.border_primary);
    renderer.draw_line(Pos2::new(350.0, 100.0), Pos2::new(450.0, 150.0), THEME.node_cpp);
    renderer.draw_text("SIMPLE RENDER", Pos2::new(50.0, 200.0), THEME.text_primary);
    
    // Demostrar nodos
    renderer.draw_node(Pos2::new(50.0, 250.0), Vec2::new(200.0, 80.0), "RUST NODE", THEME.node_rust);
    renderer.draw_node(Pos2::new(300.0, 250.0), Vec2::new(200.0, 80.0), "CPP NODE", THEME.node_cpp);
    
    // Demostrar conexión
    renderer.draw_connection(Pos2::new(250.0, 290.0), Pos2::new(300.0, 290.0), THEME.accent_primary);
    
    renderer
}
