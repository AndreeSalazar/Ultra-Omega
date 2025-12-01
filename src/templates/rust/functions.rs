// ═══════════════════════════════════════
// Funciones en Rust
// ═══════════════════════════════════════

// ─────────────────────────────────────
// Función simple
// ─────────────────────────────────────
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // Sin punto y coma = retorno implícito
}

// ─────────────────────────────────────
// Función con múltiples retornos
// ─────────────────────────────────────
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}

// ─────────────────────────────────────
// Función genérica
// ─────────────────────────────────────
fn maximo<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// ─────────────────────────────────────
// Función con closures
// ─────────────────────────────────────
fn aplicar_operacion<F>(x: i32, operacion: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operacion(x)
}

// ─────────────────────────────────────
// Función recursiva
// ─────────────────────────────────────
fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

// ─────────────────────────────────────
// Función con Option
// ─────────────────────────────────────
fn encontrar_primero_par(numeros: &[i32]) -> Option<i32> {
    numeros.iter().find(|&&x| x % 2 == 0).copied()
}

fn main() {
    // Función simple
    println!("5 + 3 = {}", sumar(5, 3));
    
    // Result
    match dividir(10, 2) {
        Ok(resultado) => println!("10 / 2 = {}", resultado),
        Err(e) => println!("Error: {}", e),
    }
    
    // Genéricos
    println!("Max(5, 3) = {}", maximo(5, 3));
    println!("Max(3.14, 2.71) = {}", maximo(3.14, 2.71));
    
    // Closures
    let doble = |x| x * 2;
    let triple = |x| x * 3;
    println!("Doble de 5: {}", aplicar_operacion(5, doble));
    println!("Triple de 5: {}", aplicar_operacion(5, triple));
    
    // Recursión
    println!("5! = {}", factorial(5));
    
    // Option
    let nums = [1, 3, 5, 8, 9];
    match encontrar_primero_par(&nums) {
        Some(n) => println!("Primer par: {}", n),
        None => println!("No hay pares"),
    }
}

