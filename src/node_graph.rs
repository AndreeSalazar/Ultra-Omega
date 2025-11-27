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

    #[allow(dead_code)] // Se usará cuando se implemente la conexión manual de nodos
    pub fn add_link(&mut self, from: PinId, to: PinId, color: Color32) {
        self.links.push(Link { from, to, color });
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
