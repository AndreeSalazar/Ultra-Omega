// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Variables y Tipos de Datos
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Tipos primitivos
        int entero = 42;
        double decimal = 3.14159;
        boolean verdadero = true;
        char caracter = 'A';
        String texto = "Hola Java 25";
        
        // Inferencia de tipos con 'var' (desde Java 10)
        var numero = 100;
        var nombre = "Ultra-Omega";
        
        // Variables finales (constantes)
        final int CONSTANTE = 100;
        
        System.out.println("Entero: " + entero);
        System.out.println("Decimal: " + decimal);
        System.out.println("Booleano: " + verdadero);
        System.out.println("Caracter: " + caracter);
        System.out.println("Texto: " + texto);
        System.out.println("Var numero: " + numero);
        System.out.println("Var nombre: " + nombre);
        System.out.println("Constante: " + CONSTANTE);
    }
}

