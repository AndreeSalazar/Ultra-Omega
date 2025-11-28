/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN FRAGMENT SHADER
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: shader.frag
 * Descripción: Fragment shader para coloreado de píxeles
 * Compilar: glslc shader.frag -o frag.spv
 * ═══════════════════════════════════════════════════════════════════════════
 */

#version 450

/* ═══════════════════════════════════════════════════════════════════════════
 * INPUTS - Datos del vertex shader
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;

/* ═══════════════════════════════════════════════════════════════════════════
 * OUTPUTS - Color final del píxel
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(location = 0) out vec4 outColor;

/* ═══════════════════════════════════════════════════════════════════════════
 * SAMPLER - Textura (opcional)
 * ═══════════════════════════════════════════════════════════════════════════
 */
layout(binding = 1) uniform sampler2D texSampler;

/* ═══════════════════════════════════════════════════════════════════════════
 * MAIN - Calcular color final
 * ═══════════════════════════════════════════════════════════════════════════
 */
void main() {
    // Opción 1: Solo color de vértice
    outColor = vec4(fragColor, 1.0);
    
    // Opción 2: Textura con color de vértice (descomentar si tienes textura)
    // outColor = texture(texSampler, fragTexCoord) * vec4(fragColor, 1.0);
    
    // Opción 3: Efecto de gradiente con coordenadas UV
    // outColor = vec4(fragTexCoord, 0.5, 1.0);
}

