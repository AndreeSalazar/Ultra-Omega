// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Reflection (Reflexión)
// ═══════════════════════════════════════════════════════════════════════════

import java.lang.reflect.*;

public class Main {
    public static void main(String[] args) throws Exception {
        // Obtener información de clase
        Class<?> clase = Persona.class;
        
        System.out.println("Nombre de la clase: " + clase.getName());
        System.out.println("Campos:");
        for (Field campo : clase.getDeclaredFields()) {
            System.out.println("  - " + campo.getName() + " : " + campo.getType());
        }
        
        System.out.println("Métodos:");
        for (Method metodo : clase.getDeclaredMethods()) {
            System.out.println("  - " + metodo.getName());
        }
        
        // Crear instancia usando reflection
        Constructor<?> constructor = clase.getConstructor(String.class, int.class);
        Object persona = constructor.newInstance("Alice", 25);
        
        // Invocar método usando reflection
        Method metodo = clase.getMethod("saludar");
        metodo.invoke(persona);
        
        // Acceder a campo privado
        Field campo = clase.getDeclaredField("nombre");
        campo.setAccessible(true);
        String nombre = (String) campo.get(persona);
        System.out.println("Nombre obtenido por reflection: " + nombre);
    }
}

class Persona {
    private String nombre;
    private int edad;
    
    public Persona(String nombre, int edad) {
        this.nombre = nombre;
        this.edad = edad;
    }
    
    public void saludar() {
        System.out.println("Hola, soy " + nombre);
    }
}

