# Análisis de Problemas Potenciales

## Problemas Identificados y Soluciones

### 1. ❌ CRÍTICO: Canales huérfanos al eliminar nodos
**Problema**: Cuando se elimina un nodo, sus canales quedan en el `ChannelManager`, causando referencias a nodos inexistentes.

**Solución**: Limpiar canales automáticamente cuando se eliminan nodos.

### 2. ⚠️ MEDIO: Referencias circulares
**Problema**: Un nodo puede referenciarse a sí mismo o crear ciclos (A -> B -> A), causando loops infinitos.

**Solución**: Agregar validación de referencias circulares antes de evaluar expresiones.

### 3. ⚠️ MEDIO: Sincronización de nombres
**Problema**: Si un nodo cambia de nombre, el canal viejo queda huérfano y el nuevo no se registra automáticamente.

**Solución**: Actualizar canales cuando cambia el título de un nodo.

### 4. ℹ️ BAJO: Warnings de código no usado
**Problema**: Algunos métodos tienen warnings porque aún no se usan activamente.

**Solución**: Agregar `#[allow(dead_code)]` o implementar el uso completo.

### 5. ⚠️ MEDIO: Validación de expresiones
**Problema**: No hay validación de que las expresiones `ch()` referencien nodos que existen.

**Solución**: Validar referencias antes de evaluar.

## Implementación de Soluciones

Ver código en `src/app.rs` y `src/expressions/evaluator.rs` para las correcciones.

