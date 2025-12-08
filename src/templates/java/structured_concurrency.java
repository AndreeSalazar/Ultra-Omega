// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Structured Concurrency (Java 21+)
// ═══════════════════════════════════════════════════════════════════════════
// Structured Concurrency garantiza que todas las subtareas completen
// antes de continuar, y cancela automáticamente si una falla

import java.util.concurrent.*;
import java.util.List;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) throws InterruptedException, ExecutionException {
        System.out.println("=== Structured Concurrency en Java 25 ===\n");
        
        // Ejemplo con ExecutorService y manejo estructurado
        // Nota: En Java 21+ real, usar java.util.concurrent.StructuredTaskScope
        
        System.out.println("Ejecutando 3 tareas en paralelo...");
        List<Future<String>> resultados = new ArrayList<>();
        
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            // Crear subtareas estructuradas
            Future<String> tarea1 = executor.submit(() -> {
                Thread.sleep(100);
                return "Resultado 1: Completado";
            });
            
            Future<String> tarea2 = executor.submit(() -> {
                Thread.sleep(150);
                return "Resultado 2: Completado";
            });
            
            Future<String> tarea3 = executor.submit(() -> {
                Thread.sleep(200);
                return "Resultado 3: Completado";
            });
            
            resultados.add(tarea1);
            resultados.add(tarea2);
            resultados.add(tarea3);
            
            // Esperar a que todas completen
            executor.shutdown();
            executor.awaitTermination(5, TimeUnit.SECONDS);
            
            // Obtener resultados
            for (int i = 0; i < resultados.size(); i++) {
                System.out.println("Tarea " + (i + 1) + ": " + resultados.get(i).get());
            }
        }
        
        // Ejemplo con manejo de errores estructurado
        System.out.println("\n=== Ejemplo con manejo de errores ===");
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            List<Future<String>> tareas = new ArrayList<>();
            
            for (int i = 0; i < 5; i++) {
                final int id = i;
                Future<String> tarea = executor.submit(() -> {
                    if (id == 2) {
                        throw new RuntimeException("Error en tarea " + id);
                    }
                    Thread.sleep(50);
                    return "Tarea " + id + " completada";
                });
                tareas.add(tarea);
            }
            
            executor.shutdown();
            
            // Procesar resultados con manejo de errores
            for (int i = 0; i < tareas.size(); i++) {
                try {
                    System.out.println(tareas.get(i).get());
                } catch (ExecutionException e) {
                    System.out.println("Error en tarea " + i + ": " + e.getCause().getMessage());
                }
            }
        }
        
        // Structured concurrency garantiza:
        System.out.println("\n=== Beneficios de Structured Concurrency ===");
        System.out.println("✅ Todas las subtareas completan antes de continuar");
        System.out.println("✅ Si una falla, las demás se cancelan automáticamente");
        System.out.println("✅ Mejor manejo de errores y limpieza de recursos");
        System.out.println("✅ Evita fugas de recursos (threads, conexiones)");
    }
}

