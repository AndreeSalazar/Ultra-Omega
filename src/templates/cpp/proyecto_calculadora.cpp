// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Proyecto Completo (Calculadora)
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <cmath>
#include <stdexcept>

// Excepción personalizada para la calculadora
class CalculadoraException : public std::exception {
private:
    std::string mensaje;
    
public:
    CalculadoraException(const std::string& msg) : mensaje(msg) {}
    const char* what() const noexcept override {
        return mensaje.c_str();
    }
};

// Clase base para operaciones
class Operacion {
public:
    virtual ~Operacion() = default;
    virtual double ejecutar(double a, double b) const = 0;
    virtual std::string get_nombre() const = 0;
    virtual std::string get_simbolo() const = 0;
};

// Operaciones concretas
class Suma : public Operacion {
public:
    double ejecutar(double a, double b) const override {
        return a + b;
    }
    std::string get_nombre() const override {
        return "Suma";
    }
    std::string get_simbolo() const override {
        return "+";
    }
};

class Resta : public Operacion {
public:
    double ejecutar(double a, double b) const override {
        return a - b;
    }
    std::string get_nombre() const override {
        return "Resta";
    }
    std::string get_simbolo() const override {
        return "-";
    }
};

class Multiplicacion : public Operacion {
public:
    double ejecutar(double a, double b) const override {
        return a * b;
    }
    std::string get_nombre() const override {
        return "Multiplicación";
    }
    std::string get_simbolo() const override {
        return "*";
    }
};

class Division : public Operacion {
public:
    double ejecutar(double a, double b) const override {
        if (b == 0.0) {
            throw CalculadoraException("Error: División por cero");
        }
        return a / b;
    }
    std::string get_nombre() const override {
        return "División";
    }
    std::string get_simbolo() const override {
        return "/";
    }
};

class Potencia : public Operacion {
public:
    double ejecutar(double a, double b) const override {
        return std::pow(a, b);
    }
    std::string get_nombre() const override {
        return "Potencia";
    }
    std::string get_simbolo() const override {
        return "^";
    }
};

// Registro de operaciones
struct RegistroOperacion {
    double operando1;
    double operando2;
    std::string operacion;
    double resultado;
    std::string timestamp;
    
    RegistroOperacion(double a, double b, const std::string& op, double res)
        : operando1(a), operando2(b), operacion(op), resultado(res) {
        // Timestamp simple
        timestamp = "2024-01-01 12:00:00"; // Simplificado
    }
};

// Clase principal de la calculadora
class Calculadora {
private:
    std::vector<std::unique_ptr<Operacion>> operaciones;
    std::vector<RegistroOperacion> historial;
    
public:
    Calculadora() {
        // Registrar operaciones disponibles
        operaciones.push_back(std::make_unique<Suma>());
        operaciones.push_back(std::make_unique<Resta>());
        operaciones.push_back(std::make_unique<Multiplicacion>());
        operaciones.push_back(std::make_unique<Division>());
        operaciones.push_back(std::make_unique<Potencia>());
    }
    
    void mostrar_operaciones() const {
        std::cout << "🔷 Operaciones Disponibles:" << std::endl;
        std::cout << "========================" << std::endl;
        
        for (size_t i = 0; i < operaciones.size(); i++) {
            std::cout << (i + 1) << ". " 
                     << operaciones[i]->get_nombre() 
                     << " (" << operaciones[i]->get_simbolo() << ")" << std::endl;
        }
        std::cout << std::endl;
    }
    
    double calcular(size_t indice_op, double a, double b) {
        if (indice_op >= operaciones.size()) {
            throw CalculadoraException("Índice de operación inválido");
        }
        
        const auto& op = operaciones[indice_op];
        double resultado = op->ejecutar(a, b);
        
        // Registrar en historial
        historial.emplace_back(a, b, op->get_nombre(), resultado);
        
        return resultado;
    }
    
    void mostrar_historial() const {
        std::cout << "📊 Historial de Operaciones:" << std::endl;
        std::cout << "============================" << std::endl;
        
        if (historial.empty()) {
            std::cout << "No hay operaciones en el historial." << std::endl;
            return;
        }
        
        for (size_t i = 0; i < historial.size(); i++) {
            const auto& reg = historial[i];
            std::cout << (i + 1) << ". " 
                     << reg.operando1 << " " 
                     << reg.operacion << " " 
                     << reg.operando2 << " = " 
                     << reg.resultado << std::endl;
        }
        std::cout << std::endl;
    }
    
    void limpiar_historial() {
        historial.clear();
        std::cout << "✅ Historial limpiado" << std::endl;
    }
    
    void mostrar_estadisticas() const {
        std::cout << "📈 Estadísticas:" << std::endl;
        std::cout << "================" << std::endl;
        std::cout << "Total de operaciones: " << historial.size() << std::endl;
        
        if (historial.empty()) return;
        
        // Contar operaciones por tipo
        std::map<std::string, int> contador;
        for (const auto& reg : historial) {
            contador[reg.operacion]++;
        }
        
        std::cout << "Operaciones por tipo:" << std::endl;
        for (const auto& [tipo, cantidad] : contador) {
            std::cout << "  " << tipo << ": " << cantidad << std::endl;
        }
        
        // Calcular promedio de resultados
        double suma = 0.0;
        for (const auto& reg : historial) {
            suma += reg.resultado;
        }
        double promedio = suma / historial.size();
        
        std::cout << "Promedio de resultados: " << promedio << std::endl;
        std::cout << std::endl;
    }
};

// Interfaz de usuario
class InterfazUsuario {
private:
    Calculadora calculadora;
    bool ejecutando;
    
public:
    InterfazUsuario() : ejecutando(true) {}
    
    void mostrar_menu() {
        std::cout << "🔷 Ultra-Omega Calculadora C++" << std::endl;
        std::cout << "==============================" << std::endl;
        std::cout << "1. Realizar operación" << std::endl;
        std::cout << "2. Ver historial" << std::endl;
        std::cout << "3. Ver estadísticas" << std::endl;
        std::cout << "4. Limpiar historial" << std::endl;
        std::cout << "5. Salir" << std::endl;
        std::cout << "==============================" << std::endl;
    }
    
    void realizar_operacion() {
        calculadora.mostrar_operaciones();
        
        size_t indice_op;
        double a, b;
        
        std::cout << "Seleccione operación (1-" << 5 << "): ";
        std::cin >> indice_op;
        
        if (indice_op < 1 || indice_op > 5) {
            std::cout << "❌ Operación inválida" << std::endl;
            return;
        }
        
        std::cout << "Ingrese primer operando: ";
        std::cin >> a;
        
        std::cout << "Ingrese segundo operando: ";
        std::cin >> b;
        
        try {
            double resultado = calculadora.calcular(indice_op - 1, a, b);
            std::cout << "✅ Resultado: " << a << " + " << b << " = " << resultado << std::endl;
        } catch (const CalculadoraException& e) {
            std::cout << "❌ " << e.what() << std::endl;
        }
    }
    
    void ejecutar() {
        while (ejecutando) {
            mostrar_menu();
            
            int opcion;
            std::cout << "Seleccione opción: ";
            std::cin >> opcion;
            
            try {
                switch (opcion) {
                    case 1:
                        realizar_operacion();
                        break;
                    case 2:
                        calculadora.mostrar_historial();
                        break;
                    case 3:
                        calculadora.mostrar_estadisticas();
                        break;
                    case 4:
                        calculadora.limpiar_historial();
                        break;
                    case 5:
                        ejecutando = false;
                        std::cout << "👋 ¡Gracias por usar Ultra-Omega Calculadora!" << std::endl;
                        break;
                    default:
                        std::cout << "❌ Opción inválida" << std::endl;
                        break;
                }
            } catch (const std::exception& e) {
                std::cout << "❌ Error: " << e.what() << std::endl;
            }
            
            std::cout << std::endl;
        }
    }
};

int main() {
    std::cout << "🔷 Proyecto Completo - Calculadora C++" << std::endl;
    std::cout << "=====================================" << std::endl;
    std::cout << "Bienvenido a la calculadora Ultra-Omega" << std::endl;
    std::cout << "Implementada con POO, templates y manejo de errores" << std::endl;
    std::cout << std::endl;
    
    try {
        InterfazUsuario interfaz;
        interfaz.ejecutar();
    } catch (const std::exception& e) {
        std::cout << "❌ Error fatal: " << e.what() << std::endl;
        return 1;
    }
    
    return 0;
}
