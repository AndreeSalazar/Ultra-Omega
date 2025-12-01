// ═══════════════════════════════════════════════════════════════════════════
// Integración Mojo para Ultra Omega Node Lab
// ═══════════════════════════════════════════════════════════════════════════
// 
// Este módulo proporciona la interfaz entre Rust y Mojo para:
// - Evaluación de expresiones acelerada
// - Generación de código con IA
// - Procesamiento matemático en GPU
// - Análisis inteligente de código
//
// Arquitectura:
//   Rust (Core) ←→ FFI Bridge ←→ Mojo Runtime ←→ Python/ML Libraries
// ═══════════════════════════════════════════════════════════════════════════

pub mod bridge;
pub mod ai;
pub mod math;
pub mod evaluator;

use crate::expressions::ChannelValue;
use crate::node_graph::NodeLanguage;

/// Motor principal de Mojo - gestiona el runtime y las operaciones
pub struct MojoEngine {
    initialized: bool,
    // El runtime Mojo se carga dinámicamente
}

impl MojoEngine {
    /// Crear una nueva instancia del motor Mojo
    pub fn new() -> Result<Self, String> {
        // Intentar inicializar el runtime Mojo
        // Por ahora, retornamos un error si no está disponible
        // En producción, esto cargaría la librería dinámicamente
        
        #[cfg(feature = "mojo")]
        {
            match bridge::initialize_mojo_runtime() {
                Ok(_) => Ok(Self { initialized: true }),
                Err(e) => Err(format!("Failed to initialize Mojo runtime: {}", e)),
            }
        }
        
        #[cfg(not(feature = "mojo"))]
        {
            Err("Mojo support not compiled. Enable 'mojo' feature.".to_string())
        }
    }
    
    /// Verificar si el motor está inicializado
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Evaluar una expresión usando Mojo (acelerado)
    pub fn evaluate_expression(
        &self,
        expr: &str,
        context: &ExpressionContext,
    ) -> Result<ChannelValue, String> {
        if !self.initialized {
            return Err("Mojo engine not initialized".to_string());
        }
        
        evaluator::evaluate_expression_mojo(expr, context)
    }
    
    /// Generar código usando IA
    pub fn generate_code(
        &self,
        prompt: &str,
        language: NodeLanguage,
    ) -> Result<String, String> {
        if !self.initialized {
            return Err("Mojo engine not initialized".to_string());
        }
        
        ai::generate_code_with_ai(prompt, language)
    }
    
    /// Procesar cálculos matemáticos en GPU
    pub fn process_math(
        &self,
        expression: &str,
        values: &MathContext,
    ) -> Result<f64, String> {
        if !self.initialized {
            return Err("Mojo engine not initialized".to_string());
        }
        
        math::process_math_expression(expression, values)
    }
    
    /// Analizar código con IA
    pub fn analyze_code(
        &self,
        code: &str,
        language: NodeLanguage,
    ) -> Result<CodeAnalysis, String> {
        if !self.initialized {
            return Err("Mojo engine not initialized".to_string());
        }
        
        ai::analyze_code_with_ai(code, language)
    }
}

impl Default for MojoEngine {
    fn default() -> Self {
        Self { initialized: false }
    }
}

/// Contexto para evaluación de expresiones
#[derive(Clone, Debug)]
pub struct ExpressionContext {
    pub channels: std::collections::HashMap<String, ChannelValue>,
    pub variables: std::collections::HashMap<String, ChannelValue>,
}

impl Default for ExpressionContext {
    fn default() -> Self {
        Self {
            channels: std::collections::HashMap::new(),
            variables: std::collections::HashMap::new(),
        }
    }
}

/// Contexto para cálculos matemáticos
#[derive(Clone, Debug)]
pub struct MathContext {
    pub values: std::collections::HashMap<String, f64>,
    pub use_gpu: bool,
}

impl Default for MathContext {
    fn default() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            use_gpu: true, // Por defecto usar GPU
        }
    }
}

/// Resultado del análisis de código
#[derive(Clone, Debug)]
pub struct CodeAnalysis {
    pub bugs: Vec<BugReport>,
    pub suggestions: Vec<Suggestion>,
    pub complexity_score: f64,
    pub optimization_opportunities: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct BugReport {
    pub line: usize,
    pub message: String,
    pub severity: BugSeverity,
}

#[derive(Clone, Debug)]
pub enum BugSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct Suggestion {
    pub line: usize,
    pub message: String,
    pub code: Option<String>, // Código sugerido
}

// Re-exportar módulos públicos
pub use bridge::MojoRuntime;
pub use ai::{AIGenerator, CodeAnalyzer};
pub use math::MathProcessor;
pub use evaluator::ExpressionEvaluatorMojo;

