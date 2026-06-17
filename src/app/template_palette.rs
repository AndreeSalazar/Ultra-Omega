use crate::templates::{all_templates, Template};
use winit::keyboard::KeyCode;

pub enum PaletteAction {
    None,
    Create(usize),
}

pub struct TemplatePalette {
    templates: Vec<Template>,
    open: bool,
    selected_index: usize,
}

impl TemplatePalette {
    pub fn new() -> Self {
        Self {
            templates: all_templates(),
            open: false,
            selected_index: 0,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn template(&self, index: usize) -> Option<&Template> {
        self.templates.get(index)
    }

    pub fn templates(&self) -> &[Template] {
        &self.templates
    }

    pub fn visible_start(&self, limit: usize) -> usize {
        if self.templates.len() <= limit || limit == 0 {
            return 0;
        }

        let half = limit / 2;
        self.selected_index
            .saturating_sub(half)
            .min(self.templates.len() - limit)
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.print_hint();
        }
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn handle_key(&mut self, key: KeyCode) -> PaletteAction {
        match key {
            KeyCode::Tab | KeyCode::Escape => {
                self.close();
                PaletteAction::None
            }
            KeyCode::ArrowDown => {
                self.select_next();
                PaletteAction::None
            }
            KeyCode::ArrowUp => {
                self.select_previous();
                PaletteAction::None
            }
            KeyCode::Enter => PaletteAction::Create(self.selected_index),
            KeyCode::Digit1 => PaletteAction::Create(0),
            KeyCode::Digit2 => PaletteAction::Create(1),
            KeyCode::Digit3 => PaletteAction::Create(2),
            KeyCode::Digit4 => PaletteAction::Create(3),
            KeyCode::Digit5 => PaletteAction::Create(4),
            KeyCode::Digit6 => PaletteAction::Create(5),
            KeyCode::Digit7 => PaletteAction::Create(6),
            KeyCode::Digit8 => PaletteAction::Create(7),
            KeyCode::Digit9 => PaletteAction::Create(8),
            KeyCode::Digit0 => PaletteAction::Create(9),
            _ => PaletteAction::None,
        }
    }

    fn select_next(&mut self) {
        if !self.templates.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.templates.len();
            self.print_selected();
        }
    }

    fn select_previous(&mut self) {
        if self.templates.is_empty() {
            return;
        }

        self.selected_index = if self.selected_index == 0 {
            self.templates.len() - 1
        } else {
            self.selected_index - 1
        };
        self.print_selected();
    }

    fn print_hint(&self) {
        log::info!("TAB: Paleta Rust abierta estilo Houdini");
        log::info!("Flechas: navegar | Enter: crear | 1-0: crear template rapido | Esc/TAB: cerrar");
        for (index, template) in self.templates.iter().take(10).enumerate() {
            log::info!("{}: {} / {} / {}", index + 1, template.category, template.subcategory, template.name);
        }
        self.print_selected();
    }

    fn print_selected(&self) {
        if let Some(template) = self.templates.get(self.selected_index) {
            log::info!(
                "Template seleccionado: [{}] {} / {} / {}",
                self.selected_index + 1,
                template.category,
                template.subcategory,
                template.name
            );
        }
    }
}
