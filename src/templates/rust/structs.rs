// ═══════════════════════════════════════
// Structs y Enums en Rust
// ═══════════════════════════════════════

// ─────────────────────────────────────
// Struct básico
// ─────────────────────────────────────
#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u32,
    altura: f64,
}

impl Persona {
    // Constructor
    fn new(nombre: &str, edad: u32, altura: f64) -> Self {
        Persona {
            nombre: nombre.to_string(),
            edad,
            altura,
        }
    }
    
    // Método
    fn presentarse(&self) {
        println!("Hola, soy {} y tengo {} años", self.nombre, self.edad);
    }
    
    // Método mutable
    fn cumplir_anios(&mut self) {
        self.edad += 1;
    }
}

// ─────────────────────────────────────
// Tuple Struct
// ─────────────────────────────────────
#[derive(Debug)]
struct Punto3D(f64, f64, f64);

// ─────────────────────────────────────
// Enum con datos
// ─────────────────────────────────────
#[derive(Debug)]
enum Mensaje {
    Texto(String),
    Numero(i32),
    Posicion { x: i32, y: i32 },
    Vacio,
}

impl Mensaje {
    fn procesar(&self) {
        match self {
            Mensaje::Texto(s) => println!("Texto: {}", s),
            Mensaje::Numero(n) => println!("Número: {}", n),
            Mensaje::Posicion { x, y } => println!("Posición: ({}, {})", x, y),
            Mensaje::Vacio => println!("Mensaje vacío"),
        }
    }
}

// ─────────────────────────────────────
// Struct genérico
// ─────────────────────────────────────
#[derive(Debug)]
struct Contenedor<T> {
    valor: T,
}

impl<T> Contenedor<T> {
    fn new(valor: T) -> Self {
        Contenedor { valor }
    }
    
    fn obtener(&self) -> &T {
        &self.valor
    }
}

fn main() {
    // Crear persona
    let mut persona = Persona::new("Juan", 25, 1.75);
    persona.presentarse();
    
    persona.cumplir_anios();
    println!("Después de cumpleaños: {:?}", persona);
    
    // Tuple struct
    let punto = Punto3D(1.0, 2.0, 3.0);
    println!("Punto: {:?}", punto);
    
    // Enums
    let mensajes = vec![
        Mensaje::Texto(String::from("Hola")),
        Mensaje::Numero(42),
        Mensaje::Posicion { x: 10, y: 20 },
        Mensaje::Vacio,
    ];
    
    println!("\n=== Mensajes ===");
    for msg in &mensajes {
        msg.procesar();
    }
    
    // Genéricos
    let num = Contenedor::new(42);
    let texto = Contenedor::new("Hola Rust");
    
    println!("\n=== Contenedores ===");
    println!("Número: {}", num.obtener());
    println!("Texto: {}", texto.obtener());
}

