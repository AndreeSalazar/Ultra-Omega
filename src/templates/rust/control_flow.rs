// ═══════════════════════════════════════
// Control de Flujo en Rust
// if/else, loops, match, pattern matching
// ═══════════════════════════════════════

fn main() {
    // ═══════════════════════════════════════
    // IF/ELSE
    // ═══════════════════════════════════════
    println!("=== IF/ELSE ===");
    
    let numero = 42;
    
    if numero > 50 {
        println!("Número grande");
    } else if numero > 25 {
        println!("Número mediano");
    } else {
        println!("Número pequeño");
    }
    
    // If como expresión
    let resultado = if numero % 2 == 0 {
        "par"
    } else {
        "impar"
    };
    println!("{} es {}", numero, resultado);
    
    // ═══════════════════════════════════════
    // LOOPS
    // ═══════════════════════════════════════
    println!("\n=== LOOPS ===");
    
    // Loop infinito con break
    let mut contador = 0;
    let resultado_loop = loop {
        contador += 1;
        if contador == 5 {
            break contador * 2;  // Retorna valor
        }
    };
    println!("Resultado del loop: {}", resultado_loop);
    
    // While
    let mut x = 3;
    while x > 0 {
        println!("x = {}", x);
        x -= 1;
    }
    
    // For con rango
    println!("\nFor con rango:");
    for i in 1..=5 {
        println!("i = {}", i);
    }
    
    // For con iterador
    println!("\nFor con array:");
    let array = [10, 20, 30, 40, 50];
    for (indice, valor) in array.iter().enumerate() {
        println!("[{}] = {}", indice, valor);
    }
    
    // ═══════════════════════════════════════
    // MATCH (Pattern Matching)
    // ═══════════════════════════════════════
    println!("\n=== MATCH ===");
    
    let valor = 3;
    match valor {
        1 => println!("Uno"),
        2 | 3 => println!("Dos o Tres"),
        4..=10 => println!("Entre 4 y 10"),
        _ => println!("Otro valor"),
    }
    
    // Match con enums
    #[derive(Debug)]
    enum Estado {
        Cargando,
        Listo,
        Error(String),
    }
    
    let estados = vec![
        Estado::Cargando,
        Estado::Listo,
        Estado::Error(String::from("Conexión perdida")),
    ];
    
    for estado in estados {
        match estado {
            Estado::Cargando => println!("⏳ Cargando..."),
            Estado::Listo => println!("✅ Listo"),
            Estado::Error(msg) => println!("❌ Error: {}", msg),
        }
    }
    
    // Match con guards
    let numero = Some(42);
    match numero {
        Some(n) if n > 50 => println!("Número grande: {}", n),
        Some(n) if n > 0 => println!("Número positivo: {}", n),
        Some(n) => println!("Número: {}", n),
        None => println!("Sin número"),
    }
    
    // ═══════════════════════════════════════
    // IF LET / WHILE LET
    // ═══════════════════════════════════════
    println!("\n=== IF LET ===");
    
    let opcion = Some(5);
    if let Some(valor) = opcion {
        println!("Valor encontrado: {}", valor);
    }
    
    // ═══════════════════════════════════════
    // PATTERN MATCHING AVANZADO
    // ═══════════════════════════════════════
    println!("\n=== PATTERN MATCHING AVANZADO ===");
    
    // Destructuring de tuplas
    let tupla = (1, 2.5, "hola");
    match tupla {
        (x, y, z) => println!("x={}, y={}, z={}", x, y, z),
    }
    
    // Destructuring de structs
    #[derive(Debug)]
    struct Punto {
        x: i32,
        y: i32,
    }
    
    let punto = Punto { x: 10, y: 20 };
    match punto {
        Punto { x, y } if x == y => println!("Punto diagonal: ({}, {})", x, y),
        Punto { x, y } => println!("Punto: ({}, {})", x, y),
    }
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: Calculadora
    // ═══════════════════════════════════════
    println!("\n=== CALCULADORA ===");
    
    enum Operacion {
        Suma(i32, i32),
        Resta(i32, i32),
        Multiplicacion(i32, i32),
        Division(i32, i32),
    }
    
    let operaciones = vec![
        Operacion::Suma(10, 5),
        Operacion::Resta(10, 5),
        Operacion::Multiplicacion(10, 5),
        Operacion::Division(10, 5),
    ];
    
    for op in operaciones {
        let resultado = match op {
            Operacion::Suma(a, b) => Some(a + b),
            Operacion::Resta(a, b) => Some(a - b),
            Operacion::Multiplicacion(a, b) => Some(a * b),
            Operacion::Division(a, b) => {
                if b != 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
        };
        
        match resultado {
            Some(r) => println!("Resultado: {}", r),
            None => println!("Error: División por cero"),
        }
    }
}

