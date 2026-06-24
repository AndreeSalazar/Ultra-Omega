use std::path::Path;

#[derive(Clone, Debug)]
pub struct SidebarEntry {
    pub name: String,
    pub rel_path: String,
    pub depth: usize,
    pub is_dir: bool,
    pub is_expanded: bool,
}

pub fn list_files(root: &Path) -> Vec<SidebarEntry> {
    let mut entries = Vec::new();
    collect_entries(root, "", 0, &mut entries, 200);
    entries
}

fn collect_entries(base: &Path, rel: &str, depth: usize, out: &mut Vec<SidebarEntry>, max: usize) {
    if out.len() >= max { return; }
    let full = base.join(rel);
    let Ok(read_dir) = std::fs::read_dir(&full) else { return; };
    let mut entries: Vec<_> = read_dir.flatten().collect();
    entries.sort_by_key(|e| {
        let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
        (!is_dir, e.file_name().to_string_lossy().to_lowercase())
    });
    for entry in entries {
        if out.len() >= max { break; }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') { continue; }
        if name == "target" || name == "node_modules" { continue; }
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let new_rel = if rel.is_empty() { name.clone() } else { format!("{}/{}", rel, name) };
        out.push(SidebarEntry {
            name: name.clone(),
            rel_path: new_rel.clone(),
            depth,
            is_dir,
            is_expanded: true,
        });
        if is_dir {
            collect_entries(base, &new_rel, depth + 1, out, max);
        }
    }
}
