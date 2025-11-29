// ═══════════════════════════════════════
// Closures en Rust
// Funciones anónimas, captura de variables
// ═══════════════════════════════════════

fn main() {
    println!("=== CLOSURES EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // CLOSURE BÁSICO
    // ═══════════════════════════════════════
    println!("=== CLOSURE BÁSICO ===");
    
    let sumar = |a: i32, b: i32| a + b;
    println!("5 + 3 = {}", sumar(5, 3));
    
    let multiplicar = |x, y| x * y;  // Inferencia de tipos
    println!("4 * 7 = {}", multiplicar(4, 7));
    
    // ═══════════════════════════════════════
    // CLOSURE CON MÚLTIPLES LÍNEAS
    // ═══════════════════════════════════════
    println!("\n=== CLOSURE MULTILÍNEA ===");
    
    let calcular = |x: i32| {
        let resultado = x * 2;
        resultado + 10
    };
    println!("calcular(5) = {}", calcular(5));
    
    // ═══════════════════════════════════════
    // CAPTURA DE VARIABLES
    // ═══════════════════════════════════════
    println!("\n=== CAPTURA DE VARIABLES ===");
    
    let factor = 10;
    
    // Captura por referencia (Fn)
    let multiplicar_por_factor = |x| x * factor;
    println!("5 * {} = {}", factor, multiplicar_por_factor(5));
    println!("factor sigue siendo: {}", factor);
    
    // Captura por valor (move)
    let mensaje = String::from("Hola");
    let closure_move = move || {
        println!("Mensaje capturado: {}", mensaje);
    };
    closure_move();
    // mensaje ya no es accesible aquí (movido)
    
    // ═══════════════════════════════════════
    // CLOSURES COMO PARÁMETROS
    // ═══════════════════════════════════════
    println!("\n=== CLOSURES COMO PARÁMETROS ===");
    
    fn aplicar_a_numeros<F>(numeros: &[i32], operacion: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        numeros.iter().map(|&x| operacion(x)).collect()
    }
    
    let numeros = vec![1, 2, 3, 4, 5];
    
    let dobles = aplicar_a_numeros(&numeros, |x| x * 2);
    println!("Dobles: {:?}", dobles);
    
    let cuadrados = aplicar_a_numeros(&numeros, |x| x * x);
    println!("Cuadrados: {:?}", cuadrados);
    
    // ═══════════════════════════════════════
    // CLOSURES CON ITERADORES
    // ═══════════════════════════════════════
    println!("\n=== CLOSURES CON ITERADORES ===");
    
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // filter
    let pares: Vec<i32> = numeros.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    println!("Pares: {:?}", pares);
    
    // map
    let cuadrados: Vec<i32> = numeros.iter()
        .map(|x| x * x)
        .collect();
    println!("Cuadrados: {:?}", cuadrados);
    
    // fold (reduce)
    let suma: i32 = numeros.iter().fold(0, |acc, x| acc + x);
    println!("Suma total: {}", suma);
    
    // ═══════════════════════════════════════
    // CLOSURES QUE RETORNAN CLOSURES
    // ═══════════════════════════════════════
    println!("\n=== CLOSURES QUE RETORNAN CLOSURES ===");
    
    fn crear_multiplicador(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    
    let multiplicar_por_3 = crear_multiplicador(3);
    let multiplicar_por_5 = crear_multiplicador(5);
    
    println!("4 * 3 = {}", multiplicar_por_3(4));
    println!("4 * 5 = {}", multiplicar_por_5(4));
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: Sistema de Filtros
    // ═══════════════════════════════════════
    println!("\n=== SISTEMA DE FILTROS ===");
    
    #[derive(Debug)]
    struct Producto {
        nombre: String,
        precio: f64,
        categoria: String,
    }
    
    let productos = vec![
        Producto { nombre: "Laptop".to_string(), precio: 999.99, categoria: "Electrónica".to_string() },
        Producto { nombre: "Mouse".to_string(), precio: 29.99, categoria: "Electrónica".to_string() },
        Producto { nombre: "Silla".to_string(), precio: 199.99, categoria: "Muebles".to_string() },
        Producto { nombre: "Mesa".to_string(), precio: 299.99, categoria: "Muebles".to_string() },
    ];
    
    // Filtrar por categoría
    let electronicos: Vec<&Producto> = productos.iter()
        .filter(|p| p.categoria == "Electrónica")
        .collect();
    println!("Electrónicos:");
    for p in electronicos {
        println!("  - {}: ${:.2}", p.nombre, p.precio);
    }
    
    // Filtrar por precio
    let caros: Vec<&Producto> = productos.iter()
        .filter(|p| p.precio > 100.0)
        .collect();
    println!("\nProductos caros (>$100):");
    for p in caros {
        println!("  - {}: ${:.2}", p.nombre, p.precio);
    }
    
    // Combinar filtros
    let resultado: Vec<String> = productos.iter()
        .filter(|p| p.categoria == "Electrónica")
        .filter(|p| p.precio < 50.0)
        .map(|p| p.nombre.clone())
        .collect();
    println!("\nElectrónicos baratos: {:?}", resultado);
}

