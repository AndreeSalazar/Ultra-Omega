// ═══════════════════════════════════════
// Ownership y Borrowing en Rust
// ═══════════════════════════════════════

// ─────────────────────────────────────
// Ownership - Propiedad
// ─────────────────────────────────────
fn demostrar_ownership() {
    let s1 = String::from("hola");
    let s2 = s1;  // s1 se mueve a s2
    
    // println!("{}", s1);  // ¡Error! s1 ya no es válido
    println!("s2: {}", s2);
    
    // Clone para copiar datos
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}

// ─────────────────────────────────────
// Referencias inmutables (&T)
// ─────────────────────────────────────
fn calcular_longitud(s: &String) -> usize {
    s.len()  // Solo lectura, no modifica
}

// ─────────────────────────────────────
// Referencias mutables (&mut T)
// ─────────────────────────────────────
fn agregar_mundo(s: &mut String) {
    s.push_str(" mundo");
}

// ─────────────────────────────────────
// Slices
// ─────────────────────────────────────
fn primera_palabra(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    s
}

// ─────────────────────────────────────
// Lifetimes explícitos
// ─────────────────────────────────────
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ─────────────────────────────────────
// Struct con lifetimes
// ─────────────────────────────────────
#[derive(Debug)]
struct Extracto<'a> {
    parte: &'a str,
}

impl<'a> Extracto<'a> {
    fn nuevo(texto: &'a str, inicio: usize, fin: usize) -> Self {
        Extracto {
            parte: &texto[inicio..fin],
        }
    }
}

fn main() {
    println!("=== Ownership ===");
    demostrar_ownership();
    
    println!("\n=== Referencias Inmutables ===");
    let texto = String::from("Hola Rust");
    let longitud = calcular_longitud(&texto);
    println!("'{}' tiene {} caracteres", texto, longitud);
    
    println!("\n=== Referencias Mutables ===");
    let mut saludo = String::from("Hola");
    agregar_mundo(&mut saludo);
    println!("Resultado: {}", saludo);
    
    println!("\n=== Slices ===");
    let oracion = "Rust es genial";
    let palabra = primera_palabra(oracion);
    println!("Primera palabra de '{}': '{}'", oracion, palabra);
    
    println!("\n=== Lifetimes ===");
    let str1 = "largo";
    let str2 = "más largo aún";
    let resultado = mas_largo(str1, str2);
    println!("Más largo: {}", resultado);
    
    println!("\n=== Struct con Lifetime ===");
    let texto = String::from("Hola Mundo Rust");
    let extracto = Extracto::nuevo(&texto, 5, 10);
    println!("Extracto: {:?}", extracto);
}

