// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Switch Expressions (Java 14+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        int dia = 3;
        
        // Switch expression (Java 14+)
        String nombreDia = switch (dia) {
            case 1 -> "Lunes";
            case 2 -> "Martes";
            case 3 -> "Miércoles";
            case 4 -> "Jueves";
            case 5 -> "Viernes";
            case 6 -> "Sábado";
            case 7 -> "Domingo";
            default -> "Día inválido";
        };
        
        System.out.println("Día " + dia + " es: " + nombreDia);
        
        // Switch con múltiples casos
        String tipo = switch (dia) {
            case 1, 2, 3, 4, 5 -> "Día laboral";
            case 6, 7 -> "Fin de semana";
            default -> "Desconocido";
        };
        System.out.println("Tipo: " + tipo);
        
        // Switch con bloques
        int resultado = switch (dia) {
            case 1, 2, 3 -> {
                System.out.println("Primera parte de la semana");
                yield 100;
            }
            case 4, 5 -> {
                System.out.println("Final de semana laboral");
                yield 200;
            }
            default -> 0;
        };
        System.out.println("Resultado: " + resultado);
        
        // Pattern matching con switch (Java 17+)
        Object objeto = "Java 25";
        String descripcion = switch (objeto) {
            case String s when s.length() > 5 -> "String largo: " + s;
            case String s -> "String: " + s;
            case Integer i -> "Integer: " + i;
            case null -> "null";
            default -> "Desconocido";
        };
        System.out.println("Descripción: " + descripcion);
    }
}

