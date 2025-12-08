// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Librería de Programación Asíncrona
// ═══════════════════════════════════════════════════════════════════════════
// Esta librería proporciona utilidades para programación asíncrona
// que pueden ser heredadas por otros nodos usando ch("lib_async")

import java.util.concurrent.*;
import java.util.function.Supplier;
import java.util.stream.IntStream;

public class AsyncUtils {
    // Ejecutar tarea asíncrona con CompletableFuture
    public static <T> CompletableFuture<T> async(Supplier<T> task) {
        return CompletableFuture.supplyAsync(task);
    }
    
    // Ejecutar múltiples tareas en paralelo
    @SafeVarargs
    public static <T> CompletableFuture<List<T>> allOf(CompletableFuture<T>... futures) {
        return CompletableFuture.allOf(futures)
            .thenApply(v -> Arrays.stream(futures)
                .map(CompletableFuture::join)
                .toList());
    }
    
    // Ejecutar tarea con timeout
    public static <T> CompletableFuture<T> withTimeout(
        CompletableFuture<T> future,
        long timeout,
        TimeUnit unit
    ) {
        return future.orTimeout(timeout, unit);
    }
    
    // Retry asíncrono
    public static <T> CompletableFuture<T> retryAsync(
        Supplier<CompletableFuture<T>> operation,
        int maxAttempts
    ) {
        CompletableFuture<T> result = new CompletableFuture<>();
        
        retryAsyncHelper(operation, maxAttempts, 0, result);
        return result;
    }
    
    private static <T> void retryAsyncHelper(
        Supplier<CompletableFuture<T>> operation,
        int maxAttempts,
        int attempt,
        CompletableFuture<T> result
    ) {
        operation.get().whenComplete((value, exception) -> {
            if (exception == null) {
                result.complete(value);
            } else if (attempt < maxAttempts - 1) {
                try {
                    Thread.sleep(100 * (attempt + 1));
                    retryAsyncHelper(operation, maxAttempts, attempt + 1, result);
                } catch (InterruptedException e) {
                    result.completeExceptionally(e);
                }
            } else {
                result.completeExceptionally(exception);
            }
        });
    }
    
    // Ejecutar tareas en paralelo con Virtual Threads
    public static <T> List<T> parallelExecute(
        List<Supplier<T>> tasks
    ) throws InterruptedException, ExecutionException {
        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            List<CompletableFuture<T>> futures = tasks.stream()
                .map(task -> CompletableFuture.supplyAsync(task, executor))
                .toList();
            
            CompletableFuture<List<T>> allFutures = CompletableFuture.allOf(
                futures.toArray(new CompletableFuture[0])
            ).thenApply(v -> futures.stream()
                .map(CompletableFuture::join)
                .toList());
            
            return allFutures.get();
        }
    }
    
    // Ejemplo de uso
    public static void main(String[] args) throws Exception {
        // Tarea asíncrona simple
        async(() -> {
            try { Thread.sleep(100); } catch (InterruptedException e) {}
            return "Tarea completada";
        }).thenAccept(result -> System.out.println("Resultado: " + result));
        
        // Múltiples tareas en paralelo
        List<Supplier<String>> tareas = IntStream.range(0, 5)
            .mapToObj(i -> (Supplier<String>) () -> {
                try { Thread.sleep(100); } catch (InterruptedException e) {}
                return "Tarea " + i;
            })
            .toList();
        
        List<String> resultados = parallelExecute(tareas);
        System.out.println("Resultados: " + resultados);
        
        Thread.sleep(500); // Esperar a que completen las tareas asíncronas
    }
}

