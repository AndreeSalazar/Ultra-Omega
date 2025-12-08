/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 ROOT SIGNATURE - Root Signature para shaders
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: root_signature.cpp
 * Descripción: Creación de root signature para binding de recursos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include <iostream>

// Root signature
ComPtr<ID3D12RootSignature> g_rootSignature;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR ROOT SIGNATURE
// ═══════════════════════════════════════════════════════════════════════════

void CreateRootSignature() {
    // Root parameter: constante buffer (constant buffer view)
    CD3DX12_ROOT_PARAMETER1 rootParameters[1];
    rootParameters[0].InitAsConstantBufferView(0, 0, D3D12_ROOT_DESCRIPTOR_FLAG_NONE, D3D12_SHADER_VISIBILITY_ALL);
    
    // Descripción del root signature
    CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC rootSignatureDesc;
    rootSignatureDesc.Init_1_1(
        _countof(rootParameters),
        rootParameters,
        0,
        nullptr,
        D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT);
    
    // Serializar root signature
    ComPtr<ID3DBlob> signature;
    ComPtr<ID3DBlob> error;
    
    HRESULT hr = D3DX12SerializeVersionedRootSignature(
        &rootSignatureDesc,
        D3D_ROOT_SIGNATURE_VERSION_1_1,
        &signature,
        &error);
    
    if (FAILED(hr)) {
        if (error) {
            std::cout << "Error al serializar root signature: " 
                      << (char*)error->GetBufferPointer() << std::endl;
        }
        throw std::runtime_error("Error al crear root signature");
    }
    
    // Crear root signature
    DX_CHECK(GetDevice()->CreateRootSignature(
        0,
        signature->GetBufferPointer(),
        signature->GetBufferSize(),
        IID_PPV_ARGS(&g_rootSignature)));
    
    std::cout << "Root Signature creada" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER ROOT SIGNATURE
// ═══════════════════════════════════════════════════════════════════════════

ID3D12RootSignature* GetRootSignature() {
    return g_rootSignature.Get();
}

