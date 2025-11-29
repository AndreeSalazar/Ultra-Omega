// ═══════════════════════════════════════
// Concurrencia en Rust
// Threads, channels, Arc, Mutex, RwLock
// ═══════════════════════════════════════

use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::time::Duration;

// ═══════════════════════════════════════
// THREADS BÁSICOS
// ═══════════════════════════════════════
fn ejemplo_threads() {
    println!("=== THREADS ===");
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..=5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
}

// ═══════════════════════════════════════
// CHANNELS (Comunicación entre threads)
// ═══════════════════════════════════════
fn ejemplo_channels() {
    println!("\n=== CHANNELS ===");
    
    let (tx, rx) = mpsc::channel();
    
    // Thread productor
    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Thread consumidor (main)
    for recibido in rx {
        println!("Recibido: {}", recibido);
    }
}

// ═══════════════════════════════════════
// MÚLTIPLES PRODUCTORES
// ═══════════════════════════════════════
fn ejemplo_multiple_productores() {
    println!("\n=== MÚLTIPLES PRODUCTORES ===");
    
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("Thread-1: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("Thread-2: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });
    
    drop(tx);  // Cerrar el canal original
    
    for mensaje in rx {
        println!("{}", mensaje);
    }
}

// ═══════════════════════════════════════
// ARC (Atomic Reference Counting)
// ═══════════════════════════════════════
fn ejemplo_arc() {
    println!("\n=== ARC ===");
    
    let contador = Arc::new(0);
    let mut handles = vec![];
    
    for _ in 0..5 {
        let contador = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            // Arc no permite mutación directa
            // Necesitamos Mutex para eso
            println!("Thread accediendo al contador: {}", contador);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

// ═══════════════════════════════════════
// MUTEX (Mutual Exclusion)
// ═══════════════════════════════════════
fn ejemplo_mutex() {
    println!("\n=== MUTEX ===");
    
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let contador = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Contador final: {}", *contador.lock().unwrap());
}

// ═══════════════════════════════════════
// RWLOCK (Read-Write Lock)
// ═══════════════════════════════════════
fn ejemplo_rwlock() {
    println!("\n=== RWLOCK ===");
    
    let datos = Arc::new(RwLock::new(String::from("Inicial")));
    let mut handles = vec![];
    
    // Lectores (múltiples simultáneos)
    for i in 0..3 {
        let datos = Arc::clone(&datos);
        let handle = thread::spawn(move || {
            let lectura = datos.read().unwrap();
            println!("Lector {}: {}", i, *lectura);
        });
        handles.push(handle);
    }
    
    // Escritor (exclusivo)
    let datos_escritor = Arc::clone(&datos);
    let handle_escritor = thread::spawn(move || {
        let mut escritura = datos_escritor.write().unwrap();
        *escritura = String::from("Modificado");
        println!("Escritor: valor modificado");
    });
    handles.push(handle_escritor);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Valor final: {}", *datos.read().unwrap());
}

// ═══════════════════════════════════════
// EJEMPLO PRÁCTICO: Pool de Trabajadores
// ═══════════════════════════════════════
fn ejemplo_pool_trabajadores() {
    println!("\n=== POOL DE TRABAJADORES ===");
    
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];
    
    // Crear 4 trabajadores
    for id in 0..4 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let trabajo = rx.lock().unwrap().recv();
                match trabajo {
                    Ok(tarea) => {
                        println!("Trabajador {} procesando: {}", id, tarea);
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => {
                        println!("Trabajador {} terminando", id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    // Enviar trabajos
    for i in 1..=10 {
        tx.send(format!("Tarea {}", i)).unwrap();
    }
    
    drop(tx);  // Cerrar canal para que los trabajadores terminen
    
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    println!("=== CONCURRENCIA EN RUST ===\n");
    
    ejemplo_threads();
    ejemplo_channels();
    ejemplo_multiple_productores();
    ejemplo_arc();
    ejemplo_mutex();
    ejemplo_rwlock();
    ejemplo_pool_trabajadores();
    
    println!("\n✅ Todos los ejemplos de concurrencia completados");
}

