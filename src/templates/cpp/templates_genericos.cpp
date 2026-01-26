// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++ Template - Templates Genéricos
// ═══════════════════════════════════════════════════════════════════════════

#include <iostream>
#include <vector>
#include <string>
#include <type_traits>

// Template de función básico
template<typename T>
T maximo(T a, T b) {
    return (a > b) ? a : b;
}

// Template de función con múltiples parámetros
template<typename T, typename U>
void imprimir_pair(T primero, U segundo) {
    std::cout << "(" << primero << ", " << segundo << ")" << std::endl;
}

// Template de clase básico
template<typename T>
class Contenedor {
private:
    std::vector<T> elementos;
    
public:
    void agregar(const T& elemento) {
        elementos.push_back(elemento);
    }
    
    void mostrar() const {
        std::cout << "Contenedor [";
        for (size_t i = 0; i < elementos.size(); i++) {
            std::cout << elementos[i];
            if (i < elementos.size() - 1) std::cout << ", ";
        }
        std::cout << "]" << std::endl;
    }
    
    T get_maximo() const {
        if (elementos.empty()) {
            throw std::runtime_error("Contenedor vacío");
        }
        
        T max_val = elementos[0];
        for (const T& elem : elementos) {
            if (elem > max_val) {
                max_val = elem;
            }
        }
        return max_val;
    }
};

// Template con especialización
template<typename T>
class Comparador {
public:
    static bool es_mayor(T a, T b) {
        return a > b;
    }
};

// Especialización para const char*
template<>
class Comparador<const char*> {
public:
    static bool es_mayor(const char* a, const char* b) {
        return strcmp(a, b) > 0;
    }
};

// Template con SFINAE
template<typename T>
typename std::enable_if<std::is_arithmetic<T>::value, T>::type
duplicar(T valor) {
    return valor * 2;
}

template<typename T>
typename std::enable_if<!std::is_arithmetic<T>::value, std::string>::type
duplicar(const T& valor) {
    return std::string(2, valor);
}

// Template variadic
template<typename... Args>
void imprimir_variadic(Args... args) {
    ((std::cout << args << " "), ...);
    std::cout << std::endl;
}

// Template con parámetros de template no tipo
template<typename T, size_t N>
class ArrayFijo {
private:
    T datos[N];
    
public:
    ArrayFijo() {
        for (size_t i = 0; i < N; i++) {
            datos[i] = T{};
        }
    }
    
    T& operator[](size_t indice) {
        if (indice >= N) {
            throw std::out_of_range("Índice fuera de rango");
        }
        return datos[indice];
    }
    
    const T& operator[](size_t indice) const {
        if (indice >= N) {
            throw std::out_of_range("Índice fuera de rango");
        }
        return datos[indice];
    }
    
    constexpr size_t size() const {
        return N;
    }
    
    void mostrar() const {
        std::cout << "ArrayFijo[";
        for (size_t i = 0; i < N; i++) {
            std::cout << datos[i];
            if (i < N - 1) std::cout << ", ";
        }
        std::cout << "]" << std::endl;
    }
};

// Template de clase con múltiples parámetros
template<typename K, typename V>
class MapaSimple {
private:
    struct Par {
        K clave;
        V valor;
    };
    
    std::vector<Par> pares;
    
public:
    void agregar(const K& clave, const V& valor) {
        pares.push_back({clave, valor});
    }
    
    V* buscar(const K& clave) {
        for (auto& par : pares) {
            if (par.clave == clave) {
                return &par.valor;
            }
        }
        return nullptr;
    }
    
    void mostrar() const {
        std::cout << "MapaSimple {" << std::endl;
        for (const auto& par : pares) {
            std::cout << "  " << par.clave << ": " << par.valor << std::endl;
        }
        std::cout << "}" << std::endl;
    }
};

void demostrar_templates_funcion() {
    std::cout << "🔧 Templates de Función:" << std::endl;
    
    std::cout << "Máximo de 5 y 10: " << maximo(5, 10) << std::endl;
    std::cout << "Máximo de 3.14 y 2.71: " << maximo(3.14, 2.71) << std::endl;
    std::cout << "Máximo de 'Z' y 'A': " << maximo('Z', 'A') << std::endl;
    
    std::cout << "Pairs: ";
    imprimir_pair(42, "respuesta");
    imprimir_pair(3.14, "pi");
    imprimir_pair("Hola", 100);
    
    std::cout << std::endl;
}

void demostrar_templates_clase() {
    std::cout << "🔧 Templates de Clase:" << std::endl;
    
    Contenedor<int> contenedor_int;
    contenedor_int.agregar(5);
    contenedor_int.agregar(2);
    contenedor_int.agregar(8);
    contenedor_int.agregar(1);
    
    std::cout << "Contenedor de enteros: ";
    contenedor_int.mostrar();
    std::cout << "Máximo: " << contenedor_int.get_maximo() << std::endl;
    
    Contenedor<std::string> contenedor_str;
    contenedor_str.agregar("Manzana");
    contenedor_str.agregar("Naranja");
    contenedor_str.agregar("Plátano");
    
    std::cout << "Contenedor de strings: ";
    contenedor_str.mostrar();
    std::cout << "Máximo: " << contenedor_str.get_maximo() << std::endl;
    
    std::cout << std::endl;
}

void demostrar_especializacion() {
    std::cout << "🔧 Especialización de Templates:" << std::endl;
    
    std::cout << "Comparador<int>: " << Comparador<int>::es_mayor(10, 5) << std::endl;
    std::cout << "Comparador<double>: " << Comparador<double>::es_mayor(3.14, 2.71) << std::endl;
    std::cout << "Comparador<const char*>: " << Comparador<const char*>::es_mayor("Zebra", "Apple") << std::endl;
    
    std::cout << std::endl;
}

void demostrar_sfinaa() {
    std::cout << "🔧 SFINAE (Substitution Failure Is Not An Error):" << std::endl;
    
    std::cout << "Duplicar numérico (5): " << duplicar(5) << std::endl;
    std::cout << "Duplicar numérico (3.14): " << duplicar(3.14) << std::endl;
    std::cout << "Duplicar no numérico ('A'): " << duplicar('A') << std::endl;
    
    std::cout << std::endl;
}

void demostrar_templates_variadic() {
    std::cout << "🔧 Templates Variádicos:" << std::endl;
    
    imprimir_variadic(1, 2, 3, 4, 5);
    imprimir_variadic("Hola", "mundo", "C++");
    imprimir_variadic(42, 3.14, "Ultra-Omega", 'A');
    
    std::cout << std::endl;
}

void demostrar_array_fijo() {
    std::cout << "🔧 Array Fijo con Template:" << std::endl;
    
    ArrayFijo<int, 5> array_int;
    array_int[0] = 10;
    array_int[1] = 20;
    array_int[2] = 30;
    array_int[3] = 40;
    array_int[4] = 50;
    
    std::cout << "Array de enteros: ";
    array_int.mostrar();
    std::cout << "Tamaño: " << array_int.size() << std::endl;
    
    ArrayFijo<std::string, 3> array_str;
    array_str[0] = "Uno";
    array_str[1] = "Dos";
    array_str[2] = "Tres";
    
    std::cout << "Array de strings: ";
    array_str.mostrar();
    
    std::cout << std::endl;
}

void demostrar_mapa_simple() {
    std::cout << "🔧 Mapa Simple con Template:" << std::endl;
    
    MapaSimple<std::string, int> mapa_edades;
    mapa_edades.agregar("Ana", 25);
    mapa_edades.agregar("Carlos", 30);
    mapa_edades.agregar("Luis", 22);
    
    mapa_edades.mostrar();
    
    if (auto* edad = mapa_edades.buscar("Ana")) {
        std::cout << "Edad de Ana: " << *edad << std::endl;
    }
    
    MapaSimple<int, std::string> mapa_nombres;
    mapa_nombres.agregar(1, "Primero");
    mapa_nombres.agregar(2, "Segundo");
    mapa_nombres.agregar(3, "Tercero");
    
    mapa_nombres.mostrar();
    
    std::cout << std::endl;
}

int main() {
    std::cout << "🔷 Templates Genéricos en C++" << std::endl;
    std::cout << "=============================" << std::endl;
    
    demostrar_templates_funcion();
    demostrar_templates_clase();
    demostrar_especializacion();
    demostrar_sfinaa();
    demostrar_templates_variadic();
    demostrar_array_fijo();
    demostrar_mapa_simple();
    
    std::cout << "💡 Ventajas de los templates:" << std::endl;
    std::cout << "- Reutilización de código" << std::endl;
    std::cout << "- Type safety en tiempo de compilación" << std::endl;
    std::cout << "- Performance (sin overhead en tiempo de ejecución)" << std::endl;
    std::cout << "- Flexibilidad para trabajar con cualquier tipo" << std::endl;
    
    return 0;
}
