// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Virtual Threads (Project Loom - Java 19+)
// ═══════════════════════════════════════════════════════════════════════════
// Virtual Threads permiten crear millones de threads concurrentes
// sin la sobrecarga de los platform threads tradicionales

import java.util.concurrent.*;
import java.util.stream.IntStream;
import java.util.concurrent.atomic.AtomicInteger;

public class Main {
    private static AtomicInteger contador = new AtomicInteger(0);
    
    public static void main(String[] args) throws InterruptedException {
        System.out.println("=== Virtual Threads en Java 25 ===\n");
        
        // Crear Virtual Thread individual (Java 19+)
        Thread virtualThread = Thread.ofVirtual()
            .name("virtual-thread-1")
            .start(() -> {
                System.out.println("Hilo virtual: " + Thread.currentThread().getName());
                System.out.println("¿Es virtual? " + Thread.currentThread().isVirtual());
                System.out.println("Thread ID: " + Thread.currentThread().threadId());
            });
        
        virtualThread.join();
        
        // Crear múltiples virtual threads con ExecutorService
        System.out.println("\n=== Creando 1000 virtual threads ===");
        long inicio = System.currentTimeMillis();
        
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            IntStream.range(0, 1000)
                .forEach(i -> executor.submit(() -> {
                    try {
                        Thread.sleep(100); // Simular I/O blocking
                        int valor = contador.incrementAndGet();
                        if (valor % 100 == 0) {
                            System.out.println("Progreso: " + valor + "/1000");
                        }
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    }
                }));
            
            executor.shutdown();
            executor.awaitTermination(10, TimeUnit.SECONDS);
        }
        
        long fin = System.currentTimeMillis();
        System.out.println("\nTiempo total: " + (fin - inicio) + " ms");
        System.out.println("Tareas completadas: " + contador.get());
        
        // Comparar con platform threads
        System.out.println("\n=== Comparación: Virtual vs Platform Threads ===");
        System.out.println("✅ Virtual threads son ligeros y escalables");
        System.out.println("✅ Puedes crear millones de virtual threads");
        System.out.println("✅ Perfectos para I/O blocking operations");
        System.out.println("✅ Mejor rendimiento que platform threads para I/O");
        
        // Ejemplo: Múltiples requests HTTP simulados
        System.out.println("\n=== Simulando 10,000 requests HTTP ===");
        AtomicInteger requests = new AtomicInteger(0);
        
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            IntStream.range(0, 10_000)
                .forEach(i -> executor.submit(() -> {
                    try {
                        // Simular request HTTP (I/O blocking)
                        Thread.sleep(50);
                        requests.incrementAndGet();
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    }
                }));
            
            executor.shutdown();
            executor.awaitTermination(15, TimeUnit.SECONDS);
        }
        
        System.out.println("Requests completados: " + requests.get() + "/10,000");
    }
}
