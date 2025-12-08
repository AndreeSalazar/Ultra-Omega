// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Sistema de Almacenamiento
// Gestión de proyectos con código separado del mapa de nodos
// ═══════════════════════════════════════════════════════════════════════════════

pub mod workspace;
pub mod node_storage;
pub mod project;
pub mod migration;

pub use workspace::Workspace;
pub use node_storage::NodeStorage;
pub use project::{ProjectMetadata, ProjectConfig};

