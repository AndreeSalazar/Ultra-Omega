// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Excepciones y Manejo de Errores
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

// Excepción personalizada
class UltraOmegaException : public std::exception {
private:
    std::string mensaje;
    
public:
    UltraOmegaException(const std::string& msg) : mensaje(msg) {}
    
    const char* what() const noexcept override {
        return mensaje.c_str();
    }
};

// Clase para demostrar manejo de errores
class Calculadora {
private:
    std::vector<double> historial;
    
public:
    double dividir(double a, double b) {
        if (b == 0.0) {
            throw UltraOmegaException("Error: División por cero no permitida");
        }
        double resultado = a / b;
        historial.push_back(resultado);
        return resultado;
    }
    
    double raiz_cuadrada(double x) {
        if (x < 0.0) {
            throw UltraOmegaException("Error: No se puede calcular raíz cuadrada de número negativo");
        }
        return std::sqrt(x);
    }
    
    void mostrar_historial() const {
        std::cout << "📊 Historial de cálculos:" << std::endl;
        for (size_t i = 0; i < historial.size(); i++) {
            std::cout << i + 1 << ": " << historial[i] << std::endl;
        }
    }
};

// Función que puede lanzar excepciones
void procesar_datos(int valor) {
    if (valor < 0) {
        throw std::invalid_argument("El valor no puede ser negativo");
    }
    if (valor > 100) {
        throw std::out_of_range("El valor no puede ser mayor a 100");
    }
    if (valor == 13) {
        throw UltraOmegaException("Error: El número 13 no está permitido (superstición)");
    }
    
    std::cout << "✅ Valor " << valor << " procesado exitosamente" << std::endl;
}

void demostrar_excepciones_basicas() {
    std::cout << "⚠️ Excepciones Básicas:" << std::endl;
    
    try {
        // Código que puede lanzar excepciones
        int valor = 13;
        procesar_datos(valor);
    } catch (const std::invalid_argument& e) {
        std::cout << "❌ Argumento inválido: " << e.what() << std::endl;
    } catch (const std::out_of_range& e) {
        std::cout << "❌ Fuera de rango: " << e.what() << std::endl;
    } catch (const UltraOmegaException& e) {
        std::cout << "❌ Error personalizado: " << e.what() << std::endl;
    } catch (...) {
        std::cout << "❌ Error desconocido capturado" << std::endl;
    }
    std::cout << std::endl;
}

void demostrar_excepciones_anidadas() {
    std::cout << "⚠️ Excepciones Anidadas:" << std::endl;
    
    try {
        try {
            Calculadora calc;
            double resultado = calc.dividir(10.0, 0.0);
            std::cout << "Resultado: " << resultado << std::endl;
        } catch (const UltraOmegaException& e) {
            std::cout << "Error interno: " << e.what() << std::endl;
            // Relanzar con contexto adicional
            throw std::runtime_error("Error en cálculo matemático");
        }
    } catch (const std::exception& e) {
        std::cout << "Error capturado externamente: " << e.what() << std::endl;
    }
    std::cout << std::endl;
}

void demostrar_noexcept() {
    std::cout << "⚠️ Especificador noexcept:" << std::endl;
    
    // Función que no lanza excepciones
    auto funcion_segura = [](int x) noexcept -> int {
        return x * 2;
    };
    
    try {
        int resultado = funcion_segura(21);
        std::cout << "✅ Función segura: " << resultado << std::endl;
    } catch (...) {
        std::cout << "❌ Esto nunca debería ejecutarse" << std::endl;
    }
    std::cout << std::endl;
}

class GestorRecursos {
private:
    std::string nombre;
    
public:
    GestorRecursos(const std::string& nom) : nombre(nom) {
        std::cout << "🔧 Adquiriendo recurso: " << nombre << std::endl;
    }
    
    ~GestorRecursos() {
        std::cout << "🗑️ Liberando recurso: " << nombre << std::endl;
    }
    
    void operacion_riesgosa() {
        throw UltraOmegaException("Error durante operación riesgosa");
    }
};

void demostrar_raii() {
    std::cout << "⚠️ RAII (Resource Acquisition Is Initialization):" << std::endl;
    
    try {
        GestorRecursos gestor("Archivo importante");
        gestor.operacion_riesgosa();
    } catch (const UltraOmegaException& e) {
        std::cout << "❌ Error capturado: " << e.what() << std::endl;
    }
    // El recurso se libera automáticamente gracias al destructor
    std::cout << std::endl;
}

void demostrar_try_catch_moderno() {
    std::cout << "⚠️ Manejo Moderno con Smart Pointers:" << std::endl;
    
    try {
        auto calc = std::make_unique<Calculadora>();
        
        // Operaciones seguras
        double resultado1 = calc->dividir(20.0, 4.0);
        std::cout << "20 / 4 = " << resultado1 << std::endl;
        
        double resultado2 = calc->raiz_cuadrada(16.0);
        std::cout << "√16 = " << resultado2 << std::endl;
        
        // Operación que fallará
        double resultado3 = calc->dividir(10.0, 0.0);
        std::cout << "10 / 0 = " << resultado3 << std::endl;
        
        calc->mostrar_historial();
        
    } catch (const UltraOmegaException& e) {
        std::cout << "❌ Error de cálculo: " << e.what() << std::endl;
    } catch (const std::exception& e) {
        std::cout << "❌ Error estándar: " << e.what() << std::endl;
    }
    std::cout << std::endl;
}

int main() {
    std::cout << "🔷 Excepciones y Manejo de Errores en C++" << std::endl;
    std::cout << "=====================================" << std::endl;
    
    demostrar_excepciones_basicas();
    demostrar_excepciones_anidadas();
    demostrar_noexcept();
    demostrar_raii();
    demostrar_try_catch_moderno();
    
    std::cout << "💡 Mejores prácticas:" << std::endl;
    std::cout << "- Usar excepciones para errores excepcionales" << std::endl;
    std::cout << "- Usar códigos de error para errores esperados" << std::endl;
    std::cout << "- Atrapar excepciones por referencia constante" << std::endl;
    std::cout << "- Usar RAII para gestión automática de recursos" << std::endl;
    std::cout << "- Especificar noexcept cuando sea posible" << std::endl;
    
    return 0;
}
