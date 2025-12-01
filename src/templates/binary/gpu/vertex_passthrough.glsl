// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA GPU TEMPLATE: vertex_passthrough
// GLSL Equivalente para referencia (compilar con glslc a SPIR-V)
// ═══════════════════════════════════════════════════════════════════════════════

#version 450

// ─────────────────────────────────────────────────────────────────────────────
// INPUTS (desde vertex buffer)
// ─────────────────────────────────────────────────────────────────────────────
layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;

// ─────────────────────────────────────────────────────────────────────────────
// OUTPUTS (hacia fragment shader)
// ─────────────────────────────────────────────────────────────────────────────
layout(location = 0) out vec3 fragColor;

// ─────────────────────────────────────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────────────────────────────────────
void main() {
    // Pasar posición directamente (sin transformación MVP)
    gl_Position = vec4(inPosition, 1.0);
    
    // Pasar color al fragment shader
    fragColor = inColor;
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPILACIÓN:
//   glslc vertex_passthrough.glsl -o vertex_passthrough.spv
// ═══════════════════════════════════════════════════════════════════════════════

