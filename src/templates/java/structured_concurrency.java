// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Structured Concurrency (Java 21+)
// ═══════════════════════════════════════════════════════════════════════════

import java.util.concurrent.*;
import java.util.List;

public class Main {
    public static void main(String[] args) throws InterruptedException, ExecutionException {
        System.out.println("=== Structured Concurrency en Java 25 ===\n");
        
        // Structured concurrency con Executors.newStructuredTaskScope() (Java 21+)
        // Nota: Requiere --enable-preview
        
        try (var scope = new StructuredTaskScope<String>()) {
            // Crear subtareas estructuradas
            Future<String> tarea1 = scope.fork(() -> {
                Thread.sleep(100);
                return "Resultado 1";
            });
            
            Future<String> tarea2 = scope.fork(() -> {
                Thread.sleep(150);
                return "Resultado 2";
            });
            
            Future<String> tarea3 = scope.fork(() -> {
                Thread.sleep(200);
                return "Resultado 3";
            });
            
            // Esperar a que todas completen
            scope.join();
            
            System.out.println("Tarea 1: " + tarea1.get());
            System.out.println("Tarea 2: " + tarea2.get());
            System.out.println("Tarea 3: " + tarea3.get());
        }
        
        // Structured concurrency garantiza:
        System.out.println("\nBeneficios:");
        System.out.println("- Todas las subtareas completan antes de continuar");
        System.out.println("- Si una falla, las demás se cancelan automáticamente");
        System.out.println("- Mejor manejo de errores y limpieza de recursos");
    }
}

// Clase helper para StructuredTaskScope (simplificada)
// En Java 21+ real, usar java.util.concurrent.StructuredTaskScope
class StructuredTaskScope<T> implements AutoCloseable {
    private final ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor();
    
    public Future<T> fork(Callable<T> task) {
        return executor.submit(task);
    }
    
    public void join() throws InterruptedException {
        // Esperar a que todas las tareas completen
        executor.shutdown();
        executor.awaitTermination(10, TimeUnit.SECONDS);
    }
    
    @Override
    public void close() {
        executor.shutdownNow();
    }
}

