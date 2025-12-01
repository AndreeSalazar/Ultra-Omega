use eframe::egui::{Pos2, Rect, Vec2};

#[derive(Clone, Copy)]
pub struct Viewport2D {
    pub pan: Vec2,
    pub zoom: f32,
}

impl Default for Viewport2D {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Viewport2D {
    pub fn world_to_screen(&self, pos: Pos2, canvas: Rect) -> Pos2 {
        pos2(
            canvas.min.x + self.pan.x + pos.x * self.zoom,
            canvas.min.y + self.pan.y + pos.y * self.zoom,
        )
    }

    pub fn screen_to_world(&self, pos: Pos2, canvas: Rect) -> Pos2 {
        pos2(
            (pos.x - canvas.min.x - self.pan.x) / self.zoom,
            (pos.y - canvas.min.y - self.pan.y) / self.zoom,
        )
    }

    pub fn pan_zoom(&mut self, canvas: Rect, zoom_delta: f32, pan_delta: Vec2, pointer_pos: Option<Pos2>) {
        // Pan
        self.pan += pan_delta;

        // Zoom
        if zoom_delta != 0.0 {
            let anchor = pointer_pos.unwrap_or(canvas.center());
            let world_anchor = self.screen_to_world(anchor, canvas);
            
            let factor = (1.0 + zoom_delta * 0.0015).clamp(0.5, 1.5);
            self.zoom = (self.zoom * factor).clamp(0.1, 5.0);

            let new_screen = self.world_to_screen(world_anchor, canvas);
            self.pan += anchor - new_screen;
        }
    }

    pub fn focus_on(&mut self, bounds: Rect, canvas: Rect) {
        if !bounds.is_positive() { return; }

        let margin = 50.0;
        let target_width = bounds.width() + margin * 2.0;
        let target_height = bounds.height() + margin * 2.0;

        let scale_x = canvas.width() / target_width;
        let scale_y = canvas.height() / target_height;
        
        // Fit to screen
        self.zoom = scale_x.min(scale_y).clamp(0.1, 2.0);

        // Center
        let bounds_center_world = bounds.center();
        let _canvas_center_screen = canvas.center();
        
        // We want: canvas_min + pan + bounds_center * zoom = canvas_center
        // pan = canvas_center - canvas_min - bounds_center * zoom
        // pan = canvas_half_size - bounds_center * zoom
        
        let canvas_half_size = canvas.size() / 2.0;
        self.pan = canvas_half_size - bounds_center_world.to_vec2() * self.zoom;
    }
}

fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2::new(x, y)
}

