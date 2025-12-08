// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Clases y Objetos
// ═══════════════════════════════════════════════════════════════════════════

// Clase principal
public class Main {
    public static void main(String[] args) {
        // Crear instancias
        Persona persona1 = new Persona("Alice", 25);
        Persona persona2 = new Persona("Bob", 30);
        
        // Usar métodos
        persona1.saludar();
        persona2.saludar();
        
        // Acceder a propiedades
        System.out.println(persona1.getNombre() + " tiene " + persona1.getEdad() + " años");
    }
}

// Clase Persona
class Persona {
    // Campos privados
    private String nombre;
    private int edad;
    
    // Constructor
    public Persona(String nombre, int edad) {
        this.nombre = nombre;
        this.edad = edad;
    }
    
    // Métodos getter
    public String getNombre() {
        return nombre;
    }
    
    public int getEdad() {
        return edad;
    }
    
    // Método público
    public void saludar() {
        System.out.println("Hola, soy " + nombre);
    }
}

