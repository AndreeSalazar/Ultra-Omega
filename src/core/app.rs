use crate::core::node_graph::{NodeGraph, NodeId, NodeLanguage};
use crate::storage::Workspace;
use crate::config::AppConfig;
use crate::expressions::ChannelManager;

/// Estructura principal de la aplicación.
/// Esta versión está simplificada para enfocarse en la lógica del grafo de nodos
/// y la gestión del workspace. El renderizado ahora se maneja directamente con Vulkan.
pub struct NodeGraphApp {
    pub graph: NodeGraph,
    pub workspace: Workspace,
    pub channel_manager: ChannelManager,
    pub config: AppConfig,
}

impl NodeGraphApp {
    /// Crea una nueva instancia de la aplicación desde la configuración.
    pub fn from_config(config: AppConfig) -> Self {
        let mut workspace = Workspace::new();
        workspace.auto_save = config.auto_save;
        
        let initial_graph = if let Some(workspace_path) = config.workspace_path.as_ref() {
            let path = std::path::PathBuf::from(workspace_path);
            if path.exists() {
                workspace.set_root(path.clone());
                workspace.load_graph().unwrap_or_else(|_| {
                    eprintln!("Error loading workspace, using demo");
                    NodeGraph::demo()
                })
            } else {
                NodeGraph::demo()
            }
        } else {
            NodeGraph::demo()
        };
        
        let mut app = Self {
            graph: initial_graph.clone(),
            workspace,
            channel_manager: ChannelManager::new(),
            config,
        };
        
        // Load workspace if configured
        if let Some(workspace_path) = &app.config.workspace_path {
            let path = std::path::PathBuf::from(workspace_path);
            if path.exists() {
                app.workspace.set_root(path);
                if let Err(e) = app.load_graph_from_workspace() {
                    eprintln!("Error loading workspace: {}", e);
                    app.graph = NodeGraph::demo();
                }
            }
        }
        
        app
    }
    
    /// Guarda la configuración actual.
    pub fn save_config(&self) {
        let mut config = self.config.clone();
        config.workspace_path = self.workspace.root_path.as_ref()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string());
        config.auto_save = self.workspace.auto_save;
        
        if let Err(e) = config.save() {
            eprintln!("Error saving config: {}", e);
        }
    }
    
    /// Carga el grafo desde el workspace.
    pub fn load_graph_from_workspace(&mut self) -> Result<(), String> {
        let graph = self.workspace.load_graph()?;
        self.graph = graph;
        self.graph.recalculate_ids();
        Ok(())
    }
    
    /// Guarda el grafo actual en el workspace.
    pub fn save_current_graph(&mut self) -> Result<(), String> {
        self.workspace.save_graph(&mut self.graph)?;
        Ok(())
    }
    
    /// Obtiene el grafo actual.
    pub fn graph(&self) -> &NodeGraph {
        &self.graph
    }
    
    /// Obtiene el grafo actual mutable.
    pub fn graph_mut(&mut self) -> &mut NodeGraph {
        &mut self.graph
    }
}

impl Default for NodeGraphApp {
    fn default() -> Self {
        Self::from_config(AppConfig::default())
    }
}
