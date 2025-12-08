// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Métodos y Funciones
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Llamar métodos estáticos
        saludar();
        saludarConNombre("Java 25");
        
        // Método con retorno
        int resultado = sumar(5, 3);
        System.out.println("5 + 3 = " + resultado);
        
        // Método con múltiples parámetros
        double promedio = calcularPromedio(10.0, 20.0, 30.0);
        System.out.println("Promedio: " + promedio);
        
        // Método sobrecargado
        System.out.println(sumar(5, 3));
        System.out.println(sumar(5.5, 3.2));
    }
    
    // Método sin parámetros ni retorno
    static void saludar() {
        System.out.println("¡Hola desde Java 25!");
    }
    
    // Método con parámetros
    static void saludarConNombre(String nombre) {
        System.out.println("¡Hola " + nombre + "!");
    }
    
    // Método con retorno
    static int sumar(int a, int b) {
        return a + b;
    }
    
    // Sobrecarga de métodos
    static double sumar(double a, double b) {
        return a + b;
    }
    
    // Método con varargs (parámetros variables)
    static double calcularPromedio(double... numeros) {
        if (numeros.length == 0) return 0.0;
        
        double suma = 0.0;
        for (double num : numeros) {
            suma += num;
        }
        return suma / numeros.length;
    }
}

