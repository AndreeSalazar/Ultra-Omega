// Sistema de expresiones inspirado en Houdini
// Permite usar ch() para referenciar valores de otros nodos

pub mod parser;
pub mod evaluator;
pub mod channels;

// Ejemplos de uso (opcional, no se carga por defecto)
// pub mod examples;

pub use parser::{ExpressionParser, Expression, ExpressionToken};
pub use evaluator::ExpressionEvaluator;
pub use channels::{ChannelManager, ChannelValue};

// Silenciar warnings de código no usado (está listo para usar)
#[allow(dead_code)]
pub mod examples;

