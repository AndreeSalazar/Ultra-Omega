# ☕ Java 25 - Templates y Guía Completa

## 📋 Tabla de Contenidos

1. [Introducción](#introducción)
2. [Características de Java 25](#características-de-java-25)
3. [Templates Disponibles](#templates-disponibles)
4. [Instalación y Configuración](#instalación-y-configuración)
5. [Uso en Ultra-Omega](#uso-en-ultra-omega)
6. [Ejemplos](#ejemplos)
7. [Mejores Prácticas](#mejores-prácticas)

---

## 🎯 Introducción

Java 25 es la última versión del lenguaje de programación Java, introduciendo características modernas y mejoras significativas en rendimiento, seguridad y productividad. Ultra-Omega proporciona soporte completo para Java 25 con templates organizados por nivel de complejidad.

### ¿Por qué Java 25?

- **Virtual Threads**: Concurrencia ligera y escalable (Project Loom)
- **Pattern Matching**: Sintaxis moderna para matching de patrones
- **Records**: Clases de datos inmutables de forma concisa
- **Sealed Classes**: Control explícito de herencia
- **Text Blocks**: Strings multilínea mejorados
- **Foreign Memory API**: Acceso seguro a memoria nativa
- **Structured Concurrency**: Gestión estructurada de concurrencia

---

## 🚀 Características de Java 25

### Características Principales

#### 1. **Virtual Threads (Java 19+)**
Threads ligeros que permiten crear millones de threads concurrentes sin sobrecarga significativa.

```java
Thread virtualThread = Thread.ofVirtual().start(() -> {
    // Tu código aquí
});
```

#### 2. **Pattern Matching (Java 16+)**
Sintaxis moderna para matching de patrones con `instanceof` y `switch`.

```java
if (obj instanceof String s) {
    System.out.println(s.toUpperCase());
}

String result = switch (obj) {
    case String s -> "String: " + s;
    case Integer i -> "Integer: " + i;
    default -> "Unknown";
};
```

#### 3. **Records (Java 14+)**
Clases de datos inmutables de forma concisa.

```java
record Persona(String nombre, int edad) {
    public String presentacion() {
        return "Hola, soy " + nombre;
    }
}
```

#### 4. **Sealed Classes (Java 17+)**
Control explícito de qué clases pueden extender una clase base.

```java
sealed class Forma permits Circulo, Rectangulo {}
final class Circulo extends Forma {}
final class Rectangulo extends Forma {}
```

#### 5. **Text Blocks (Java 13+)**
Strings multilínea mejorados.

```java
String json = """
    {
        "nombre": "Java 25",
        "version": "25.0"
    }
    """;
```

#### 6. **Switch Expressions (Java 14+)**
Expresiones `switch` que retornan valores.

```java
String dia = switch (numero) {
    case 1 -> "Lunes";
    case 2 -> "Martes";
    default -> "Desconocido";
};
```

#### 7. **Record Patterns (Java 19+)**
Pattern matching con records.

```java
if (punto instanceof Punto(int x, int y)) {
    System.out.println("Coordenadas: " + x + ", " + y);
}
```

#### 8. **Foreign Memory API (Java 19+)**
Acceso seguro a memoria fuera del heap de Java.

#### 9. **Structured Concurrency (Java 21+)**
Gestión estructurada de concurrencia con mejor manejo de errores.

---

## 📚 Templates Disponibles

### 🟢 Básico

1. **Hola Mundo** - Primer programa en Java
2. **Variables y Tipos** - Tipos de datos primitivos y referencias
3. **Clases y Objetos** - Programación orientada a objetos básica
4. **Métodos y Funciones** - Definición y uso de métodos

### 🟡 Intermedio

5. **Colecciones** - List, Set, Map y Stream API
6. **Genéricos** - Programación genérica con `<T>`
7. **Excepciones** - Manejo de errores con try-catch
8. **Interfaces** - Contratos y polimorfismo
9. **Herencia** - Extensión de clases y polimorfismo

### 🔴 Avanzado

10. **Streams API** - Programación funcional con streams
11. **Lambdas** - Expresiones lambda y functional interfaces
12. **Concurrencia** - Threads, Executors, CompletableFuture
13. **Reflection** - Introspección de clases en tiempo de ejecución
14. **Anotaciones** - Metadatos y anotaciones personalizadas
15. **Records** - Clases de datos inmutables (Java 14+)
16. **Sealed Classes** - Control de herencia (Java 17+)
17. **Pattern Matching** - Matching de patrones moderno (Java 16+)
18. **Text Blocks** - Strings multilínea (Java 13+)
19. **Virtual Threads** - Threads ligeros (Java 19+)
20. **Switch Expressions** - Expresiones switch (Java 14+)
21. **Record Patterns** - Pattern matching con records (Java 19+)
22. **Foreign Memory API** - Acceso a memoria nativa (Java 19+)
23. **Structured Concurrency** - Concurrencia estructurada (Java 21+)

### 📚 Librerías

24. **Utils** - Utilidades generales (validación, retry, medición de tiempo)
25. **Collections** - Extensiones de colecciones (partición, reversión, etc.)
26. **I/O** - Operaciones de entrada/salida mejoradas
27. **Async** - Programación asíncrona con CompletableFuture

---

## 🔧 Instalación y Configuración

### Requisitos

- **JDK 25** (Java Development Kit 25)
- **javac** (compilador Java) en PATH
- **java** (runtime Java) en PATH

### Instalación del JDK 25

#### Windows

```powershell
# Opción 1: Chocolatey
choco install temurin25-jdk

# Opción 2: Descarga manual
# Visita: https://adoptium.net/
# Descarga JDK 25 y agrega a PATH
```

#### Linux

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install openjdk-25-jdk

# O desde Adoptium
wget https://github.com/adoptium/temurin25-binaries/releases/download/...
```

#### macOS

```bash
# Homebrew
brew install --cask temurin25

# O descarga desde Adoptium
```

### Verificar Instalación

```bash
java -version
javac -version
```

Deberías ver algo como:
```
openjdk version "25" 2025-XX-XX
OpenJDK Runtime Environment (build 25+XX)
OpenJDK 64-Bit Server VM (build 25+XX, mixed mode, sharing)
```

---

## 💻 Uso en Ultra-Omega

### Crear un Nodo Java

1. **Abrir el menú de templates**: Presiona `Shift+A` o haz clic derecho
2. **Seleccionar categoría "Java"**: En el panel izquierdo
3. **Elegir template**: Selecciona el template deseado de la lista
4. **Editar código**: Haz doble clic en el nodo o presiona `Ctrl+I`

### Compilar y Ejecutar

1. **Seleccionar nodo**: Haz clic en el nodo Java
2. **Compilar**: Presiona `F5` o `Run > Run Selected Node`
3. **Ver resultado**: El terminal mostrará la compilación y ejecución

### Características Especiales

- **Detección automática**: Ultra-Omega detecta automáticamente la clase principal
- **Compilación con Java 25**: Usa `--source 25 --target 25 --enable-preview`
- **Ejecución automática**: Ejecuta el programa después de compilar exitosamente

---

## 📖 Ejemplos

### Ejemplo 1: Hola Mundo

```java
public class Main {
    public static void main(String[] args) {
        System.out.println("¡Hola desde Java 25 en Ultra-Omega! ☕");
    }
}
```

### Ejemplo 2: Records y Pattern Matching

```java
record Persona(String nombre, int edad) {}

public class Main {
    public static void main(String[] args) {
        Object obj = new Persona("Alice", 25);
        
        if (obj instanceof Persona(String nombre, int edad)) {
            System.out.println("Nombre: " + nombre);
            System.out.println("Edad: " + edad);
        }
    }
}
```

### Ejemplo 3: Virtual Threads

```java
import java.util.concurrent.*;

public class Main {
    public static void main(String[] args) {
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            for (int i = 0; i < 1000; i++) {
                final int id = i;
                executor.submit(() -> {
                    System.out.println("Tarea " + id + " ejecutada");
                });
            }
        }
    }
}
```

### Ejemplo 4: Streams API

```java
import java.util.*;

public class Main {
    public static void main(String[] args) {
        List<Integer> numeros = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        
        int suma = numeros.stream()
            .filter(n -> n % 2 == 0)
            .mapToInt(Integer::intValue)
            .sum();
        
        System.out.println("Suma de pares: " + suma);
    }
}
```

---

## ✨ Mejores Prácticas

### 1. Usar Records para Datos Inmutables

```java
// ✅ Bueno
record Punto(int x, int y) {}

// ❌ Evitar clases verbosas para datos simples
class Punto {
    private final int x, y;
    // ... getters, equals, hashCode, toString ...
}
```

### 2. Aprovechar Pattern Matching

```java
// ✅ Bueno (Java 16+)
if (obj instanceof String s) {
    System.out.println(s.toUpperCase());
}

// ❌ Evitar casting manual
if (obj instanceof String) {
    String s = (String) obj;
    System.out.println(s.toUpperCase());
}
```

### 3. Usar Virtual Threads para I/O

```java
// ✅ Bueno (Java 19+)
try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
    executor.submit(() -> {
        // Operación I/O blocking
    });
}

// ❌ Evitar threads pesados para I/O
ExecutorService executor = Executors.newFixedThreadPool(100);
```

### 4. Aprovechar Text Blocks

```java
// ✅ Bueno (Java 13+)
String json = """
    {
        "nombre": "Java 25",
        "version": "25.0"
    }
    """;

// ❌ Evitar concatenación manual
String json = "{\n" +
    "    \"nombre\": \"Java 25\",\n" +
    "    \"version\": \"25.0\"\n" +
    "}";
```

### 5. Usar Sealed Classes para Control de Herencia

```java
// ✅ Bueno (Java 17+)
sealed class Forma permits Circulo, Rectangulo {}
final class Circulo extends Forma {}
final class Rectangulo extends Forma {}

// Permite pattern matching exhaustivo
double area = switch (forma) {
    case Circulo c -> Math.PI * c.radio() * c.radio();
    case Rectangulo r -> r.ancho() * r.alto();
};
```

---

## 🔗 Recursos Adicionales

- **Documentación Oficial**: https://docs.oracle.com/en/java/javase/25/
- **Adoptium (JDK)**: https://adoptium.net/
- **Java Language Updates**: https://openjdk.org/projects/jdk/25/
- **Project Loom (Virtual Threads)**: https://openjdk.org/projects/loom/
- **JEP Index**: https://openjdk.org/jeps/

---

## 📝 Notas de Compilación

### Flags de Compilación

Ultra-Omega compila automáticamente con:
- `--source 25`: Especifica versión fuente Java 25
- `--target 25`: Especifica versión objetivo Java 25
- `--enable-preview`: Habilita características preview de Java 25

### Características Preview

Algunas características de Java 25 pueden estar en estado "preview". Para usarlas:
- Compilar con `--enable-preview`
- Ejecutar con `--enable-preview`

Ultra-Omega maneja esto automáticamente.

---

## 🎓 Aprendizaje Progresivo

### Ruta Recomendada

1. **Básico**: Hola Mundo → Variables → Clases → Métodos
2. **Intermedio**: Colecciones → Genéricos → Excepciones → Interfaces → Herencia
3. **Avanzado**: Streams → Lambdas → Concurrencia → Records → Pattern Matching
4. **Especializado**: Virtual Threads → Structured Concurrency → Foreign Memory

### Orden Sugerido de Templates

1. `hello_world.java` - Primer contacto
2. `variables.java` - Fundamentos
3. `classes.java` - OOP básico
4. `collections.java` - Estructuras de datos
5. `streams.java` - Programación funcional
6. `records.java` - Datos inmutables modernos
7. `pattern_matching.java` - Sintaxis moderna
8. `virtual_threads.java` - Concurrencia moderna

---

## 🐛 Solución de Problemas

### Error: "javac no encontrado"

**Solución**: Instala el JDK 25 y agrega `javac` al PATH.

### Error: "UnsupportedClassVersionError"

**Solución**: Asegúrate de tener Java 25 instalado y en PATH.

### Error: "preview features are not enabled"

**Solución**: Ultra-Omega debería habilitar `--enable-preview` automáticamente. Si persiste, verifica que estés usando JDK 25.

### Error: "cannot find symbol"

**Solución**: Verifica que todas las clases estén en el mismo paquete o que uses imports correctos.

---

## 📊 Comparación con Otras Versiones

| Característica | Java 8 | Java 11 | Java 17 | Java 21 | **Java 25** |
|---------------|--------|---------|---------|---------|-------------|
| Records | ❌ | ❌ | ✅ | ✅ | ✅ |
| Pattern Matching | ❌ | ❌ | ✅ | ✅ | ✅ |
| Text Blocks | ❌ | ❌ | ✅ | ✅ | ✅ |
| Sealed Classes | ❌ | ❌ | ✅ | ✅ | ✅ |
| Virtual Threads | ❌ | ❌ | ❌ | ✅ | ✅ |
| Structured Concurrency | ❌ | ❌ | ❌ | ✅ | ✅ |
| Foreign Memory API | ❌ | ❌ | ❌ | ✅ | ✅ |

---

## 🎉 Conclusión

Java 25 trae características modernas que hacen la programación más productiva y segura. Ultra-Omega proporciona templates completos y actualizados para aprovechar al máximo estas características.

¡Disfruta programando con Java 25 en Ultra-Omega! ☕

---

**Última actualización**: 2025-01-27  
**Versión de Java soportada**: Java 25 (JDK 25)  
**Templates disponibles**: 27 templates organizados en 4 categorías

