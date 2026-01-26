// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: Sistema de Almacenamiento
// Gestión de proyectos con código separado del mapa de nodos
// ═══════════════════════════════════════════════════════════════════════════════

pub mod workspace;
pub mod node_storage;
pub mod project;
pub mod migration;
pub mod hda;
pub mod file_watcher;

pub use workspace::Workspace;
pub use node_storage::NodeStorage;
pub use project::{ProjectMetadata, ProjectConfig};
pub use migration::{MigrationResult, needs_migration, migrate_project, create_backup};
pub use hda::{HDAManager, HDA, HDAInfo, HDAParameter, ParameterType, create_hda_from_nodes, create_hda_from_subnetwork};
pub use file_watcher::{FileWatcherState, DetectedFile, DetectedFolder, FileWatcherStats};

