use crate::core::{NodeGraph, NodeId};
use crate::core::node_graph::PinKind;
use crate::core::types::Color32;
use crate::vulkan::renderer::{pin_screen_center, Viewport2D, NODE_HEIGHT, NODE_WIDTH, PIN_SIZE};

#[derive(Clone, Copy, Debug)]
pub struct HitPin {
    pub node_id: NodeId,
    pub kind: PinKind,
    pub slot: usize,
}

pub fn node_at_screen(graph: &NodeGraph, viewport: Viewport2D, screen: (f32, f32)) -> Option<NodeId> {
    let world = viewport.screen_to_world(screen.0, screen.1);
    graph
        .nodes()
        .iter()
        .rev()
        .find(|node| {
            world.0 >= node.position.x
                && world.0 <= node.position.x + NODE_WIDTH
                && world.1 >= node.position.y
                && world.1 <= node.position.y + NODE_HEIGHT
        })
        .map(|node| node.id)
}

pub fn pin_at_screen(graph: &NodeGraph, viewport: Viewport2D, screen: (f32, f32)) -> Option<HitPin> {
    let radius = (PIN_SIZE * viewport.zoom).max(8.0);
    let radius_sq = radius * radius;

    for node in graph.nodes().iter().rev() {
        for (slot, _) in node.outputs.iter().enumerate() {
            let center = pin_screen_center(node, PinKind::Output, slot, viewport);
            if distance_sq(screen, center) <= radius_sq {
                return Some(HitPin { node_id: node.id, kind: PinKind::Output, slot });
            }
        }

        for (slot, _) in node.inputs.iter().enumerate() {
            let center = pin_screen_center(node, PinKind::Input, slot, viewport);
            if distance_sq(screen, center) <= radius_sq {
                return Some(HitPin { node_id: node.id, kind: PinKind::Input, slot });
            }
        }
    }

    None
}

pub fn try_finish_link(
    graph: &mut NodeGraph,
    source: Option<HitPin>,
    cursor: Option<(f32, f32)>,
    viewport: Viewport2D,
    selected: &mut Option<NodeId>,
) -> (bool, Option<HitPin>) {
    let Some(src) = source else { return (false, source); };
    let Some(cur) = cursor else { return (false, source); };
    let Some(target) = pin_at_screen(graph, viewport, cur) else { return (false, source); };

    if src.node_id == target.node_id || target.kind != PinKind::Input {
        return (false, None);
    }

    let Some(from_pin) = graph.pin_id(src.node_id, PinKind::Output, src.slot) else {
        return (false, None);
    };
    let Some(to_pin) = graph.pin_id(target.node_id, PinKind::Input, target.slot) else {
        return (false, None);
    };

    graph.add_link(from_pin, to_pin, Color32::from_rgb(168, 112, 62));
    *selected = Some(target.node_id);
    (true, None)
}

pub fn try_start_link(
    graph: &NodeGraph,
    viewport: Viewport2D,
    cursor: Option<(f32, f32)>,
    selected: &mut Option<NodeId>,
) -> Option<HitPin> {
    let Some(screen) = cursor else { return None; };
    let pin = pin_at_screen(graph, viewport, screen)?;
    if pin.kind != PinKind::Output {
        return None;
    }
    *selected = Some(pin.node_id);
    Some(pin)
}

pub fn start_link_from_selected(selected: Option<NodeId>) -> Option<HitPin> {
    selected.map(|node_id| HitPin {
        node_id,
        kind: PinKind::Output,
        slot: 0,
    })
}

fn distance_sq(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}
