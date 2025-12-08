/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 PIPELINE STATE - Pipeline State Object (PSO)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: pipeline_state.cpp
 * Descripción: Creación del Pipeline State Object para renderizado
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include "root_signature.cpp"
#include "shader.hlsl"
#include <iostream>
#include <fstream>
#include <vector>

// Pipeline State Object
ComPtr<ID3D12PipelineState> g_pipelineState;

// ═══════════════════════════════════════════════════════════════════════════
// CARGAR SHADER COMPILADO (HLSL)
// ═══════════════════════════════════════════════════════════════════════════

std::vector<BYTE> LoadShader(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary | std::ios::ate);
    if (!file.is_open()) {
        throw std::runtime_error("No se pudo abrir el archivo de shader: " + filename);
    }
    
    size_t fileSize = (size_t)file.tellg();
    std::vector<BYTE> buffer(fileSize);
    
    file.seekg(0);
    file.read((char*)buffer.data(), fileSize);
    file.close();
    
    return buffer;
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR PIPELINE STATE OBJECT
// ═══════════════════════════════════════════════════════════════════════════

void CreatePipelineState() {
    // Cargar shaders compilados (.cso)
    // Nota: En producción, compila los shaders con fxc o dxc
    // Por ahora, usamos shaders inline para el template
    
    // Vertex shader (simple)
    const char* vsCode = R"(
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
        };
        
        VSOutput main(VSInput input) {
            VSOutput output;
            float4 pos = float4(input.position, 1.0);
            pos = mul(pos, world);
            pos = mul(pos, view);
            pos = mul(pos, projection);
            output.position = pos;
            output.color = input.color;
            output.texCoord = input.texCoord;
            return output;
        }
    )";
    
    // Pixel shader (simple)
    const char* psCode = R"(
        struct PSInput {
            float4 position : SV_POSITION;
            float4 color : COLOR;
            float2 texCoord : TEXCOORD;
        };
        
        float4 main(PSInput input) : SV_TARGET {
            return input.color;
        }
    )";
    
    // Compilar shaders (en producción usa fxc o dxc)
    // Por ahora, asumimos que ya están compilados
    
    // Input layout
    UINT numElements;
    D3D12_INPUT_ELEMENT_DESC* inputLayout = Vertex::GetInputLayout(numElements);
    
    // Descripción del PSO
    D3D12_GRAPHICS_PIPELINE_STATE_DESC psoDesc = {};
    psoDesc.pRootSignature = GetRootSignature();
    // psoDesc.VS = { vsShader->GetBufferPointer(), vsShader->GetBufferSize() };
    // psoDesc.PS = { psShader->GetBufferPointer(), psShader->GetBufferSize() };
    psoDesc.BlendState.RenderTarget[0].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL;
    psoDesc.SampleMask = UINT_MAX;
    psoDesc.RasterizerState.FillMode = D3D12_FILL_MODE_SOLID;
    psoDesc.RasterizerState.CullMode = D3D12_CULL_MODE_BACK;
    psoDesc.DepthStencilState.DepthEnable = FALSE;
    psoDesc.InputLayout = { inputLayout, numElements };
    psoDesc.PrimitiveTopologyType = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
    psoDesc.NumRenderTargets = 1;
    psoDesc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;
    psoDesc.SampleDesc.Count = 1;
    
    DX_CHECK(GetDevice()->CreateGraphicsPipelineState(&psoDesc, IID_PPV_ARGS(&g_pipelineState)));
    
    std::cout << "Pipeline State Object creado" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER PIPELINE STATE
// ═══════════════════════════════════════════════════════════════════════════

ID3D12PipelineState* GetPipelineState() {
    return g_pipelineState.Get();
}

