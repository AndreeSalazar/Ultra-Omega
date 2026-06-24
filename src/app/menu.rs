#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuKind {
    File,
    Edit,
    View,
    Run,
}

pub struct MenuBar;

impl MenuBar {
    pub fn click(pos: (f32, f32)) -> Option<MenuKind> {
        if pos.1 > 32.0 { return None; }
        let items = [
            (MenuKind::File, 152.0, 4.0 * 9.0 + 24.0),
            (MenuKind::Edit, 200.0, 4.0 * 9.0 + 24.0),
            (MenuKind::View, 248.0, 4.0 * 9.0 + 24.0),
            (MenuKind::Run, 300.0, 3.0 * 9.0 + 24.0),
        ];
        for (kind, base_x, label_w) in items {
            if pos.0 >= base_x && pos.0 <= base_x + label_w {
                return Some(kind);
            }
        }
        None
    }

    pub fn items(menu: Option<MenuKind>) -> Vec<(&'static str, &'static str)> {
        match menu {
            Some(MenuKind::File) => vec![
                ("New Project", "Ctrl+N"),
                ("Open Folder...", "Ctrl+O"),
                ("Save", "Ctrl+S"),
                ("Export Graph", ""),
            ],
            Some(MenuKind::Edit) => vec![
                ("Delete Selected", "Del"),
                ("Duplicate Node", "Ctrl+D"),
                ("Select All", "Ctrl+A"),
            ],
            Some(MenuKind::View) => vec![
                ("Reset Zoom", "R"),
                ("Zoom In", "Ctrl++"),
                ("Zoom Out", "Ctrl+-"),
                ("Toggle Grid", "G"),
            ],
            Some(MenuKind::Run) => vec![
                ("Run Active Node", "F5"),
                ("Build Project", "Ctrl+B"),
                ("Clean Build", ""),
            ],
            None => vec![],
        }
    }

    pub fn hit_test(pos: (f32, f32), menu: Option<MenuKind>) -> Option<usize> {
        let m = menu?;
        let menu_x = match m {
            MenuKind::File => 152.0,
            MenuKind::Edit => 200.0,
            MenuKind::View => 248.0,
            MenuKind::Run => 300.0,
        };
        let menu_y = 32.0;
        let mw = 240.0;
        let items = Self::items(menu);
        for (i, _) in items.iter().enumerate() {
            let item_y = menu_y + 6.0 + i as f32 * 32.0;
            if pos.0 >= menu_x && pos.0 <= menu_x + mw && pos.1 >= item_y && pos.1 <= item_y + 28.0 {
                return Some(i);
            }
        }
        None
    }
}
