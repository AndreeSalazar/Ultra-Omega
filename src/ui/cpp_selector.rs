// ═══════════════════════════════════════════════════════════════════════════════
// Ultra-Omega: C++ Version Selector UI (Simplified for Integration)
// Sistema de selección de versiones C++ fácil de integrar en el editor
// ═══════════════════════════════════════════════════════════════════════════════

use eframe::egui;
use crate::templates::Template;
use crate::core::node_graph::NodeLanguage;

// Estructura Template alternativa para owned data
#[derive(Clone)]
pub struct CppNodeTemplate {
    pub name: String,
    pub code: String,
    pub category: String,
    pub subcategory: String,
    pub color: (u8, u8, u8),
    pub icon: String,
    pub language: NodeLanguage,
}

impl From<CppNodeTemplate> for Template {
    fn from(_template: CppNodeTemplate) -> Self {
        // Esta conversión se usará solo cuando sea necesario
        // Por ahora, vamos a usar CppNodeTemplate directamente
        Template {
            name: "C++ Template",
            code: crate::templates::cpp::HELLO_WORLD,
            category: "C++",
            subcategory: "Basic",
            color: (0, 100, 200),
            icon: "🔷",
            language: NodeLanguage::Cpp,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CppVersion {
    Cpp11,
    Cpp14,
    Cpp17,
    Auto,
}

impl CppVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            CppVersion::Cpp11 => "c++11",
            CppVersion::Cpp14 => "c++14",
            CppVersion::Cpp17 => "c++17",
            CppVersion::Auto => "auto",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CppVersion::Cpp11 => "C++11 (2011)",
            CppVersion::Cpp14 => "C++14 (2014)",
            CppVersion::Cpp17 => "C++17 (2017)",
            CppVersion::Auto => "🔧 Auto",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CppVersion::Cpp11 => "Fundamentos modernos: auto, lambda, smart pointers",
            CppVersion::Cpp14 => "Mejoras intermedias: generic lambdas, make_unique",
            CppVersion::Cpp17 => "Características modernas: structured bindings, optional",
            CppVersion::Auto => "Detectar automáticamente la mejor versión disponible",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            CppVersion::Cpp11 => egui::Color32::from_rgb(0x00, 0x60, 0xAA),
            CppVersion::Cpp14 => egui::Color32::from_rgb(0x00, 0x80, 0x80),
            CppVersion::Cpp17 => egui::Color32::from_rgb(0x00, 0x40, 0x80),
            CppVersion::Auto => egui::Color32::from_rgb(0x80, 0x80, 0x80),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            CppVersion::Cpp11 => "🔷",
            CppVersion::Cpp14 => "🔶",
            CppVersion::Cpp17 => "🔹",
            CppVersion::Auto => "🔧",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CppTemplate {
    pub name: String,
    pub code: String,
    pub version: CppVersion,
    pub category: String,
    pub subcategory: String,
    pub description: String,
    pub features: Vec<String>,
}

impl CppTemplate {
    pub fn new(
        name: &str,
        code: &str,
        version: CppVersion,
        category: &str,
        subcategory: &str,
        description: &str,
        features: Vec<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            version,
            category: category.to_string(),
            subcategory: subcategory.to_string(),
            description: description.to_string(),
            features: features.iter().map(|f| f.to_string()).collect(),
        }
    }

    pub fn to_node_template(&self) -> CppNodeTemplate {
        CppNodeTemplate {
            name: format!("{} {}", self.version.icon(), self.name),
            code: self.code.clone(),
            category: format!("C++ {}", self.version.as_str().to_uppercase()),
            subcategory: self.subcategory.clone(),
            color: (
                (self.version.color().r() as u8),
                (self.version.color().g() as u8),
                (self.version.color().b() as u8),
            ),
            icon: self.version.icon().to_string(),
            language: NodeLanguage::Cpp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CppTemplateManager {
    pub templates: Vec<CppTemplate>,
}

impl CppTemplateManager {
    pub fn new() -> Self {
        let mut templates = Vec::new();
        
        // C++11 Templates
        templates.push(CppTemplate::new(
            "Hello Modern",
            include_str!("../templates/cpp/cpp11/hello_simple.cpp"),
            CppVersion::Cpp11,
            "Fundamentos",
            "Básico",
            "Demostración de características fundamentales C++11",
            vec!["auto", "lambda", "smart pointers", "range-based for", "move semantics"]
        ));

        templates.push(CppTemplate::new(
            "Auto Type Deduction",
            r#"// Auto Type Deduction - C++11
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
            CppVersion::Cpp11,
            "Fundamentos",
            "Tipado",
            "Demostración del keyword auto para deducción de tipos",
            vec!["auto", "type deduction", "containers"]
        ));

        // C++14 Templates
        templates.push(CppTemplate::new(
            "Generic Lambdas",
            r#"// Generic Lambdas - C++14
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::cout << "🔶 Generic Lambdas - C++14" << std::endl;
    
    // Datos de diferentes tipos
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    std::vector<double> doubles = {1.1, 2.2, 3.3, 4.4, 5.5};
    
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
    
    return 0;
}"#,
            CppVersion::Cpp14,
            "Generic Lambdas",
            "Funcionales",
            "Lambdas con parámetros auto para mayor flexibilidad",
            vec!["generic lambda", "auto parameters", "type deduction"]
        ));

        // C++17 Templates
        templates.push(CppTemplate::new(
            "Structured Bindings",
            r#"// Structured Bindings - C++17
#include <iostream>
#include <map>
#include <tuple>
#include <string>

std::tuple<std::string, int, double> get_student_info() {
    return {"Alice", 25, 95.5};
}

int main() {
    std::cout << "🔹 Structured Bindings - C++17" << std::endl;
    
    // Structured binding con tuple
    auto [name, age, score] = get_student_info();
    std::cout << "Student: " << name << ", Age: " << age << ", Score: " << score << std::endl;
    
    // Structured binding con map
    std::map<int, std::string> database = {
        {1, "Alice"},
        {2, "Bob"},
        {3, "Charlie"}
    };
    
    std::cout << "\nDatabase entries:" << std::endl;
    for (const auto& [id, student_name] : database) {
        std::cout << "  ID: " << id << ", Name: " << student_name << std::endl;
    }
    
    return 0;
}"#,
            CppVersion::Cpp17,
            "Structured Bindings",
            "Desestructuración",
            "Desestructuración elegante de tuplas y estructuras",
            vec!["structured bindings", "tuple unpacking", "map iteration"]
        ));

        Self { templates }
    }

    pub fn get_templates_by_version(&self, version: &CppVersion) -> Vec<&CppTemplate> {
        self.templates.iter().filter(|t| &t.version == version).collect()
    }

    pub fn get_templates_by_category(&self, category: &str) -> Vec<&CppTemplate> {
        self.templates.iter().filter(|t| t.category == category).collect()
    }

    pub fn get_all_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.templates
            .iter()
            .map(|t| t.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    pub fn get_template_by_name(&self, name: &str) -> Option<&CppTemplate> {
        self.templates.iter().find(|t| t.name == name)
    }

    pub fn search_templates(&self, query: &str) -> Vec<&CppTemplate> {
        let query_lower = query.to_lowercase();
        self.templates
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower) ||
                t.category.to_lowercase().contains(&query_lower) ||
                t.description.to_lowercase().contains(&query_lower) ||
                t.features.iter().any(|f| f.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct CppVersionSelector {
    pub selected_version: CppVersion,
    pub selected_template: Option<CppTemplate>,
    pub search_query: String,
    pub show_all_versions: bool,
    template_manager: CppTemplateManager,
}

impl Default for CppVersionSelector {
    fn default() -> Self {
        Self {
            selected_version: CppVersion::Auto,
            selected_template: None,
            search_query: String::new(),
            show_all_versions: false,
            template_manager: CppTemplateManager::new(),
        }
    }
}

impl CppVersionSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_ui(&mut self, ui: &mut egui::Ui) -> Option<CppNodeTemplate> {
        let mut result = None;

        // Header
        ui.vertical_centered(|ui| {
            ui.heading("🔷 C++ Template Selector");
            ui.label("Selecciona versión y template según estándar C++");
        });
        
        ui.add_space(10.0);

        // Version selection
        self.render_version_selection(ui);
        
        ui.add_space(15.0);

        // Search bar
        self.render_search_bar(ui);
        
        ui.add_space(15.0);

        // Template list
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                result = self.render_template_list(ui);
            });

        result
    }

    fn render_version_selection(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("📋 Versión C++");
            
            ui.horizontal(|ui| {
                let mut new_version = self.selected_version.clone();
                
                ui.radio_value(&mut new_version, CppVersion::Cpp11, CppVersion::Cpp11.display_name())
                    .on_hover_text(CppVersion::Cpp11.description());
                
                ui.radio_value(&mut new_version, CppVersion::Cpp14, CppVersion::Cpp14.display_name())
                    .on_hover_text(CppVersion::Cpp14.description());
                
                ui.radio_value(&mut new_version, CppVersion::Cpp17, CppVersion::Cpp17.display_name())
                    .on_hover_text(CppVersion::Cpp17.description());
                
                ui.radio_value(&mut new_version, CppVersion::Auto, CppVersion::Auto.display_name())
                    .on_hover_text(CppVersion::Auto.description());
                
                if new_version != self.selected_version {
                    self.selected_version = new_version;
                    self.selected_template = None;
                }
            });
            
            // Version info
            ui.add_space(5.0);
            self.render_version_info(ui);
        });
    }

    fn render_version_info(&self, ui: &mut egui::Ui) {
        let version = &self.selected_version;
        let color = version.color();
        
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} {}", version.icon(), version.display_name()))
                .color(color).size(16.0));
            ui.label(egui::RichText::new(version.description())
                .color(egui::Color32::GRAY).size(12.0));
        });
        
        // Features
        let features = match version {
            CppVersion::Cpp11 => vec!["✅ auto", "✅ lambda", "✅ smart pointers", "✅ range-based for"],
            CppVersion::Cpp14 => vec!["✅ generic lambdas", "✅ auto return", "✅ make_unique", "✅ relaxed constexpr"],
            CppVersion::Cpp17 => vec!["✅ structured bindings", "✅ optional", "✅ variant", "✅ string_view"],
            CppVersion::Auto => vec!["🔍 Auto-detección", "🔍 Mejor versión disponible", "🔍 Compatible con tu compilador"],
        };
        
        ui.horizontal_wrapped(|ui| {
            for feature in features {
                ui.label(egui::RichText::new(feature).size(11.0).color(egui::Color32::GRAY));
            }
        });
    }

    fn render_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍 Buscar:");
            let response = ui.text_edit_singleline(&mut self.search_query);
            
            if response.changed() {
                self.selected_template = None;
            }
            
            if !self.search_query.is_empty() {
                if ui.button("🗑️").clicked() {
                    self.search_query.clear();
                    self.selected_template = None;
                }
            }
        });
    }

    fn render_template_list(&mut self, ui: &mut egui::Ui) -> Option<CppNodeTemplate> {
        let mut selected = None;
        
        let templates_to_show = if self.search_query.is_empty() {
            if self.show_all_versions {
                self.template_manager.templates.iter().collect()
            } else {
                self.template_manager.get_templates_by_version(&self.selected_version)
            }
        } else {
            self.template_manager.search_templates(&self.search_query)
        };

        // Group by category
        let mut categories: std::collections::HashMap<String, Vec<&CppTemplate>> = std::collections::HashMap::new();
        for template in templates_to_show {
            categories.entry(template.category.clone()).or_insert_with(Vec::new).push(template);
        }

        for (category_name, category_templates) in categories {
            ui.collapsing(format!("📁 {} ({} templates)", category_name, category_templates.len()), |ui| {
                for template in category_templates {
                    let is_selected = self.selected_template.as_ref().map(|t| &t.name) == Some(&template.name);
                    let version_color = template.version.color();
                    
                    let template_response = ui.horizontal(|ui| {
                        // Version icon
                        ui.label(egui::RichText::new(template.version.icon())
                            .color(version_color).size(14.0));
                        
                        // Template name
                        let name_text = if is_selected {
                            egui::RichText::new(&template.name).strong().color(version_color)
                        } else {
                            egui::RichText::new(&template.name)
                        };
                        ui.label(name_text);
                        
                        // Features badges
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            for feature in template.features.iter().take(3) {
                                ui.label(egui::RichText::new(feature)
                                    .size(9.0).color(egui::Color32::GRAY));
                            }
                        });
                    }).response;

                    if template_response.clicked() {
                        self.selected_template = Some(template.clone());
                        selected = Some(template.clone());
                    }

                    // Selection indicator
                    if is_selected {
                        ui.painter().rect_filled(
                            template_response.rect,
                            2.0,
                            version_color.linear_multiply(0.1),
                        );
                        ui.painter().rect_stroke(
                            template_response.rect,
                            2.0,
                            egui::Stroke::new(1.0, version_color),
                        );
                    }

                    // Tooltip
                    template_response.on_hover_text(format!("{}\n{}\nCaracterísticas: {}", 
                        template.description, 
                        template.subcategory,
                        template.features.join(", ")));
                }
            });
        }

        selected.map(|t| t.to_node_template())
    }

    pub fn get_selected_template(&self) -> Option<&CppTemplate> {
        self.selected_template.as_ref()
    }

    pub fn clear_selection(&mut self) {
        self.selected_template = None;
        self.search_query.clear();
    }
}

// Función de utilidad para mostrar el selector en un modal
pub fn show_cpp_template_selector(ctx: &egui::Context) -> Option<CppNodeTemplate> {
    let mut selector = CppVersionSelector::new();
    let mut result = None;
    let mut open = true;

    egui::Window::new("🔷 Seleccionar Template C++")
        .collapsible(false)
        .resizable(true)
        .default_width(700.0)
        .default_height(500.0)
        .open(&mut open)
        .show(ctx, |ui| {
            result = selector.render_ui(ui);
        });

    if !open {
        None
    } else {
        result
    }
}
