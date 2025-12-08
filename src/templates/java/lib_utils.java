// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Librería de Utilidades
// ═══════════════════════════════════════════════════════════════════════════
// Esta librería proporciona funciones de utilidad reutilizables
// que pueden ser heredadas por otros nodos usando ch("lib_utils")

import java.util.*;
import java.util.function.*;
import java.util.stream.Collectors;

public class Utils {
    // Validación de entrada
    public static <T> T requireNonNull(T obj, String mensaje) {
        if (obj == null) {
            throw new IllegalArgumentException(mensaje);
        }
        return obj;
    }
    
    // Verificar rango
    public static void requireRange(int value, int min, int max) {
        if (value < min || value > max) {
            throw new IllegalArgumentException(
                String.format("Valor %d fuera del rango [%d, %d]", value, min, max)
            );
        }
    }
    
    // Retry con reintentos
    public static <T> T retry(Supplier<T> operation, int maxAttempts) {
        Exception lastException = null;
        
        for (int i = 0; i < maxAttempts; i++) {
            try {
                return operation.get();
            } catch (Exception e) {
                lastException = e;
                if (i < maxAttempts - 1) {
                    try {
                        Thread.sleep(100 * (i + 1)); // Backoff exponencial
                    } catch (InterruptedException ie) {
                        Thread.currentThread().interrupt();
                        throw new RuntimeException(ie);
                    }
                }
            }
        }
        
        throw new RuntimeException("Operación falló después de " + maxAttempts + " intentos", lastException);
    }
    
    // Medir tiempo de ejecución
    public static <T> T measureTime(String operationName, Supplier<T> operation) {
        long start = System.nanoTime();
        try {
            return operation.get();
        } finally {
            long duration = System.nanoTime() - start;
            System.out.println(operationName + " tomó: " + (duration / 1_000_000.0) + " ms");
        }
    }
    
    // Transformar lista
    public static <T, R> List<R> map(List<T> lista, Function<T, R> mapper) {
        return lista.stream().map(mapper).toList();
    }
    
    // Filtrar lista
    public static <T> List<T> filter(List<T> lista, Predicate<T> predicate) {
        return lista.stream().filter(predicate).toList();
    }
    
    // Agrupar por criterio
    public static <T, K> Map<K, List<T>> groupBy(List<T> lista, Function<T, K> classifier) {
        return lista.stream().collect(
            java.util.stream.Collectors.groupingBy(classifier)
        );
    }
    
    // Ejemplo de uso
    public static void main(String[] args) {
        List<Integer> numeros = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        
        // Filtrar pares
        List<Integer> pares = filter(numeros, n -> n % 2 == 0);
        System.out.println("Pares: " + pares);
        
        // Mapear a cuadrados
        List<Integer> cuadrados = map(numeros, n -> n * n);
        System.out.println("Cuadrados: " + cuadrados);
        
        // Medir tiempo
        measureTime("Operación costosa", () -> {
            try { Thread.sleep(100); } catch (InterruptedException e) {}
            return "Completado";
        });
    }
}

