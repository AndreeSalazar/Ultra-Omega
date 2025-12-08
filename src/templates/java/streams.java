// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Streams API (Java 8+)
// ═══════════════════════════════════════════════════════════════════════════

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        List<Integer> numeros = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        
        // Filtrar números pares
        List<Integer> pares = numeros.stream()
            .filter(n -> n % 2 == 0)
            .collect(Collectors.toList());
        System.out.println("Números pares: " + pares);
        
        // Mapear (transformar)
        List<Integer> cuadrados = numeros.stream()
            .map(n -> n * n)
            .collect(Collectors.toList());
        System.out.println("Cuadrados: " + cuadrados);
        
        // Reducir (sumar todos)
        int suma = numeros.stream()
            .reduce(0, Integer::sum);
        System.out.println("Suma total: " + suma);
        
        // Operaciones encadenadas
        List<String> nombres = Arrays.asList("Alice", "Bob", "Charlie", "David", "Eve");
        List<String> resultado = nombres.stream()
            .filter(n -> n.length() > 4)
            .map(String::toUpperCase)
            .sorted()
            .collect(Collectors.toList());
        System.out.println("Nombres filtrados y en mayúsculas: " + resultado);
        
        // Operaciones estadísticas
        OptionalDouble promedio = numeros.stream()
            .mapToInt(Integer::intValue)
            .average();
        promedio.ifPresent(p -> System.out.println("Promedio: " + p));
        
        // Stream de rangos
        IntStream.range(1, 11)
            .forEach(n -> System.out.print(n + " "));
        System.out.println();
    }
}

