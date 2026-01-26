// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Clases y POO
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <string>
#include <vector>
#include <memory>

// Clase básica
class Persona {
private:
    std::string nombre;
    int edad;
    
public:
    // Constructor
    Persona(const std::string& nom, int ed) : nombre(nom), edad(ed) {
        std::cout << "Creando persona: " << nombre << std::endl;
    }
    
    // Destructor
    ~Persona() {
        std::cout << "Destruyendo persona: " << nombre << std::endl;
    }
    
    // Métodos públicos
    void presentarse() const {
        std::cout << "Hola, soy " << nombre << " y tengo " << edad << " años." << std::endl;
    }
    
    // Getters y setters
    std::string getNombre() const { return nombre; }
    void setNombre(const std::string& nom) { nombre = nom; }
    
    int getEdad() const { return edad; }
    void setEdad(int ed) { edad = ed; }
    
    // Método estático
    static int getEdadMaxima() { return 120; }
};

// Clase con herencia
class Estudiante : public Persona {
private:
    std::string carrera;
    std::vector<double> calificaciones;
    
public:
    // Constructor que llama al constructor base
    Estudiante(const std::string& nom, int ed, const std::string& carr)
        : Persona(nom, ed), carrera(carr) {}
    
    // Método sobreescrito
    void presentarse() const {
        Persona::presentarse(); // Llama al método de la clase base
        std::cout << "Soy estudiante de " << carrera << "." << std::endl;
    }
    
    // Métodos específicos
    void agregarCalificacion(double calif) {
        calificaciones.push_back(calif);
    }
    
    double getPromedio() const {
        if (calificaciones.empty()) return 0.0;
        
        double suma = 0.0;
        for (double calif : calificaciones) {
            suma += calif;
        }
        return suma / calificaciones.size();
    }
};

// Clase con composición
class Universidad {
private:
    std::string nombre;
    std::vector<std::unique_ptr<Estudiante>> estudiantes;
    
public:
    Universidad(const std::string& nom) : nombre(nom) {}
    
    void agregarEstudiante(std::unique_ptr<Estudiante> est) {
        estudiantes.push_back(std::move(est));
    }
    
    void mostrarEstudiantes() const {
        std::cout << "Universidad " << nombre << std::endl;
        std::cout << "========================" << std::endl;
        
        for (const auto& est : estudiantes) {
            est->presentarse();
            std::cout << "Promedio: " << est->getPromedio() << std::endl;
            std::cout << "---" << std::endl;
        }
    }
};

int main() {
    std::cout << "🔷 Clases y Programación Orientada a Objetos" << std::endl;
    std::cout << "============================================" << std::endl;
    
    // Crear objetos usando punteros inteligentes
    auto persona1 = std::make_unique<Persona>("Carlos", 25);
    auto estudiante1 = std::make_unique<Estudiante>("Ana", 20, "Ingeniería");
    auto estudiante2 = std::make_unique<Estudiante>("Luis", 22, "Ciencias");
    
    // Usar objetos
    persona1->presentarse();
    
    estudiante1->agregarCalificacion(8.5);
    estudiante1->agregarCalificacion(9.0);
    estudiante1->agregarCalificacion(7.5);
    estudiante1->presentarse();
    
    estudiante2->agregarCalificacion(9.5);
    estudiante2->agregarCalificacion(8.8);
    estudiante2->presentarse();
    
    // Composición
    auto universidad = std::make_unique<Universidad>("Ultra-Omega Tech");
    universidad->agregarEstudiante(std::move(estudiante1));
    universidad->agregarEstudiante(std::move(estudiante2));
    
    std::cout << std::endl;
    universidad->mostrarEstudiantes();
    
    // Método estático
    std::cout << "Edad máxima permitida: " << Persona::getEdadMaxima() << std::endl;
    
    return 0;
}
