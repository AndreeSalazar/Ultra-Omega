// ═══════════════════════════════════════
// Módulos y Organización en Rust
// mod, pub, use, crate, super
// ═══════════════════════════════════════

// ═══════════════════════════════════════
// MÓDULO PRIVADO
// ═══════════════════════════════════════
mod matematicas {
    // Función privada (solo dentro del módulo)
    fn raiz_cuadrada(x: f64) -> f64 {
        x.sqrt()
    }
    
    // Función pública (accesible desde fuera)
    pub fn sumar(a: f64, b: f64) -> f64 {
        a + b
    }
    
    pub fn restar(a: f64, b: f64) -> f64 {
        a - b
    }
    
    pub fn multiplicar(a: f64, b: f64) -> f64 {
        a * b
    }
    
    // Submódulo
    pub mod avanzadas {
        pub fn potencia(base: f64, exponente: f64) -> f64 {
            base.powf(exponente)
        }
        
        pub fn logaritmo(x: f64) -> f64 {
            x.ln()
        }
    }
}

// ═══════════════════════════════════════
// MÓDULO CON STRUCT
// ═══════════════════════════════════════
mod geometria {
    pub struct Rectangulo {
        pub ancho: f64,
        pub alto: f64,
    }
    
    impl Rectangulo {
        pub fn new(ancho: f64, alto: f64) -> Self {
            Rectangulo { ancho, alto }
        }
        
        pub fn area(&self) -> f64 {
            self.ancho * self.alto
        }
        
        pub fn perimetro(&self) -> f64 {
            2.0 * (self.ancho + self.alto)
        }
    }
    
    // Struct privado
    struct Circulo {
        radio: f64,
    }
    
    impl Circulo {
        pub fn new(radio: f64) -> Self {
            Circulo { radio }
        }
        
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radio * self.radio
        }
    }
}

// ═══════════════════════════════════════
// USE PARA IMPORTAR
// ═══════════════════════════════════════
use matematicas::{sumar, restar, multiplicar};
use matematicas::avanzadas::{potencia, logaritmo};
use geometria::Rectangulo;

// ═══════════════════════════════════════
// RE-EXPORTAR
// ═══════════════════════════════════════
pub use matematicas::sumar as add;  // Re-exportar con alias

fn main() {
    println!("=== MÓDULOS EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // USAR FUNCIONES DEL MÓDULO
    // ═══════════════════════════════════════
    println!("=== FUNCIONES MATEMÁTICAS ===");
    
    println!("10 + 5 = {}", sumar(10.0, 5.0));
    println!("10 - 5 = {}", restar(10.0, 5.0));
    println!("10 * 5 = {}", multiplicar(10.0, 5.0));
    
    println!("\n=== FUNCIONES AVANZADAS ===");
    println!("2^8 = {}", potencia(2.0, 8.0));
    println!("ln(e) = {:.2}", logaritmo(std::f64::consts::E));
    
    // ═══════════════════════════════════════
    // USAR STRUCT DEL MÓDULO
    // ═══════════════════════════════════════
    println!("\n=== GEOMETRÍA ===");
    
    let rect = Rectangulo::new(10.0, 5.0);
    println!("Rectángulo: {}x{}", rect.ancho, rect.alto);
    println!("Área: {}", rect.area());
    println!("Perímetro: {}", rect.perimetro());
    
    // ═══════════════════════════════════════
    // RE-EXPORT
    // ═══════════════════════════════════════
    println!("\n=== RE-EXPORT ===");
    println!("add(3, 4) = {}", add(3.0, 4.0));
    
    // ═══════════════════════════════════════
    // USAR CON NOMBRE COMPLETO
    // ═══════════════════════════════════════
    println!("\n=== NOMBRE COMPLETO ===");
    let resultado = matematicas::sumar(100.0, 200.0);
    println!("matematicas::sumar(100, 200) = {}", resultado);
}

