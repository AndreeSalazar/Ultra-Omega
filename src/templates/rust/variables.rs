// ═══════════════════════════════════════
// Variables y Tipos de Datos en Rust
// ═══════════════════════════════════════

fn main() {
    // Variables inmutables (por defecto)
    let entero: i32 = 42;
    let decimal: f64 = 3.14159;
    let booleano: bool = true;
    let caracter: char = '🦀';
    
    // Variables mutables
    let mut contador = 0;
    contador += 1;
    
    // Inferencia de tipos
    let automatico = 100;  // i32
    let flotante = 3.14;   // f64
    
    // Tipos específicos
    let byte: u8 = 255;
    let entero_64: i64 = 9_223_372_036_854_775_807;
    let sin_signo: usize = 1000;
    
    // Strings
    let string_literal: &str = "Hola Rust";
    let string_owned: String = String::from("String propio");
    
    // Tuplas
    let tupla: (i32, f64, &str) = (42, 3.14, "hola");
    let (x, y, z) = tupla;  // Destructuring
    
    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_repetido = [0; 10];  // [0, 0, 0, ..., 0]
    
    // Vectores
    let mut vector: Vec<i32> = vec![1, 2, 3];
    vector.push(4);
    
    // Constantes
    const PI: f64 = 3.141592653589793;
    
    // Imprimir
    println!("Entero: {}", entero);
    println!("Decimal: {:.5}", decimal);
    println!("Caracter: {}", caracter);
    println!("String: {}", string_owned);
    println!("Tupla: {:?}", tupla);
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
    println!("PI: {}", PI);
}

