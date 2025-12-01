// ═══════════════════════════════════════
// Traits en Rust
// ═══════════════════════════════════════

use std::fmt::Display;

// ─────────────────────────────────────
// Definir un trait
// ─────────────────────────────────────
trait Saludable {
    fn saludar(&self) -> String;
    
    // Método con implementación por defecto
    fn despedirse(&self) -> String {
        String::from("¡Adiós!")
    }
}

// ─────────────────────────────────────
// Structs que implementan el trait
// ─────────────────────────────────────
struct Persona {
    nombre: String,
}

struct Robot {
    modelo: String,
}

impl Saludable for Persona {
    fn saludar(&self) -> String {
        format!("¡Hola! Soy {}", self.nombre)
    }
}

impl Saludable for Robot {
    fn saludar(&self) -> String {
        format!("BEEP BOOP. Soy modelo {}", self.modelo)
    }
    
    fn despedirse(&self) -> String {
        String::from("SISTEMA APAGÁNDOSE...")
    }
}

// ─────────────────────────────────────
// Trait bounds
// ─────────────────────────────────────
fn imprimir_saludo<T: Saludable>(item: &T) {
    println!("{}", item.saludar());
}

// Sintaxis alternativa con where
fn interaccion<T, U>(a: &T, b: &U)
where
    T: Saludable + Display,
    U: Saludable,
{
    println!("{} saluda a {}", a, b.saludar());
}

// ─────────────────────────────────────
// impl Trait
// ─────────────────────────────────────
fn crear_saludador() -> impl Saludable {
    Persona {
        nombre: String::from("Anónimo"),
    }
}

// ─────────────────────────────────────
// Trait objects (dyn)
// ─────────────────────────────────────
fn procesar_saludadores(saludadores: Vec<Box<dyn Saludable>>) {
    for s in saludadores {
        println!("{}", s.saludar());
    }
}

// Implementar Display para Persona
impl Display for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Persona({})", self.nombre)
    }
}

fn main() {
    let persona = Persona { nombre: String::from("Juan") };
    let robot = Robot { modelo: String::from("R2-D2") };
    
    println!("=== Saludos ===");
    imprimir_saludo(&persona);
    imprimir_saludo(&robot);
    
    println!("\n=== Despedidas ===");
    println!("{}", persona.despedirse());
    println!("{}", robot.despedirse());
    
    println!("\n=== impl Trait ===");
    let anonimo = crear_saludador();
    println!("{}", anonimo.saludar());
    
    println!("\n=== Trait Objects ===");
    let saludadores: Vec<Box<dyn Saludable>> = vec![
        Box::new(Persona { nombre: String::from("María") }),
        Box::new(Robot { modelo: String::from("C-3PO") }),
    ];
    procesar_saludadores(saludadores);
}

