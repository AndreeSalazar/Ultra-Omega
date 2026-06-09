pub mod terminal;
pub mod compiler_detector;

pub use terminal::{TerminalManager, TerminalTab, Language};
pub use compiler_detector::{CompilerStatus, detect_all_compilers};

