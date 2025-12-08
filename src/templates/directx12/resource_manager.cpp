/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 RESOURCE MANAGER - Gestión de recursos (wrappers)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: resource_manager.cpp
 * Descripción: Wrappers y utilidades para gestión de recursos DirectX12
 * 
 * USO: Este nodo puede ser heredado para usar funciones de gestión de recursos
 * Ejemplo: Conecta este nodo a otros para acceder a resource_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include "helpers.cpp"

// ═══════════════════════════════════════════════════════════════════════════
// WRAPPER PARA BUFFERS
// ═══════════════════════════════════════════════════════════════════════════

struct BufferResource {
    ComPtr<ID3D12Resource> resource;
    D3D12_GPU_VIRTUAL_ADDRESS gpuAddress;
    void* cpuAddress;
    UINT size;
    
    void Release() {
        if (resource) {
            if (cpuAddress) {
                resource->Unmap(0, nullptr);
                cpuAddress = nullptr;
            }
            resource.Reset();
        }
    }
};

// Crear vertex buffer
BufferResource CreateVertexBuffer(ID3D12Device* device, const void* data, UINT size) {
    BufferResource buffer = {};
    buffer.size = size;
    
    // Crear buffer en upload heap (temporal, para copiar)
    CD3DX12_HEAP_PROPERTIES uploadHeapProps(D3D12_HEAP_TYPE_UPLOAD);
    CD3DX12_RESOURCE_DESC bufferDesc = CD3DX12_RESOURCE_DESC::Buffer(size);
    
    ComPtr<ID3D12Resource> uploadBuffer;
    if (FAILED(device->CreateCommittedResource(
        &uploadHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(&uploadBuffer)))) {
        return buffer; // Error
    }
    
    // Crear buffer final en default heap
    CD3DX12_HEAP_PROPERTIES defaultHeapProps(D3D12_HEAP_TYPE_DEFAULT);
    if (FAILED(device->CreateCommittedResource(
        &defaultHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_COPY_DEST,
        nullptr,
        IID_PPV_ARGS(&buffer.resource)))) {
        return buffer; // Error
    }
    
    buffer.gpuAddress = buffer.resource->GetGPUVirtualAddress();
    
    // Copiar datos
    D3D12_SUBRESOURCE_DATA subresourceData = {};
    subresourceData.pData = data;
    subresourceData.RowPitch = size;
    subresourceData.SlicePitch = size;
    
    // Nota: En producción usar UpdateSubresources con command list
    // Por ahora, mapear directamente
    void* pData;
    uploadBuffer->Map(0, nullptr, &pData);
    memcpy(pData, data, size);
    uploadBuffer->Unmap(0, nullptr);
    
    return buffer;
}

// Crear index buffer
BufferResource CreateIndexBuffer(ID3D12Device* device, const UINT* indices, UINT count) {
    return CreateVertexBuffer(device, indices, sizeof(UINT) * count);
}

// Crear constant buffer mapeable
BufferResource CreateConstantBuffer(ID3D12Device* device, UINT size) {
    BufferResource buffer = {};
    buffer.size = GetConstantBufferAlignedSize(size);
    
    CD3DX12_HEAP_PROPERTIES uploadHeapProps(D3D12_HEAP_TYPE_UPLOAD);
    CD3DX12_RESOURCE_DESC bufferDesc = CD3DX12_RESOURCE_DESC::Buffer(buffer.size);
    
    if (SUCCEEDED(device->CreateCommittedResource(
        &uploadHeapProps,
        D3D12_HEAP_FLAG_NONE,
        &bufferDesc,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        nullptr,
        IID_PPV_ARGS(&buffer.resource)))) {
        buffer.gpuAddress = buffer.resource->GetGPUVirtualAddress();
        buffer.resource->Map(0, nullptr, &buffer.cpuAddress);
    }
    
    return buffer;
}

// Actualizar constant buffer
void UpdateConstantBuffer(BufferResource& buffer, const void* data, UINT size) {
    if (buffer.cpuAddress && size <= buffer.size) {
        memcpy(buffer.cpuAddress, data, size);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WRAPPER PARA DESCRIPTOR HEAPS
// ═══════════════════════════════════════════════════════════════════════════

struct DescriptorHeapManager {
    ComPtr<ID3D12DescriptorHeap> heap;
    UINT descriptorSize;
    UINT capacity;
    UINT currentIndex;
    
    bool Initialize(ID3D12Device* device, D3D12_DESCRIPTOR_HEAP_TYPE type, UINT count, bool shaderVisible = false) {
        D3D12_DESCRIPTOR_HEAP_DESC desc = {};
        desc.NumDescriptors = count;
        desc.Type = type;
        desc.Flags = shaderVisible ? D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE : D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
        
        if (FAILED(device->CreateDescriptorHeap(&desc, IID_PPV_ARGS(&heap)))) {
            return false;
        }
        
        descriptorSize = device->GetDescriptorHandleIncrementSize(type);
        capacity = count;
        currentIndex = 0;
        return true;
    }
    
    CD3DX12_CPU_DESCRIPTOR_HANDLE GetCPUHandle(UINT index) {
        return CD3DX12_CPU_DESCRIPTOR_HANDLE(
            heap->GetCPUDescriptorHandleForHeapStart(),
            index,
            descriptorSize);
    }
    
    CD3DX12_GPU_DESCRIPTOR_HANDLE GetGPUHandle(UINT index) {
        return CD3DX12_GPU_DESCRIPTOR_HANDLE(
            heap->GetGPUDescriptorHandleForHeapStart(),
            index,
            descriptorSize);
    }
    
    UINT Allocate() {
        if (currentIndex >= capacity) {
            return UINT_MAX; // Heap lleno
        }
        return currentIndex++;
    }
};

