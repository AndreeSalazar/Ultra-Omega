pub mod terminal;
pub mod auto_linker;
pub mod compiler_detector;

pub use terminal::{TerminalManager, TerminalTab, Language};
pub use auto_linker::auto_link;
pub use compiler_detector::{CompilerStatus, detect_all_compilers};

