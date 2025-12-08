// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Manejo de Excepciones
// ═══════════════════════════════════════════════════════════════════════════

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // Try-catch básico
        try {
            int resultado = dividir(10, 0);
            System.out.println("Resultado: " + resultado);
        } catch (ArithmeticException e) {
            System.out.println("Error: " + e.getMessage());
        }
        
        // Try-catch-finally
        Scanner scanner = null;
        try {
            scanner = new Scanner(System.in);
            System.out.print("Ingresa un número: ");
            int numero = scanner.nextInt();
            System.out.println("Número ingresado: " + numero);
        } catch (Exception e) {
            System.out.println("Error al leer: " + e.getMessage());
        } finally {
            if (scanner != null) {
                scanner.close();
            }
        }
        
        // Try-with-resources (Java 7+)
        try (Scanner sc = new Scanner(System.in)) {
            System.out.println("Recurso manejado automáticamente");
        }
        
        // Lanzar excepción personalizada
        try {
            validarEdad(-5);
        } catch (EdadInvalidaException e) {
            System.out.println("Excepción personalizada: " + e.getMessage());
        }
    }
    
    static int dividir(int a, int b) {
        if (b == 0) {
            throw new ArithmeticException("División por cero no permitida");
        }
        return a / b;
    }
    
    static void validarEdad(int edad) throws EdadInvalidaException {
        if (edad < 0) {
            throw new EdadInvalidaException("La edad no puede ser negativa");
        }
        System.out.println("Edad válida: " + edad);
    }
}

// Excepción personalizada
class EdadInvalidaException extends Exception {
    public EdadInvalidaException(String mensaje) {
        super(mensaje);
    }
}

