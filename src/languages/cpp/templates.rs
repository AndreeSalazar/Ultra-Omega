// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: C++ Templates Organizados por Estándar
// Templates organizados por versión y categoría con sintaxis estándar-respetuosa
// ═══════════════════════════════════════════════════════════════════════════════

use crate::core::node_graph::{Template, NodeLanguage};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CppTemplateCategory {
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color: (u8, u8, u8),
    pub templates: Vec<Template>,
}

#[derive(Debug, Clone)]
pub struct CppVersionManager {
    pub cpp11_categories: Vec<CppTemplateCategory>,
    pub cpp14_categories: Vec<CppTemplateCategory>,
    pub cpp17_categories: Vec<CppTemplateCategory>,
}

impl CppVersionManager {
    pub fn new() -> Self {
        Self {
            cpp11_categories: Self::get_cpp11_categories(),
            cpp14_categories: Self::get_cpp14_categories(),
            cpp17_categories: Self::get_cpp17_categories(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // C++11 CATEGORIES - Fundamentos Modernos
    // ═══════════════════════════════════════════════════════════════════════════════
    fn get_cpp11_categories() -> Vec<CppTemplateCategory> {
        vec![
            // Categoría: Fundamentos Básicos
            CppTemplateCategory {
                name: "Fundamentos",
                description: "Características fundamentales de C++11",
                icon: "🏗️",
                color: (0x00, 0x60, 0xAA),
                templates: vec![
                    Template {
                        name: "🔷 Hello Modern C++11",
                        code: include_str!("../../templates/cpp/cpp11/hello_simple.cpp"),
                        category: "C++11",
                        subcategory: "Fundamentos",
                        color: (0x00, 0x60, 0xAA),
                        icon: "👋",
                        language: NodeLanguage::Cpp,
                    },
                    Template {
                        name: "🔷 Auto Type Deduction",
                        code: r#"// Auto Type Deduction - C++11
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::cout << "🔷 Auto Type Deduction - C++11" << std::endl;
    
    // Auto con tipos básicos
    auto integer = 42;
    auto floating = 3.14;
    auto text = std::string("Hello C++11");
    
    // Auto con contenedores
    auto numbers = std::vector<int>{1, 2, 3, 4, 5};
    auto characters = std::vector<char>{'a', 'b', 'c'};
    
    std::cout << "Integer: " << integer << std::endl;
    std::cout << "Floating: " << floating << std::endl;
    std::cout << "Text: " << text << std::endl;
    
    std::cout << "Numbers: ";
    for (const auto& num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Fundamentos",
                        color: (0x00, 0x60, 0xAA),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                    Template {
                        name: "🔷 Range-Based For Loop",
                        code: r#"// Range-Based For Loop - C++11
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::cout << "🔷 Range-Based For Loop - C++11" << std::endl;
    
    // Vector de números
    std::vector<int> numbers = {10, 20, 30, 40, 50};
    
    std::cout << "Iterando sobre vector de números:" << std::endl;
    for (const auto& num : numbers) {
        std::cout << "  " << num << std::endl;
    }
    
    // Vector de strings
    std::vector<std::string> names = {"Alice", "Bob", "Charlie"};
    
    std::cout << "\nIterando sobre vector de nombres:" << std::endl;
    for (const auto& name : names) {
        std::cout << "  " << name << std::endl;
    }
    
    // Array tradicional
    int scores[] = {95, 87, 92, 78, 88};
    
    std::cout << "\nIterando sobre array tradicional:" << std::endl;
    for (const auto& score : scores) {
        std::cout << "  Score: " << score << std::endl;
    }
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Fundamentos",
                        color: (0x00, 0x60, 0xAA),
                        icon: "🔄",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
            
            // Categoría: Smart Pointers
            CppTemplateCategory {
                name: "Smart Pointers",
                description: "Gestión automática de memoria",
                icon: "🧠",
                color: (0x00, 0x70, 0xBB),
                templates: vec![
                    Template {
                        name: "🔷 Unique Pointer Demo",
                        code: r#"// Unique Pointer - C++11
#include <iostream>
#include <memory>
#include <vector>

class Resource {
private:
    std::string name;
public:
    Resource(const std::string& n) : name(n) {
        std::cout << "📦 Resource '" << name << "' creado" << std::endl;
    }
    
    ~Resource() {
        std::cout << "🗑️  Resource '" << name << "' destruido" << std::endl;
    }
    
    void use() {
        std::cout << "🔧 Usando resource: " << name << std::endl;
    }
};

int main() {
    std::cout << "🔷 Unique Pointer Demo - C++11" << std::endl;
    
    // Crear unique_ptr
    auto resource = std::make_unique<Resource>("MainResource");
    resource->use();
    
    // Vector de unique_ptrs
    std::vector<std::unique_ptr<Resource>> resources;
    
    // Agregar recursos al vector
    resources.push_back(std::make_unique<Resource("Resource1"));
    resources.push_back(std::make_unique<Resource>("Resource2"));
    resources.push_back(std::make_unique<Resource>("Resource3"));
    
    std::cout << "\nUsando recursos en vector:" << std::endl;
    for (const auto& res : resources) {
        res->use();
    }
    
    std::cout << "\n🔄 Moviendo ownership:" << std::endl;
    auto moved_resource = std::move(resource);
    if (moved_resource) {
        moved_resource->use();
    }
    
    std::cout << "\n✅ Todos los recursos se destruirán automáticamente" << std::endl;
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Smart Pointers",
                        color: (0x00, 0x70, 0xBB),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                    Template {
                        name: "🔷 Shared Pointer Demo",
                        code: r#"// Shared Pointer - C++11
#include <iostream>
#include <memory>
#include <vector>

class Sensor {
private:
    std::string id;
    double value;
public:
    Sensor(const std::string& sensor_id, double initial_value) 
        : id(sensor_id), value(initial_value) {
        std::cout << "📡 Sensor '" << id << "' creado con valor " << value << std::endl;
    }
    
    ~Sensor() {
        std::cout << "🔌 Sensor '" << id << "' destruido" << std::endl;
    }
    
    void update(double new_value) {
        value = new_value;
        std::cout << "📊 Sensor '" << id << "' actualizado a " << value << std::endl;
    }
    
    double read() const {
        return value;
    }
    
    const std::string& get_id() const {
        return id;
    }
};

int main() {
    std::cout << "🔷 Shared Pointer Demo - C++11" << std::endl;
    
    // Crear sensor compartido
    auto temperature_sensor = std::make_shared<Sensor>("TEMP_001", 25.5);
    
    // Múltiples observadores del mismo sensor
    std::vector<std::shared_ptr<Sensor>> observers;
    
    observers.push_back(temperature_sensor);
    observers.push_back(temperature_sensor);
    observers.push_back(temperature_sensor);
    
    std::cout << "\n👥 " << observers.size() << " observadores conectados al sensor" << std::endl;
    
    // Actualizar el sensor (todos los observadores ven el cambio)
    temperature_sensor->update(26.8);
    
    std::cout << "\n📖 Lecturas desde todos los observadores:" << std::endl;
    for (size_t i = 0; i < observers.size(); ++i) {
        std::cout << "  Observer " << (i + 1) << ": " 
                  << observers[i]->read() << "°C" << std::endl;
    }
    
    std::cout << "\n🔍 Reference count: " << temperature_sensor.use_count() << std::endl;
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Smart Pointers",
                        color: (0x00, 0x70, 0xBB),
                        icon: "🔗",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
            
            // Categoría: Lambda Expressions
            CppTemplateCategory {
                name: "Lambda Expressions",
                description: "Funciones anónimas y programación funcional",
                icon: "⚡",
                color: (0x00, 0x80, 0xCC),
                templates: vec![
                    Template {
                        name: "🔷 Lambda Basics",
                        code: r#"// Lambda Expressions Basics - C++11
#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

int main() {
    std::cout << "🔷 Lambda Expressions Basics - C++11" << std::endl;
    
    // Vector de números
    std::vector<int> numbers = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    
    // Lambda simple para imprimir
    auto print_number = [](int n) {
        std::cout << n << " ";
    };
    
    std::cout << "Números originales: ";
    std::for_each(numbers.begin(), numbers.end(), print_number);
    std::cout << std::endl;
    
    // Lambda con captura por valor
    int multiplier = 3;
    auto multiply_by_three = [multiplier](int n) {
        return n * multiplier;
    };
    
    std::cout << "Multiplicados por " << multiplier << ": ";
    std::for_each(numbers.begin(), numbers.end(), 
        [&print_number, multiply_by_three](int n) {
            print_number(multiply_by_three(n));
        });
    std::cout << std::endl;
    
    // Lambda con captura por referencia
    int sum = 0;
    auto accumulate_sum = [&sum](int n) {
        sum += n;
    };
    
    std::for_each(numbers.begin(), numbers.end(), accumulate_sum);
    std::cout << "Suma total: " << sum << std::endl;
    
    // Lambda para encontrar números pares
    std::vector<int> even_numbers;
    auto is_even = [](int n) {
        return n % 2 == 0;
    };
    
    std::copy_if(numbers.begin(), numbers.end(), 
                 std::back_inserter(even_numbers), is_even);
    
    std::cout << "Números pares: ";
    std::for_each(even_numbers.begin(), even_numbers.end(), print_number);
    std::cout << std::endl;
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Lambda Expressions",
                        color: (0x00, 0x80, 0xCC),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
            
            // Categoría: Move Semantics
            CppTemplateCategory {
                name: "Move Semantics",
                description: "Optimización de transferencia de recursos",
                icon: "🚀",
                color: (0x00, 0x90, 0xDD),
                templates: vec![
                    Template {
                        name: "🔷 Move Semantics Demo",
                        code: r#"// Move Semantics - C++11
#include <iostream>
#include <vector>
#include <string>

class DataProcessor {
private:
    std::string name;
    std::vector<int> data;
    
public:
    // Constructor
    DataProcessor(const std::string& processor_name, std::vector<int> input_data)
        : name(processor_name), data(std::move(input_data)) {
        std::cout << "📦 DataProcessor '" << name << "' creado con " 
                  << data.size() << " elementos" << std::endl;
    }
    
    // Copy Constructor
    DataProcessor(const DataProcessor& other) 
        : name(other.name + " (copiado)"), data(other.data) {
        std::cout << "📋 DataProcessor '" << name << "' copiado" << std::endl;
    }
    
    // Move Constructor
    DataProcessor(DataProcessor&& other) noexcept
        : name(std::move(other.name)), data(std::move(other.data)) {
        std::cout << "🚀 DataProcessor '" << name << "' movido" << std::endl;
    }
    
    // Copy Assignment
    DataProcessor& operator=(const DataProcessor& other) {
        if (this != &other) {
            name = other.name + " (asignado)";
            data = other.data;
            std::cout << "📋 DataProcessor copiado por asignación" << std::endl;
        }
        return *this;
    }
    
    // Move Assignment
    DataProcessor& operator=(DataProcessor&& other) noexcept {
        if (this != &other) {
            name = std::move(other.name);
            data = std::move(other.data);
            std::cout << "🚀 DataProcessor movido por asignación" << std::endl;
        }
        return *this;
    }
    
    void process() {
        std::cout << "🔧 Procesando '" << name << "': ";
        for (const auto& item : data) {
            std::cout << item << " ";
        }
        std::cout << std::endl;
    }
    
    size_t size() const {
        return data.size();
    }
};

int main() {
    std::cout << "🔷 Move Semantics Demo - C++11" << std::endl;
    
    // Crear datos iniciales
    std::vector<int> initial_data = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    
    std::cout << "\n1️⃣ Creación con move semantics:" << std::endl;
    DataProcessor processor1("Processor1", std::move(initial_data));
    
    std::cout << "\n2️⃣ Copy semantics:" << std::endl;
    DataProcessor processor2 = processor1;  // Copy constructor
    processor2.process();
    
    std::cout << "\n3️⃣ Move semantics:" << std::endl;
    DataProcessor processor3 = std::move(processor1);  // Move constructor
    processor3.process();
    
    std::cout << "\n4️⃣ Move assignment:" << std::endl;
    DataProcessor processor4("Temp", {});
    processor4 = std::move(processor3);  // Move assignment
    processor4.process();
    
    std::cout << "\n✅ Demostración completada" << std::endl;
    
    return 0;
}"#,
                        category: "C++11",
                        subcategory: "Move Semantics",
                        color: (0x00, 0x90, 0xDD),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
        ]
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // C++14 CATEGORIES - Mejoras Intermedias
    // ═══════════════════════════════════════════════════════════════════════════════
    fn get_cpp14_categories() -> Vec<CppTemplateCategory> {
        vec![
            // Categoría: Generic Lambdas
            CppTemplateCategory {
                name: "Generic Lambdas",
                description: "Lambdas con parámetros auto",
                icon: "🎭",
                color: (0x00, 0x80, 0x80),
                templates: vec![
                    Template {
                        name: "🔶 Generic Lambda Demo",
                        code: r#"// Generic Lambdas - C++14
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>

int main() {
    std::cout << "🔶 Generic Lambdas Demo - C++14" << std::endl;
    
    // Datos de diferentes tipos
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::vector<double> doubles = {1.1, 2.2, 3.3, 4.4, 5.5};
    std::vector<std::string> strings = {"hello", "world", "c++14"};
    
    // Generic lambda para imprimir cualquier tipo
    auto print_any = [](const auto& item) {
        std::cout << item << " ";
    };
    
    std::cout << "Numbers: ";
    std::for_each(numbers.begin(), numbers.end(), print_any);
    std::cout << std::endl;
    
    std::cout << "Doubles: ";
    std::for_each(doubles.begin(), doubles.end(), print_any);
    std::cout << std::endl;
    
    std::cout << "Strings: ";
    std::for_each(strings.begin(), strings.end(), print_any);
    std::cout << std::endl;
    
    // Generic lambda para transformar
    auto transform_any = [](const auto& item) {
        if constexpr (std::is_integral_v<decltype(item)>) {
            return item * 2;
        } else if constexpr (std::is_floating_point_v<decltype(item)>) {
            return item * 2.0;
        } else {
            return item + std::string("_transformed");
        }
    };
    
    std::cout << "\nTransformed:" << std::endl;
    std::cout << "Numbers: ";
    std::for_each(numbers.begin(), numbers.end(), 
        [&print_any, transform_any](const auto& item) {
            print_any(transform_any(item));
        });
    std::cout << std::endl;
    
    return 0;
}"#,
                        category: "C++14",
                        subcategory: "Generic Lambdas",
                        color: (0x00, 0x80, 0x80),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
        ]
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // C++17 CATEGORIES - Características Modernas
    // ═══════════════════════════════════════════════════════════════════════════════
    fn get_cpp17_categories() -> Vec<CppTemplateCategory> {
        vec![
            // Categoría: Structured Bindings
            CppTemplateCategory {
                name: "Structured Bindings",
                description: "Desestructuración elegante",
                icon: "🎪",
                color: (0x00, 0x40, 0x80),
                templates: vec![
                    Template {
                        name: "🔹 Structured Bindings Demo",
                        code: r#"// Structured Bindings - C++17
#include <iostream>
#include <map>
#include <tuple>
#include <string>
#include <vector>

std::tuple<std::string, int, double> get_student_info() {
    return {"Alice", 25, 95.5};
}

std::map<int, std::string> get_database() {
    return {
        {1, "Alice"},
        {2, "Bob"},
        {3, "Charlie"},
        {4, "Diana"}
    };
}

int main() {
    std::cout << "🔹 Structured Bindings Demo - C++17" << std::endl;
    
    // Structured binding con tuple
    auto [name, age, score] = get_student_info();
    std::cout << "Student: " << name << ", Age: " << age << ", Score: " << score << std::endl;
    
    // Structured binding con map
    auto database = get_database();
    std::cout << "\nDatabase entries:" << std::endl;
    
    for (const auto& [id, student_name] : database) {
        std::cout << "  ID: " << id << ", Name: " << student_name << std::endl;
    }
    
    // Structured binding con pair
    std::vector<std::pair<std::string, int>> employees = {
        {"John", 50000},
        {"Jane", 60000},
        {"Bob", 55000}
    };
    
    std::cout << "\nEmployee salaries:" << std::endl;
    for (const auto& [employee_name, salary] : employees) {
        std::cout << "  " << employee_name << ": $" << salary << std::endl;
    }
    
    // Structured binding con array
    int coordinates[3] = {10, 20, 30};
    auto [x, y, z] = coordinates;
    std::cout << "\nCoordinates: x=" << x << ", y=" << y << ", z=" << z << std::endl;
    
    return 0;
}"#,
                        category: "C++17",
                        subcategory: "Structured Bindings",
                        color: (0x00, 0x40, 0x80),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
            
            // Categoría: Optional y Variant
            CppTemplateCategory {
                name: "Optional & Variant",
                description: "Tipos sum y manejo de valores nulos",
                icon: "🎲",
                color: (0x00, 0x50, 0x90),
                templates: vec![
                    Template {
                        name: "🔹 Optional & Variant Demo",
                        code: r#"// Optional & Variant - C++17
#include <iostream>
#include <optional>
#include <variant>
#include <string>
#include <vector>

// Función que puede o no retornar un valor
std::optional<int> divide(int a, int b) {
    if (b == 0) {
        return std::nullopt;  // No hay resultado
    }
    return a / b;  // Hay resultado
}

// Variant para diferentes tipos de datos
using DataValue = std::variant<int, double, std::string>;

void process_data(const DataValue& data) {
    std::visit([](const auto& value) {
        std::cout << "Processed: " << value << " (type: " << typeid(value).name() << ")" << std::endl;
    }, data);
}

int main() {
    std::cout << "🔹 Optional & Variant Demo - C++17" << std::endl;
    
    // Demostración de optional
    std::cout << "\n📋 Optional Demo:" << std::endl;
    
    auto result1 = divide(10, 2);
    auto result2 = divide(10, 0);
    
    if (result1.has_value()) {
        std::cout << "10 / 2 = " << result1.value() << std::endl;
    } else {
        std::cout << "10 / 0: División por cero" << std::endl;
    }
    
    if (result2) {  // Conversión implícita a bool
        std::cout << "10 / 0 = " << *result2 << std::endl;
    } else {
        std::cout << "10 / 0: División por cero" << std::endl;
    }
    
    // Valor por defecto con optional
    auto safe_result = result2.value_or(-1);
    std::cout << "Resultado seguro: " << safe_result << std::endl;
    
    // Demostración de variant
    std::cout << "\n🎲 Variant Demo:" << std::endl;
    
    std::vector<DataValue> data_values = {
        42,
        3.14159,
        "Hello C++17"
    };
    
    for (const auto& data : data_values) {
        process_data(data);
    }
    
    // Acceder a variant con get_if
    std::variant<int, double, std::string> value = 42;
    
    if (auto int_ptr = std::get_if<int>(&value)) {
        std::cout << "Valor entero: " << *int_ptr << std::endl;
    }
    
    // Cambiar el tipo de variant
    value = 3.14159;
    if (auto double_ptr = std::get_if<double>(&value)) {
        std::cout << "Valor double: " << *double_ptr << std::endl;
    }
    
    return 0;
}"#,
                        category: "C++17",
                        subcategory: "Optional & Variant",
                        color: (0x00, 0x50, 0x90),
                        icon: "🎯",
                        language: NodeLanguage::Cpp,
                    },
                ],
            },
        ]
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // MÉTODOS PÚBLICOS
    // ═══════════════════════════════════════════════════════════════════════════════
    pub fn get_all_templates(&self) -> Vec<Template> {
        let mut all_templates = Vec::new();
        
        // Agregar templates de C++11
        for category in &self.cpp11_categories {
            all_templates.extend(category.templates.clone());
        }
        
        // Agregar templates de C++14
        for category in &self.cpp14_categories {
            all_templates.extend(category.templates.clone());
        }
        
        // Agregar templates de C++17
        for category in &self.cpp17_categories {
            all_templates.extend(category.templates.clone());
        }
        
        all_templates
    }
    
    pub fn get_cpp11_templates(&self) -> Vec<Template> {
        let mut templates = Vec::new();
        for category in &self.cpp11_categories {
            templates.extend(category.templates.clone());
        }
        templates
    }
    
    pub fn get_cpp14_templates(&self) -> Vec<Template> {
        let mut templates = Vec::new();
        for category in &self.cpp14_categories {
            templates.extend(category.templates.clone());
        }
        templates
    }
    
    pub fn get_cpp17_templates(&self) -> Vec<Template> {
        let mut templates = Vec::new();
        for category in &self.cpp17_categories {
            templates.extend(category.templates.clone());
        }
        templates
    }
    
    pub fn get_category_info(&self, version: &str, category_name: &str) -> Option<&CppTemplateCategory> {
        match version {
            "cpp11" => self.cpp11_categories.iter().find(|cat| cat.name == category_name),
            "cpp14" => self.cpp14_categories.iter().find(|cat| cat.name == category_name),
            "cpp17" => self.cpp17_categories.iter().find(|cat| cat.name == category_name),
            _ => None,
        }
    }
}
