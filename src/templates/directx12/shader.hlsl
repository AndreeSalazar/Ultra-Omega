/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 SHADER - Shaders HLSL
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: shader.hlsl
 * Descripción: Shaders para DirectX 12 (Vertex y Pixel)
 * ═══════════════════════════════════════════════════════════════════════════
 */

// ═══════════════════════════════════════════════════════════════════════════
// VERTEX SHADER
// ═══════════════════════════════════════════════════════════════════════════

struct VSInput {
    float3 position : POSITION;
    float4 color : COLOR;
    float2 texCoord : TEXCOORD;
};

struct VSOutput {
    float4 position : SV_POSITION;
    float4 color : COLOR;
    float2 texCoord : TEXCOORD;
};

cbuffer FrameConstants : register(b0) {
    float4x4 projection;
    float4x4 view;
    float4x4 world;
    float time;
    float3 padding;
};

VSOutput VSMain(VSInput input) {
    VSOutput output;
    
    // Transformar posición
    float4 pos = float4(input.position, 1.0);
    pos = mul(pos, world);
    pos = mul(pos, view);
    pos = mul(pos, projection);
    output.position = pos;
    
    // Pasar color y coordenadas de textura
    output.color = input.color;
    output.texCoord = input.texCoord;
    
    return output;
}

// ═══════════════════════════════════════════════════════════════════════════
// PIXEL SHADER
// ═══════════════════════════════════════════════════════════════════════════

struct PSInput {
    float4 position : SV_POSITION;
    float4 color : COLOR;
    float2 texCoord : TEXCOORD;
};

Texture2D texture0 : register(t0);
SamplerState sampler0 : register(s0);

float4 PSMain(PSInput input) : SV_TARGET {
    // Retornar color interpolado
    // Opcionalmente, muestrear textura:
    // return texture0.Sample(sampler0, input.texCoord) * input.color;
    
    return input.color;
}

