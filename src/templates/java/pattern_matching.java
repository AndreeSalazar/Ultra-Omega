// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Pattern Matching (Java 16+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        Object objeto1 = "Hola Java 25";
        Object objeto2 = 42;
        Object objeto3 = new Persona("Alice", 25);
        
        // Pattern matching con instanceof (Java 16+)
        if (objeto1 instanceof String s) {
            System.out.println("Es un String: " + s.toUpperCase());
        }
        
        if (objeto2 instanceof Integer i) {
            System.out.println("Es un Integer: " + (i * 2));
        }
        
        if (objeto3 instanceof Persona p) {
            System.out.println("Es una Persona: " + p.nombre());
        }
        
        // Pattern matching con switch (Java 17+)
        System.out.println("Tipo de objeto1: " + obtenerTipo(objeto1));
        System.out.println("Tipo de objeto2: " + obtenerTipo(objeto2));
        System.out.println("Tipo de objeto3: " + obtenerTipo(objeto3));
        
        // Pattern matching exhaustivo
        Forma forma1 = new Circulo(5.0);
        Forma forma2 = new Cuadrado(4.0);
        
        imprimirForma(forma1);
        imprimirForma(forma2);
    }
    
    static String obtenerTipo(Object obj) {
        return switch (obj) {
            case String s -> "String: " + s;
            case Integer i -> "Integer: " + i;
            case Persona p -> "Persona: " + p.nombre();
            case null -> "null";
            default -> "Desconocido: " + obj.getClass().getSimpleName();
        };
    }
    
    static void imprimirForma(Forma forma) {
        switch (forma) {
            case Circulo c -> System.out.println("Círculo con radio " + c.radio());
            case Cuadrado s -> System.out.println("Cuadrado con lado " + s.lado());
        }
    }
}

record Persona(String nombre, int edad) {}

sealed interface Forma permits Circulo, Cuadrado {}
record Circulo(double radio) implements Forma {}
record Cuadrado(double lado) implements Forma {}

