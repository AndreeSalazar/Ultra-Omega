// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Anotaciones
// ═══════════════════════════════════════════════════════════════════════════

import java.lang.annotation.*;

public class Main {
    public static void main(String[] args) {
        // Obtener información de anotaciones
        Class<?> clase = MiClase.class;
        
        if (clase.isAnnotationPresent(Info.class)) {
            Info info = clase.getAnnotation(Info.class);
            System.out.println("Autor: " + info.autor());
            System.out.println("Versión: " + info.version());
            System.out.println("Fecha: " + info.fecha());
        }
        
        // Usar clase anotada
        MiClase objeto = new MiClase();
        objeto.metodoImportante();
    }
}

// Definir anotación personalizada
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
@interface Info {
    String autor() default "Desconocido";
    String version() default "1.0";
    String fecha() default "2025-01-01";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface Deprecado {
    String razon() default "";
}

// Usar anotaciones
@Info(autor = "Ultra-Omega Team", version = "25.0", fecha = "2025-01-27")
class MiClase {
    @Deprecado(razon = "Usar nuevoMetodo() en su lugar")
    public void metodoAntiguo() {
        System.out.println("Este método está deprecado");
    }
    
    public void metodoImportante() {
        System.out.println("Este es un método importante");
    }
}

