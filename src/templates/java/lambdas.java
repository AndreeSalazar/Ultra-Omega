// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Expresiones Lambda y Functional Interfaces
// ═══════════════════════════════════════════════════════════════════════════

import java.util.*;
import java.util.function.*;

public class Main {
    public static void main(String[] args) {
        List<String> nombres = Arrays.asList("Alice", "Bob", "Charlie", "David");
        
        // Lambda básica
        nombres.forEach(nombre -> System.out.println(nombre));
        
        // Lambda con referencia a método
        nombres.forEach(System.out::println);
        
        // Functional Interfaces comunes
        // Predicate - evalúa una condición
        Predicate<Integer> esPar = n -> n % 2 == 0;
        System.out.println("¿10 es par? " + esPar.test(10));
        
        // Function - transforma un valor
        Function<String, Integer> longitud = s -> s.length();
        System.out.println("Longitud de 'Java': " + longitud.apply("Java"));
        
        // Consumer - consume un valor
        Consumer<String> imprimir = s -> System.out.println("→ " + s);
        nombres.forEach(imprimir);
        
        // Supplier - provee un valor
        Supplier<Double> numeroAleatorio = () -> Math.random();
        System.out.println("Número aleatorio: " + numeroAleatorio.get());
        
        // BinaryOperator - opera dos valores del mismo tipo
        BinaryOperator<Integer> sumar = (a, b) -> a + b;
        System.out.println("5 + 3 = " + sumar.apply(5, 3));
        
        // Lambda con múltiples líneas
        nombres.forEach(nombre -> {
            String mensaje = "Hola " + nombre;
            System.out.println(mensaje.toUpperCase());
        });
    }
}

