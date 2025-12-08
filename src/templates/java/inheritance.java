// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Herencia
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Crear instancias
        Vehiculo vehiculo = new Vehiculo("Vehículo Genérico", 2020);
        Automovil auto = new Automovil("Toyota", 2023, 4);
        Motocicleta moto = new Motocicleta("Yamaha", 2022, 250);
        
        // Polimorfismo
        vehiculo.mostrarInfo();
        System.out.println();
        
        auto.mostrarInfo();
        auto.acelerar();
        System.out.println();
        
        moto.mostrarInfo();
        moto.acelerar();
    }
}

// Clase base
class Vehiculo {
    protected String marca;
    protected int año;
    
    public Vehiculo(String marca, int año) {
        this.marca = marca;
        this.año = año;
    }
    
    public void mostrarInfo() {
        System.out.println("Vehículo: " + marca + " (" + año + ")");
    }
    
    public void acelerar() {
        System.out.println("El vehículo está acelerando");
    }
}

// Clase derivada
class Automovil extends Vehiculo {
    private int puertas;
    
    public Automovil(String marca, int año, int puertas) {
        super(marca, año); // Llamar constructor de la clase padre
        this.puertas = puertas;
    }
    
    @Override
    public void mostrarInfo() {
        super.mostrarInfo(); // Llamar método de la clase padre
        System.out.println("Tipo: Automóvil");
        System.out.println("Puertas: " + puertas);
    }
    
    @Override
    public void acelerar() {
        System.out.println("El automóvil acelera suavemente");
    }
}

class Motocicleta extends Vehiculo {
    private int cilindrada;
    
    public Motocicleta(String marca, int año, int cilindrada) {
        super(marca, año);
        this.cilindrada = cilindrada;
    }
    
    @Override
    public void mostrarInfo() {
        super.mostrarInfo();
        System.out.println("Tipo: Motocicleta");
        System.out.println("Cilindrada: " + cilindrada + "cc");
    }
    
    @Override
    public void acelerar() {
        System.out.println("La motocicleta acelera rápidamente");
    }
}

