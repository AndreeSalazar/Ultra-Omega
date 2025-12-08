// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Librería de I/O
// ═══════════════════════════════════════════════════════════════════════════
// Esta librería proporciona operaciones de entrada/salida mejoradas
// que pueden ser heredadas por otros nodos usando ch("lib_io")

import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.stream.*;

public class IOUtils {
    // Leer archivo completo como String
    public static String readFile(String ruta) throws IOException {
        return Files.readString(Path.of(ruta));
    }
    
    // Escribir String a archivo
    public static void writeFile(String ruta, String contenido) throws IOException {
        Files.writeString(Path.of(ruta), contenido);
    }
    
    // Leer archivo línea por línea
    public static List<String> readLines(String ruta) throws IOException {
        return Files.readAllLines(Path.of(ruta));
    }
    
    // Escribir líneas a archivo
    public static void writeLines(String ruta, List<String> lineas) throws IOException {
        Files.write(Path.of(ruta), lineas);
    }
    
    // Leer archivo como Stream de líneas
    public static Stream<String> readLinesStream(String ruta) throws IOException {
        return Files.lines(Path.of(ruta));
    }
    
    // Verificar si archivo existe
    public static boolean fileExists(String ruta) {
        return Files.exists(Path.of(ruta));
    }
    
    // Crear directorio si no existe
    public static void createDirectory(String ruta) throws IOException {
        Files.createDirectories(Path.of(ruta));
    }
    
    // Listar archivos en directorio
    public static List<String> listFiles(String directorio) throws IOException {
        return Files.list(Path.of(directorio))
            .map(p -> p.getFileName().toString())
            .collect(Collectors.toList());
    }
    
    // Copiar archivo
    public static void copyFile(String origen, String destino) throws IOException {
        Files.copy(Path.of(origen), Path.of(destino), StandardCopyOption.REPLACE_EXISTING);
    }
    
    // Leer propiedades desde archivo
    public static Properties loadProperties(String ruta) throws IOException {
        Properties props = new Properties();
        try (FileInputStream fis = new FileInputStream(ruta)) {
            props.load(fis);
        }
        return props;
    }
    
    // Guardar propiedades a archivo
    public static void saveProperties(Properties props, String ruta) throws IOException {
        try (FileOutputStream fos = new FileOutputStream(ruta)) {
            props.store(fos, "Propiedades guardadas");
        }
    }
    
    // Ejemplo de uso
    public static void main(String[] args) {
        try {
            // Crear archivo de prueba
            String ruta = "test.txt";
            writeFile(ruta, """
                Línea 1
                Línea 2
                Línea 3
                """);
            
            // Leer archivo
            String contenido = readFile(ruta);
            System.out.println("Contenido:\n" + contenido);
            
            // Leer líneas
            List<String> lineas = readLines(ruta);
            System.out.println("Líneas: " + lineas);
            
            // Limpiar
            Files.deleteIfExists(Path.of(ruta));
        } catch (IOException e) {
            System.err.println("Error de I/O: " + e.getMessage());
        }
    }
}

