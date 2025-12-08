/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 ADAPTER - Enumeración de adaptadores gráficos
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: adapter.cpp
 * Descripción: Enumeración y selección de adaptadores (GPUs) disponibles
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include <iostream>

// ═══════════════════════════════════════════════════════════════════════════
// ENUMERAR ADAPTADORES DISPONIBLES
// ═══════════════════════════════════════════════════════════════════════════

std::vector<AdapterInfo> EnumerateAdapters() {
    std::vector<AdapterInfo> adapters;
    
    ComPtr<IDXGIFactory7> factory;
    DX_CHECK(CreateDXGIFactory2(
#ifdef _DEBUG
        DXGI_CREATE_FACTORY_DEBUG,
#else
        0,
#endif
        IID_PPV_ARGS(&factory)));
    
    ComPtr<IDXGIAdapter4> adapter;
    for (UINT adapterIndex = 0; 
         DXGI_ERROR_NOT_FOUND != factory->EnumAdapterByGpuPreference(
             adapterIndex,
             DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
             IID_PPV_ARGS(&adapter));
         ++adapterIndex) {
        
        AdapterInfo info;
        info.adapter = adapter;
        
        DXGI_ADAPTER_DESC3 desc;
        adapter->GetDesc3(&desc);
        info.desc = desc;
        
        // Verificar si es hardware (no es software)
        info.isHardware = (desc.Flags & DXGI_ADAPTER_FLAG3_SOFTWARE) == 0;
        
        // Verificar soporte DirectX 12
        info.supportsDx12 = false;
        for (const auto& level : FEATURE_LEVELS) {
            if (SUCCEEDED(D3D12CreateDevice(adapter.Get(), level, _uuidof(ID3D12Device), nullptr))) {
                info.supportsDx12 = true;
                break;
            }
        }
        
        adapters.push_back(info);
    }
    
    return adapters;
}

// ═══════════════════════════════════════════════════════════════════════════
// SELECCIONAR MEJOR ADAPTADOR
// ═══════════════════════════════════════════════════════════════════════════

ComPtr<IDXGIAdapter4> SelectBestAdapter() {
    auto adapters = EnumerateAdapters();
    
    if (adapters.empty()) {
        throw std::runtime_error("No se encontraron adaptadores gráficos");
    }
    
    // Priorizar adaptadores de hardware con soporte DX12
    for (const auto& info : adapters) {
        if (info.isHardware && info.supportsDx12) {
            std::wcout << L"Adaptador seleccionado: " << info.desc.Description << std::endl;
            return info.adapter;
        }
    }
    
    // Fallback: primer adaptador disponible
    std::wcout << L"Usando adaptador: " << adapters[0].desc.Description << std::endl;
    return adapters[0].adapter;
}

// ═══════════════════════════════════════════════════════════════════════════
// IMPRIMIR INFORMACIÓN DE ADAPTADORES
// ═══════════════════════════════════════════════════════════════════════════

void PrintAdapterInfo() {
    auto adapters = EnumerateAdapters();
    
    std::cout << "=== Adaptadores Disponibles ===" << std::endl;
    for (size_t i = 0; i < adapters.size(); ++i) {
        const auto& info = adapters[i];
        std::wcout << "[" << i << "] " << info.desc.Description << std::endl;
        std::cout << "  - Hardware: " << (info.isHardware ? "Sí" : "No") << std::endl;
        std::cout << "  - Soporte DX12: " << (info.supportsDx12 ? "Sí" : "No") << std::endl;
        std::cout << "  - Memoria: " << (info.desc.DedicatedVideoMemory / (1024 * 1024)) << " MB" << std::endl;
    }
}

