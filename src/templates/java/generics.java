// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Generics (Genéricos)
// ═══════════════════════════════════════════════════════════════════════════

import java.util.*;

public class Main {
    public static void main(String[] args) {
        // Caja genérica
        Caja<String> cajaTexto = new Caja<>("Hola Java 25");
        Caja<Integer> cajaNumero = new Caja<>(42);
        
        System.out.println("Caja texto: " + cajaTexto.obtener());
        System.out.println("Caja numero: " + cajaNumero.obtener());
        
        // Lista genérica
        ListaGenerica<String> lista = new ListaGenerica<>();
        lista.agregar("Elemento 1");
        lista.agregar("Elemento 2");
        lista.agregar("Elemento 3");
        lista.mostrar();
        
        // Método genérico
        System.out.println("\nMáximo de 5 y 10: " + maximo(5, 10));
        System.out.println("Máximo de 'a' y 'z': " + maximo('a', 'z'));
    }
    
    // Método genérico con bounded type
    public static <T extends Comparable<T>> T maximo(T a, T b) {
        return a.compareTo(b) > 0 ? a : b;
    }
}

// Clase genérica simple
class Caja<T> {
    private T contenido;
    
    public Caja(T contenido) {
        this.contenido = contenido;
    }
    
    public T obtener() {
        return contenido;
    }
    
    public void establecer(T contenido) {
        this.contenido = contenido;
    }
}

// Clase genérica con múltiples tipos
class ListaGenerica<T> {
    private List<T> elementos = new ArrayList<>();
    
    public void agregar(T elemento) {
        elementos.add(elemento);
    }
    
    public void mostrar() {
        elementos.forEach(System.out::println);
    }
}

