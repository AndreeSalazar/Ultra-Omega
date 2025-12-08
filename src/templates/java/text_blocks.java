// ═══════════════════════════════════════════════════════════════════════════
// Java 25 - Text Blocks (Java 13+)
// ═══════════════════════════════════════════════════════════════════════════

public class Main {
    public static void main(String[] args) {
        // Text block básico (Java 13+)
        String texto = """
            Este es un
            texto multilínea
            en Java 25
            """;
        System.out.println(texto);
        
        // Text block con formato JSON
        String json = """
            {
                "nombre": "Ultra-Omega",
                "version": "25.0",
                "lenguaje": "Java"
            }
            """;
        System.out.println(json);
        
        // Text block con interpolación (usando String.format o concat)
        String nombre = "Java 25";
        String mensaje = """
            Hola desde %s
            Este es un mensaje
            multilínea
            """.formatted(nombre);
        System.out.println(mensaje);
        
        // Text block con SQL
        String sql = """
            SELECT nombre, edad
            FROM personas
            WHERE edad > 18
            ORDER BY nombre
            """;
        System.out.println("SQL:\n" + sql);
        
        // Text block con HTML
        String html = """
            <html>
                <head>
                    <title>Java 25</title>
                </head>
                <body>
                    <h1>Ultra-Omega</h1>
                </body>
            </html>
            """;
        System.out.println("HTML:\n" + html);
    }
}

