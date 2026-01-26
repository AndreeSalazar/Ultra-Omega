// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Manejo de Archivos
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <fstream>
#include <string>
#include <vector>

void escribir_archivo_texto() {
    std::cout << "📝 Escribiendo archivo de texto..." << std::endl;
    
    std::ofstream archivo("datos.txt");
    if (archivo.is_open()) {
        archivo << "Ultra-Omega C++ - Manejo de Archivos\n";
        archivo << "=====================================\n";
        archivo << "Este es un archivo de ejemplo.\n";
        archivo << "C++ facilita el manejo de archivos.\n";
        archivo << "Línea 4: Datos importantes\n";
        archivo << "Línea 5: Más información\n";
        archivo.close();
        std::cout << "✅ Archivo escrito exitosamente" << std::endl;
    } else {
        std::cout << "❌ Error al abrir el archivo para escritura" << std::endl;
    }
    std::cout << std::endl;
}

void leer_archivo_texto() {
    std::cout << "📖 Leyendo archivo de texto..." << std::endl;
    
    std::ifstream archivo("datos.txt");
    if (archivo.is_open()) {
        std::string linea;
        int numero_linea = 1;
        
        while (std::getline(archivo, linea)) {
            std::cout << "Línea " << numero_linea << ": " << linea << std::endl;
            numero_linea++;
        }
        archivo.close();
        std::cout << "✅ Archivo leído exitosamente" << std::endl;
    } else {
        std::cout << "❌ Error al abrir el archivo para lectura" << std::endl;
    }
    std::cout << std::endl;
}

void escribir_archivo_binario() {
    std::cout << "💾 Escribiendo archivo binario..." << std::endl;
    
    std::ofstream archivo("datos.bin", std::ios::binary);
    if (archivo.is_open()) {
        // Escribir diferentes tipos de datos
        int entero = 42;
        double decimal = 3.14159;
        std::string texto = "Ultra-Omega";
        
        archivo.write(reinterpret_cast<const char*>(&entero), sizeof(entero));
        archivo.write(reinterpret_cast<const char*>(&decimal), sizeof(decimal));
        
        // Escribir string: primero el tamaño, luego los caracteres
        size_t tamano = texto.size();
        archivo.write(reinterpret_cast<const char*>(&tamano), sizeof(tamano));
        archivo.write(texto.c_str(), tamano);
        
        archivo.close();
        std::cout << "✅ Archivo binario escrito exitosamente" << std::endl;
    } else {
        std::cout << "❌ Error al abrir el archivo binario para escritura" << std::endl;
    }
    std::cout << std::endl;
}

void leer_archivo_binario() {
    std::cout << "📖 Leyendo archivo binario..." << std::endl;
    
    std::ifstream archivo("datos.bin", std::ios::binary);
    if (archivo.is_open()) {
        // Leer entero
        int entero;
        archivo.read(reinterpret_cast<char*>(&entero), sizeof(entero));
        
        // Leer double
        double decimal;
        archivo.read(reinterpret_cast<char*>(&decimal), sizeof(decimal));
        
        // Leer string
        size_t tamano;
        archivo.read(reinterpret_cast<char*>(&tamano), sizeof(tamano));
        
        std::string texto(tamano, '\0');
        archivo.read(&texto[0], tamano);
        
        std::cout << "Entero: " << entero << std::endl;
        std::cout << "Decimal: " << decimal << std::endl;
        std::cout << "Texto: " << texto << std::endl;
        
        archivo.close();
        std::cout << "✅ Archivo binario leído exitosamente" << std::endl;
    } else {
        std::cout << "❌ Error al abrir el archivo binario para lectura" << std::endl;
    }
    std::cout << std::endl;
}

void procesar_archivo_csv() {
    std::cout << "📊 Procesando archivo CSV..." << std::endl;
    
    // Crear archivo CSV de ejemplo
    std::ofstream csv("datos.csv");
    if (csv.is_open()) {
        csv << "Nombre,Edad,Ciudad\n";
        csv << "Ana,25,Madrid\n";
        csv << "Carlos,30,Barcelona\n";
        csv << "Luis,22,Valencia\n";
        csv << "María,28,Sevilla\n";
        csv.close();
        
        // Leer y procesar CSV
        std::ifstream archivo("datos.csv");
        if (archivo.is_open()) {
            std::string linea;
            int numero_linea = 0;
            
            while (std::getline(archivo, linea)) {
                if (numero_linea == 0) {
                    std::cout << "Encabezado: " << linea << std::endl;
                } else {
                    // Procesar línea CSV (simple)
                    size_t pos1 = linea.find(',');
                    size_t pos2 = linea.find(',', pos1 + 1);
                    
                    if (pos1 != std::string::npos && pos2 != std::string::npos) {
                        std::string nombre = linea.substr(0, pos1);
                        std::string edad = linea.substr(pos1 + 1, pos2 - pos1 - 1);
                        std::string ciudad = linea.substr(pos2 + 1);
                        
                        std::cout << "Nombre: " << nombre 
                                 << ", Edad: " << edad 
                                 << ", Ciudad: " << ciudad << std::endl;
                    }
                }
                numero_linea++;
            }
            archivo.close();
            std::cout << "✅ CSV procesado exitosamente" << std::endl;
        }
    }
    std::cout << std::endl;
}

void verificar_existencia_archivo() {
    std::cout << "🔍 Verificando existencia de archivos..." << std::endl;
    
    std::vector<std::string> archivos = {"datos.txt", "datos.bin", "datos.csv", "no_existe.txt"};
    
    for (const auto& nombre : archivos) {
        std::ifstream archivo(nombre);
        if (archivo.good()) {
            std::cout << "✅ " << nombre << " - EXISTE" << std::endl;
        } else {
            std::cout << "❌ " << nombre << " - NO EXISTE" << std::endl;
        }
    }
    std::cout << std::endl;
}

int main() {
    std::cout << "🔷 Manejo de Archivos en C++" << std::endl;
    std::cout << "=============================" << std::endl;
    
    escribir_archivo_texto();
    leer_archivo_texto();
    
    escribir_archivo_binario();
    leer_archivo_binario();
    
    procesar_archivo_csv();
    
    verificar_existencia_archivo();
    
    return 0;
}
