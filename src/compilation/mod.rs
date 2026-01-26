pub mod terminal;
pub mod auto_linker;
pub mod compiler_detector;
pub mod cpp_linker;

pub use terminal::{TerminalManager, TerminalTab, Language};
pub use auto_linker::auto_link;
pub use compiler_detector::{CompilerStatus, detect_all_compilers};
pub use cpp_linker::{CppLinkerManager, get_best_cpp_linker, compile_cpp_auto, detect_cpp_version, find_cpp_compilers};

