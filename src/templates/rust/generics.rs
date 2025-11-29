// ═══════════════════════════════════════
// Genéricos Avanzados en Rust
// Generic functions, structs, enums, traits
// ═══════════════════════════════════════

use std::fmt::Display;
use std::cmp::PartialOrd;

// ═══════════════════════════════════════
// FUNCIÓN GENÉRICA
// ═══════════════════════════════════════
fn encontrar_mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Sintaxis alternativa con where
fn encontrar_menor<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b { a } else { b }
}

// ═══════════════════════════════════════
// STRUCT GENÉRICO
// ═══════════════════════════════════════
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

// Implementación específica para i32
impl Contenedor<i32> {
    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }
}

// ═══════════════════════════════════════
// ENUM GENÉRICO
// ═══════════════════════════════════════
#[derive(Debug)]
enum Resultado<T, E> {
    Exito(T),
    Error(E),
}

impl<T, E> Resultado<T, E> {
    fn es_exito(&self) -> bool {
        matches!(self, Resultado::Exito(_))
    }
    
    fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Resultado::Exito(valor) => valor,
            Resultado::Error(_) => T::default(),
        }
    }
}

// ═══════════════════════════════════════
// TRAIT GENÉRICO
// ═══════════════════════════════════════
trait Comparar<T> {
    fn es_mayor_que(&self, otro: &T) -> bool;
    fn es_igual(&self, otro: &T) -> bool;
}

#[derive(Debug, Clone)]
struct Punto {
    x: f64,
    y: f64,
}

impl Comparar<Punto> for Punto {
    fn es_mayor_que(&self, otro: &Punto) -> bool {
        let distancia_self = (self.x * self.x + self.y * self.y).sqrt();
        let distancia_otro = (otro.x * otro.x + otro.y * otro.y).sqrt();
        distancia_self > distancia_otro
    }
    
    fn es_igual(&self, otro: &Punto) -> bool {
        (self.x - otro.x).abs() < 0.001 && (self.y - otro.y).abs() < 0.001
    }
}

// ═══════════════════════════════════════
// MÚLTIPLES GENÉRICOS
// ═══════════════════════════════════════
#[derive(Debug)]
struct Par<T, U> {
    primero: T,
    segundo: U,
}

impl<T, U> Par<T, U> {
    fn new(primero: T, segundo: U) -> Self {
        Par { primero, segundo }
    }
    
    fn intercambiar(self) -> Par<U, T> {
        Par {
            primero: self.segundo,
            segundo: self.primero,
        }
    }
}

// ═══════════════════════════════════════
// GENÉRICOS CON TRAIT BOUNDS
// ═══════════════════════════════════════
fn imprimir_y_retornar<T: Display + Clone>(valor: T) -> T {
    println!("Valor: {}", valor);
    valor.clone()
}

// ═══════════════════════════════════════
// GENÉRICOS CON LIFETIMES
// ═══════════════════════════════════════
fn referencia_mas_larga<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: Display,
{
    println!("Comparando: {} y {}", x, y);
    x  // Simplificado
}

// ═══════════════════════════════════════
// EJEMPLO: PILA GENÉRICA
// ═══════════════════════════════════════
struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }
    
    fn push(&mut self, elemento: T) {
        self.elementos.push(elemento);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.elementos.last()
    }
    
    fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }
    
    fn len(&self) -> usize {
        self.elementos.len()
    }
}

fn main() {
    println!("=== GENÉRICOS EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // FUNCIONES GENÉRICAS
    // ═══════════════════════════════════════
    println!("=== FUNCIONES GENÉRICAS ===");
    
    println!("Mayor(5, 3) = {}", encontrar_mayor(5, 3));
    println!("Mayor(3.14, 2.71) = {}", encontrar_mayor(3.14, 2.71));
    println!("Mayor('z', 'a') = {}", encontrar_mayor('z', 'a'));
    
    // ═══════════════════════════════════════
    // STRUCTS GENÉRICOS
    // ═══════════════════════════════════════
    println!("\n=== STRUCTS GENÉRICOS ===");
    
    let contenedor_int = Contenedor::new(42);
    let contenedor_str = Contenedor::new("Hola Rust");
    let contenedor_float = Contenedor::new(3.14);
    
    println!("Contenedor int: {:?}", contenedor_int);
    println!("Contenedor str: {:?}", contenedor_str);
    println!("Contenedor float: {:?}", contenedor_float);
    
    println!("¿42 es par? {}", contenedor_int.es_par());
    
    // ═══════════════════════════════════════
    // ENUMS GENÉRICOS
    // ═══════════════════════════════════════
    println!("\n=== ENUMS GENÉRICOS ===");
    
    let exito = Resultado::Exito(42);
    let error = Resultado::Error("Algo salió mal");
    
    println!("¿Exito es éxito? {}", exito.is_exito());
    println!("¿Error es éxito? {}", error.is_exito());
    
    let valor = exito.unwrap_or_default();
    println!("Valor por defecto: {}", valor);
    
    // ═══════════════════════════════════════
    // TRAITS GENÉRICOS
    // ═══════════════════════════════════════
    println!("\n=== TRAITS GENÉRICOS ===");
    
    let p1 = Punto { x: 3.0, y: 4.0 };
    let p2 = Punto { x: 1.0, y: 2.0 };
    
    println!("p1 > p2? {}", p1.es_mayor_que(&p2));
    println!("p1 == p2? {}", p1.es_igual(&p2));
    
    // ═══════════════════════════════════════
    // MÚLTIPLES GENÉRICOS
    // ═══════════════════════════════════════
    println!("\n=== MÚLTIPLES GENÉRICOS ===");
    
    let par = Par::new(42, "Hola");
    println!("Par original: {:?}", par);
    
    let par_intercambiado = par.intercambiar();
    println!("Par intercambiado: {:?}", par_intercambiado);
    
    // ═══════════════════════════════════════
    // PILA GENÉRICA
    // ═══════════════════════════════════════
    println!("\n=== PILA GENÉRICA ===");
    
    let mut pila_int: Pila<i32> = Pila::new();
    pila_int.push(1);
    pila_int.push(2);
    pila_int.push(3);
    
    println!("Tamaño de la pila: {}", pila_int.len());
    println!("Top de la pila: {:?}", pila_int.peek());
    
    while let Some(valor) = pila_int.pop() {
        println!("Pop: {}", valor);
    }
    
    // Pila de strings
    let mut pila_str: Pila<&str> = Pila::new();
    pila_str.push("Primero");
    pila_str.push("Segundo");
    pila_str.push("Tercero");
    
    println!("\nPila de strings:");
    while let Some(valor) = pila_str.pop() {
        println!("  {}", valor);
    }
}

