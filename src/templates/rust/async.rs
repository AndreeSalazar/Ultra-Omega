// ═══════════════════════════════════════
// Programación Asíncrona en Rust
// async/await, Future, tokio
// ═══════════════════════════════════════

// Nota: Este ejemplo requiere tokio o async-std
// Para compilar, agregar a Cargo.toml:
// [dependencies]
// tokio = { version = "1", features = ["full"] }

// ═══════════════════════════════════════
// FUNCIÓN ASYNC BÁSICA
// ═══════════════════════════════════════
async fn tarea_simple() -> i32 {
    // Simular trabajo asíncrono
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    42
}

// ═══════════════════════════════════════
// MÚLTIPLES TAREAS ASYNC
// ═══════════════════════════════════════
async fn descargar_datos(id: u32) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    format!("Datos del recurso {}", id)
}

async fn procesar_datos(datos: String) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    format!("Procesado: {}", datos)
}

// ═══════════════════════════════════════
// EJECUTAR EN PARALELO
// ═══════════════════════════════════════
async fn ejecutar_paralelo() {
    let tarea1 = descargar_datos(1);
    let tarea2 = descargar_datos(2);
    let tarea3 = descargar_datos(3);
    
    // Ejecutar todas en paralelo
    let (resultado1, resultado2, resultado3) = 
        tokio::join!(tarea1, tarea2, tarea3);
    
    println!("Resultado 1: {}", resultado1);
    println!("Resultado 2: {}", resultado2);
    println!("Resultado 3: {}", resultado3);
}

// ═══════════════════════════════════════
// STREAM (Flujo de datos asíncrono)
// ═══════════════════════════════════════
async fn generar_numeros() -> impl tokio_stream::Stream<Item = i32> {
    use tokio_stream::{self as stream, StreamExt};
    
    stream::iter(1..=10)
        .then(|n| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            n * 2
        })
}

// ═══════════════════════════════════════
// CHANNELS ASYNC
// ═══════════════════════════════════════
async fn ejemplo_channels() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);
    
    // Productor
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    
    // Consumidor
    while let Some(valor) = rx.recv().await {
        println!("Recibido: {}", valor);
    }
}

// ═══════════════════════════════════════
// SELECT (Esperar múltiples futuros)
// ═══════════════════════════════════════
async fn ejemplo_select() {
    let mut tarea1 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        "Tarea 1 completada"
    });
    
    let mut tarea2 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        "Tarea 2 completada"
    });
    
    tokio::select! {
        resultado1 = &mut tarea1 => {
            println!("{}", resultado1.unwrap());
        }
        resultado2 = &mut tarea2 => {
            println!("{}", resultado2.unwrap());
        }
    }
}

// ═══════════════════════════════════════
// MAIN ASYNC
// ═══════════════════════════════════════
#[tokio::main]
async fn main() {
    println!("=== PROGRAMACIÓN ASÍNCRONA EN RUST ===\n");
    
    // ═══════════════════════════════════════
    // FUNCIÓN ASYNC SIMPLE
    // ═══════════════════════════════════════
    println!("=== FUNCIÓN ASYNC SIMPLE ===");
    let resultado = tarea_simple().await;
    println!("Resultado: {}\n", resultado);
    
    // ═══════════════════════════════════════
    // EJECUCIÓN EN PARALELO
    // ═══════════════════════════════════════
    println!("=== EJECUCIÓN EN PARALELO ===");
    ejecutar_paralelo().await;
    println!();
    
    // ═══════════════════════════════════════
    // PIPELINE ASYNC
    // ═══════════════════════════════════════
    println!("=== PIPELINE ASYNC ===");
    let datos = descargar_datos(100).await;
    let procesado = procesar_datos(datos).await;
    println!("{}", procesado);
    println!();
    
    // ═══════════════════════════════════════
    // STREAM
    // ═══════════════════════════════════════
    println!("=== STREAM ===");
    let mut stream = generar_numeros().await;
    use tokio_stream::StreamExt;
    
    while let Some(numero) = stream.next().await {
        println!("Número del stream: {}", numero);
    }
    println!();
    
    // ═══════════════════════════════════════
    // CHANNELS
    // ═══════════════════════════════════════
    println!("=== CHANNELS ===");
    ejemplo_channels().await;
    println!();
    
    // ═══════════════════════════════════════
    // SELECT
    // ═══════════════════════════════════════
    println!("=== SELECT ===");
    ejemplo_select().await;
}

