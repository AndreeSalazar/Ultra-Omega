// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Virtual Threads (Project Loom - Java 19+)
// ═══════════════════════════════════════════════════════════════════════════

import java.util.concurrent.*;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        System.out.println("=== Virtual Threads en Java 25 ===\n");
        
        // Crear Virtual Thread (Java 19+)
        Thread virtualThread = Thread.ofVirtual().start(() -> {
            System.out.println("Hilo virtual: " + Thread.currentThread());
            System.out.println("¿Es virtual? " + Thread.currentThread().isVirtual());
        });
        
        virtualThread.join();
        
        // Crear múltiples virtual threads
        System.out.println("\n=== Creando 1000 virtual threads ===");
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            IntStream.range(0, 1000)
                .forEach(i -> executor.submit(() -> {
                    try {
                        Thread.sleep(100);
                        System.out.println("Tarea " + i + " completada");
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    }
                }));
            
            executor.shutdown();
            executor.awaitTermination(5, TimeUnit.SECONDS);
        }
        
        // Comparar con platform threads
        System.out.println("\n=== Comparación: Virtual vs Platform Threads ===");
        System.out.println("Virtual threads son ligeros y escalables");
        System.out.println("Puedes crear millones de virtual threads");
        System.out.println("Perfectos para I/O blocking operations");
    }
}

