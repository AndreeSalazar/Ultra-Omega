// ═══════════════════════════════════════
// Colecciones en Rust
// Vec, HashMap, HashSet, BTreeMap, etc.
// ═══════════════════════════════════════

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    // ═══════════════════════════════════════
    // VECTOR (Vec<T>)
    // ═══════════════════════════════════════
    println!("=== VECTOR ===");
    
    // Crear vector
    let mut numeros = vec![1, 2, 3, 4, 5];
    println!("Vector inicial: {:?}", numeros);
    
    // Agregar elementos
    numeros.push(6);
    numeros.push(7);
    println!("Después de push: {:?}", numeros);
    
    // Acceder a elementos
    println!("Primer elemento: {}", numeros[0]);
    println!("Último elemento: {:?}", numeros.last());
    
    // Iterar
    println!("Elementos:");
    for (i, num) in numeros.iter().enumerate() {
        println!("  [{}] = {}", i, num);
    }
    
    // Operaciones funcionales
    let pares: Vec<i32> = numeros.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    println!("Números pares: {:?}", pares);
    
    let suma: i32 = numeros.iter().sum();
    println!("Suma total: {}", suma);
    
    // ═══════════════════════════════════════
    // HASHMAP (Diccionario)
    // ═══════════════════════════════════════
    println!("\n=== HASHMAP ===");
    
    let mut inventario = HashMap::new();
    
    // Insertar valores
    inventario.insert("manzanas", 50);
    inventario.insert("naranjas", 30);
    inventario.insert("plátanos", 20);
    
    println!("Inventario: {:?}", inventario);
    
    // Acceder a valores
    match inventario.get("manzanas") {
        Some(cantidad) => println!("Manzanas en stock: {}", cantidad),
        None => println!("Sin manzanas"),
    }
    
    // Actualizar valores
    let manzanas = inventario.entry("manzanas").or_insert(0);
    *manzanas += 10;
    println!("Después de comprar 10 manzanas: {:?}", inventario);
    
    // Iterar
    println!("\nInventario completo:");
    for (fruta, cantidad) in &inventario {
        println!("  {}: {} unidades", fruta, cantidad);
    }
    
    // ═══════════════════════════════════════
    // HASHSET (Conjunto)
    // ═══════════════════════════════════════
    println!("\n=== HASHSET ===");
    
    let mut nombres = HashSet::new();
    
    nombres.insert("Alice");
    nombres.insert("Bob");
    nombres.insert("Charlie");
    nombres.insert("Alice");  // Duplicado, se ignora
    
    println!("Nombres únicos: {:?}", nombres);
    println!("¿Contiene 'Alice'? {}", nombres.contains("Alice"));
    println!("¿Contiene 'David'? {}", nombres.contains("David"));
    
    // Operaciones de conjunto
    let mut otros_nombres: HashSet<&str> = ["David", "Eve", "Alice"].iter().copied().collect();
    
    let union: HashSet<_> = nombres.union(&otros_nombres).copied().collect();
    println!("Unión: {:?}", union);
    
    let interseccion: HashSet<_> = nombres.intersection(&otros_nombres).copied().collect();
    println!("Intersección: {:?}", interseccion);
    
    // ═══════════════════════════════════════
    // BTREEMAP (Mapa ordenado)
    // ═══════════════════════════════════════
    println!("\n=== BTREEMAP ===");
    
    let mut calificaciones = BTreeMap::new();
    
    calificaciones.insert("Alice", 95);
    calificaciones.insert("Bob", 87);
    calificaciones.insert("Charlie", 92);
    
    println!("Calificaciones (ordenadas):");
    for (nombre, nota) in &calificaciones {
        println!("  {}: {}", nombre, nota);
    }
    
    // ═══════════════════════════════════════
    // VECDEQUE (Cola de doble extremo)
    // ═══════════════════════════════════════
    println!("\n=== VECDEQUE ===");
    
    let mut cola = VecDeque::new();
    
    // Agregar al final
    cola.push_back(1);
    cola.push_back(2);
    cola.push_back(3);
    
    // Agregar al inicio
    cola.push_front(0);
    
    println!("Cola: {:?}", cola);
    
    // Remover del inicio
    if let Some(primero) = cola.pop_front() {
        println!("Removido del inicio: {}", primero);
    }
    
    // Remover del final
    if let Some(ultimo) = cola.pop_back() {
        println!("Removido del final: {}", ultimo);
    }
    
    println!("Cola final: {:?}", cola);
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: Sistema de Votos
    // ═══════════════════════════════════════
    println!("\n=== SISTEMA DE VOTOS ===");
    
    let votos = vec![
        "Alice", "Bob", "Alice", "Charlie", 
        "Bob", "Alice", "David", "Alice"
    ];
    
    let mut conteo: HashMap<&str, u32> = HashMap::new();
    
    for candidato in votos {
        let count = conteo.entry(candidato).or_insert(0);
        *count += 1;
    }
    
    println!("Resultados de votación:");
    for (candidato, votos) in &conteo {
        println!("  {}: {} votos", candidato, votos);
    }
    
    // Encontrar ganador
    let ganador = conteo.iter()
        .max_by_key(|(_, &votos)| votos)
        .map(|(nombre, _)| nombre);
    
    match ganador {
        Some(nombre) => println!("\n🏆 Ganador: {}", nombre),
        None => println!("\nSin votos"),
    }
}

