// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Punteros y Gestión de Memoria
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <memory>
#include <vector>

void demostrar_punteros_basicos() {
    std::cout << "📍 Punteros Básicos:" << std::endl;
    
    int numero = 42;
    int* ptr_numero = &numero;  // Puntero a número
    
    std::cout << "Valor: " << numero << std::endl;
    std::cout << "Dirección: " << ptr_numero << std::endl;
    std::cout << "Valor a través de puntero: " << *ptr_numero << std::endl;
    
    // Modificar valor a través de puntero
    *ptr_numero = 100;
    std::cout << "Nuevo valor: " << numero << std::endl;
    std::cout << std::endl;
}

void demostrar_punteros_y_arrays() {
    std::cout << "📍 Punteros y Arrays:" << std::endl;
    
    int numeros[] = {10, 20, 30, 40, 50};
    int* ptr_array = numeros;  // Array decae a puntero
    
    std::cout << "Array usando puntero: ";
    for (int i = 0; i < 5; i++) {
        std::cout << *(ptr_array + i) << " ";  // Aritmética de punteros
    }
    std::cout << std::endl;
    
    // Puntero a puntero
    int** ptr_ptr = &ptr_array;
    std::cout << "Primer elemento usando puntero a puntero: " << **ptr_ptr << std::endl;
    std::cout << std::endl;
}

void demostrar_asignacion_dinamica() {
    std::cout << "📍 Asignación Dinámica:" << std::endl;
    
    // Asignar un entero dinámicamente
    int* dinamico = new int(99);
    std::cout << "Valor dinámico: " << *dinamico << std::endl;
    delete dinamico;  // Liberar memoria
    
    // Asignar array dinámico
    int* array_dinamico = new int[5];
    for (int i = 0; i < 5; i++) {
        array_dinamico[i] = i * 10;
    }
    
    std::cout << "Array dinámico: ";
    for (int i = 0; i < 5; i++) {
        std::cout << array_dinamico[i] << " ";
    }
    std::cout << std::endl;
    
    delete[] array_dinamico;  // Liberar array
    std::cout << std::endl;
}

void demostrar_smart_pointers() {
    std::cout << "📍 Smart Pointers (C++11):" << std::endl;
    
    // unique_ptr - ownership exclusivo
    {
        auto unico = std::make_unique<int>(42);
        std::cout << "unique_ptr: " << *unico << std::endl;
        
        // Transferencia de ownership
        auto transferido = std::move(unico);
        std::cout << "Transferido: " << *transferido << std::endl;
        // unico ya no es válido aquí
    } // Liberado automáticamente
    
    // shared_ptr - ownership compartido
    {
        auto compartido = std::make_shared<std::string>("Ultra-Omega");
        std::cout << "shared_ptr: " << *compartido << std::endl;
        std::cout << "Referencias: " << compartido.use_count() << std::endl;
        
        {
            auto copia = compartido;  // Nueva referencia
            std::cout << "Referencias después de copia: " << compartido.use_count() << std::endl;
        } // copia se destruye aquí
        
        std::cout << "Referencias finales: " << compartido.use_count() << std::endl;
    } // Liberado cuando última referencia se destruye
    
    // weak_ptr - referencia débil
    {
        auto fuerte = std::make_shared<int>(100);
        auto debil = std::weak_ptr<int>(fuerte);
        
        std::cout << "Referencias fuertes: " << fuerte.use_count() << std::endl;
        
        if (auto bloqueado = debil.lock()) {  // Intentar obtener shared_ptr
            std::cout << "Valor a través de weak_ptr: " << *bloqueado << std::endl;
        }
    }
    std::cout << std::endl;
}

void demostrar_punteros_a_funciones() {
    std::cout << "📍 Punteros a Funciones:" << std::endl;
    
    // Función que toma puntero
    void (*ptr_funcion)(int) = [](int x) {
        std::cout << "Lambda con puntero: " << x << std::endl;
    };
    
    ptr_funcion(123);
    
    // Array de punteros a funciones
    void (*funciones[3])(int) = {
        [](int x) { std::cout << "Función 1: " << x << std::endl; },
        [](int x) { std::cout << "Función 2: " << x * 2 << std::endl; },
        [](int x) { std::cout << "Función 3: " << x * 3 << std::endl; }
    };
    
    for (int i = 0; i < 3; i++) {
        funciones[i](10);
    }
    std::cout << std::endl;
}

class Recurso {
private:
    std::string nombre;
    
public:
    Recurso(const std::string& nom) : nombre(nom) {
        std::cout << "🔧 Creando recurso: " << nombre << std::endl;
    }
    
    ~Recurso() {
        std::cout << "🗑️ Destruyendo recurso: " << nombre << std::endl;
    }
    
    void usar() const {
        std::cout << "📖 Usando recurso: " << nombre << std::endl;
    }
};

void demostrar_gestion_recursos() {
    std::cout << "📍 Gestión de Recursos:" << std::endl;
    
    // Forma tradicional (propensa a errores)
    std::cout << "Forma tradicional:" << std::endl;
    {
        Recurso* tradicional = new Recurso("Tradicional");
        tradicional->usar();
        delete tradicional;  // Si olvidamos esto, hay memory leak
    }
    
    // Con smart pointers (seguro)
    std::cout << "Con smart pointers:" << std::endl;
    {
        auto seguro = std::make_unique<Recurso("Seguro");
        seguro->usar();
        // Se libera automáticamente al salir del scope
    }
    std::cout << std::endl;
}

int main() {
    std::cout << "🔷 Punteros y Gestión de Memoria en C++" << std::endl;
    std::cout << "=====================================" << std::endl;
    
    demostrar_punteros_basicos();
    demostrar_punteros_y_arrays();
    demostrar_asignacion_dinamica();
    demostrar_smart_pointers();
    demostrar_punteros_a_funciones();
    demostrar_gestion_recursos();
    
    return 0;
}
