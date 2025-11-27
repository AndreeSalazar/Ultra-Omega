mod node_graph;

use crate::node_graph::{NodeGraph, NodeId, PinId, PinKind};
use eframe::{
    egui::epaint::{CubicBezierShape, RectShape, Shape},
    egui::{
        self, Align2, Color32, FontId, Painter, PointerButton, Pos2, Rect, Sense, Stroke,
        TextureId, Vec2, Visuals, pos2,
    },
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("ultra-omega-node-lab")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([960.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ultra Omega Node Lab",
        options,
        Box::new(|_cc| Box::<NodeGraphApp>::default()),
    )
}

struct NodeGraphApp {
    graph: NodeGraph,
    viewport: Viewport2D,
    interaction: InteractionState,
}

#[derive(Default)]
struct InteractionState {
    dragging_node: Option<NodeId>,
    new_node_requested: bool,
    editing_node: Option<NodeId>,
}

struct Viewport2D {
    pan: Vec2,
    zoom: f32,
}

impl Default for Viewport2D {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl NodeGraphApp {
    fn toolbar_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar")
            .frame(egui::Frame::default().fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Nuevo nodo").clicked() {
                        self.interaction.new_node_requested = true;
                    }

                    if ui.button("Reset vista").clicked() {
                        self.viewport = Viewport2D::default();
                    }

                    ui.separator();
                    ui.label(format!("Zoom: {:>3.0}%", self.viewport.zoom * 100.0));
                    ui.label(format!("Nodos: {}", self.graph.nodes().len()));
                });
            });
    }

    fn canvas_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(&ctx.style()))
            .show(ctx, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
                let rect = response.rect;

                let input = ui.input(|i| PointerSnapshot {
                    pos: i.pointer.interact_pos(),
                    delta: i.pointer.delta(),
                    primary_pressed: i.pointer.button_pressed(PointerButton::Primary),
                    primary_down: i.pointer.button_down(PointerButton::Primary),
                    secondary_pressed: i.pointer.button_pressed(PointerButton::Secondary),
                    middle_down: i.pointer.button_down(PointerButton::Middle),
                    ctrl_scroll: if i.modifiers.ctrl {
                        i.smooth_scroll_delta.y
                    } else {
                        0.0
                    },
                });

                if response.hovered() {
                    self.handle_zoom(input.ctrl_scroll, input.pos, rect);
                }

                if response.hovered() && input.middle_down {
                    self.viewport.pan += input.delta;
                }

                if self.interaction.new_node_requested {
                    let world = self.viewport.screen_to_world(rect.center(), rect);
                    self.graph.add_default_node(world);
                    self.interaction.new_node_requested = false;
                }

                self.paint_grid(&painter, rect, ui.visuals());
                self.paint_links(&painter, rect);
                self.paint_nodes(&painter, rect, ui.visuals());

                self.handle_node_dragging(&input, rect);

                if input.primary_down {
                    ctx.request_repaint();
                }
            });
    }

    fn paint_grid(&self, painter: &Painter, rect: Rect, visuals: &Visuals) {
        const GRID_SPACING: f32 = 32.0;
        let spacing = (GRID_SPACING * self.viewport.zoom).clamp(12.0, 256.0);

        let offset_x = self.viewport.pan.x.rem_euclid(spacing);
        let offset_y = self.viewport.pan.y.rem_euclid(spacing);

        painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);

        let mut count_x = 0;
        let mut x = rect.min.x + offset_x;
        while x < rect.max.x {
            let major = count_x % 4 == 0;
            let color = if major {
                visuals.extreme_bg_color.gamma_multiply(1.4)
            } else {
                visuals.extreme_bg_color.gamma_multiply(1.15)
            };
            painter.line_segment(
                [pos2(x, rect.min.y), pos2(x, rect.max.y)],
                Stroke::new(1.0, color),
            );
            x += spacing;
            count_x += 1;
        }

        let mut count_y = 0;
        let mut y = rect.min.y + offset_y;
        while y < rect.max.y {
            let major = count_y % 4 == 0;
            let color = if major {
                visuals.extreme_bg_color.gamma_multiply(1.4)
            } else {
                visuals.extreme_bg_color.gamma_multiply(1.15)
            };
            painter.line_segment(
                [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                Stroke::new(1.0, color),
            );
            y += spacing;
            count_y += 1;
        }
    }

    fn paint_links(&self, painter: &Painter, rect: Rect) {
        for link in self.graph.links() {
            let Some(start) = self.pin_screen_position(link.from, rect) else {
                continue;
            };
            let Some(end) = self.pin_screen_position(link.to, rect) else {
                continue;
            };

            let pull = Vec2::X * 120.0 * self.viewport.zoom;
            let mut points = [start, start + pull, end - pull, end];
            if start.x > end.x {
                points[1].x = start.x + 40.0 * self.viewport.zoom;
                points[2].x = end.x - 40.0 * self.viewport.zoom;
            }

            painter.add(Shape::CubicBezier(CubicBezierShape {
                points,
                closed: false,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(3.0, link.color),
            }));
        }
    }

    fn paint_nodes(&self, painter: &Painter, rect: Rect, visuals: &Visuals) {
        let text_zoom = self.viewport.zoom.clamp(0.5, 1.25);
        let title_font = FontId::proportional(18.0 * text_zoom);
        let pin_font = FontId::proportional(14.0 * text_zoom);

        for node in self.graph.nodes() {
            let node_rect = self.node_rect(node, rect);
            let rounding = egui::Rounding::same(9.0);
            let header_height = HEADER_HEIGHT * self.viewport.zoom;

            let body_fill = node.color.gamma_multiply(0.22);
            painter.add(Self::rect_shape(
                node_rect,
                rounding,
                body_fill,
                Stroke::new(1.0, node.color),
            ));

            let header_rect =
                Rect::from_min_size(node_rect.min, Vec2::new(node_rect.width(), header_height));
            let header_rounding = egui::Rounding {
                nw: rounding.nw,
                ne: rounding.ne,
                sw: 0.0,
                se: 0.0,
            };
            painter.add(Self::rect_shape(
                header_rect,
                header_rounding,
                node.color.gamma_multiply(0.8),
                Stroke::NONE,
            ));

            painter.text(
                header_rect.left_center() + Vec2::new(12.0 * self.viewport.zoom, 0.0),
                Align2::LEFT_CENTER,
                &node.title,
                title_font.clone(),
                Color32::WHITE,
            );

            painter.line_segment(
                [
                    pos2(node_rect.min.x, header_rect.max.y),
                    pos2(node_rect.max.x, header_rect.max.y),
                ],
                Stroke::new(1.0, visuals.panel_fill.gamma_multiply(0.6)),
            );

            for (index, pin) in node.inputs.iter().enumerate() {
                let center = self.pin_slot_position(node, rect, PinKind::Input, index);
                self.paint_pin(painter, center, pin, &pin_font);
            }

            for (index, pin) in node.outputs.iter().enumerate() {
                let center = self.pin_slot_position(node, rect, PinKind::Output, index);
                self.paint_pin(painter, center, pin, &pin_font);
            }
        }
    }

    fn paint_pin(&self, painter: &Painter, center: Pos2, pin: &node_graph::Pin, font: &FontId) {
        let (align, direction) = match pin.kind {
            PinKind::Input => (Align2::LEFT_CENTER, 1.0),
            PinKind::Output => (Align2::RIGHT_CENTER, -1.0),
        };
        let radius = PIN_RADIUS * self.viewport.zoom;
        painter.circle_filled(center, radius, Color32::WHITE);
        painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::BLACK));

        let offset = Vec2::new(
            direction * (radius + PIN_TEXT_GAP * self.viewport.zoom),
            0.0,
        );
        painter.text(
            center + offset,
            align,
            &pin.label,
            font.clone(),
            Color32::WHITE,
        );
    }

    fn handle_node_dragging(&mut self, input: &PointerSnapshot, rect: Rect) {
        if let Some(pointer_pos) = input.pos {
            if input.primary_pressed
                && rect.contains(pointer_pos)
                && self.interaction.dragging_node.is_none()
            {
                self.interaction.dragging_node = self.hit_test(pointer_pos, rect);
            }
            if input.secondary_pressed && rect.contains(pointer_pos) {
                if let Some(node_id) = self.hit_test(pointer_pos, rect) {
                    self.interaction.editing_node = Some(node_id);
                }
            }
            if !input.primary_down {
                self.interaction.dragging_node = None;
            }
            if let Some(node_id) = self.interaction.dragging_node {
                if let Some(node) = self.graph.node_mut(node_id) {
                    node.position += input.delta / self.viewport.zoom;
                }
            }
        } else {
            self.interaction.dragging_node = None;
        }
    }

    fn handle_zoom(&mut self, scroll: f32, pointer: Option<Pos2>, rect: Rect) {
        if scroll.abs() < f32::EPSILON {
            return;
        }

        let anchor = pointer.unwrap_or(rect.center());
        let world_anchor = self.viewport.screen_to_world(anchor, rect);
        let factor = (1.0 + scroll * 0.0015).clamp(0.5, 1.5);
        self.viewport.zoom = (self.viewport.zoom * factor).clamp(0.35, 2.5);

        let new_screen = self.viewport.world_to_screen(world_anchor, rect);
        self.viewport.pan += anchor - new_screen;
    }

    fn hit_test(&self, pointer: Pos2, rect: Rect) -> Option<NodeId> {
        self.graph.nodes().iter().rev().find_map(|node| {
            let r = self.node_rect(node, rect);
            if r.contains(pointer) {
                Some(node.id)
            } else {
                None
            }
        })
    }

    fn node_rect(&self, node: &node_graph::Node, canvas: Rect) -> Rect {
        let size = self.node_size(node) * self.viewport.zoom;
        let min = self.viewport.world_to_screen(node.position, canvas);
        Rect::from_min_size(min, size)
    }

    fn node_size(&self, node: &node_graph::Node) -> Vec2 {
        let rows = node.inputs.len().max(node.outputs.len()).max(1) as f32;
        let height = HEADER_HEIGHT + rows * PIN_SPACING + CONTENT_PADDING * 2.0;
        Vec2::new(NODE_WIDTH, height)
    }

    fn pin_slot_position(
        &self,
        node: &node_graph::Node,
        canvas: Rect,
        kind: PinKind,
        index: usize,
    ) -> Pos2 {
        let rect = self.node_rect(node, canvas);
        let y = rect.min.y
            + HEADER_HEIGHT * self.viewport.zoom
            + PIN_SPACING * self.viewport.zoom * (index as f32 + 0.5);

        match kind {
            PinKind::Input => pos2(rect.min.x + CONTENT_PADDING * self.viewport.zoom, y),
            PinKind::Output => pos2(rect.max.x - CONTENT_PADDING * self.viewport.zoom, y),
        }
    }

    fn pin_screen_position(&self, pin_id: PinId, canvas: Rect) -> Option<Pos2> {
        let address = self.graph.locate_pin(pin_id)?;
        let node = &self.graph.nodes()[address.node_index];
        Some(self.pin_slot_position(node, canvas, address.kind, address.slot))
    }

    fn editor_ui(&mut self, ctx: &egui::Context) {
        let mut open = self.interaction.editing_node.is_some();
        let node_id = self.interaction.editing_node;

        if open {
            let mut should_close = false;
            egui::Window::new("Editor de Código")
                .open(&mut open)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    if let Some(id) = node_id {
                        if let Some(node) = self.graph.node_mut(id) {
                            ui.horizontal(|ui| {
                                ui.heading(&node.title);
                                if ui.button("Cerrar").clicked() {
                                    should_close = true;
                                }
                            });
                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let font_id = egui::FontId::monospace(14.0);
                                let _row_height = ui.fonts(|f| f.row_height(&font_id));
                                let num_lines = node.code.lines().count().max(1);

                                ui.horizontal_top(|ui| {
                                    // Line numbers column
                                    ui.vertical(|ui| {
                                        ui.set_width(40.0);
                                        for i in 1..=num_lines {
                                            ui.label(
                                                egui::RichText::new(format!("{}", i))
                                                    .font(font_id.clone())
                                                    .color(Color32::GRAY),
                                            );
                                        }
                                    });

                                    // Code editor
                                    ui.add_sized(
                                        ui.available_size(),
                                        egui::TextEdit::multiline(&mut node.code)
                                            .font(egui::TextStyle::Monospace)
                                            .code_editor()
                                            .lock_focus(true)
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                            });
                        }
                    }
                });
            if should_close {
                open = false;
            }
        }

        if !open {
            self.interaction.editing_node = None;
        }
    }

    fn rect_shape(rect: Rect, rounding: egui::Rounding, fill: Color32, stroke: Stroke) -> Shape {
        Shape::Rect(RectShape {
            rect,
            rounding,
            fill,
            stroke,
            fill_texture_id: TextureId::default(),
            uv: Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        })
    }
}

impl Viewport2D {
    fn world_to_screen(&self, pos: Pos2, canvas: Rect) -> Pos2 {
        pos2(
            canvas.min.x + self.pan.x + pos.x * self.zoom,
            canvas.min.y + self.pan.y + pos.y * self.zoom,
        )
    }

    fn screen_to_world(&self, pos: Pos2, canvas: Rect) -> Pos2 {
        pos2(
            (pos.x - canvas.min.x - self.pan.x) / self.zoom,
            (pos.y - canvas.min.y - self.pan.y) / self.zoom,
        )
    }
}

impl Default for NodeGraphApp {
    fn default() -> Self {
        Self {
            graph: NodeGraph::demo(),
            viewport: Viewport2D::default(),
            interaction: InteractionState::default(),
        }
    }
}

impl eframe::App for NodeGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.toolbar_ui(ctx);
        self.canvas_ui(ctx);
        self.editor_ui(ctx);
    }
}

#[derive(Clone, Copy)]
struct PointerSnapshot {
    pos: Option<Pos2>,
    delta: Vec2,
    primary_pressed: bool,
    primary_down: bool,
    secondary_pressed: bool,
    middle_down: bool,
    ctrl_scroll: f32,
}

const NODE_WIDTH: f32 = 220.0;
const HEADER_HEIGHT: f32 = 36.0;
const PIN_SPACING: f32 = 26.0;
const PIN_RADIUS: f32 = 5.0;
const PIN_TEXT_GAP: f32 = 10.0;
const CONTENT_PADDING: f32 = 14.0;
