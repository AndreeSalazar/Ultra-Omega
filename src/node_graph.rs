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
}
