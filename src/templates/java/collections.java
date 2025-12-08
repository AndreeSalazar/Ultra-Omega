// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Colecciones
// ═══════════════════════════════════════════════════════════════════════════

import java.util.*;

public class Main {
    public static void main(String[] args) {
        // List (ArrayList)
        List<String> lista = new ArrayList<>();
        lista.add("Java");
        lista.add("25");
        lista.add("Ultra-Omega");
        System.out.println("Lista: " + lista);
        
        // Set (HashSet)
        Set<Integer> conjunto = new HashSet<>();
        conjunto.add(1);
        conjunto.add(2);
        conjunto.add(3);
        conjunto.add(1); // Duplicado, no se agrega
        System.out.println("Set: " + conjunto);
        
        // Map (HashMap)
        Map<String, Integer> mapa = new HashMap<>();
        mapa.put("Alice", 25);
        mapa.put("Bob", 30);
        mapa.put("Charlie", 35);
        System.out.println("Map: " + mapa);
        System.out.println("Edad de Alice: " + mapa.get("Alice"));
        
        // Iteración con for-each mejorado
        System.out.println("\nIterando lista:");
        for (String elemento : lista) {
            System.out.println("  - " + elemento);
        }
        
        // Iteración con forEach (Java 8+)
        System.out.println("\nIterando con forEach:");
        lista.forEach(elemento -> System.out.println("  → " + elemento));
        
        // Stream API (Java 8+)
        System.out.println("\nFiltrando elementos:");
        lista.stream()
            .filter(s -> s.length() > 3)
            .forEach(s -> System.out.println("  ✓ " + s));
    }
}

