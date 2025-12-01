// ═══════════════════════════════════════
// Manejo de Errores en Rust
// Result, Option, ?, unwrap, expect
// ═══════════════════════════════════════

use std::fs::File;
use std::io::{self, Read};

// ═══════════════════════════════════════
// ERROR PERSONALIZADO
// ═══════════════════════════════════════
#[derive(Debug)]
enum MiError {
    ArchivoNoEncontrado(String),
    DivisionPorCero,
    NumeroInvalido(String),
    IOError(String),
}

impl std::fmt::Display for MiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MiError::ArchivoNoEncontrado(nombre) => {
                write!(f, "Archivo no encontrado: {}", nombre)
            }
            MiError::DivisionPorCero => write!(f, "División por cero"),
            MiError::NumeroInvalido(msg) => write!(f, "Número inválido: {}", msg),
            MiError::IOError(msg) => write!(f, "Error de IO: {}", msg),
        }
    }
}

impl std::error::Error for MiError {}

// ═══════════════════════════════════════
// FUNCIONES QUE RETORNAN RESULT
// ═══════════════════════════════════════

fn dividir(a: f64, b: f64) -> Result<f64, MiError> {
    if b == 0.0 {
        Err(MiError::DivisionPorCero)
    } else {
        Ok(a / b)
    }
}

fn parsear_numero(texto: &str) -> Result<i32, MiError> {
    texto.parse::<i32>()
        .map_err(|_| MiError::NumeroInvalido(texto.to_string()))
}

fn leer_archivo(nombre: &str) -> Result<String, MiError> {
    // Simulación - en realidad usarías std::fs::read_to_string
    if nombre == "no_existe.txt" {
        Err(MiError::ArchivoNoEncontrado(nombre.to_string()))
    } else {
        Ok(format!("Contenido del archivo: {}", nombre))
    }
}

// ═══════════════════════════════════════
// PROPAGACIÓN DE ERRORES CON ?
// ═══════════════════════════════════════

fn procesar_calculo(texto_a: &str, texto_b: &str) -> Result<f64, MiError> {
    // El ? propaga el error automáticamente
    let a = parsear_numero(texto_a)?;
    let b = parsear_numero(texto_b)?;
    
    let resultado = dividir(a as f64, b as f64)?;
    
    Ok(resultado)
}

fn procesar_archivo_y_calcular(nombre: &str) -> Result<f64, MiError> {
    let contenido = leer_archivo(nombre)?;
    
    // Extraer números del contenido (simplificado)
    let partes: Vec<&str> = contenido.split_whitespace().collect();
    if partes.len() >= 2 {
        procesar_calculo(partes[0], partes[1])
    } else {
        Err(MiError::NumeroInvalido("Formato inválido".to_string()))
    }
}

// ═══════════════════════════════════════
// COMBINADORES: map, and_then, or_else
// ═══════════════════════════════════════

fn ejemplo_combinadores() {
    println!("\n=== COMBINADORES ===");
    
    // map: transforma el valor Ok
    let resultado = dividir(10.0, 2.0)
        .map(|x| x * 2.0)
        .map(|x| x + 1.0);
    println!("(10/2)*2+1 = {:?}", resultado);
    
    // and_then: encadena operaciones que retornan Result
    let resultado = dividir(20.0, 4.0)
        .and_then(|x| dividir(x, 2.0));
    println!("(20/4)/2 = {:?}", resultado);
    
    // or_else: maneja errores
    let resultado = dividir(10.0, 0.0)
        .or_else(|_| dividir(10.0, 1.0));
    println!("Fallback: {:?}", resultado);
    
    // unwrap_or: valor por defecto
    let resultado = dividir(10.0, 0.0)
        .unwrap_or(0.0);
    println!("Con unwrap_or: {}", resultado);
}

// ═══════════════════════════════════════
// OPTION Y RESULT JUNTOS
// ═══════════════════════════════════════

fn buscar_y_dividir(numeros: &[i32], indice: usize, divisor: i32) -> Result<Option<f64>, MiError> {
    // Option para índice válido
    let valor = numeros.get(indice)
        .ok_or_else(|| MiError::NumeroInvalido("Índice fuera de rango".to_string()))?;
    
    // Result para división
    let resultado = dividir(*valor as f64, divisor as f64)?;
    
    Ok(Some(resultado))
}

// ═══════════════════════════════════════
// MAIN
// ═══════════════════════════════════════

fn main() {
    println!("=== MANEJO DE ERRORES EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // MATCH CON RESULT
    // ═══════════════════════════════════════
    println!("=== MATCH CON RESULT ===");
    
    match dividir(10.0, 2.0) {
        Ok(resultado) => println!("✅ 10 / 2 = {}", resultado),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    match dividir(10.0, 0.0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // ═══════════════════════════════════════
    // PROPAGACIÓN CON ?
    // ═══════════════════════════════════════
    println!("\n=== PROPAGACIÓN CON ? ===");
    
    match procesar_calculo("20", "4") {
        Ok(resultado) => println!("✅ 20 / 4 = {}", resultado),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    match procesar_calculo("20", "0") {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    match procesar_calculo("abc", "4") {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // ═══════════════════════════════════════
    // COMBINADORES
    // ═══════════════════════════════════════
    ejemplo_combinadores();
    
    // ═══════════════════════════════════════
    // OPTION Y RESULT
    // ═══════════════════════════════════════
    println!("\n=== OPTION Y RESULT ===");
    
    let numeros = vec![10, 20, 30, 40];
    
    match buscar_y_dividir(&numeros, 1, 4) {
        Ok(Some(resultado)) => println!("✅ números[1] / 4 = {}", resultado),
        Ok(None) => println!("Sin resultado"),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    match buscar_y_dividir(&numeros, 10, 4) {
        Ok(Some(resultado)) => println!("Resultado: {}", resultado),
        Ok(None) => println!("Sin resultado"),
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // ═══════════════════════════════════════
    // UNWRAP Y EXPECT (¡Cuidado!)
    // ═══════════════════════════════════════
    println!("\n=== UNWRAP Y EXPECT ===");
    println!("⚠️  Usar solo cuando estés 100% seguro");
    
    // unwrap: pánico si hay error
    let resultado = dividir(10.0, 2.0).unwrap();
    println!("unwrap exitoso: {}", resultado);
    
    // expect: pánico con mensaje personalizado
    let resultado = dividir(20.0, 4.0)
        .expect("La división debería funcionar");
    println!("expect exitoso: {}", resultado);
    
    // unwrap_or: valor por defecto
    let resultado = dividir(10.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or: {}", resultado);
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: Calculadora Segura
    // ═══════════════════════════════════════
    println!("\n=== CALCULADORA SEGURA ===");
    
    fn calculadora_segura(operacion: &str, a: &str, b: &str) -> Result<f64, MiError> {
        let num_a = parsear_numero(a)?;
        let num_b = parsear_numero(b)?;
        
        match operacion {
            "+" => Ok((num_a + num_b) as f64),
            "-" => Ok((num_a - num_b) as f64),
            "*" => Ok((num_a * num_b) as f64),
            "/" => dividir(num_a as f64, num_b as f64),
            _ => Err(MiError::NumeroInvalido(format!("Operación desconocida: {}", operacion))),
        }
    }
    
    let operaciones = vec![
        ("+", "10", "5"),
        ("-", "10", "5"),
        ("*", "10", "5"),
        ("/", "10", "5"),
        ("/", "10", "0"),
        ("%", "10", "5"),
    ];
    
    for (op, a, b) in operaciones {
        match calculadora_segura(op, a, b) {
            Ok(resultado) => println!("  {} {} {} = {}", a, op, b, resultado),
            Err(e) => println!("  {} {} {} = Error: {}", a, op, b, e),
        }
    }
}

