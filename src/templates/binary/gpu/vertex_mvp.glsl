// ═══════════════════════════════════════════════════════════════════════════════
// ULTRA-OMEGA GPU TEMPLATE: vertex_mvp
// Vertex shader con transformación Model-View-Projection
// ═══════════════════════════════════════════════════════════════════════════════

#version 450

// ─────────────────────────────────────────────────────────────────────────────
// UNIFORM BUFFER: Matrices MVP
// ─────────────────────────────────────────────────────────────────────────────
layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

// ─────────────────────────────────────────────────────────────────────────────
// INPUTS (vertex attributes)
// ─────────────────────────────────────────────────────────────────────────────
layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;

// ─────────────────────────────────────────────────────────────────────────────
// OUTPUTS (hacia fragment shader)
// ─────────────────────────────────────────────────────────────────────────────
layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 fragTexCoord;

// ─────────────────────────────────────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────────────────────────────────────
void main() {
    // Transformación MVP completa
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 1.0);
    
    // Pasar atributos al fragment shader
    fragColor = inColor;
    fragTexCoord = inTexCoord;
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPILACIÓN:
//   glslc vertex_mvp.glsl -o vertex_mvp.spv
// ═══════════════════════════════════════════════════════════════════════════════

