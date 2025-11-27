use eframe::egui::{Color32, Pos2, pos2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PinId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinKind {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub struct Pin {
    pub id: PinId,
    pub label: String,
    pub kind: PinKind,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: NodeId,
    pub title: String,
    pub position: Pos2,
    pub color: Color32,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub code: String,
}

#[derive(Clone, Debug)]
pub struct Link {
    pub from: PinId,
    pub to: PinId,
    pub color: Color32,
}

#[derive(Default)]
pub struct NodeGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
    next_node_id: u64,
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
            node.code = "section .text\nglobal _start\n\n_start:\n    mov eax, 1\n    mov ebx, 42\n    int 0x80".to_string();
        }

        // Node 2: Constructor / Visualizador
        let builder_node = graph.add_node(
            "Visualizador",
            pos2(500.0, 150.0),
            Color32::from_rgb(0x00, 0xa3, 0xff), // Cyan for tech/visuals
            &["Entrada ASM"],
            &["Vista Previa", "Binario"],
        );

        // Link them together
        if let (Some(source_out), Some(builder_in)) = (
            graph.pin_id(asm_node_id, PinKind::Output, 0),
            graph.pin_id(builder_node, PinKind::Input, 0),
        ) {
            graph.add_link(source_out, builder_in, Color32::from_rgb(0xff, 0xaa, 0x00));
        }

        graph
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn links(&self) -> &[Link] {
        &self.links
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

    pub fn add_default_node(&mut self, position: Pos2) -> NodeId {
        let palette = [
            Color32::from_rgb(0x9f, 0x7a, 0xff),
            Color32::from_rgb(0x58, 0xb0, 0xf6),
            Color32::from_rgb(0xff, 0x8c, 0x64),
            Color32::from_rgb(0x65, 0xf2, 0xb3),
        ];
        let color = palette[self.nodes.len() % palette.len()];
        let title = format!("Nodo {}", self.nodes.len() + 1);
        self.add_node(title, position, color, &["Entrada"], &["Salida"])
    }

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
