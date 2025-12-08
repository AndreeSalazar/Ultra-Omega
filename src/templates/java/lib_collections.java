// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Librería de Colecciones Extendidas
// ═══════════════════════════════════════════════════════════════════════════

import java.util.*;
import java.util.stream.Collectors;

public class CollectionUtils {
    // Obtener primer elemento o valor por defecto
    public static <T> Optional<T> first(List<T> lista) {
        return lista.isEmpty() ? Optional.empty() : Optional.of(lista.get(0));
    }
    
    // Obtener último elemento
    public static <T> Optional<T> last(List<T> lista) {
        return lista.isEmpty() ? Optional.empty() : Optional.of(lista.get(lista.size() - 1));
    }
    
    // Particionar lista en chunks
    public static <T> List<List<T>> partition(List<T> lista, int tamaño) {
        List<List<T>> resultado = new ArrayList<>();
        for (int i = 0; i < lista.size(); i += tamaño) {
            resultado.add(lista.subList(i, Math.min(i + tamaño, lista.size())));
        }
        return resultado;
    }
    
    // Invertir lista
    public static <T> List<T> reverse(List<T> lista) {
        List<T> resultado = new ArrayList<>(lista);
        Collections.reverse(resultado);
        return resultado;
    }
    
    // Remover duplicados manteniendo orden
    public static <T> List<T> unique(List<T> lista) {
        return lista.stream()
            .distinct()
            .collect(Collectors.toList());
    }
    
    // Combinar múltiples listas
    @SafeVarargs
    public static <T> List<T> concat(List<T>... listas) {
        List<T> resultado = new ArrayList<>();
        for (List<T> lista : listas) {
            resultado.addAll(lista);
        }
        return resultado;
    }
    
    // Agrupar por tamaño
    public static <T> Map<Integer, List<T>> groupBySize(List<T> lista, int tamaño) {
        Map<Integer, List<T>> resultado = new HashMap<>();
        for (int i = 0; i < lista.size(); i++) {
            int grupo = i / tamaño;
            resultado.computeIfAbsent(grupo, k -> new ArrayList<>()).add(lista.get(i));
        }
        return resultado;
    }
    
    // Ejemplo de uso
    public static void main(String[] args) {
        List<Integer> numeros = Arrays.asList(1, 2, 3, 2, 4, 5, 1, 6);
        
        System.out.println("Lista original: " + numeros);
        System.out.println("Sin duplicados: " + unique(numeros));
        System.out.println("Invertida: " + reverse(numeros));
        System.out.println("Particionada (tamaño 3): " + partition(numeros, 3));
        
        first(numeros).ifPresent(f -> System.out.println("Primero: " + f));
        last(numeros).ifPresent(l -> System.out.println("Último: " + l));
    }
}

