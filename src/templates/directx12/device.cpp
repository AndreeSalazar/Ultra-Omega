/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 DEVICE - Creación del dispositivo DirectX 12
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: device.cpp
 * Descripción: Creación y configuración del dispositivo DirectX 12
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "adapter.cpp"
#include <iostream>

// Variables globales del dispositivo
ComPtr<ID3D12Device> g_device;
ComPtr<ID3D12CommandQueue> g_commandQueue;
D3D_FEATURE_LEVEL g_featureLevel;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR DISPOSITIVO DIRECTX 12
// ═══════════════════════════════════════════════════════════════════════════

void CreateDevice() {
    // Seleccionar adaptador
    auto adapter = SelectBestAdapter();
    
    // Crear dispositivo con el feature level más alto soportado
    HRESULT hr = E_FAIL;
    for (const auto& level : FEATURE_LEVELS) {
        hr = D3D12CreateDevice(
            adapter.Get(),
            level,
            IID_PPV_ARGS(&g_device));
        
        if (SUCCEEDED(hr)) {
            g_featureLevel = level;
            std::cout << "Dispositivo creado con feature level: " << (int)level << std::endl;
            break;
        }
    }
    
    if (FAILED(hr)) {
        throw std::runtime_error("No se pudo crear el dispositivo DirectX 12");
    }
    
#ifdef _DEBUG
    // Habilitar capa de depuración
    ComPtr<ID3D12InfoQueue> infoQueue;
    if (SUCCEEDED(g_device.As(&infoQueue))) {
        infoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_CORRUPTION, TRUE);
        infoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_ERROR, TRUE);
        infoQueue->SetBreakOnSeverity(D3D12_MESSAGE_SEVERITY_WARNING, TRUE);
        
        // Filtrar mensajes de depuración
        D3D12_MESSAGE_SEVERITY severities[] = {
            D3D12_MESSAGE_SEVERITY_INFO
        };
        
        D3D12_INFO_QUEUE_FILTER filter = {};
        filter.DenyList.NumSeverities = _countof(severities);
        filter.DenyList.pSeverityList = severities;
        infoQueue->PushStorageFilter(&filter);
    }
#endif
}

// ═══════════════════════════════════════════════════════════════════════════
// CREAR COLA DE COMANDOS
// ═══════════════════════════════════════════════════════════════════════════

void CreateCommandQueue() {
    D3D12_COMMAND_QUEUE_DESC queueDesc = {};
    queueDesc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
    queueDesc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    
    DX_CHECK(g_device->CreateCommandQueue(&queueDesc, IID_PPV_ARGS(&g_commandQueue)));
    
    std::cout << "Cola de comandos creada" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER DISPOSITIVO Y COLA DE COMANDOS
// ═══════════════════════════════════════════════════════════════════════════

ID3D12Device* GetDevice() {
    return g_device.Get();
}

ID3D12CommandQueue* GetCommandQueue() {
    return g_commandQueue.Get();
}

D3D_FEATURE_LEVEL GetFeatureLevel() {
    return g_featureLevel;
}

