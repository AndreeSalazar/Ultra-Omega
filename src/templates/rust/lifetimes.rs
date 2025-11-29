// ═══════════════════════════════════════
// Lifetimes en Rust
// Referencias, préstamos, lifetime annotations
// ═══════════════════════════════════════

// ═══════════════════════════════════════
// LIFETIME BÁSICO
// ═══════════════════════════════════════
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ═══════════════════════════════════════
// STRUCT CON LIFETIME
// ═══════════════════════════════════════
#[derive(Debug)]
struct Extracto<'a> {
    texto: &'a str,
}

impl<'a> Extracto<'a> {
    fn new(texto: &'a str) -> Self {
        Extracto { texto }
    }
    
    fn longitud(&self) -> usize {
        self.texto.len()
    }
    
    fn obtener_primeras(&self, n: usize) -> &'a str {
        if n > self.texto.len() {
            self.texto
        } else {
            &self.texto[..n]
        }
    }
}

// ═══════════════════════════════════════
// MÚLTIPLES LIFETIMES
// ═══════════════════════════════════════
fn combinar<'a, 'b>(x: &'a str, y: &'b str) -> String {
    format!("{} {}", x, y)
}

// Lifetime elision (Rust infiere automáticamente)
fn primera_palabra(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

// ═══════════════════════════════════════
// LIFETIME STATIC
// ═══════════════════════════════════════
fn obtener_mensaje() -> &'static str {
    "Este string vive durante todo el programa"
}

// ═══════════════════════════════════════
// ENUM CON LIFETIME
// ═══════════════════════════════════════
#[derive(Debug)]
enum Mensaje<'a> {
    Texto(&'a str),
    Numero(i32),
    Combinado { parte1: &'a str, parte2: &'a str },
}

impl<'a> Mensaje<'a> {
    fn mostrar(&self) {
        match self {
            Mensaje::Texto(t) => println!("Texto: {}", t),
            Mensaje::Numero(n) => println!("Número: {}", n),
            Mensaje::Combinado { parte1, parte2 } => {
                println!("Combinado: {} + {}", parte1, parte2)
            }
        }
    }
}

// ═══════════════════════════════════════
// LIFETIME EN TRAITS
// ═══════════════════════════════════════
trait Procesador<'a> {
    fn procesar(&self, entrada: &'a str) -> &'a str;
}

struct ProcesadorTexto;

impl<'a> Procesador<'a> for ProcesadorTexto {
    fn procesar(&self, entrada: &'a str) -> &'a str {
        entrada.trim()
    }
}

// ═══════════════════════════════════════
// LIFETIME EN GENÉRICOS
// ═══════════════════════════════════════
struct ContenedorRef<'a, T> {
    valor: &'a T,
}

impl<'a, T> ContenedorRef<'a, T> {
    fn new(valor: &'a T) -> Self {
        ContenedorRef { valor }
    }
    
    fn obtener(&self) -> &'a T {
        self.valor
    }
}

fn main() {
    println!("=== LIFETIMES EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // LIFETIME BÁSICO
    // ═══════════════════════════════════════
    println!("=== LIFETIME BÁSICO ===");
    
    let str1 = "largo";
    let str2 = "más largo aún";
    let resultado = mas_largo(str1, str2);
    println!("Más largo: '{}'", resultado);
    
    // ═══════════════════════════════════════
    // STRUCT CON LIFETIME
    // ═══════════════════════════════════════
    println!("\n=== STRUCT CON LIFETIME ===");
    
    let texto = String::from("Hola Mundo Rust");
    let extracto = Extracto::new(&texto);
    
    println!("Extracto: {:?}", extracto);
    println!("Longitud: {}", extracto.longitud());
    println!("Primeras 5 letras: '{}'", extracto.obtener_primeras(5));
    
    // ═══════════════════════════════════════
    // LIFETIME ELISION
    // ═══════════════════════════════════════
    println!("\n=== LIFETIME ELISION ===");
    
    let oracion = "Rust es genial para sistemas";
    let palabra = primera_palabra(oracion);
    println!("Primera palabra de '{}': '{}'", oracion, palabra);
    
    // ═══════════════════════════════════════
    // LIFETIME STATIC
    // ═══════════════════════════════════════
    println!("\n=== LIFETIME STATIC ===");
    
    let mensaje = obtener_mensaje();
    println!("Mensaje estático: {}", mensaje);
    
    // ═══════════════════════════════════════
    // ENUM CON LIFETIME
    // ═══════════════════════════════════════
    println!("\n=== ENUM CON LIFETIME ===");
    
    let texto1 = "Hola";
    let texto2 = "Mundo";
    
    let mensajes = vec![
        Mensaje::Texto("Saludo"),
        Mensaje::Numero(42),
        Mensaje::Combinado { parte1: texto1, parte2: texto2 },
    ];
    
    for msg in &mensajes {
        msg.mostrar();
    }
    
    // ═══════════════════════════════════════
    // TRAIT CON LIFETIME
    // ═══════════════════════════════════════
    println!("\n=== TRAIT CON LIFETIME ===");
    
    let procesador = ProcesadorTexto;
    let entrada = "  texto con espacios  ";
    let resultado = procesador.procesar(entrada);
    println!("Procesado: '{}'", resultado);
    
    // ═══════════════════════════════════════
    // GENÉRICOS CON LIFETIMES
    // ═══════════════════════════════════════
    println!("\n=== GENÉRICOS CON LIFETIMES ===");
    
    let valor = 42;
    let contenedor = ContenedorRef::new(&valor);
    println!("Valor en contenedor: {}", contenedor.obtener());
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: Parser de Texto
    // ═══════════════════════════════════════
    println!("\n=== PARSER DE TEXTO ===");
    
    struct Parser<'a> {
        texto: &'a str,
    }
    
    impl<'a> Parser<'a> {
        fn new(texto: &'a str) -> Self {
            Parser { texto }
        }
        
        fn buscar(&self, patron: &str) -> Option<&'a str> {
            self.texto.find(patron).map(|i| &self.texto[i..i + patron.len()])
        }
        
        fn extraer_entre(&self, inicio: &str, fin: &str) -> Option<&'a str> {
            let start = self.texto.find(inicio)?;
            let start_pos = start + inicio.len();
            let end = self.texto[start_pos..].find(fin)?;
            Some(&self.texto[start_pos..start_pos + end])
        }
    }
    
    let texto = "Nombre: Juan, Edad: 25";
    let parser = Parser::new(texto);
    
    if let Some(nombre) = parser.extraer_entre("Nombre: ", ",") {
        println!("Nombre extraído: '{}'", nombre);
    }
    
    if let Some(edad) = parser.extraer_entre("Edad: ", "") {
        println!("Edad extraída: '{}'", edad);
    }
}

