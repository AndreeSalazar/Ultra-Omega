// ═══════════════════════════════════════════════════════════════════════════
// Tipos básicos para el motor de nodos (reemplazo de eframe::egui)
// ═══════════════════════════════════════════════════════════════════════════

use std::ops::{Add, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Pos2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Pos2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

pub fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2::new(x, y)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color32 {
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const DARK_GRAY: Self = Self { r: 30, g: 30, b: 30, a: 255 }; // #1E1E1E (VS Code)
    pub const NODE_BG: Self = Self { r: 45, g: 45, b: 48, a: 255 };   // #2D2D30
    pub const NODE_HEADER: Self = Self { r: 0, g: 122, b: 204, a: 255 }; // VS Code Blue

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convierte a [f32; 4] normalizado (0.0 - 1.0) para enviar a los Shaders de Vulkan
    pub fn to_normalized_f32(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}
