// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Concurrencia (Threads y Executors)
// ═══════════════════════════════════════════════════════════════════════════

import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Main {
    private static AtomicInteger contador = new AtomicInteger(0);
    
    public static void main(String[] args) throws InterruptedException {
        // Thread básico
        Thread hilo1 = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                System.out.println("Hilo 1: " + i);
                try { Thread.sleep(100); } catch (InterruptedException e) {}
            }
        });
        hilo1.start();
        
        // ExecutorService (recomendado)
        ExecutorService executor = Executors.newFixedThreadPool(3);
        
        for (int i = 0; i < 5; i++) {
            final int id = i;
            executor.submit(() -> {
                System.out.println("Tarea " + id + " ejecutada por: " + Thread.currentThread().getName());
                contador.incrementAndGet();
            });
        }
        
        executor.shutdown();
        executor.awaitTermination(1, TimeUnit.SECONDS);
        
        System.out.println("Contador final: " + contador.get());
        
        // CompletableFuture (Java 8+)
        CompletableFuture<String> futuro = CompletableFuture.supplyAsync(() -> {
            try { Thread.sleep(500); } catch (InterruptedException e) {}
            return "Resultado asíncrono";
        });
        
        futuro.thenAccept(resultado -> System.out.println("Resultado: " + resultado));
        
        try { Thread.sleep(1000); } catch (InterruptedException e) {}
    }
}

