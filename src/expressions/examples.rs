// Ejemplos de uso del sistema de expresiones
// Este archivo muestra cómo integrar el sistema con el grafo de nodos

use crate::expressions::{ChannelManager, ChannelValue, ExpressionEvaluator};
use crate::node_graph::NodeId;

/// Ejemplo básico de uso de ch()
pub fn example_basic_ch() -> Result<(), String> {
    // Crear administrador de canales
    let mut channels = ChannelManager::new();
    
    // Registrar código de un nodo
    channels.set_channel(
        "nodo_asm".to_string(),
        ChannelValue::Code("section .text\n    global _start".to_string()),
    );
    
    // Crear evaluador
    let mut evaluator = ExpressionEvaluator::new(channels);
    
    // Evaluar expresión que referencia el nodo
    let result = evaluator.evaluate_string(r#"ch("nodo_asm")"#)?;
    
    match result {
        ChannelValue::Code(code) => {
            println!("Código obtenido: {}", code);
        },
        _ => return Err("Expected Code value".to_string()),
    }
    
    Ok(())
}

/// Ejemplo de combinación de código de múltiples nodos
pub fn example_combine_nodes() -> Result<(), String> {
    let mut channels = ChannelManager::new();
    
    // Registrar múltiples nodos
    channels.set_channel(
        "header".to_string(),
        ChannelValue::Code("#include <stdio.h>\n".to_string()),
    );
    
    channels.set_channel(
        "main_func".to_string(),
        ChannelValue::Code("int main() { return 0; }".to_string()),
    );
    
    let mut evaluator = ExpressionEvaluator::new(channels);
    
    // En el futuro, se podría hacer algo como:
    // ch("header") + "\n" + ch("main_func")
    // Por ahora, solo podemos obtener valores individuales
    
    let header = evaluator.evaluate_string(r#"ch("header")"#)?;
    let main = evaluator.evaluate_string(r#"ch("main_func")"#)?;
    
    if let (ChannelValue::Code(h), ChannelValue::Code(m)) = (header, main) {
        let combined = format!("{}\n{}", h, m);
        println!("Código combinado:\n{}", combined);
    }
    
    Ok(())
}

/// Ejemplo de uso con variables
pub fn example_variables() -> Result<(), String> {
    let mut channels = ChannelManager::new();
    let mut evaluator = ExpressionEvaluator::new(channels);
    
    // Definir variables
    evaluator.set_variable("version".to_string(), ChannelValue::Number(1.0));
    evaluator.set_variable("name".to_string(), ChannelValue::String("Ultra-Omega".to_string()));
    
    // Usar variables en expresiones (futuro)
    // Por ahora, las variables se pueden usar directamente
    
    Ok(())
}

/// Integración con el sistema de nodos
pub fn integrate_with_node_graph(
    node_id: NodeId,
    node_name: &str,
    code: &str,
    channels: &mut ChannelManager,
) {
    // Registrar el código del nodo en el sistema de canales
    channels.set_channel(
        node_name.to_string(),
        ChannelValue::Code(code.to_string()),
    );
    
    // También registrar por ID del nodo
    channels.set_node_channel(
        node_id,
        "code".to_string(),
        ChannelValue::Code(code.to_string()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_ch() {
        assert!(example_basic_ch().is_ok());
    }
    
    #[test]
    fn test_combine_nodes() {
        assert!(example_combine_nodes().is_ok());
    }
}

