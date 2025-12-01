// ═══════════════════════════════════════
// Clases y OOP en C++
// ═══════════════════════════════════════

#include <iostream>
#include <string>
#include <memory>

// ─────────────────────────────────────
// Clase base
// ─────────────────────────────────────
class Animal {
protected:
    std::string nombre;
    int edad;

public:
    Animal(const std::string& n, int e) : nombre(n), edad(e) {}
    virtual ~Animal() = default;
    
    virtual void hablar() const = 0;  // Método virtual puro
    
    std::string getNombre() const { return nombre; }
    int getEdad() const { return edad; }
};

// ─────────────────────────────────────
// Clase derivada
// ─────────────────────────────────────
class Perro : public Animal {
private:
    std::string raza;

public:
    Perro(const std::string& n, int e, const std::string& r)
        : Animal(n, e), raza(r) {}
    
    void hablar() const override {
        std::cout << nombre << " dice: ¡Guau!" << std::endl;
    }
    
    void correr() const {
        std::cout << nombre << " está corriendo!" << std::endl;
    }
};

class Gato : public Animal {
public:
    Gato(const std::string& n, int e) : Animal(n, e) {}
    
    void hablar() const override {
        std::cout << nombre << " dice: ¡Miau!" << std::endl;
    }
};

// ─────────────────────────────────────
// Clase con templates
// ─────────────────────────────────────
template<typename T>
class Contenedor {
private:
    T valor;

public:
    Contenedor(T v) : valor(v) {}
    T getValor() const { return valor; }
    void setValor(T v) { valor = v; }
};

int main() {
    // Crear objetos
    Perro perro("Max", 3, "Labrador");
    Gato gato("Luna", 2);
    
    // Polimorfismo
    Animal* animales[] = {&perro, &gato};
    
    std::cout << "=== Polimorfismo ===" << std::endl;
    for (const auto* animal : animales) {
        animal->hablar();
    }
    
    // Smart pointers
    std::cout << "\n=== Smart Pointers ===" << std::endl;
    auto ptr = std::make_unique<Perro>("Rocky", 5, "Pastor Alemán");
    ptr->hablar();
    ptr->correr();
    
    // Templates
    std::cout << "\n=== Templates ===" << std::endl;
    Contenedor<int> numerico(42);
    Contenedor<std::string> texto("Hola Mundo");
    
    std::cout << "Numérico: " << numerico.getValor() << std::endl;
    std::cout << "Texto: " << texto.getValor() << std::endl;
    
    return 0;
}

