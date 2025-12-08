// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Interfaces
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Implementación de interfaz
        Perro perro = new Perro();
        Gato gato = new Gato();
        
        perro.hacerSonido();
        perro.mover();
        
        gato.hacerSonido();
        gato.mover();
        
        // Métodos default (Java 8+)
        perro.dormir();
        gato.dormir();
        
        // Métodos estáticos en interfaces (Java 8+)
        Animal.mostrarInfo();
    }
}

// Interfaz con métodos abstractos
interface Animal {
    void hacerSonido();
    void mover();
    
    // Método default (Java 8+)
    default void dormir() {
        System.out.println("El animal está durmiendo...");
    }
    
    // Método estático (Java 8+)
    static void mostrarInfo() {
        System.out.println("Esta es una interfaz Animal");
    }
}

// Clase implementando interfaz
class Perro implements Animal {
    @Override
    public void hacerSonido() {
        System.out.println("¡Guau guau!");
    }
    
    @Override
    public void mover() {
        System.out.println("El perro está corriendo");
    }
}

class Gato implements Animal {
    @Override
    public void hacerSonido() {
        System.out.println("¡Miau miau!");
    }
    
    @Override
    public void mover() {
        System.out.println("El gato está caminando");
    }
}

