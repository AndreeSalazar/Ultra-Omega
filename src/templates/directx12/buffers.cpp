/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 BUFFERS - Vertex, Index y Constant Buffers
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: buffers.cpp
 * Descripción: Creación y manejo de buffers (Vertex, Index, Constant)
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include <iostream>

// Buffers
ComPtr<ID3D12Resource> g_vertexBuffer;
ComPtr<ID3D12Resource> g_indexBuffer;
ComPtr<ID3D12Resource> g_constantBuffers[BACK_BUFFER_COUNT];
D3D12_VERTEX_BUFFER_VIEW g_vertexBufferView;
D3D12_INDEX_BUFFER_VIEW g_indexBufferView;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR VERTEX BUFFER
// ═══════════════════════════════════════════════════════════════════════════

void CreateVertexBuffer(const Vertex* vertices, UINT vertexCount) {
    const UINT vertexBufferSize = sizeof(Vertex) * vertexCount;
    
    // Crear buffer de upload (staging)
    CD3DX12_HEAP_PROPERTIES uploadHeapProps(D3D12_HEAP_TYPE_UPLOAD);
    CD3DX12_RESOURCE_DESC bufferDesc = CD3DX12_RESOURCE_DESC::Buffer(vertexBufferSize);
    
    ComPtr<ID3D12Resource> uploadBuffer;
    DX_CHECK(GetDevice()->CreateCommittedResource(
        &uploadHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(&uploadBuffer)));
    
    // Crear buffer final (default heap)
    CD3DX12_HEAP_PROPERTIES defaultHeapProps(D3D12_HEAP_TYPE_DEFAULT);
    DX_CHECK(GetDevice()->CreateCommittedResource(
        &defaultHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_COPY_DEST,
        nullptr,
        IID_PPV_ARGS(&g_vertexBuffer)));
    
    // Copiar datos
    D3D12_SUBRESOURCE_DATA vertexData = {};
    vertexData.pData = vertices;
    vertexData.RowPitch = vertexBufferSize;
    vertexData.SlicePitch = vertexBufferSize;
    
    // Usar UpdateSubresources para copiar (necesita command list)
    // Por ahora, mapear directamente en upload buffer
    void* pData;
    uploadBuffer->Map(0, nullptr, &pData);
    memcpy(pData, vertices, vertexBufferSize);
    uploadBuffer->Unmap(0, nullptr);
    
    // View del vertex buffer
    g_vertexBufferView.BufferLocation = g_vertexBuffer->GetGPUVirtualAddress();
    g_vertexBufferView.SizeInBytes = vertexBufferSize;
    g_vertexBufferView.StrideInBytes = sizeof(Vertex);
    
    std::cout << "Vertex Buffer creado (" << vertexCount << " vértices)" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR INDEX BUFFER
// ═══════════════════════════════════════════════════════════════════════════

void CreateIndexBuffer(const UINT* indices, UINT indexCount) {
    const UINT indexBufferSize = sizeof(UINT) * indexCount;
    
    CD3DX12_HEAP_PROPERTIES uploadHeapProps(D3D12_HEAP_TYPE_UPLOAD);
    CD3DX12_RESOURCE_DESC bufferDesc = CD3DX12_RESOURCE_DESC::Buffer(indexBufferSize);
    
    ComPtr<ID3D12Resource> uploadBuffer;
    DX_CHECK(GetDevice()->CreateCommittedResource(
        &uploadHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(&uploadBuffer)));
    
    CD3DX12_HEAP_PROPERTIES defaultHeapProps(D3D12_HEAP_TYPE_DEFAULT);
    DX_CHECK(GetDevice()->CreateCommittedResource(
        &defaultHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_COPY_DEST,
        nullptr,
        IID_PPV_ARGS(&g_indexBuffer)));
    
    // Copiar datos
    void* pData;
    uploadBuffer->Map(0, nullptr, &pData);
    memcpy(pData, indices, indexBufferSize);
    uploadBuffer->Unmap(0, nullptr);
    
    // View del index buffer
    g_indexBufferView.BufferLocation = g_indexBuffer->GetGPUVirtualAddress();
    g_indexBufferView.SizeInBytes = indexBufferSize;
    g_indexBufferView.Format = DXGI_FORMAT_R32_UINT;
    
    std::cout << "Index Buffer creado (" << indexCount << " índices)" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR CONSTANT BUFFERS
// ═══════════════════════════════════════════════════════════════════════════

void CreateConstantBuffers() {
    const UINT constantBufferSize = (sizeof(FrameConstants) + CONSTANT_BUFFER_ALIGNMENT - 1) & ~(CONSTANT_BUFFER_ALIGNMENT - 1);
    
    CD3DX12_HEAP_PROPERTIES uploadHeapProps(D3D12_HEAP_TYPE_UPLOAD);
    CD3DX12_RESOURCE_DESC bufferDesc = CD3DX12_RESOURCE_DESC::Buffer(constantBufferSize);
    
    for (UINT i = 0; i < BACK_BUFFER_COUNT; i++) {
        DX_CHECK(GetDevice()->CreateCommittedResource(
            &uploadHeapProps,
            D3D12_HEAP_FLAG_NONE,
            &bufferDesc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            nullptr,
            IID_PPV_ARGS(&g_constantBuffers[i])));
    }
    
    std::cout << "Constant Buffers creados (" << BACK_BUFFER_COUNT << ")" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// ACTUALIZAR CONSTANT BUFFER
// ═══════════════════════════════════════════════════════════════════════════

void UpdateConstantBuffer(UINT frameIndex, const FrameConstants& constants) {
    void* pData;
    CD3DX12_RANGE readRange(0, 0); // No leer
    DX_CHECK(g_constantBuffers[frameIndex]->Map(0, &readRange, &pData));
    memcpy(pData, &constants, sizeof(FrameConstants));
    g_constantBuffers[frameIndex]->Unmap(0, nullptr);
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER VIEWS
// ═══════════════════════════════════════════════════════════════════════════

D3D12_VERTEX_BUFFER_VIEW* GetVertexBufferView() {
    return &g_vertexBufferView;
}

D3D12_INDEX_BUFFER_VIEW* GetIndexBufferView() {
    return &g_indexBufferView;
}

D3D12_GPU_VIRTUAL_ADDRESS GetConstantBufferGpuAddress(UINT frameIndex) {
    return g_constantBuffers[frameIndex]->GetGPUVirtualAddress();
}

