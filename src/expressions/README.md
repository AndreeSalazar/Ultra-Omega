# Sistema de Expresiones Inspirado en Houdini

Este módulo implementa un sistema de expresiones similar a Houdini, permitiendo referenciar valores de otros nodos mediante la función `ch()`.

## Características

- **Expresiones tipo Houdini**: Soporta `ch("nodo/param")` para acceder a valores de otros nodos
- **Operadores**: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Variables**: Soporta variables con `$variable`
- **Canales**: Sistema de canales para almacenar y acceder a valores de nodos

## Uso Básico

```rust
use crate::expressions::{ChannelManager, ExpressionEvaluator, ExpressionParser};

// Crear administrador de canales
let mut channels = ChannelManager::new();

// Registrar código de un nodo
channels.set_channel("nodo1".to_string(), ChannelValue::Code("int main() { return 0; }".to_string()));

// Crear evaluador
let evaluator = ExpressionEvaluator::new(channels);

// Evaluar expresión
let result = evaluator.evaluate_string(r#"ch("nodo1")"#)?;
```

## Sintaxis de Expresiones

### Referencias a Canales
- `ch("nodo")` - Obtiene el código completo del nodo "nodo"
- `ch("nodo/param")` - Obtiene un parámetro específico del nodo (futuro)

### Operaciones
- Aritméticas: `ch("nodo1") + ch("nodo2")`
- Comparación: `ch("valor") > 10`
- Concatenación: `ch("texto1") + ch("texto2")`

### Variables
- `$variable` - Accede a una variable definida

## Integración con el Sistema de Nodos

El sistema se integra con el grafo de nodos existente:
- Los nodos pueden registrar sus valores en el ChannelManager
- Las expresiones pueden referenciar otros nodos por nombre
- El código heredado puede usar expresiones para combinar código de múltiples nodos

## Archivos

- `mod.rs` - Módulo principal
- `channels.rs` - Sistema de canales
- `parser.rs` - Parser de expresiones
- `evaluator.rs` - Evaluador de expresiones

## Próximos Pasos

1. Integrar con el sistema de herencia de código
2. Permitir expresiones en el editor de código
3. Soporte para más operadores y funciones
4. Cache de resultados de expresiones
5. Validación de referencias circulares

