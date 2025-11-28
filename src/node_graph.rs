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

    /// Crear proyecto FastOS completo con todos los nodos organizados y conectados
    pub fn create_fastos_project() -> Self {
        use crate::templates::fastos;
        
        let mut graph = Self::default();
        
        // Colores para cada categoría
        let color_boot = Color32::from_rgb(0xff, 0x00, 0x00);      // Rojo - Bootloader
        let color_kernel = Color32::from_rgb(0xff, 0x44, 0x00);    // Naranja - Kernel
        let color_drivers = Color32::from_rgb(0x00, 0xaa, 0x00);   // Verde - Drivers
        let color_system = Color32::from_rgb(0xaa, 0x00, 0xaa);    // Púrpura - Sistema
        let color_utils = Color32::from_rgb(0x00, 0x88, 0xcc);     // Azul - Utilidades
        let color_build = Color32::from_rgb(0x88, 0x88, 0x00);     // Amarillo - Build
        let color_final = Color32::from_rgb(0xff, 0xd7, 0x00);     // Dorado - Final
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 1: BOOTLOADER (ASM) - Y: 0
        // ═══════════════════════════════════════════════════════════════════
        
        // Boot Sector (raíz)
        let boot_sector_id = graph.add_node(
            "💿 Boot Sector",
            pos2(100.0, 50.0),
            color_boot,
            &[],
            &["Salida"],
            NodeLanguage::Asm,
        );
        if let Some(node) = graph.node_mut(boot_sector_id) {
            node.code = fastos::BOOT_SECTOR.to_string();
        }
        
        // Stage 2
        let stage2_id = graph.add_node(
            "🚀 Stage 2",
            pos2(400.0, 50.0),
            color_boot,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::Asm,
        );
        if let Some(node) = graph.node_mut(stage2_id) {
            node.code = fastos::STAGE2.to_string();
        }
        
        // Kernel Entry
        let kernel_entry_id = graph.add_node(
            "⚡ Kernel Entry",
            pos2(700.0, 50.0),
            color_boot,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::Asm,
        );
        if let Some(node) = graph.node_mut(kernel_entry_id) {
            node.code = fastos::KERNEL_ENTRY.to_string();
        }
        
        // Conectar bootloader
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(boot_sector_id, PinKind::Output, 0),
            graph.pin_id(stage2_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_boot);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(stage2_id, PinKind::Output, 0),
            graph.pin_id(kernel_entry_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_boot);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 2: UTILIDADES BASE (Headers) - Y: 250
        // ═══════════════════════════════════════════════════════════════════
        
        // Types Header
        let types_h_id = graph.add_node(
            "📋 types.h",
            pos2(100.0, 250.0),
            color_utils,
            &[],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(types_h_id) {
            node.code = fastos::TYPES_H.to_string();
        }
        
        // Ports Header
        let ports_h_id = graph.add_node(
            "📋 ports.h",
            pos2(400.0, 250.0),
            color_utils,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(ports_h_id) {
            node.code = fastos::PORTS_H.to_string();
        }
        
        // Conectar types -> ports
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(types_h_id, PinKind::Output, 0),
            graph.pin_id(ports_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_utils);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 3: STRING LIBRARY - Y: 450
        // ═══════════════════════════════════════════════════════════════════
        
        // String Header
        let string_h_id = graph.add_node(
            "📋 string.h",
            pos2(100.0, 450.0),
            color_utils,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(string_h_id) {
            node.code = fastos::STRING_H.to_string();
        }
        
        // String Implementation
        let string_c_id = graph.add_node(
            "📝 string.c",
            pos2(400.0, 450.0),
            color_utils,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(string_c_id) {
            node.code = fastos::STRING.to_string();
        }
        
        // Conectar ports -> string.h -> string.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(ports_h_id, PinKind::Output, 0),
            graph.pin_id(string_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_utils);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(string_h_id, PinKind::Output, 0),
            graph.pin_id(string_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_utils);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 4: VGA DRIVER - Y: 650
        // ═══════════════════════════════════════════════════════════════════
        
        // VGA Header
        let vga_h_id = graph.add_node(
            "📋 vga.h",
            pos2(100.0, 650.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(vga_h_id) {
            node.code = fastos::VGA_H.to_string();
        }
        
        // VGA Driver
        let vga_c_id = graph.add_node(
            "🖥️ vga_driver.c",
            pos2(400.0, 650.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(vga_c_id) {
            node.code = fastos::VGA_DRIVER.to_string();
        }
        
        // Conectar string.c -> vga.h -> vga_driver.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(string_c_id, PinKind::Output, 0),
            graph.pin_id(vga_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(vga_h_id, PinKind::Output, 0),
            graph.pin_id(vga_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 5: IDT (Interrupciones) - Y: 850
        // ═══════════════════════════════════════════════════════════════════
        
        // IDT Header
        let idt_h_id = graph.add_node(
            "📋 idt.h",
            pos2(100.0, 850.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(idt_h_id) {
            node.code = fastos::IDT_H.to_string();
        }
        
        // IDT Implementation
        let idt_c_id = graph.add_node(
            "⚡ idt.c",
            pos2(400.0, 850.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(idt_c_id) {
            node.code = fastos::IDT.to_string();
        }
        
        // Conectar vga_driver.c -> idt.h -> idt.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(vga_c_id, PinKind::Output, 0),
            graph.pin_id(idt_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(idt_h_id, PinKind::Output, 0),
            graph.pin_id(idt_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 6: KEYBOARD DRIVER - Y: 1050
        // ═══════════════════════════════════════════════════════════════════
        
        // Keyboard Header
        let keyboard_h_id = graph.add_node(
            "📋 keyboard.h",
            pos2(100.0, 1050.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(keyboard_h_id) {
            node.code = fastos::KEYBOARD_H.to_string();
        }
        
        // Keyboard Driver
        let keyboard_c_id = graph.add_node(
            "⌨️ keyboard_driver.c",
            pos2(400.0, 1050.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(keyboard_c_id) {
            node.code = fastos::KEYBOARD_DRIVER.to_string();
        }
        
        // Conectar idt.c -> keyboard.h -> keyboard_driver.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(idt_c_id, PinKind::Output, 0),
            graph.pin_id(keyboard_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(keyboard_h_id, PinKind::Output, 0),
            graph.pin_id(keyboard_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 7: TIMER DRIVER - Y: 1250
        // ═══════════════════════════════════════════════════════════════════
        
        // Timer Header
        let timer_h_id = graph.add_node(
            "📋 timer.h",
            pos2(100.0, 1250.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(timer_h_id) {
            node.code = fastos::TIMER_H.to_string();
        }
        
        // Timer Driver
        let timer_c_id = graph.add_node(
            "⏱️ timer.c",
            pos2(400.0, 1250.0),
            color_drivers,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(timer_c_id) {
            node.code = fastos::TIMER.to_string();
        }
        
        // Conectar keyboard_driver.c -> timer.h -> timer.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(keyboard_c_id, PinKind::Output, 0),
            graph.pin_id(timer_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(timer_h_id, PinKind::Output, 0),
            graph.pin_id(timer_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_drivers);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 8: MEMORY MANAGER - Y: 1450
        // ═══════════════════════════════════════════════════════════════════
        
        // Memory Header
        let memory_h_id = graph.add_node(
            "📋 memory.h",
            pos2(100.0, 1450.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(memory_h_id) {
            node.code = fastos::MEMORY_H.to_string();
        }
        
        // Memory Manager
        let memory_c_id = graph.add_node(
            "💾 memory.c",
            pos2(400.0, 1450.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(memory_c_id) {
            node.code = fastos::MEMORY.to_string();
        }
        
        // Conectar timer.c -> memory.h -> memory.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(timer_c_id, PinKind::Output, 0),
            graph.pin_id(memory_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(memory_h_id, PinKind::Output, 0),
            graph.pin_id(memory_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 9: SHELL - Y: 1650
        // ═══════════════════════════════════════════════════════════════════
        
        // Shell Header
        let shell_h_id = graph.add_node(
            "📋 shell.h",
            pos2(100.0, 1650.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(shell_h_id) {
            node.code = fastos::SHELL_H.to_string();
        }
        
        // Shell
        let shell_c_id = graph.add_node(
            "💻 shell.c",
            pos2(400.0, 1650.0),
            color_system,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(shell_c_id) {
            node.code = fastos::SHELL.to_string();
        }
        
        // Conectar memory.c -> shell.h -> shell.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(memory_c_id, PinKind::Output, 0),
            graph.pin_id(shell_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(shell_h_id, PinKind::Output, 0),
            graph.pin_id(shell_c_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_system);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 10: KERNEL - Y: 1850
        // ═══════════════════════════════════════════════════════════════════
        
        // Kernel Header
        let kernel_h_id = graph.add_node(
            "📋 kernel.h",
            pos2(100.0, 1850.0),
            color_kernel,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(kernel_h_id) {
            node.code = fastos::KERNEL_H.to_string();
        }
        
        // Kernel Main
        let kernel_main_id = graph.add_node(
            "🧠 kernel_main.c",
            pos2(400.0, 1850.0),
            color_kernel,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(kernel_main_id) {
            node.code = fastos::KERNEL_MAIN.to_string();
        }
        
        // Conectar shell.c -> kernel.h -> kernel_main.c
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(shell_c_id, PinKind::Output, 0),
            graph.pin_id(kernel_h_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_kernel);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(kernel_h_id, PinKind::Output, 0),
            graph.pin_id(kernel_main_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_kernel);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // FILA 11: BUILD SYSTEM - Y: 2050
        // ═══════════════════════════════════════════════════════════════════
        
        // Linker Script
        let linker_id = graph.add_node(
            "🔗 linker.ld",
            pos2(100.0, 2050.0),
            color_build,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::Asm,
        );
        if let Some(node) = graph.node_mut(linker_id) {
            node.code = fastos::LINKER.to_string();
        }
        
        // Makefile
        let makefile_id = graph.add_node(
            "🛠️ Makefile",
            pos2(400.0, 2050.0),
            color_build,
            &["Entrada"],
            &["Salida"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(makefile_id) {
            node.code = fastos::MAKEFILE.to_string();
        }
        
        // Conectar kernel_main.c -> linker.ld -> Makefile
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(kernel_main_id, PinKind::Output, 0),
            graph.pin_id(linker_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_build);
        }
        
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(linker_id, PinKind::Output, 0),
            graph.pin_id(makefile_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_build);
        }
        
        // ═══════════════════════════════════════════════════════════════════
        // NODO FINAL: FASTOS COMPLETO - Y: 2250
        // ═══════════════════════════════════════════════════════════════════
        
        // Nodo final que hereda todo
        let fastos_final_id = graph.add_node(
            "🔥 FASTOS COMPLETO",
            pos2(250.0, 2250.0),
            color_final,
            &["Entrada"],
            &["Código Final"],
            NodeLanguage::C,
        );
        if let Some(node) = graph.node_mut(fastos_final_id) {
            node.code = r#"/* ═══════════════════════════════════════════════════════════════════════════
 * FastOS - Sistema Operativo Completo
 * ═══════════════════════════════════════════════════════════════════════════
 * Este nodo hereda TODO el código de los nodos anteriores.
 * Usa Ctrl+I para ver la herencia completa.
 * 
 * Para compilar:
 * 1. Exporta todos los archivos a una carpeta
 * 2. Ejecuta 'make' para compilar
 * 3. Ejecuta 'make run' para probar en QEMU
 * 
 * Estructura del proyecto:
 * ├── boot_sector.asm    (Bootloader Stage 1)
 * ├── bootloader_stage2.asm (Modo protegido)
 * ├── kernel_entry.asm   (Punto de entrada)
 * ├── kernel_main.c      (Kernel principal)
 * ├── vga_driver.c       (Driver de video)
 * ├── keyboard_driver.c  (Driver de teclado)
 * ├── timer.c            (Driver de timer)
 * ├── idt.c              (Interrupciones)
 * ├── memory.c           (Gestor de memoria)
 * ├── shell.c            (Intérprete de comandos)
 * ├── string.c           (Librería de strings)
 * ├── linker.ld          (Script de enlazado)
 * └── Makefile           (Sistema de compilación)
 * ═══════════════════════════════════════════════════════════════════════════
 */

// ¡Tu sistema operativo FastOS está listo!
// Modifica los nodos anteriores para personalizar cada componente.
"#.to_string();
        }
        
        // Conectar Makefile -> FastOS Final
        if let (Some(out_pin), Some(in_pin)) = (
            graph.pin_id(makefile_id, PinKind::Output, 0),
            graph.pin_id(fastos_final_id, PinKind::Input, 0)
        ) {
            graph.add_link(out_pin, in_pin, color_final);
        }
        
        graph
    }
}
