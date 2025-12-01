// ═══════════════════════════════════════
// Macros en Rust
// Declarative macros, procedural macros
// ═══════════════════════════════════════

// ═══════════════════════════════════════
// MACRO DECLARATIVA SIMPLE
// ═══════════════════════════════════════
macro_rules! saludar {
    () => {
        println!("¡Hola!");
    };
    ($nombre:expr) => {
        println!("¡Hola, {}!", $nombre);
    };
    ($nombre:expr, $apellido:expr) => {
        println!("¡Hola, {} {}!", $nombre, $apellido);
    };
}

// ═══════════════════════════════════════
// MACRO CON REPETICIÓN
// ═══════════════════════════════════════
macro_rules! crear_vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

// ═══════════════════════════════════════
// MACRO PARA CREAR STRUCT
// ═══════════════════════════════════════
macro_rules! crear_punto {
    ($x:expr, $y:expr) => {
        Punto { x: $x, y: $y }
    };
}

#[derive(Debug)]
struct Punto {
    x: f64,
    y: f64,
}

// ═══════════════════════════════════════
// MACRO CON PATRONES COMPLEJOS
// ═══════════════════════════════════════
macro_rules! calcular {
    (suma $a:expr, $b:expr) => {
        $a + $b
    };
    (resta $a:expr, $b:expr) => {
        $a - $b
    };
    (multiplica $a:expr, $b:expr) => {
        $a * $b
    };
    (divide $a:expr, $b:expr) => {
        $a / $b
    };
}

// ═══════════════════════════════════════
// MACRO PARA LOGGING
// ═══════════════════════════════════════
macro_rules! log {
    ($nivel:ident, $mensaje:expr) => {
        println!("[{}] {}", stringify!($nivel), $mensaje);
    };
    ($nivel:ident, $formato:expr, $($arg:expr),*) => {
        println!(concat!("[{}] ", $formato), stringify!($nivel), $($arg),*);
    };
}

// ═══════════════════════════════════════
// MACRO PARA ASSERTION
// ═══════════════════════════════════════
macro_rules! assert_igual {
    ($izq:expr, $der:expr) => {
        if $izq != $der {
            panic!("Assertion falló: {} != {}", stringify!($izq), stringify!($der));
        }
    };
    ($izq:expr, $der:expr, $mensaje:expr) => {
        if $izq != $der {
            panic!("{}: {} != {}", $mensaje, stringify!($izq), stringify!($der));
        }
    };
}

// ═══════════════════════════════════════
// MACRO PARA MATCH MEJORADO
// ═══════════════════════════════════════
macro_rules! match_resultado {
    ($expr:expr, $patron:pat => $accion:expr) => {
        match $expr {
            $patron => $accion,
            _ => panic!("Pattern no coincide"),
        }
    };
}

// ═══════════════════════════════════════
// MACRO PARA CREAR ENUM CON VALORES
// ═══════════════════════════════════════
macro_rules! crear_estado {
    (
        enum $nombre:ident {
            $($variante:ident = $valor:expr),*
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $nombre {
            $($variante = $valor),*
        }
        
        impl $nombre {
            fn valor(&self) -> i32 {
                *self as i32
            }
        }
    };
}

crear_estado! {
    enum EstadoHTTP {
        OK = 200,
        NotFound = 404,
        ServerError = 500
    }
}

fn main() {
    println!("=== MACROS EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // MACRO SIMPLE
    // ═══════════════════════════════════════
    println!("=== MACRO SIMPLE ===");
    saludar!();
    saludar!("Juan");
    saludar!("Juan", "Pérez");
    println!();
    
    // ═══════════════════════════════════════
    // MACRO CON REPETICIÓN
    // ═══════════════════════════════════════
    println!("=== MACRO CON REPETICIÓN ===");
    let vector = crear_vector!(1, 2, 3, 4, 5);
    println!("Vector creado: {:?}", vector);
    println!();
    
    // ═══════════════════════════════════════
    // MACRO PARA STRUCT
    // ═══════════════════════════════════════
    println!("=== MACRO PARA STRUCT ===");
    let punto = crear_punto!(3.0, 4.0);
    println!("Punto: {:?}", punto);
    println!();
    
    // ═══════════════════════════════════════
    // MACRO CON PATRONES
    // ═══════════════════════════════════════
    println!("=== MACRO CON PATRONES ===");
    println!("10 + 5 = {}", calcular!(suma 10, 5));
    println!("10 - 5 = {}", calcular!(resta 10, 5));
    println!("10 * 5 = {}", calcular!(multiplica 10, 5));
    println!("10 / 5 = {}", calcular!(divide 10, 5));
    println!();
    
    // ═══════════════════════════════════════
    // MACRO DE LOGGING
    // ═══════════════════════════════════════
    println!("=== MACRO DE LOGGING ===");
    log!(INFO, "Aplicación iniciada");
    log!(ERROR, "Error en línea {}", 42);
    log!(WARNING, "Advertencia: {}", "Memoria baja");
    println!();
    
    // ═══════════════════════════════════════
    // MACRO DE ASSERTION
    // ═══════════════════════════════════════
    println!("=== MACRO DE ASSERTION ===");
    assert_igual!(2 + 2, 4);
    assert_igual!(10, 10, "Los valores deben ser iguales");
    println!("✅ Todas las assertions pasaron");
    println!();
    
    // ═══════════════════════════════════════
    // MACRO DE ENUM
    // ═══════════════════════════════════════
    println!("=== MACRO DE ENUM ===");
    let estado = EstadoHTTP::OK;
    println!("Estado: {:?}, Valor: {}", estado, estado.valor());
    
    let estado2 = EstadoHTTP::NotFound;
    println!("Estado: {:?}, Valor: {}", estado2, estado2.valor());
    println!();
    
    // ═══════════════════════════════════════
    // EJEMPLO PRÁCTICO: DSL para Queries
    // ═══════════════════════════════════════
    println!("=== DSL PARA QUERIES ===");
    
    macro_rules! query {
        (SELECT $($campo:ident),* FROM $tabla:ident WHERE $condicion:expr) => {
            {
                println!("SELECT {} FROM {} WHERE {}", 
                    stringify!($($campo),*), 
                    stringify!($tabla),
                    $condicion
                );
            }
        };
    }
    
    query!(SELECT nombre, edad FROM usuarios WHERE edad > 18);
}

