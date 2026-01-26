// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA: Sistema de Organización de Lenguajes
// Facilita el acceso rápido a los lenguajes de programación importantes
// ═══════════════════════════════════════════════════════════════════════════════

use crate::core::node_graph::NodeLanguage;
use crate::ui::theme::THEME;
use eframe::egui::{Color32, RichText};

/// Categoría de lenguaje para organización
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageCategory {
    /// Lenguajes de sistemas y bajo nivel
    Systems,
    /// Lenguajes de alto nivel y scripting
    HighLevel,
    /// Lenguajes web y frontend
    Web,
    /// Lenguajes de datos y análisis
    Data,
    /// Lenguajes especializados y dominio específico
    Specialized,
    /// Lenguajes legacy y educativos
    Legacy,
}

/// Información completa de un lenguaje
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub language: NodeLanguage,
    pub category: LanguageCategory,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub color: Color32,
    pub priority: u8, // 1 = más importante, 10 = menos importante
    pub features: Vec<&'static str>,
    pub use_cases: Vec<&'static str>,
    pub quick_access: bool, // Si debe aparecer en acceso rápido
}

/// Sistema de organización de lenguajes
pub struct LanguageManager {
    pub languages: Vec<LanguageInfo>,
    pub favorites: Vec<NodeLanguage>,
    pub recently_used: Vec<NodeLanguage>,
}

impl LanguageManager {
    pub fn new() -> Self {
        let languages = vec![
            // ═══════════════════════════════════════════════════════════════════════════════
            // LENGUAJES DE SISTEMAS (Prioridad Alta)
            // ═══════════════════════════════════════════════════════════════════════════════
            LanguageInfo {
                language: NodeLanguage::Rust,
                category: LanguageCategory::Systems,
                name: "Rust",
                description: "Lenguaje de sistemas seguro y rápido",
                icon: "🦀",
                color: THEME.node_rust,
                priority: 1,
                features: vec!["Memory Safety", "Zero Cost Abstractions", "Concurrency", "WebAssembly"],
                use_cases: vec!["Sistemas Operativos", "WebAssembly", "Embedded", "CLI Tools"],
                quick_access: true,
            },
            LanguageInfo {
                language: NodeLanguage::Cpp,
                category: LanguageCategory::Systems,
                name: "C++",
                description: "Lenguaje de alto rendimiento con control de memoria",
                icon: "🔷",
                color: THEME.node_cpp,
                priority: 2,
                features: vec!["OOP", "Templates", "RAII", "STL", "Modern C++"],
                use_cases: vec!["Sistemas", "Juegos", "HPC", "Embedded", "Desktop Apps"],
                quick_access: true,
            },
            LanguageInfo {
                language: NodeLanguage::Asm,
                category: LanguageCategory::Systems,
                name: "Assembly",
                description: "Lenguaje ensamblador de bajo nivel",
                icon: "⚡",
                color: THEME.node_asm,
                priority: 3,
                features: vec!["Control Total", "Optimización Extrema", "Hardware Direct"],
                use_cases: vec!["Bootloaders", "Drivers", "Optimization", "Reverse Engineering"],
                quick_access: false,
            },

            // ═══════════════════════════════════════════════════════════════════════════════
            // LENGUAJES DE ALTO NIVEL (Prioridad Alta)
            // ═══════════════════════════════════════════════════════════════════════════════
            LanguageInfo {
                language: NodeLanguage::Python,
                category: LanguageCategory::HighLevel,
                name: "Python",
                description: "Lenguaje versátil y fácil de aprender",
                icon: "🐍",
                color: THEME.node_python,
                priority: 4,
                features: vec!["Sintaxis Limpia", "Librerías Extensas", "Data Science", "AI/ML"],
                use_cases: vec!["Data Science", "Web Backend", "Automation", "Prototyping"],
                quick_access: true,
            },
            LanguageInfo {
                language: NodeLanguage::Java,
                category: LanguageCategory::HighLevel,
                name: "Java",
                description: "Lenguaje orientado a objetos multiplataforma",
                icon: "☕",
                color: THEME.node_java,
                priority: 5,
                features: vec!["JVM", "OOP", "Garbage Collection", "Enterprise"],
                use_cases: vec!["Enterprise Apps", "Android", "Web Backend", "Big Data"],
                quick_access: true,
            },

            // ═══════════════════════════════════════════════════════════════════════════════
            // LENGUAJES DE SOPORTE
            // ═══════════════════════════════════════════════════════════════════════════════
            LanguageInfo {
                language: NodeLanguage::Text,
                category: LanguageCategory::Legacy,
                name: "Texto/Documentación",
                description: "Documentación y texto sin compilación",
                icon: "📄",
                color: THEME.node_text,
                priority: 10,
                features: vec!["Markdown", "Documentación", "Notas"],
                use_cases: vec!["Documentación", "README", "Comentarios"],
                quick_access: false,
            },
        ];

        Self {
            languages,
            favorites: vec![NodeLanguage::Rust, NodeLanguage::Python, NodeLanguage::Cpp],
            recently_used: Vec::new(),
        }
    }

    /// Obtener lenguajes por categoría
    pub fn get_by_category(&self, category: LanguageCategory) -> Vec<&LanguageInfo> {
        self.languages
            .iter()
            .filter(|lang| lang.category == category)
            .collect()
    }

    /// Obtener lenguajes de acceso rápido
    pub fn get_quick_access(&self) -> Vec<&LanguageInfo> {
        self.languages
            .iter()
            .filter(|lang| lang.quick_access)
            .collect()
    }

    /// Obtener lenguajes favoritos
    pub fn get_favorites(&self) -> Vec<&LanguageInfo> {
        self.favorites
            .iter()
            .filter_map(|fav| self.languages.iter().find(|lang| lang.language == *fav))
            .collect()
    }

    /// Obtener lenguajes usados recientemente
    pub fn get_recently_used(&self) -> Vec<&LanguageInfo> {
        self.recently_used
            .iter()
            .filter_map(|recent| self.languages.iter().find(|lang| lang.language == *recent))
            .collect()
    }

    /// Agregar a lenguaje a usados recientemente
    pub fn add_to_recently_used(&mut self, language: NodeLanguage) {
        // Eliminar si ya existe
        self.recently_used.retain(|&lang| lang != language);
        
        // Agregar al principio
        self.recently_used.insert(0, language);
        
        // Mantener solo los últimos 5
        self.recently_used.truncate(5);
    }

    /// Buscar lenguajes por texto
    pub fn search(&self, query: &str) -> Vec<&LanguageInfo> {
        let query_lower = query.to_lowercase();
        self.languages
            .iter()
            .filter(|lang| {
                lang.name.to_lowercase().contains(&query_lower) ||
                lang.description.to_lowercase().contains(&query_lower) ||
                lang.features.iter().any(|&f| f.to_lowercase().contains(&query_lower)) ||
                lang.use_cases.iter().any(|&u| u.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Obtener información de un lenguaje específico
    pub fn get_info(&self, language: NodeLanguage) -> Option<&LanguageInfo> {
        self.languages.iter().find(|lang| lang.language == language)
    }

    /// Formatear texto con estilo de lenguaje
    pub fn format_language_text(&self, language: NodeLanguage, text: &str) -> RichText {
        if let Some(info) = self.get_info(language) {
            RichText::new(text)
                .color(info.color)
                .size(14.0)
        } else {
            RichText::new(text)
                .color(THEME.text_secondary)
                .size(14.0)
        }
    }

    /// Obtener icono de lenguaje
    pub fn get_language_icon(&self, language: NodeLanguage) -> &'static str {
        self.get_info(language)
            .map(|info| info.icon)
            .unwrap_or("❓")
    }

    /// Obtener color de lenguaje
    pub fn get_language_color(&self, language: NodeLanguage) -> Color32 {
        self.get_info(language)
            .map(|info| info.color)
            .unwrap_or(THEME.text_muted)
    }
}

/// Instancia global del gestor de lenguajes
pub static LANGUAGE_MANAGER: std::sync::LazyLock<LanguageManager> = std::sync::LazyLock::new(LanguageManager::new);

/// Funciones de utilidad para el gestor de lenguajes
pub mod utils {
    use super::*;

    /// Obtener texto formateado para categoría
    pub fn format_category_text(category: LanguageCategory) -> &'static str {
        match category {
            LanguageCategory::Systems => "🔧 Sistemas",
            LanguageCategory::HighLevel => "🚀 Alto Nivel",
            LanguageCategory::Web => "🌐 Web",
            LanguageCategory::Data => "📊 Datos",
            LanguageCategory::Specialized => "⚙️ Especializados",
            LanguageCategory::Legacy => "📚 Legacy",
        }
    }

    /// Obtener descripción de categoría
    pub fn get_category_description(category: LanguageCategory) -> &'static str {
        match category {
            LanguageCategory::Systems => "Lenguajes de bajo nivel y sistemas",
            LanguageCategory::HighLevel => "Lenguajes de alto nivel y scripting",
            LanguageCategory::Web => "Lenguajes para desarrollo web",
            LanguageCategory::Data => "Lenguajes para análisis de datos",
            LanguageCategory::Specialized => "Lenguajes para dominios específicos",
            LanguageCategory::Legacy => "Lenguajes legacy y educativos",
        }
    }
}
