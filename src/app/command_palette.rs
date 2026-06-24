pub struct CommandPalette {
    pub open: bool,
    pub query: String,
    pub selected: usize,
    commands: Vec<Command>,
}

pub struct Command {
    pub name: &'static str,
    pub shortcut: &'static str,
    pub action: CommandAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandAction {
    NewNode,
    OpenFolder,
    Save,
    RunNode,
    DeleteNode,
    ZoomIn,
    ZoomOut,
    ResetZoom,
    ToggleTemplates,
    ExportGraph,
    BuildProject,
    CleanBuild,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected: 0,
            commands: vec![
                Command { name: "New Rust Node", shortcut: "N", action: CommandAction::NewNode },
                Command { name: "Open Folder...", shortcut: "Ctrl+O", action: CommandAction::OpenFolder },
                Command { name: "Save", shortcut: "Ctrl+S", action: CommandAction::Save },
                Command { name: "Run Active Node", shortcut: "F5", action: CommandAction::RunNode },
                Command { name: "Delete Selected Node", shortcut: "Del", action: CommandAction::DeleteNode },
                Command { name: "Zoom In", shortcut: "Ctrl++", action: CommandAction::ZoomIn },
                Command { name: "Zoom Out", shortcut: "Ctrl+-", action: CommandAction::ZoomOut },
                Command { name: "Reset Zoom", shortcut: "R", action: CommandAction::ResetZoom },
                Command { name: "Toggle Templates", shortcut: "Tab", action: CommandAction::ToggleTemplates },
                Command { name: "Export Graph", shortcut: "", action: CommandAction::ExportGraph },
                Command { name: "Build Project", shortcut: "Ctrl+B", action: CommandAction::BuildProject },
                Command { name: "Clean Build", shortcut: "", action: CommandAction::CleanBuild },
            ],
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.query.clear();
            self.selected = 0;
        }
    }

    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
    }

    pub fn append_char(&mut self, ch: char) {
        if ch == '\u{8}' {
            self.query.pop();
        } else if !ch.is_control() {
            self.query.push(ch);
        }
        self.selected = 0;
    }

    pub fn move_selection(&mut self, delta: i32) {
        let filtered = self.filtered_count();
        if filtered == 0 { return; }
        self.selected = ((self.selected as i32 + delta).rem_euclid(filtered as i32)) as usize;
    }

    pub fn filtered_count(&self) -> usize {
        self.filtered().count()
    }

    pub fn filtered(&self) -> impl Iterator<Item = (usize, &Command)> {
        let q = self.query.to_lowercase();
        self.commands.iter().enumerate().filter(move |(_, c)| {
            q.is_empty() || c.name.to_lowercase().contains(&q)
        })
    }

    pub fn execute_selected(&self) -> Option<CommandAction> {
        self.filtered().nth(self.selected).map(|(_, c)| c.action)
    }

    pub fn selected_name(&self) -> &str {
        self.filtered().nth(self.selected).map(|(_, c)| c.name).unwrap_or("")
    }
}
