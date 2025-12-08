// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Records (Java 14+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Crear instancias de record
        Persona persona1 = new Persona("Alice", 25);
        Persona persona2 = new Persona("Bob", 30);
        
        // Records generan automáticamente:
        // - Constructor
        // - Getters (sin 'get' prefix)
        // - equals(), hashCode(), toString()
        
        System.out.println("Persona 1: " + persona1);
        System.out.println("Nombre: " + persona1.nombre());
        System.out.println("Edad: " + persona1.edad());
        
        // Comparación
        System.out.println("¿Son iguales? " + persona1.equals(persona2));
        
        // Record con métodos adicionales
        Punto punto = new Punto(3, 4);
        System.out.println("Distancia al origen: " + punto.distanciaAlOrigen());
        
        // Record compacto (Java 14+)
        Coordenada coord = new Coordenada(10, 20);
        System.out.println("Coordenada: " + coord);
    }
}

// Record básico
record Persona(String nombre, int edad) {
    // Puedes agregar métodos adicionales
    public String presentacion() {
        return "Hola, soy " + nombre + " y tengo " + edad + " años";
    }
}

// Record con validación
record Punto(int x, int y) {
    // Constructor compacto (Java 14+)
    public Punto {
        if (x < 0 || y < 0) {
            throw new IllegalArgumentException("Las coordenadas deben ser positivas");
        }
    }
    
    public double distanciaAlOrigen() {
        return Math.sqrt(x * x + y * y);
    }
}

// Record con campos adicionales
record Coordenada(int x, int y) {
    private static final int ORIGEN_X = 0;
    private static final int ORIGEN_Y = 0;
    
    public double distancia(Coordenada otra) {
        int dx = x - otra.x;
        int dy = y - otra.y;
        return Math.sqrt(dx * dx + dy * dy);
    }
}

