/* ═══════════════════════════════════════════════════════════════════════════
 * RUST MATH - Funciones matemáticas
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: lib_math.rs
 * Descripción: Funciones matemáticas reutilizables para Rust
 * 
 * USO: Este módulo puede ser heredado por otros nodos para usar estas funciones
 * Ejemplo de herencia: Conecta este nodo a otro para acceder a math con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

use std::f64::consts;

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES BÁSICAS
// ═══════════════════════════════════════════════════════════════════════════

/// Clamp valor entre min y max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Map valor de un rango a otro
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let t = (value - from_min) / (from_max - from_min);
    lerp(to_min, to_max, t)
}

/// Smoothstep (interpolación suave)
pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES DE REDONDEO
// ═══════════════════════════════════════════════════════════════════════════

/// Redondear a n decimales
pub fn round_to(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}

/// Redondear hacia arriba
pub fn ceil(value: f64) -> i64 {
    value.ceil() as i64
}

/// Redondear hacia abajo
pub fn floor(value: f64) -> i64 {
    value.floor() as i64
}

// ═══════════════════════════════════════════════════════════════════════════
// FUNCIONES TRIGONOMÉTRICAS
// ═══════════════════════════════════════════════════════════════════════════

/// Convertir grados a radianes
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * consts::PI / 180.0
}

/// Convertir radianes a grados
pub fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / consts::PI
}

/// Distancia euclidiana entre dos puntos 2D
pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Distancia euclidiana entre dos puntos 3D
pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ═══════════════════════════════════════════════════════════════════════════
// VECTORES 2D Y 3D (estructuras simples)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
    
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            Vec2::new(self.x / len, self.y / len)
        } else {
            Vec2::new(0.0, 0.0)
        }
    }
    
    pub fn distance(&self, other: &Vec2) -> f64 {
        distance_2d(self.x, self.y, other.x, other.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
    
    pub fn distance(&self, other: &Vec3) -> f64 {
        distance_3d(self.x, self.y, self.z, other.x, other.y, other.z)
    }
    
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

