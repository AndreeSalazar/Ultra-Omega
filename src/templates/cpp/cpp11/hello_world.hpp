// ═══════════════════════════════════════════════════════════════════════════
// Ultra-Omega C++11 Template - Hello World Header
// ═══════════════════════════════════════════════════════════════════════════

#ifndef HELLO_WORLD_HPP
#define HELLO_WORLD_HPP

#include <iostream>
#include <string>
#include <memory>
#include <vector>

/**
 * @brief Clase de demostración para C++11 - Hello World
 * 
 * Esta clase demuestra las características fundamentales de C++11:
 * - Smart pointers (std::unique_ptr)
 * - Auto keyword
 * - Move semantics
 * - Inicialización uniforme
 */
class HelloWorld {
private:
    std::unique_ptr<std::string> message;
    std::vector<int> numbers;
    
public:
    /**
     * @brief Constructor con inicialización uniforme (C++11)
     * @param msg Mensaje a mostrar
     */
    explicit HelloWorld(std::string msg);
    
    /**
     * @brief Destructor virtual para polimorfismo
     */
    virtual ~HelloWorld() = default;
    
    /**
     * @brief Muestra el mensaje de bienvenida (C++11 override)
     */
    virtual void show_message() const;
    
    /**
     * @brief Procesa números usando características C++11
     * @return std::vector<int> números procesados
     */
    std::vector<int> process_numbers() const;
    
    /**
     * @brief Demuestra lambda expressions (C++11)
     * @param value Valor a procesar
     * @return int resultado del procesamiento
     */
    auto lambda_demo(int value) const -> int;
    
    /**
     * @brief Obtiene información del objeto (C++11 auto return)
     * @return std::pair<size_t, std::string> información
     */
    auto get_info() const -> std::pair<size_t, std::string>;
    
    // Eliminar constructor de copia (C++11)
    HelloWorld(const HelloWorld&) = delete;
    
    // Permitir move semantics (C++11)
    HelloWorld(HelloWorld&&) = default;
    
    // Eliminar asignación de copia (C++11)
    HelloWorld& operator=(const HelloWorld&) = delete;
    
    // Permitir move assignment (C++11)
    HelloWorld& operator=(HelloWorld&&) = default;
};

#endif // HELLO_WORLD_HPP
