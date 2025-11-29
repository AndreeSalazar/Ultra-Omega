// ═══════════════════════════════════════
// Enums Avanzados en Rust
// Enums con datos, Option, Result
// ═══════════════════════════════════════

fn main() {
    // ═══════════════════════════════════════
    // ENUM BÁSICO
    // ═══════════════════════════════════════
    println!("=== ENUM BÁSICO ===");
    
    #[derive(Debug)]
    enum Direccion {
        Norte,
        Sur,
        Este,
        Oeste,
    }
    
    let direccion = Direccion::Norte;
    match direccion {
        Direccion::Norte => println!("Vamos al norte"),
        Direccion::Sur => println!("Vamos al sur"),
        Direccion::Este => println!("Vamos al este"),
        Direccion::Oeste => println!("Vamos al oeste"),
    }
    
    // ═══════════════════════════════════════
    // ENUM CON DATOS
    // ═══════════════════════════════════════
    println!("\n=== ENUM CON DATOS ===");
    
    #[derive(Debug)]
    enum Mensaje {
        Salir,
        Mover { x: i32, y: i32 },
        Escribir(String),
        CambiarColor(u8, u8, u8),
    }
    
    let mensajes = vec![
        Mensaje::Salir,
        Mensaje::Mover { x: 10, y: 20 },
        Mensaje::Escribir(String::from("Hola Rust")),
        Mensaje::CambiarColor(255, 0, 0),
    ];
    
    for msg in mensajes {
        match msg {
            Mensaje::Salir => println!("Saliendo..."),
            Mensaje::Mover { x, y } => println!("Moviendo a ({}, {})", x, y),
            Mensaje::Escribir(texto) => println!("Escribiendo: {}", texto),
            Mensaje::CambiarColor(r, g, b) => println!("Color RGB: ({}, {}, {})", r, g, b),
        }
    }
    
    // ═══════════════════════════════════════
    // OPTION<T>
    // ═══════════════════════════════════════
    println!("\n=== OPTION ===");
    
    fn buscar_numero(numeros: &[i32], objetivo: i32) -> Option<usize> {
        numeros.iter().position(|&x| x == objetivo)
    }
    
    let numeros = vec![1, 3, 5, 7, 9];
    
    match buscar_numero(&numeros, 5) {
        Some(indice) => println!("Número 5 encontrado en índice {}", indice),
        None => println!("Número 5 no encontrado"),
    }
    
    match buscar_numero(&numeros, 4) {
        Some(indice) => println!("Número 4 encontrado en índice {}", indice),
        None => println!("Número 4 no encontrado"),
    }
    
    // Métodos útiles de Option
    let valor = Some(42);
    println!("Valor: {:?}", valor);
    println!("Valor o default: {}", valor.unwrap_or(0));
    
    let valor_none: Option<i32> = None;
    println!("None o default: {}", valor_none.unwrap_or(100));
    
    // Map y and_then
    let resultado = Some(5)
        .map(|x| x * 2)
        .map(|x| x + 1);
    println!("Some(5) * 2 + 1 = {:?}", resultado);
    
    // ═══════════════════════════════════════
    // RESULT<T, E>
    // ═══════════════════════════════════════
    println!("\n=== RESULT ===");
    
    #[derive(Debug)]
    enum ErrorDivision {
        DivisionPorCero,
        NumeroNegativo,
    }
    
    fn dividir_seguro(a: i32, b: i32) -> Result<i32, ErrorDivision> {
        if b == 0 {
            Err(ErrorDivision::DivisionPorCero)
        } else if a < 0 || b < 0 {
            Err(ErrorDivision::NumeroNegativo)
        } else {
            Ok(a / b)
        }
    }
    
    // Usar match
    match dividir_seguro(10, 2) {
        Ok(resultado) => println!("10 / 2 = {}", resultado),
        Err(ErrorDivision::DivisionPorCero) => println!("Error: División por cero"),
        Err(ErrorDivision::NumeroNegativo) => println!("Error: Número negativo"),
    }
    
    match dividir_seguro(10, 0) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Métodos útiles de Result
    let resultado = dividir_seguro(20, 4)
        .map(|x| x * 2)
        .unwrap_or(0);
    println!("(20 / 4) * 2 = {}", resultado);
    
    // ═══════════════════════════════════════
    // ENUM COMPLEJO: Máquina de Estados
    // ═══════════════════════════════════════
    println!("\n=== MÁQUINA DE ESTADOS ===");
    
    #[derive(Debug, Clone, Copy)]
    enum EstadoMaquina {
        Inactiva,
        Cargando { progreso: u8 },
        Activa { tiempo: u32 },
        Error { codigo: u32 },
    }
    
    impl EstadoMaquina {
        fn siguiente(self) -> EstadoMaquina {
            match self {
                EstadoMaquina::Inactiva => EstadoMaquina::Cargando { progreso: 0 },
                EstadoMaquina::Cargando { progreso } if progreso < 100 => {
                    EstadoMaquina::Cargando { progreso: progreso + 25 }
                }
                EstadoMaquina::Cargando { .. } => EstadoMaquina::Activa { tiempo: 0 },
                EstadoMaquina::Activa { tiempo } if tiempo < 10 => {
                    EstadoMaquina::Activa { tiempo: tiempo + 1 }
                }
                EstadoMaquina::Activa { .. } => EstadoMaquina::Inactiva,
                EstadoMaquina::Error { .. } => EstadoMaquina::Inactiva,
            }
        }
        
        fn mostrar(&self) {
            match self {
                EstadoMaquina::Inactiva => println!("🔴 Inactiva"),
                EstadoMaquina::Cargando { progreso } => {
                    println!("⏳ Cargando... {}%", progreso)
                }
                EstadoMaquina::Activa { tiempo } => {
                    println!("🟢 Activa (tiempo: {}s)", tiempo)
                }
                EstadoMaquina::Error { codigo } => {
                    println!("❌ Error código: {}", codigo)
                }
            }
        }
    }
    
    let mut estado = EstadoMaquina::Inactiva;
    
    println!("Simulación de máquina de estados:");
    for i in 0..10 {
        println!("\nPaso {}:", i);
        estado.mostrar();
        estado = estado.siguiente();
    }
}

