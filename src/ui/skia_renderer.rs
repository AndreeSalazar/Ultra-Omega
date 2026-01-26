// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Renderizado con Skia-safe
// Renderizado 2D profesional y simple para node graphs
// ═══════════════════════════════════════════════════════════════════════════════

use skia_safe as sk;
use eframe::egui::{Color32, Vec2, Rect, Pos2};
use crate::ui::theme::THEME;

/// Contexto de renderizado con Skia
pub struct SkiaRenderer {
    pub surface: Option<sk::Surface>,
    pub canvas: Option<sk::Canvas>,
    pub width: i32,
    pub height: i32,
}

impl SkiaRenderer {
    /// Crear nuevo renderizador Skia
    pub fn new() -> Self {
        Self {
            surface: None,
            canvas: None,
            width: 800,
            height: 600,
        }
    }

    /// Inicializar el renderizador con dimensiones
    pub fn initialize(&mut self, width: i32, height: i32) -> Result<(), String> {
        self.width = width;
        self.height = height;

        // Crear surface para renderizado
        let info = sk::ImageInfo::new(
            sk::ISize::new(width, height),
            sk::ColorType::RGBA8888,
            sk::AlphaType::Premul,
            None,
        );

        // Crear surface raster (software rendering - más simple)
        let surface = sk::Surface::new_raster(&info, None, None)
            .ok_or("No se pudo crear la surface Skia")?;

        self.surface = Some(surface);
        self.canvas = None;

        Ok(())
    }

    /// Comenzar a dibujar
    pub fn begin_frame(&mut self) -> Result<&mut sk::Canvas, String> {
        if let Some(surface) = &mut self.surface {
            let canvas = surface.canvas();
            self.canvas = Some(canvas);
            Ok(self.canvas.as_mut().unwrap())
        } else {
            Err("Surface no inicializada".to_string())
        }
    }

    /// Finalizar el frame
    pub fn end_frame(&mut self) -> Result<sk::Image, String> {
        if let Some(surface) = &self.surface {
            // Tomar snapshot del canvas actual
            let image = surface.image_snapshot();
            Ok(image)
        } else {
            Err("Surface no inicializada".to_string())
        }
    }

    /// Limpiar el canvas con un color
    pub fn clear(&mut self, color: Color32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            let sk_color = color_to_skia(color);
            canvas.clear(sk_color);
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar un rectángulo
    pub fn draw_rect(&mut self, rect: Rect, color: Color32, radius: f32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            let paint = sk::Paint::new(color_to_skia(color), None);
            let sk_rect = rect_to_skia(rect);

            if radius > 0.0 {
                // Rectángulo redondeado
                let rrect = sk::RRect::new_rect_radii(sk_rect, &[sk::Point::new(radius, radius)]);
                canvas.draw_rrect(rrect, &paint);
            } else {
                // Rectángulo normal
                canvas.draw_rect(sk_rect, &paint);
            }
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar un círculo
    pub fn draw_circle(&mut self, center: Vec2, radius: f32, color: Color32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            let paint = sk::Paint::new(color_to_skia(color), None);
            canvas.draw_circle(
                sk::Point::new(center.x, center.y),
                radius,
                &paint,
            );
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar una línea
    pub fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color32, width: f32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            let mut paint = sk::Paint::new(color_to_skia(color), None);
            paint.set_stroke_width(width);
            paint.set_anti_alias(true);
            
            canvas.draw_line(
                sk::Point::new(start.x, start.y),
                sk::Point::new(end.x, end.y),
                &paint,
            );
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar texto
    pub fn draw_text(&mut self, text: &str, pos: Vec2, size: f32, color: Color32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            // Crear font
            let font = sk::Font::new(None, None).ok_or("No se pudo crear la fuente")?;
            
            // Crear paint para el texto
            let mut paint = sk::Paint::new(color_to_skia(color), None);
            paint.set_anti_alias(true);
            
            // Dibujar texto
            canvas.draw_str(
                text,
                sk::Point::new(pos.x, pos.y + size), // Ajustar baseline
                &font,
                &paint,
            );
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar un nodo (ejemplo completo)
    pub fn draw_node(&mut self, pos: Vec2, size: Vec2, title: &str, color: Color32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            // Fondo del nodo
            let bg_paint = sk::Paint::new(color_to_skia(color), None);
            let node_rect = sk::Rect::from_point_and_size(
                sk::Point::new(pos.x, pos.y),
                sk::ISize::new(size.x as i32, size.y as i32),
            );
            
            // Dibujar fondo redondeado
            let rrect = sk::RRect::new_rect_radii(node_rect, &[sk::Point::new(8.0, 8.0)]);
            canvas.draw_rrect(rrect, &bg_paint);

            // Borde del nodo
            let mut border_paint = sk::Paint::new(color_to_skia(THEME.border_primary), None);
            border_paint.set_style(sk::PaintStyle::Stroke);
            border_paint.set_stroke_width(2.0);
            canvas.draw_rrect(rrect, &border_paint);

            // Título del nodo
            let font = sk::Font::new(None, None).ok_or("No se pudo crear la fuente")?;
            let mut text_paint = sk::Paint::new(color_to_skia(THEME.text_primary), None);
            text_paint.set_anti_alias(true);
            
            canvas.draw_str(
                title,
                sk::Point::new(pos.x + 10.0, pos.y + 25.0),
                &font,
                &text_paint,
            );

            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Dibujar una conexión entre nodos
    pub fn draw_connection(&mut self, start: Pos2, end: Pos2, color: Color32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            let mut paint = sk::Paint::new(color_to_skia(color), None);
            paint.set_stroke_width(3.0);
            paint.set_anti_alias(true);

            // Crear path para la conexión curva
            let mut path = sk::Path::new();
            path.move_to(start.x, start.y);
            
            // Punto de control para curva suave
            let control_x = (start.x + end.x) / 2.0;
            let control_y = start.y - 20.0; // Curva hacia arriba
            path.quad_to(control_x, control_y, end.x, end.y);
            
            canvas.draw_path(&path, &paint);
            Ok(())
        } else {
            Err("Canvas no disponible".to_string())
        }
    }

    /// Obtener imagen del canvas como bytes
    pub fn get_image_bytes(&self) -> Result<Vec<u8>, String> {
        if let Some(surface) = &self.surface {
            let image = surface.image_snapshot();
            // Convertir a bytes RGBA
            let info = sk::ImageInfo::new(
                sk::ISize::new(self.width, self.height),
                sk::ColorType::RGBA8888,
                sk::AlphaType::Premul,
                None,
            );
            
            let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];
            
            // Aquí necesitaríamos implementar la conversión real
            // Por ahora retornamos un placeholder
            Ok(pixels)
        } else {
            Err("Surface no disponible".to_string())
        }
    }
}

impl Default for SkiaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convertir color egui a Skia
fn color_to_skia(color: Color32) -> sk::Color4f {
    sk::Color4f::new(
        color.r() as f32 / 255.0,
        color.g() as f32 / 255.0,
        color.b() as f32 / 255.0,
        color.a() as f32 / 255.0,
    )
}

/// Convertir rect egui a Skia
fn rect_to_skia(rect: Rect) -> sk::Rect {
    sk::Rect::from_point_and_size(
        sk::Point::new(rect.min.x, rect.min.y),
        sk::ISize::new(rect.width() as i32, rect.height() as i32),
    )
}

/// Demostración de capacidades de Skia
pub fn demo_skia_capabilities() -> Result<sk::Image, String> {
    let mut renderer = SkiaRenderer::new();
    renderer.initialize(800, 600)?;
    
    // Comenzar frame
    let canvas = renderer.begin_frame()?;
    
    // Limpiar con fondo oscuro
    canvas.clear(sk::Color::from_argb(255, 18, 18, 20));
    
    // Demostrar formas básicas
    renderer.draw_circle(Pos2::new(100.0, 100.0), 50.0, THEME.accent_primary)?;
    renderer.draw_rect(Rect::from_min_size(Pos2::new(200.0, 50.0), Vec2::new(100.0, 100.0)), THEME.node_rust, 10.0)?;
    renderer.draw_line(Pos2::new(350.0, 100.0), Pos2::new(450.0, 150.0), THEME.node_cpp, 3.0)?;
    renderer.draw_text("Skia-safe Demo", Pos2::new(50.0, 200.0), 24.0, THEME.text_primary)?;
    
    // Demostrar nodo completo
    renderer.draw_node(Pos2::new(50.0, 250.0), Vec2::new(200.0, 80.0), "🦀 Rust Node", THEME.node_rust)?;
    renderer.draw_node(Pos2::new(300.0, 250.0), Vec2::new(200.0, 80.0), "🔷 C++ Node", THEME.node_cpp)?;
    
    // Demostrar conexión
    renderer.draw_connection(Pos2::new(250.0, 290.0), Pos2::new(300.0, 290.0), THEME.accent_primary)?;
    
    // Finalizar frame
    renderer.end_frame()
}
