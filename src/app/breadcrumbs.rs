#[derive(Clone, Debug)]
pub struct Breadcrumb {
    pub label: String,
    pub kind: BreadcrumbKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreadcrumbKind {
    Workspace,
    Node,
    Language,
}

pub fn build_breadcrumbs(
    workspace_name: &str,
    node_title: Option<&str>,
    language: Option<&str>,
) -> Vec<Breadcrumb> {
    let mut crumbs = Vec::new();
    if !workspace_name.is_empty() {
        crumbs.push(Breadcrumb {
            label: workspace_name.to_string(),
            kind: BreadcrumbKind::Workspace,
        });
    }
    if let Some(title) = node_title {
        crumbs.push(Breadcrumb {
            label: format!("Nodo: {}", title),
            kind: BreadcrumbKind::Node,
        });
    }
    if let Some(lang) = language {
        crumbs.push(Breadcrumb {
            label: lang.to_string(),
            kind: BreadcrumbKind::Language,
        });
    }
    crumbs
}
