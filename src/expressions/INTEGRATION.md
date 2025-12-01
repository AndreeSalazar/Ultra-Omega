# Guía de Integración del Sistema de Expresiones

## Visión General

El sistema de expresiones inspirado en Houdini permite que los nodos referencien valores de otros nodos mediante la función `ch()`. Esto crea un sistema híbrido donde:

1. **Nodos tradicionales**: Funcionan como antes, con código directo
2. **Nodos con expresiones**: Pueden usar `ch("nodo_otro")` para referenciar código de otros nodos
3. **Combinación híbrida**: Los nodos pueden combinar código propio con código referenciado

## Integración con el Sistema Actual

### Paso 1: Agregar ChannelManager al NodeGraphApp

```rust
// En src/app.rs
use crate::expressions::ChannelManager;

pub struct NodeGraphApp {
    // ... campos existentes ...
    pub channel_manager: ChannelManager,
}
```

### Paso 2: Registrar Nodos en el ChannelManager

Cuando se crea o modifica un nodo, registrarlo:

```rust
// Al crear/modificar un nodo
app.channel_manager.set_channel(
    node.title.clone(),
    ChannelValue::Code(node.code.clone()),
);
```

### Paso 3: Procesar Expresiones en el Código

Antes de compilar/ejecutar código, detectar y procesar expresiones:

```rust
use crate::expressions::ExpressionEvaluator;

fn process_code_with_expressions(code: &str, app: &NodeGraphApp) -> String {
    let evaluator = ExpressionEvaluator::new(app.channel_manager.clone());
    
    // Buscar expresiones ch() en el código
    // Reemplazar con valores reales
    // Retornar código procesado
}
```

## Casos de Uso

### Caso 1: Herencia Mejorada

En lugar de solo heredar código del padre directo, un nodo puede referenciar múltiples nodos:

```c
// Nodo "main" puede usar:
#include "utils.h"  // De nodo "utils"
#include "config.h" // De nodo "config"

int main() {
    // Código propio
}
```

### Caso 2: Plantillas Dinámicas

Un nodo puede ser una plantilla que se rellena con valores de otros nodos:

```rust
// Nodo "template"
fn ${ch("function_name")}() {
    ${ch("function_body")}
}
```

### Caso 3: Combinación de Código

Un nodo puede combinar código de múltiples fuentes:

```c
// Nodo "combined"
${ch("header")}
${ch("definitions")}
${ch("implementation")}
```

## Implementación Sugerida

### 1. Detección de Expresiones

Agregar un método que detecte expresiones `ch()` en el código:

```rust
fn find_expressions(code: &str) -> Vec<String> {
    // Buscar patrones ch("...")
    // Retornar lista de expresiones encontradas
}
```

### 2. Procesamiento

Procesar cada expresión y reemplazarla:

```rust
fn process_expressions(code: &str, evaluator: &ExpressionEvaluator) -> Result<String, String> {
    let mut processed = code.to_string();
    
    for expr in find_expressions(&code) {
        let result = evaluator.evaluate_string(&expr)?;
        processed = processed.replace(&expr, &result.as_string());
    }
    
    Ok(processed)
}
```

### 3. Integración con el Editor

Permitir que el usuario escriba expresiones directamente en el editor:

- Autocompletado para `ch("...")`
- Validación de referencias
- Vista previa del código procesado

## Próximos Pasos

1. **Integrar con el sistema de herencia actual**
2. **Agregar procesamiento de expresiones antes de compilar**
3. **UI para mostrar referencias entre nodos**
4. **Validación de referencias circulares**
5. **Cache de resultados de expresiones**

## Ejemplo Completo

```rust
// En el editor de código, el usuario escribe:
ch("nodo_asm") + "\n" + ch("nodo_c")

// El sistema:
// 1. Detecta las expresiones
// 2. Evalúa cada una
// 3. Combina los resultados
// 4. Muestra el código combinado
// 5. Al ejecutar, usa el código combinado
```

