// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Record Patterns (Java 19+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Record patterns con instanceof (Java 19+)
        Object punto1 = new Punto(3, 4);
        Object punto2 = new Punto(0, 0);
        
        if (punto1 instanceof Punto(int x, int y)) {
            System.out.println("Punto en (" + x + ", " + y + ")");
            System.out.println("Distancia: " + Math.sqrt(x * x + y * y));
        }
        
        // Record patterns con switch (Java 19+)
        Forma forma1 = new Circulo(5.0);
        Forma forma2 = new Rectangulo(4.0, 6.0);
        
        imprimirArea(forma1);
        imprimirArea(forma2);
        
        // Record patterns anidados
        Par<Punto> par = new Par<>(new Punto(1, 2), new Punto(3, 4));
        if (par instanceof Par<Punto>(Punto(int x1, int y1), Punto(int x2, int y2))) {
            System.out.println("\nPar de puntos:");
            System.out.println("  Punto 1: (" + x1 + ", " + y1 + ")");
            System.out.println("  Punto 2: (" + x2 + ", " + y2 + ")");
        }
    }
    
    static void imprimirArea(Forma forma) {
        double area = switch (forma) {
            case Circulo(double radio) -> Math.PI * radio * radio;
            case Rectangulo(double ancho, double alto) -> ancho * alto;
        };
        System.out.println("Área: " + area);
    }
}

record Punto(int x, int y) {}
record Circulo(double radio) implements Forma {}
record Rectangulo(double ancho, double alto) implements Forma {}
sealed interface Forma permits Circulo, Rectangulo {}
record Par<T>(T primero, T segundo) {}

