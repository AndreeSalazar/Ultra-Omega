use eframe::egui::{Color32, Pos2, pos2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PinId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PinKind {
    Input,
    Output,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeLanguage {
    Auto,
    Asm,
    C,
    Cpp,
    Rust,
    /// Texto/Documentación - NO se compila, solo visualización
    Text,
    /// Mojo - Lenguaje de alto rendimiento para IA/ML
    Mojo,
    /// MojoAI - Nodos especializados con capacidades de IA
    MojoAI,
}

impl Default for NodeLanguage {
    fn default() -> Self {
        NodeLanguage::Auto
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pin {
    pub id: PinId,
    pub label: String,
    #[allow(dead_code)]
    pub kind: PinKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub title: String,
    #[serde(with = "pos2_serde")]
    pub position: Pos2,
    #[serde(with = "color32_serde")]
    pub color: Color32,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub code: String,
    #[serde(default)]
    pub language: NodeLanguage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub from: PinId,
    pub to: PinId,
    #[serde(with = "color32_serde")]
    pub color: Color32,
}

// Serialization helpers
mod pos2_serde {
    use eframe::egui::Pos2;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(pos: &Pos2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (pos.x, pos.y).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pos2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Pos2::new(x, y))
    }
}

mod color32_serde {
    use eframe::egui::Color32;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (color.r(), color.g(), color.b(), color.a()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (r, g, b, a) = <(u8, u8, u8, u8)>::deserialize(deserializer)?;
        Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct NodeGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
    #[serde(skip)]
    next_node_id: u64,
    #[serde(skip)]
    next_pin_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PinAddress {
    pub node_index: usize,
    pub slot: usize,
    pub kind: PinKind,
}

impl NodeGraph {
    pub fn demo() -> Self {
        let mut graph = Self::default();

        // Node 1: Base ASM (NASM)
        let asm_node_id = graph.add_node(
            "Base ASM (NASM)",
            pos2(100.0, 100.0),
            Color32::from_rgb(0xff, 0x47, 0x00), // Orange-Red for low level/danger
            &[],
            &["Código Fuente"],
            NodeLanguage::Asm,
        );
        if let Some(node) = graph.node_mut(asm_node_id) {
            node.code = "default rel\nsection .text\nglobal main\nextern printf\nextern exit\n\nmain:\n    sub rsp, 40\n    mov rcx, msg\n    xor eax, eax\n    call printf\n    add rsp, 40\n    ret\n\nsection .data\n    msg db 'Hola ASM desde Ultra Omega!', 10, 0".to_string();
        }

        // Node 2: Base C
        let c_node_id = graph.add_node(
            "Base C",
            pos2(100.0, 280.0),
            Color32::from_rgb(0x00, 0x59, 0x9C), // C++ Blue-ish
            &[],
            &["Código C"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(c_node_id) {
            node.code = "#include <stdio.h>\n\nint main() {\n    printf(\"Hola desde C en Ultra Omega!\\n\");\n    return 0;\n}".to_string();
        }

        graph
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }

    pub fn remove_link(&mut self, from: PinId, to: PinId) -> bool {
        let initial_len = self.links.len();
        self.links.retain(|link| !(link.from == from && link.to == to));
        initial_len > self.links.len()
    }

    // Recalculate ID counters after deserialization
    pub fn recalculate_ids(&mut self) {
        self.next_node_id = self.nodes.iter().map(|n| n.id.0).max().unwrap_or(0) + 1;
        self.next_pin_id = self.nodes.iter()
            .flat_map(|n| n.inputs.iter().chain(n.outputs.iter()))
            .map(|p| p.id.0)
            .max()
            .unwrap_or(0) + 1;
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id == id)
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == id)
    }

    #[allow(dead_code)] // Available for single node deletion if needed
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        let initial_len = self.nodes.len();
        self.nodes.retain(|node| node.id != id);
        
        // Remove links connected to this node
        if initial_len != self.nodes.len() {
            let node_pin_ids: std::collections::HashSet<PinId> = self.nodes.iter()
                .flat_map(|n| n.inputs.iter().chain(n.outputs.iter()))
                .map(|p| p.id)
                .collect();
            
            self.links.retain(|link| {
                node_pin_ids.contains(&link.from) && node_pin_ids.contains(&link.to)
            });
            
            true
        } else {
            false
        }
    }

    pub fn remove_nodes(&mut self, ids: &std::collections::HashSet<NodeId>) {
        let initial_len = self.nodes.len();
        self.nodes.retain(|node| !ids.contains(&node.id));
        
        // Remove links connected to removed nodes
        if initial_len != self.nodes.len() {
            let node_pin_ids: std::collections::HashSet<PinId> = self.nodes.iter()
                .flat_map(|n| n.inputs.iter().chain(n.outputs.iter()))
                .map(|p| p.id)
                .collect();
            
            self.links.retain(|link| {
                node_pin_ids.contains(&link.from) && node_pin_ids.contains(&link.to)
            });
        }
    }

    pub fn add_node(
        &mut self,
        title: impl Into<String>,
        position: Pos2,
        color: Color32,
        inputs: &[&str],
        outputs: &[&str],
        language: NodeLanguage,
    ) -> NodeId {
        let id = self.alloc_node_id();
        let input_pins = inputs
            .iter()
            .map(|label| self.make_pin(label, PinKind::Input))
            .collect();
        let output_pins = outputs
            .iter()
            .map(|label| self.make_pin(label, PinKind::Output))
            .collect();

        self.nodes.push(Node {
            id,
            title: title.into(),
            position,
            color,
            inputs: input_pins,
            outputs: output_pins,
            code: String::new(),
            language,
        });

        id
    }

    pub fn add_link(&mut self, from: PinId, to: PinId, color: Color32) {
        // Evitar conexiones duplicadas
        if !self.links.iter().any(|l| l.from == from && l.to == to) {
            self.links.push(Link { from, to, color });
        }
    }

    // Obtener el nodo padre (del que hereda) para un nodo dado
    pub fn get_parent_node(&self, node_id: NodeId) -> Option<NodeId> {
        // Buscar un link que conecte a una entrada de este nodo
        for link in &self.links {
            if let Some(to_addr) = self.locate_pin(link.to) {
                if self.nodes[to_addr.node_index].id == node_id && to_addr.kind == PinKind::Input {
                    // Encontrar el nodo que tiene el pin de salida
                    if let Some(from_addr) = self.locate_pin(link.from) {
                        return Some(self.nodes[from_addr.node_index].id);
                    }
                }
            }
        }
        None
    }

    // Obtener todos los nodos que heredan de un nodo dado
    #[allow(dead_code)] // Available for future use
    pub fn get_children_nodes(&self, node_id: NodeId) -> Vec<NodeId> {
        let mut children = Vec::new();
        // Buscar el pin de salida del nodo
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            for output_pin in &node.outputs {
                // Buscar links que salen de este pin
                for link in &self.links {
                    if link.from == output_pin.id {
                        if let Some(to_addr) = self.locate_pin(link.to) {
                            children.push(self.nodes[to_addr.node_index].id);
                        }
                    }
                }
            }
        }
        children
    }

    // Obtener el código heredado (del nodo padre)
    pub fn get_inherited_code(&self, node_id: NodeId) -> Option<String> {
        if let Some(parent_id) = self.get_parent_node(node_id) {
            if let Some(parent) = self.nodes.iter().find(|n| n.id == parent_id) {
                return Some(parent.code.clone());
            }
        }
        None
    }

    /// Obtener la cadena completa de herencia: A → B → C → ...
    /// Devuelve un Vec de (NodeId, título, código) ordenado desde el más antiguo al más reciente
    pub fn get_inheritance_chain(&self, node_id: NodeId) -> Vec<(NodeId, String, String)> {
        let mut chain = Vec::new();
        let mut current_id = Some(node_id);
        
        // Recolectar todos los ancestros (subir por la cadena)
        while let Some(id) = current_id {
            if let Some(parent_id) = self.get_parent_node(id) {
                if let Some(parent) = self.nodes.iter().find(|n| n.id == parent_id) {
                    chain.push((parent_id, parent.title.clone(), parent.code.clone()));
                }
                current_id = Some(parent_id);
            } else {
                current_id = None;
            }
        }
        
        // Revertir para que esté en orden: ancestro más antiguo primero
        chain.reverse();
        chain
    }

    /// Obtener el código propio de un nodo (sin el código heredado)
    pub fn get_own_code(&self, node_id: NodeId) -> String {
        if let Some(node) = self.nodes.iter().find(|n| n.id == node_id) {
            // Obtener la cadena de herencia
            let chain = self.get_inheritance_chain(node_id);
            
            if chain.is_empty() {
                // No hay herencia, todo el código es propio
                return node.code.clone();
            }
            
            // El código del último ancestro directo
            if let Some((_parent_id, _title, parent_code)) = chain.last() {
                if node.code.starts_with(parent_code) {
                    // Remover el código heredado
                    return node.code[parent_code.len()..].trim_start_matches('\n').trim_start_matches('\r').to_string();
                }
            }
            
            node.code.clone()
        } else {
            String::new()
        }
    }

    pub fn locate_pin(&self, pin_id: PinId) -> Option<PinAddress> {
        self.nodes.iter().enumerate().find_map(|(idx, node)| {
            if let Some(slot) = node.inputs.iter().position(|pin| pin.id == pin_id) {
                return Some(PinAddress {
                    node_index: idx,
                    slot,
                    kind: PinKind::Input,
                });
            }
            if let Some(slot) = node.outputs.iter().position(|pin| pin.id == pin_id) {
                return Some(PinAddress {
                    node_index: idx,
                    slot,
                    kind: PinKind::Output,
                });
            }
            None
        })
    }

    #[allow(dead_code)] // Se usará cuando se implemente la conexión manual de nodos
    pub fn pin_id(&self, node_id: NodeId, kind: PinKind, slot: usize) -> Option<PinId> {
        self.nodes
            .iter()
            .find(|node| node.id == node_id)
            .and_then(|node| match kind {
                PinKind::Input => node.inputs.get(slot),
                PinKind::Output => node.outputs.get(slot),
            })
            .map(|pin| pin.id)
    }

    fn make_pin(&mut self, label: &str, kind: PinKind) -> Pin {
        Pin {
            id: self.alloc_pin_id(),
            label: label.to_owned(),
            kind,
        }
    }

    fn alloc_node_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    fn alloc_pin_id(&mut self) -> PinId {
        let id = PinId(self.next_pin_id);
        self.next_pin_id += 1;
        id
    }

    /// Crear proyecto FastOS - Diseño en DOS RAMAS que se combinan
    /// 
    /// ```text
    ///     IZQUIERDA (ASM)          │          DERECHA (C)
    ///     ═══════════════          │          ══════════════
    ///     💿 boot_sector           │          📋 types.h
    ///          ▼                   │              ▼
    ///     🚀 stage2                │          📋 ports.h
    ///          ▼                   │              ▼
    ///     ⚡ kernel_entry          │          ... más C ...
    ///          ▼                   │              ▼
    ///     🔧 ASM FINAL             │          🧠 C FINAL
    ///          └───────────┬───────┘
    ///                      ▼
    ///               🔗 COMBINADOR
    ///                      ▼
    ///               🔥 FASTOS
    /// ```
    pub fn create_fastos_project() -> Self {
        use crate::templates::fastos;
        
        let mut graph = Self::default();
        
        // Colores por categoría
        let color_asm = Color32::from_rgb(0xff, 0x66, 0x66);       // Rojo - ASM
        let color_c_header = Color32::from_rgb(0x66, 0xbb, 0xff);  // Azul - Headers
        let color_c_source = Color32::from_rgb(0x66, 0xdd, 0x66);  // Verde - Sources
        let color_combiner = Color32::from_rgb(0xff, 0x99, 0xff);  // Rosa - Combinador
        let color_build = Color32::from_rgb(0xdd, 0xdd, 0x66);     // Amarillo - Build
        let color_final = Color32::from_rgb(0xff, 0xcc, 0x00);     // Dorado - Final
        
        // Posiciones X para las dos columnas
        let x_left = 100.0;    // Columna izquierda (ASM)
        let x_right = 450.0;   // Columna derecha (C)
        let x_center = 275.0;  // Centro (combinador)
        let y_spacing = 90.0;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // COLUMNA IZQUIERDA: ASM (NASM) - Bootloader
        // ═══════════════════════════════════════════════════════════════════════════
        
        let mut y_asm = 50.0;
        
        let boot_sector = graph.add_node(
            "💿 boot_sector.asm",
            pos2(x_left, y_asm),
            color_asm,
            &[],
            &["▼"],
            NodeLanguage::Asm,
        );
        if let Some(n) = graph.node_mut(boot_sector) {
            n.code = fastos::BOOT_SECTOR.to_string();
        }
        y_asm += y_spacing;
        
        let stage2 = graph.add_node(
            "🚀 stage2.asm",
            pos2(x_left, y_asm),
            color_asm,
            &["▲"],
            &["▼"],
            NodeLanguage::Asm,
        );
        if let Some(n) = graph.node_mut(stage2) {
            n.code = fastos::STAGE2.to_string();
        }
        Self::link_nodes(&mut graph, boot_sector, stage2, color_asm);
        y_asm += y_spacing;
        
        let kernel_entry = graph.add_node(
            "⚡ kernel_entry.asm",
            pos2(x_left, y_asm),
            color_asm,
            &["▲"],
            &["▼"],
            NodeLanguage::Asm,
        );
        if let Some(n) = graph.node_mut(kernel_entry) {
            n.code = fastos::KERNEL_ENTRY.to_string();
        }
        Self::link_nodes(&mut graph, stage2, kernel_entry, color_asm);
        y_asm += y_spacing + 30.0;
        
        // Nodo final de ASM
        let asm_final = graph.add_node(
            "🔧 ASM COMPLETO",
            pos2(x_left, y_asm),
            Color32::from_rgb(0xcc, 0x44, 0x44),
            &["▲"],
            &["→"],
            NodeLanguage::Asm,
        );
        if let Some(n) = graph.node_mut(asm_final) {
            n.code = r#"; ═══════════════════════════════════════════════════════════════
; 🔧 BOOTLOADER ASM COMPLETO
; ═══════════════════════════════════════════════════════════════
; Este nodo hereda todo el código ASM del bootloader:
;   1. boot_sector.asm  - Sector de arranque (512 bytes)
;   2. stage2.asm       - Segunda etapa (modo protegido)
;   3. kernel_entry.asm - Punto de entrada al kernel C
;
; Presiona Ctrl+I para ver el código combinado.
; ═══════════════════════════════════════════════════════════════
"#.to_string();
        }
        Self::link_nodes(&mut graph, kernel_entry, asm_final, color_asm);
        
        // ═══════════════════════════════════════════════════════════════════════════
        // COLUMNA DERECHA: C (Headers + Sources)
        // ═══════════════════════════════════════════════════════════════════════════
        
        let mut y_c = 50.0;
        
        // types.h
        let types_h = graph.add_node(
            "📋 types.h",
            pos2(x_right, y_c),
            color_c_header,
            &[],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(types_h) {
            n.code = fastos::TYPES_H.to_string();
        }
        y_c += y_spacing;
        
        // ports.h
        let ports_h = graph.add_node(
            "📋 ports.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(ports_h) {
            n.code = fastos::PORTS_H.to_string();
        }
        Self::link_nodes(&mut graph, types_h, ports_h, color_c_header);
        y_c += y_spacing;
        
        // string.h + string.c
        let string_h = graph.add_node(
            "📋 string.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(string_h) {
            n.code = fastos::STRING_H.to_string();
        }
        Self::link_nodes(&mut graph, ports_h, string_h, color_c_header);
        y_c += y_spacing;
        
        let string_c = graph.add_node(
            "📝 string.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(string_c) {
            n.code = fastos::STRING.to_string();
        }
        Self::link_nodes(&mut graph, string_h, string_c, color_c_source);
        y_c += y_spacing;
        
        // vga.h + vga.c
        let vga_h = graph.add_node(
            "📋 vga.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(vga_h) {
            n.code = fastos::VGA_H.to_string();
        }
        Self::link_nodes(&mut graph, string_c, vga_h, color_c_header);
        y_c += y_spacing;
        
        let vga_c = graph.add_node(
            "🖥️ vga.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(vga_c) {
            n.code = fastos::VGA_DRIVER.to_string();
        }
        Self::link_nodes(&mut graph, vga_h, vga_c, color_c_source);
        y_c += y_spacing;
        
        // idt.h + idt.c
        let idt_h = graph.add_node(
            "📋 idt.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(idt_h) {
            n.code = fastos::IDT_H.to_string();
        }
        Self::link_nodes(&mut graph, vga_c, idt_h, color_c_header);
        y_c += y_spacing;
        
        let idt_c = graph.add_node(
            "⚡ idt.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(idt_c) {
            n.code = fastos::IDT.to_string();
        }
        Self::link_nodes(&mut graph, idt_h, idt_c, color_c_source);
        y_c += y_spacing;
        
        // keyboard.h + keyboard.c
        let keyboard_h = graph.add_node(
            "📋 keyboard.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(keyboard_h) {
            n.code = fastos::KEYBOARD_H.to_string();
        }
        Self::link_nodes(&mut graph, idt_c, keyboard_h, color_c_header);
        y_c += y_spacing;
        
        let keyboard_c = graph.add_node(
            "⌨️ keyboard.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(keyboard_c) {
            n.code = fastos::KEYBOARD_DRIVER.to_string();
        }
        Self::link_nodes(&mut graph, keyboard_h, keyboard_c, color_c_source);
        y_c += y_spacing;
        
        // timer.h + timer.c
        let timer_h = graph.add_node(
            "📋 timer.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(timer_h) {
            n.code = fastos::TIMER_H.to_string();
        }
        Self::link_nodes(&mut graph, keyboard_c, timer_h, color_c_header);
        y_c += y_spacing;
        
        let timer_c = graph.add_node(
            "⏱️ timer.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(timer_c) {
            n.code = fastos::TIMER.to_string();
        }
        Self::link_nodes(&mut graph, timer_h, timer_c, color_c_source);
        y_c += y_spacing;
        
        // memory.h + memory.c
        let memory_h = graph.add_node(
            "📋 memory.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(memory_h) {
            n.code = fastos::MEMORY_H.to_string();
        }
        Self::link_nodes(&mut graph, timer_c, memory_h, color_c_header);
        y_c += y_spacing;
        
        let memory_c = graph.add_node(
            "💾 memory.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(memory_c) {
            n.code = fastos::MEMORY.to_string();
        }
        Self::link_nodes(&mut graph, memory_h, memory_c, color_c_source);
        y_c += y_spacing;
        
        // shell.h + shell.c
        let shell_h = graph.add_node(
            "📋 shell.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(shell_h) {
            n.code = fastos::SHELL_H.to_string();
        }
        Self::link_nodes(&mut graph, memory_c, shell_h, color_c_header);
        y_c += y_spacing;
        
        let shell_c = graph.add_node(
            "💻 shell.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(shell_c) {
            n.code = fastos::SHELL.to_string();
        }
        Self::link_nodes(&mut graph, shell_h, shell_c, color_c_source);
        y_c += y_spacing;
        
        // kernel.h + kernel.c
        let kernel_h = graph.add_node(
            "📋 kernel.h",
            pos2(x_right, y_c),
            color_c_header,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(kernel_h) {
            n.code = fastos::KERNEL_H.to_string();
        }
        Self::link_nodes(&mut graph, shell_c, kernel_h, color_c_header);
        y_c += y_spacing;
        
        let kernel_c = graph.add_node(
            "🧠 kernel.c",
            pos2(x_right, y_c),
            color_c_source,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(kernel_c) {
            n.code = fastos::KERNEL_MAIN.to_string();
        }
        Self::link_nodes(&mut graph, kernel_h, kernel_c, color_c_source);
        y_c += y_spacing + 30.0;
        
        // Nodo final de C
        let c_final = graph.add_node(
            "🧠 C COMPLETO",
            pos2(x_right, y_c),
            Color32::from_rgb(0x44, 0x88, 0x44),
            &["▲"],
            &["←"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(c_final) {
            n.code = r#"/*
 * ═══════════════════════════════════════════════════════════════
 * 🧠 KERNEL C COMPLETO
 * ═══════════════════════════════════════════════════════════════
 * Este nodo hereda todo el código C del kernel:
 *   - Headers: types.h, ports.h, string.h, vga.h, idt.h, etc.
 *   - Sources: string.c, vga.c, idt.c, keyboard.c, etc.
 *   - Kernel:  kernel.h, kernel.c
 *
 * Presiona Ctrl+I para ver el código combinado.
 * ═══════════════════════════════════════════════════════════════
 */
"#.to_string();
        }
        Self::link_nodes(&mut graph, kernel_c, c_final, color_c_source);
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NODO COMBINADOR - Une ASM y C
        // ═══════════════════════════════════════════════════════════════════════════
        
        let y_combiner = y_c.max(y_asm) + y_spacing;
        
        let combiner = graph.add_node(
            "🔗 COMBINADOR",
            pos2(x_center, y_combiner),
            color_combiner,
            &["ASM", "C"],  // Dos entradas: una de ASM, otra de C
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(combiner) {
            n.code = r#"; ═══════════════════════════════════════════════════════════════════════════════
; 🔗 COMBINADOR ASM + C
; ═══════════════════════════════════════════════════════════════════════════════
;
; Este nodo COMBINA las dos ramas del proyecto:
;
;   ┌─────────────────┐         ┌─────────────────┐
;   │   ASM (NASM)    │         │       C         │
;   │   Bootloader    │         │   Kernel Code   │
;   └────────┬────────┘         └────────┬────────┘
;            │                           │
;            └───────────┬───────────────┘
;                        │
;                        ▼
;                  🔗 COMBINADOR
;
; El bootloader ASM carga el kernel C en memoria y salta a él.
; La conexión ASM → C ocurre en kernel_entry.asm que llama a kernel_main()
;
; ═══════════════════════════════════════════════════════════════════════════════
"#.to_string();
        }
        
        // Conectar ASM final -> Combinador (entrada 0)
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(asm_final, PinKind::Output, 0),
            graph.pin_id(combiner, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_asm);
        }
        
        // Conectar C final -> Combinador (entrada 1)
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(c_final, PinKind::Output, 0),
            graph.pin_id(combiner, PinKind::Input, 1)
        ) {
            graph.add_link(out_pin, in_pin, color_c_source);
        }
        
        // ═══════════════════════════════════════════════════════════════════════════
        // BUILD SYSTEM
        // ═══════════════════════════════════════════════════════════════════════════
        
        let y_build = y_combiner + y_spacing;
        
        let linker = graph.add_node(
            "🔗 linker.ld",
            pos2(x_center, y_build),
            color_build,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(linker) {
            n.code = fastos::LINKER.to_string();
        }
        Self::link_nodes(&mut graph, combiner, linker, color_build);
        
        let y_makefile = y_build + y_spacing;
        
        let makefile = graph.add_node(
            "🛠️ Makefile",
            pos2(x_center - 100.0, y_makefile),
            color_build,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(makefile) {
            n.code = fastos::MAKEFILE.to_string();
        }
        Self::link_nodes(&mut graph, linker, makefile, color_build);
        
        // Build.bat para Windows
        let build_bat = graph.add_node(
            "🖥️ build.bat",
            pos2(x_center + 100.0, y_makefile),
            Color32::from_rgb(0x00, 0xaa, 0xdd),  // Azul cyan
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(build_bat) {
            n.code = fastos::BUILD_BAT.to_string();
        }
        // Conectar linker -> build.bat también
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(linker, PinKind::Output, 0),
            graph.pin_id(build_bat, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_build);
        }
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NODO FINAL - FASTOS COMPLETO
        // ═══════════════════════════════════════════════════════════════════════════
        
        let y_final = y_makefile + y_spacing + 30.0;
        
        let fastos_final = graph.add_node(
            "🔥 FASTOS",
            pos2(x_center, y_final),
            color_final,
            &["Make", "Bat"],  // Dos entradas
            &["💾"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(fastos_final) {
            n.code = r#"/*
╔═══════════════════════════════════════════════════════════════════════════════╗
║                          🔥 FASTOS - SISTEMA OPERATIVO                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  Este nodo HEREDA TODO el código de ambas ramas:                              ║
║                                                                               ║
║     🔴 RAMA ASM (Izquierda)          🟢 RAMA C (Derecha)                      ║
║     ├── boot_sector.asm              ├── types.h / ports.h                    ║
║     ├── stage2.asm                   ├── string.h / string.c                  ║
║     └── kernel_entry.asm             ├── vga.h / vga.c                        ║
║                                      ├── idt.h / idt.c                        ║
║                                      ├── keyboard.h / keyboard.c              ║
║                                      ├── timer.h / timer.c                    ║
║                                      ├── memory.h / memory.c                  ║
║                                      ├── shell.h / shell.c                    ║
║                                      └── kernel.h / kernel.c                  ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  📋 COMPILAR EN WINDOWS:                                                      ║
║                                                                               ║
║     OPCIÓN 1 - Script batch (recomendado):                                    ║
║       > build.bat           Compilar todo                                     ║
║       > build.bat run       Ejecutar en QEMU                                  ║
║       > build.bat check     Verificar herramientas                            ║
║                                                                               ║
║     OPCIÓN 2 - Makefile (requiere make):                                      ║
║       > make clean && make && make run                                        ║
║                                                                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  🛠️ CROSS-COMPILER CONFIGURADO:                                               ║
║                                                                               ║
║     C:\Users\Andre\Documents\Mis Programas Poderosos\FastOS\bin\              ║
║     ├── i686-elf-gcc.exe      (Compilador C)                                  ║
║     ├── i686-elf-ld.exe       (Linker)                                        ║
║     └── i686-elf-objcopy.exe  (Object copy)                                   ║
║                                                                               ║
║  ⚠️  También necesitas:                                                       ║
║     - NASM (https://nasm.us)                                                  ║
║     - QEMU (https://www.qemu.org) para ejecutar                               ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
*/
"#.to_string();
        }
        
        // Conectar Makefile -> FastOS final (entrada 0)
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(makefile, PinKind::Output, 0),
            graph.pin_id(fastos_final, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_build);
        }
        
        // Conectar build.bat -> FastOS final (entrada 1)
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(build_bat, PinKind::Output, 0),
            graph.pin_id(fastos_final, PinKind::Input, 1)
        ) {
            graph.add_link(out_pin, in_pin, Color32::from_rgb(0x00, 0xaa, 0xdd));
        }
        
        graph
    }
    
    /// Helper para conectar dos nodos (output del primero -> input del segundo)
    fn link_nodes(graph: &mut NodeGraph, from_id: NodeId, to_id: NodeId, color: Color32) {
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(from_id, PinKind::Output, 0),
            graph.pin_id(to_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color);
        }
    }
    
    /// Crear proyecto Vulkan completo como mapa de nodos
    pub fn create_vulkan_project() -> NodeGraph {
        use crate::templates::vulkan;
        
        let mut graph = NodeGraph::default();
        
        // Colores por categoría
        let color_base = Color32::from_rgb(0xac, 0x14, 0x2c);      // Rojo Vulkan
        let color_init = Color32::from_rgb(0xff, 0x44, 0x00);      // Naranja
        let color_pipeline = Color32::from_rgb(0x8b, 0x00, 0x8b);  // Púrpura
        let color_shader = Color32::from_rgb(0x00, 0xff, 0x88);    // Verde shader
        let color_resources = Color32::from_rgb(0x00, 0x80, 0x80); // Cyan
        let color_exec = Color32::from_rgb(0xff, 0x8c, 0x00);      // Naranja oscuro
        let color_render = Color32::from_rgb(0x00, 0xbf, 0xff);    // Azul cielo
        let color_final = Color32::from_rgb(0xff, 0xd7, 0x00);     // Dorado
        let color_build = Color32::from_rgb(0x06, 0x4f, 0x8c);     // Azul CMake
        
        // Posición inicial
        let x_center = 400.0;
        let x_left = 200.0;
        let x_right = 600.0;
        let mut y = 100.0;
        let y_spacing = 120.0;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 1: TIPOS BASE
        // ═══════════════════════════════════════════════════════════════════════════
        let types_h = graph.add_node(
            "📋 vulkan_types.h",
            pos2(x_center, y),
            color_base,
            &[],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(types_h) {
            n.code = vulkan::TYPES_H.to_string();
        }
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 2: INSTANCE
        // ═══════════════════════════════════════════════════════════════════════════
        let instance = graph.add_node(
            "🔌 instance.cpp",
            pos2(x_center, y),
            color_init,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(instance) {
            n.code = vulkan::INSTANCE.to_string();
        }
        Self::link_nodes(&mut graph, types_h, instance, color_base);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 3: DEVICE
        // ═══════════════════════════════════════════════════════════════════════════
        let device = graph.add_node(
            "🖥️ device.cpp",
            pos2(x_center, y),
            color_init,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(device) {
            n.code = vulkan::DEVICE.to_string();
        }
        Self::link_nodes(&mut graph, instance, device, color_init);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 4: SWAPCHAIN
        // ═══════════════════════════════════════════════════════════════════════════
        let swapchain = graph.add_node(
            "🔄 swapchain.cpp",
            pos2(x_center, y),
            color_init,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(swapchain) {
            n.code = vulkan::SWAPCHAIN.to_string();
        }
        Self::link_nodes(&mut graph, device, swapchain, color_init);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 5: PIPELINE
        // ═══════════════════════════════════════════════════════════════════════════
        let pipeline = graph.add_node(
            "🔧 pipeline.cpp",
            pos2(x_center, y),
            color_pipeline,
            &["▲"],
            &["▼ Shaders", "▼ Buffers"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(pipeline) {
            n.code = vulkan::PIPELINE.to_string();
        }
        Self::link_nodes(&mut graph, swapchain, pipeline, color_init);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 6: SHADERS (izquierda) y BUFFERS (derecha)
        // ═══════════════════════════════════════════════════════════════════════════
        
        // Vertex Shader
        let shader_vert = graph.add_node(
            "📐 shader.vert",
            pos2(x_left, y),
            color_shader,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(shader_vert) {
            n.code = vulkan::SHADER_VERT.to_string();
        }
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(pipeline, PinKind::Output, 0),
            graph.pin_id(shader_vert, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_pipeline);
        }
        
        // Buffers
        let buffers = graph.add_node(
            "📦 buffers.cpp",
            pos2(x_right, y),
            color_resources,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(buffers) {
            n.code = vulkan::BUFFERS.to_string();
        }
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(pipeline, PinKind::Output, 1),
            graph.pin_id(buffers, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_pipeline);
        }
        y += y_spacing;
        
        // Fragment Shader
        let shader_frag = graph.add_node(
            "🎨 shader.frag",
            pos2(x_left, y),
            color_shader,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(shader_frag) {
            n.code = vulkan::SHADER_FRAG.to_string();
        }
        Self::link_nodes(&mut graph, shader_vert, shader_frag, color_shader);
        
        // Texture
        let texture = graph.add_node(
            "🖼️ texture.cpp",
            pos2(x_right, y),
            color_resources,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(texture) {
            n.code = vulkan::TEXTURE.to_string();
        }
        Self::link_nodes(&mut graph, buffers, texture, color_resources);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 7: COMMANDS (centro, combina shaders y recursos)
        // ═══════════════════════════════════════════════════════════════════════════
        let commands = graph.add_node(
            "📋 commands.cpp",
            pos2(x_center, y),
            color_exec,
            &["◀ Shaders", "▶ Resources"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(commands) {
            n.code = vulkan::COMMANDS.to_string();
        }
        // Conectar shaders -> commands
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(shader_frag, PinKind::Output, 0),
            graph.pin_id(commands, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_shader);
        }
        // Conectar resources -> commands
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(texture, PinKind::Output, 0),
            graph.pin_id(commands, PinKind::Input, 1)
        ) {
            graph.add_link(out_pin, in_pin, color_resources);
        }
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 8: SYNC
        // ═══════════════════════════════════════════════════════════════════════════
        let sync = graph.add_node(
            "⏱️ sync.cpp",
            pos2(x_center, y),
            color_exec,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(sync) {
            n.code = vulkan::SYNC.to_string();
        }
        Self::link_nodes(&mut graph, commands, sync, color_exec);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 9: RENDER LOOP
        // ═══════════════════════════════════════════════════════════════════════════
        let render_loop = graph.add_node(
            "↻ render_loop.cpp",
            pos2(x_center, y),
            color_render,
            &["▲"],
            &["▼"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(render_loop) {
            n.code = vulkan::RENDER_LOOP.to_string();
        }
        Self::link_nodes(&mut graph, sync, render_loop, color_exec);
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 10: BUILD SYSTEM (CMAKE a la izquierda)
        // ═══════════════════════════════════════════════════════════════════════════
        let cmake = graph.add_node(
            "🛠️ CMakeLists.txt",
            pos2(x_left, y),
            color_build,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(cmake) {
            n.code = vulkan::CMAKE.to_string();
        }
        
        // README a la derecha
        let readme = graph.add_node(
            "📖 README.md",
            pos2(x_right, y),
            color_build,
            &["▲"],
            &["▼"],
            NodeLanguage::Text,
        );
        if let Some(n) = graph.node_mut(readme) {
            n.code = vulkan::README.to_string();
        }
        y += y_spacing;
        
        // ═══════════════════════════════════════════════════════════════════════════
        // NIVEL 11: MAIN.CPP (NODO FINAL)
        // ═══════════════════════════════════════════════════════════════════════════
        let main = graph.add_node(
            "🎮 VULKAN APP [main.cpp]",
            pos2(x_center, y),
            color_final,
            &["◀ Loop", "▶ Build"],
            &["📁"],
            NodeLanguage::Cpp,
        );
        if let Some(n) = graph.node_mut(main) {
            n.code = vulkan::MAIN.to_string();
        }
        // Conectar render_loop -> main
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(render_loop, PinKind::Output, 0),
            graph.pin_id(main, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_render);
        }
        // Conectar cmake -> main
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(cmake, PinKind::Output, 0),
            graph.pin_id(main, PinKind::Input, 1)
        ) {
            graph.add_link(out_pin, in_pin, color_build);
        }
        
        graph
    }
}
