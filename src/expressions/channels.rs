// Sistema de canales (channels) inspirado en Houdini
// Permite almacenar y acceder a valores de nodos mediante referencias

use std::collections::HashMap;
use crate::node_graph::NodeId;

#[allow(dead_code)] // Listo para usar

#[derive(Clone, Debug)]
pub enum ChannelValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Code(String), // Código completo del nodo
}

impl ChannelValue {
    pub fn as_string(&self) -> String {
        match self {
            ChannelValue::String(s) => s.clone(),
            ChannelValue::Number(n) => n.to_string(),
            ChannelValue::Boolean(b) => b.to_string(),
            ChannelValue::Code(c) => c.clone(),
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            ChannelValue::Number(n) => Some(*n),
            ChannelValue::String(s) => s.parse().ok(),
            ChannelValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            ChannelValue::Code(_) => None,
        }
    }
    
    pub fn as_bool(&self) -> bool {
        match self {
            ChannelValue::Boolean(b) => *b,
            ChannelValue::Number(n) => *n != 0.0,
            ChannelValue::String(s) => !s.is_empty(),
            ChannelValue::Code(c) => !c.is_empty(),
        }
    }
}

/// Administrador de canales - almacena valores de nodos para acceso mediante ch()
#[derive(Clone)]
pub struct ChannelManager {
    channels: HashMap<String, ChannelValue>,
    node_channels: HashMap<NodeId, HashMap<String, ChannelValue>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            node_channels: HashMap::new(),
        }
    }
    
    /// Registrar un canal global
    pub fn set_channel(&mut self, name: String, value: ChannelValue) {
        self.channels.insert(name, value);
    }
    
    /// Registrar un canal de un nodo específico
    pub fn set_node_channel(&mut self, node_id: NodeId, name: String, value: ChannelValue) {
        self.node_channels
            .entry(node_id)
            .or_insert_with(HashMap::new)
            .insert(name, value);
    }
    
    /// Obtener un canal global
    pub fn get_channel(&self, name: &str) -> Option<&ChannelValue> {
        self.channels.get(name)
    }
    
    /// Obtener un canal de un nodo específico
    pub fn get_node_channel(&self, node_id: NodeId, name: &str) -> Option<&ChannelValue> {
        self.node_channels
            .get(&node_id)
            .and_then(|channels| channels.get(name))
    }
    
    /// Obtener el código completo de un nodo por nombre
    pub fn get_node_code(&self, node_name: &str) -> Option<String> {
        // Buscar en canales globales primero
        if let Some(ChannelValue::Code(code)) = self.channels.get(node_name) {
            return Some(code.clone());
        }
        
        // Buscar en canales de nodos
        for channels in self.node_channels.values() {
            if let Some(ChannelValue::Code(code)) = channels.get(node_name) {
                return Some(code.clone());
            }
        }
        
        None
    }
    
    /// Limpiar canales de un nodo
    pub fn clear_node_channels(&mut self, node_id: NodeId) {
        self.node_channels.remove(&node_id);
    }
    
    /// Obtener todos los canales de un nodo
    pub fn get_node_channels(&self, node_id: NodeId) -> Option<&HashMap<String, ChannelValue>> {
        self.node_channels.get(&node_id)
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

