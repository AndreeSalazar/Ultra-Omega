// Evaluador de expresiones - ejecuta expresiones parseadas

use crate::expressions::parser::{Expression, ExpressionToken};
use crate::expressions::channels::{ChannelManager, ChannelValue};
use crate::core::node_graph::NodeId;

#[allow(dead_code)] // Listo para usar
pub struct ExpressionEvaluator {
    channel_manager: ChannelManager,
    variables: std::collections::HashMap<String, ChannelValue>,
    current_node_id: Option<NodeId>,
    visited_nodes: std::collections::HashSet<String>, // Para detectar referencias circulares
}

impl ExpressionEvaluator {
    pub fn new(channel_manager: ChannelManager) -> Self {
        Self {
            channel_manager,
            variables: std::collections::HashMap::new(),
            current_node_id: None,
            visited_nodes: std::collections::HashSet::new(),
        }
    }
    
    /// Validar que no haya referencias circulares
    pub fn validate_no_circular_reference(&mut self, node_name: &str) -> Result<(), String> {
        if self.visited_nodes.contains(node_name) {
            return Err(format!("Circular reference detected: {}", node_name));
        }
        self.visited_nodes.insert(node_name.to_string());
        Ok(())
    }
    
    /// Limpiar el estado de validación
    pub fn clear_validation_state(&mut self) {
        self.visited_nodes.clear();
    }
    
    pub fn set_current_node(&mut self, node_id: NodeId) {
        self.current_node_id = Some(node_id);
    }
    
    pub fn set_variable(&mut self, name: String, value: ChannelValue) {
        self.variables.insert(name, value);
    }
    
    /// Evaluar una expresión completa
    pub fn evaluate(&mut self, expr: &Expression) -> Result<ChannelValue, String> {
        // Limpiar estado de validación antes de evaluar
        self.clear_validation_state();
        if expr.tokens.is_empty() {
            return Ok(ChannelValue::String(String::new()));
        }
        
        // Evaluar usando notación polaca inversa (RPN) simplificada
        let mut stack: Vec<ChannelValue> = Vec::new();
        let mut i = 0;
        
        while i < expr.tokens.len() {
            match &expr.tokens[i] {
                ExpressionToken::Ch(path) => {
                    let value = self.resolve_channel(path)?;
                    stack.push(value);
                },
                ExpressionToken::Variable(name) => {
                    // Buscar en variables primero
                    if let Some(value) = self.variables.get(name) {
                        stack.push(value.clone());
                    } else {
                        // Intentar como canal
                        let value = self.resolve_channel(name)?;
                        stack.push(value);
                    }
                },
                ExpressionToken::Number(n) => {
                    stack.push(ChannelValue::Number(*n));
                },
                ExpressionToken::String(s) => {
                    stack.push(ChannelValue::String(s.clone()));
                },
                ExpressionToken::Operator(op) => {
                    if op == "!" {
                        // Operador unario
                        if stack.is_empty() {
                            return Err("Not enough operands for operator !".to_string());
                        }
                        let val = stack.pop().unwrap();
                        stack.push(ChannelValue::Boolean(!val.as_bool()));
                    } else {
                        // Operador binario
                        if stack.len() < 2 {
                            return Err(format!("Not enough operands for operator {}", op));
                        }
                        let right = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        
                        let result = self.apply_operator(&left, &right, op)?;
                        stack.push(result);
                    }
                },
                _ => {
                    // Ignorar paréntesis y comas por ahora (simplificado)
                },
            }
            i += 1;
        }
        
        if stack.len() != 1 {
            return Err(format!("Invalid expression: expected 1 result, got {}", stack.len()));
        }
        
        Ok(stack.pop().unwrap())
    }
    
    /// Resolver un canal ch("path") con validación de referencias circulares
    fn resolve_channel(&mut self, path: &str) -> Result<ChannelValue, String> {
        // Formato: "nodo/param" o "nodo" o "param"
        let parts: Vec<&str> = path.split('/').collect();
        
        match parts.len() {
            1 => {
                // Solo nombre: buscar en canales globales o del nodo actual
                let name = parts[0];
                
                // Buscar en canales globales
                if let Some(value) = self.channel_manager.get_channel(name) {
                    return Ok(value.clone());
                }
                
                // Buscar en canales del nodo actual
                if let Some(node_id) = self.current_node_id {
                    if let Some(value) = self.channel_manager.get_node_channel(node_id, name) {
                        return Ok(value.clone());
                    }
                }
                
                // Buscar código de nodo por nombre (con validación de circular reference)
                if let Err(e) = self.validate_no_circular_reference(name) {
                    return Err(e);
                }
                if let Some(code) = self.channel_manager.get_node_code(name) {
                    return Ok(ChannelValue::Code(code));
                }
                
                Err(format!("Channel not found: {}", name))
            },
            2 => {
                // "nodo/param" - buscar parámetro específico de un nodo
                let node_name = parts[0];
                let _param_name = parts[1];
                
                // Buscar código del nodo y extraer parámetro (simplificado)
                if let Some(code) = self.channel_manager.get_node_code(node_name) {
                    // Por ahora, retornar el código completo
                    // En el futuro, se podría parsear el código para extraer parámetros específicos
                    return Ok(ChannelValue::Code(code));
                }
                
                Err(format!("Node or parameter not found: {}", path))
            },
            _ => Err(format!("Invalid channel path: {}", path)),
        }
    }
    
    /// Aplicar un operador a dos valores
    fn apply_operator(&self, left: &ChannelValue, right: &ChannelValue, op: &str) -> Result<ChannelValue, String> {
        match op {
            "+" => {
                if let (Some(l), Some(r)) = (left.as_number(), right.as_number()) {
                    Ok(ChannelValue::Number(l + r))
                } else {
                    Ok(ChannelValue::String(left.as_string() + &right.as_string()))
                }
            },
            "-" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Number(l - r))
            },
            "*" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Number(l * r))
            },
            "/" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                if r == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(ChannelValue::Number(l / r))
            },
            "==" => Ok(ChannelValue::Boolean(left.as_string() == right.as_string())),
            "!=" => Ok(ChannelValue::Boolean(left.as_string() != right.as_string())),
            "<" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Boolean(l < r))
            },
            ">" => {
                let l = left.as_number().ok_or("Left operand as a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Boolean(l > r))
            },
            "<=" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Boolean(l <= r))
            },
            ">=" => {
                let l = left.as_number().ok_or("Left operand must be a number")?;
                let r = right.as_number().ok_or("Right operand must be a number")?;
                Ok(ChannelValue::Boolean(l >= r))
            },
            _ => Err(format!("Unknown operator: {}", op)),
        }
    }
    
    /// Evaluar una expresión desde string
    pub fn evaluate_string(&mut self, expr_str: &str) -> Result<ChannelValue, String> {
        use crate::expressions::parser::ExpressionParser;
        let expr = ExpressionParser::parse(expr_str)?;
        self.evaluate(&expr)
    }
}

