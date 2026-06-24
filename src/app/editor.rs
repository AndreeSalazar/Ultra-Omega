use crate::core::{NodeGraph, NodeId};
use winit::keyboard::KeyCode;

pub struct EditorState {
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl Default for EditorState {
    fn default() -> Self {
        Self { cursor_line: 0, cursor_col: 0 }
    }
}

impl EditorState {
    pub fn open_at_cursor(
        &mut self,
        node_id: NodeId,
        graph: &NodeGraph,
        active_editor: &mut Option<NodeId>,
        selected: &mut Option<NodeId>,
        dragging: &mut Option<NodeId>,
        link_source: &mut Option<super::interaction::HitPin>,
        template_palette: &mut super::template_palette::TemplatePalette,
    ) {
        *selected = Some(node_id);
        *active_editor = Some(node_id);
        *dragging = None;
        *link_source = None;
        template_palette.close();

        if let Some(node) = graph.node(node_id) {
            let lines: Vec<&str> = node.code.lines().collect();
            self.cursor_line = lines.len().saturating_sub(1);
            self.cursor_col = lines.last().map_or(0, |l| l.len());
        }
    }

    pub fn insert_text(
        &mut self,
        text: &str,
        node_id: NodeId,
        graph: &mut NodeGraph,
    ) -> bool {
        let mut changed = false;

        if let Some(node) = graph.node_mut(node_id) {
            for ch in text.chars() {
                if ch == '\r' { continue; }
                if ch == '\n' {
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    let line = self.cursor_line.min(lines.len().saturating_sub(1).max(0));
                    let col = self.cursor_col.min(lines.get(line).map_or(0, |l| l.len()));
                    let before: String = lines.get(line).map_or(String::new(), |l| l[..col].to_string());
                    let after: String = lines.get(line).map_or(String::new(), |l| l[col..].to_string());
                    let mut new_lines: Vec<String> = lines[..line].to_vec();
                    new_lines.push(before);
                    new_lines.push(after);
                    node.code = new_lines.join("\n");
                    self.cursor_line = (line + 1).min(new_lines.len().saturating_sub(1));
                    self.cursor_col = 0;
                    changed = true;
                } else if !ch.is_control() {
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    let line = self.cursor_line.min(lines.len().saturating_sub(1).max(0));
                    let col = self.cursor_col.min(lines.get(line).map_or(0, |l| l.len()));
                    let mut new_lines = lines;
                    if let Some(l) = new_lines.get_mut(line) {
                        l.insert(col, ch);
                    }
                    node.code = new_lines.join("\n");
                    self.cursor_col += ch.len_utf8();
                    changed = true;
                }
            }
        }

        changed
    }

    pub fn handle_key(
        &mut self,
        key: KeyCode,
        node_id: NodeId,
        graph: &mut NodeGraph,
        active_editor: &mut Option<NodeId>,
    ) -> bool {
        match key {
            KeyCode::Escape => {
                *active_editor = None;
                false
            }
            KeyCode::Backspace => {
                let mut changed = false;
                if let Some(node) = graph.node_mut(node_id) {
                    let lines: Vec<String> = node.code.lines().map(str::to_string).collect();
                    if self.cursor_col > 0 {
                        let line = self.cursor_line.min(lines.len().saturating_sub(1).max(0));
                        let col = self.cursor_col;
                        let mut new_lines = lines;
                        if let Some(l) = new_lines.get_mut(line) {
                            if col <= l.len() {
                                let byte_idx = l.char_indices().nth(col.saturating_sub(1)).map_or(l.len(), |(i, _)| i);
                                l.remove(byte_idx);
                                changed = true;
                                self.cursor_col -= 1;
                            }
                        }
                        if changed { node.code = new_lines.join("\n"); }
                    } else if self.cursor_line > 0 {
                        let prev_line_len = lines.get(self.cursor_line.saturating_sub(1)).map_or(0, |l| l.len());
                        let mut new_lines = lines;
                        if self.cursor_line < new_lines.len() {
                            let current = new_lines.remove(self.cursor_line);
                            if let Some(prev) = new_lines.last_mut() {
                                prev.push_str(&current);
                            }
                            node.code = new_lines.join("\n");
                            self.cursor_line -= 1;
                            self.cursor_col = prev_line_len;
                            changed = true;
                        }
                    }
                }
                changed
            }
            KeyCode::Enter => {
                self.insert_text("\n", node_id, graph)
            }
            KeyCode::Tab => {
                self.insert_text("    ", node_id, graph)
            }
            KeyCode::ArrowLeft => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                } else if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    if let Some(node) = graph.node(node_id) {
                        let lines: Vec<&str> = node.code.lines().collect();
                        self.cursor_col = lines.get(self.cursor_line).map_or(0, |l| l.len());
                    }
                }
                false
            }
            KeyCode::ArrowRight => {
                if let Some(node) = graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    let max_col = lines.get(self.cursor_line).map_or(0, |l| l.len());
                    if self.cursor_col < max_col {
                        self.cursor_col += 1;
                    } else if self.cursor_line + 1 < lines.len() {
                        self.cursor_line += 1;
                        self.cursor_col = 0;
                    }
                }
                false
            }
            KeyCode::ArrowUp => {
                if self.cursor_line > 0 {
                    self.cursor_line -= 1;
                    if let Some(node) = graph.node(node_id) {
                        let lines: Vec<&str> = node.code.lines().collect();
                        let max_col = lines.get(self.cursor_line).map_or(0, |l| l.len());
                        self.cursor_col = self.cursor_col.min(max_col);
                    }
                }
                false
            }
            KeyCode::ArrowDown => {
                if let Some(node) = graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    if self.cursor_line + 1 < lines.len() {
                        self.cursor_line += 1;
                        let max_col = lines.get(self.cursor_line).map_or(0, |l| l.len());
                        self.cursor_col = self.cursor_col.min(max_col);
                    }
                }
                false
            }
            KeyCode::Home => {
                self.cursor_col = 0;
                false
            }
            KeyCode::End => {
                if let Some(node) = graph.node(node_id) {
                    let lines: Vec<&str> = node.code.lines().collect();
                    self.cursor_col = lines.get(self.cursor_line).map_or(0, |l| l.len());
                }
                false
            }
            _ => false,
        }
    }
}
