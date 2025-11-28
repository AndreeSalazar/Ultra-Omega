/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN VERTEX SHADER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: shader.vert
 * Descripción: Vertex shader para transformación de vértices
 * Compilar: glslc shader.vert -o vert.spv
 * ═══════════════════════════════════════════════════════════════════════════
 */

#version 450

/* ═══════════════════════════════════════════════════════════════════════════
 * UNIFORM BUFFER OBJECT - Matrices de transformación
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

/* ═══════════════════════════════════════════════════════════════════════════
 * INPUTS - Atributos de vértice
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;

/* ═══════════════════════════════════════════════════════════════════════════
 * OUTPUTS - Datos para fragment shader
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 fragTexCoord;

/* ═══════════════════════════════════════════════════════════════════════════
 * MAIN - Transformación de vértice
 * ═══════════════════════════════════════════════════════════════════════════
 */
void main() {
    // Aplicar transformaciones MVP
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 1.0);
    
    // Pasar datos al fragment shader
    fragColor = inColor;
    fragTexCoord = inTexCoord;
}

