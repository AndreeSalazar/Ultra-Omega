// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Sealed Classes (Java 17+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Usar clases selladas
        Forma circulo = new Circulo(5.0);
        Forma rectangulo = new Rectangulo(4.0, 6.0);
        Forma triangulo = new Triangulo(3.0, 4.0, 5.0);
        
        System.out.println("Área del círculo: " + circulo.calcularArea());
        System.out.println("Área del rectángulo: " + rectangulo.calcularArea());
        System.out.println("Área del triángulo: " + triangulo.calcularArea());
        
        // Pattern matching con switch (Java 17+)
        imprimirInfo(circulo);
        imprimirInfo(rectangulo);
        imprimirInfo(triangulo);
    }
    
    static void imprimirInfo(Forma forma) {
        switch (forma) {
            case Circulo c -> System.out.println("Es un círculo con radio: " + c.radio());
            case Rectangulo r -> System.out.println("Es un rectángulo: " + r.ancho() + "x" + r.alto());
            case Triangulo t -> System.out.println("Es un triángulo con lados: " + t.a() + ", " + t.b() + ", " + t.c());
        }
    }
}

// Clase sellada - solo permite estas subclases
sealed abstract class Forma permits Circulo, Rectangulo, Triangulo {
    abstract double calcularArea();
    abstract double calcularPerimetro();
}

// Subclases permitidas
final class Circulo extends Forma {
    private final double radio;
    
    public Circulo(double radio) {
        this.radio = radio;
    }
    
    public double radio() { return radio; }
    
    @Override
    double calcularArea() {
        return Math.PI * radio * radio;
    }
    
    @Override
    double calcularPerimetro() {
        return 2 * Math.PI * radio;
    }
}

final class Rectangulo extends Forma {
    private final double ancho, alto;
    
    public Rectangulo(double ancho, double alto) {
        this.ancho = ancho;
        this.alto = alto;
    }
    
    public double ancho() { return ancho; }
    public double alto() { return alto; }
    
    @Override
    double calcularArea() {
        return ancho * alto;
    }
    
    @Override
    double calcularPerimetro() {
        return 2 * (ancho + alto);
    }
}

final class Triangulo extends Forma {
    private final double a, b, c;
    
    public Triangulo(double a, double b, double c) {
        this.a = a;
        this.b = b;
        this.c = c;
    }
    
    public double a() { return a; }
    public double b() { return b; }
    public double c() { return c; }
    
    @Override
    double calcularArea() {
        // Fórmula de Herón
        double s = (a + b + c) / 2;
        return Math.sqrt(s * (s - a) * (s - b) * (s - c));
    }
    
    @Override
    double calcularPerimetro() {
        return a + b + c;
    }
}

