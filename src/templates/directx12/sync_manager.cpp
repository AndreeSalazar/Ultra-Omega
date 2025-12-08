/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 SYNC MANAGER - Gestión de sincronización (fences)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: sync_manager.cpp
 * Descripción: Gestión de fences y sincronización CPU-GPU
 * 
 * USO: Este nodo puede ser heredado para gestión de sincronización
 * Ejemplo: Conecta este nodo a render_loop.cpp para usar sync_manager con ch()
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"

// ═══════════════════════════════════════════════════════════════════════════
// GESTIÓN DE FENCES
// ═══════════════════════════════════════════════════════════════════════════

struct FenceManager {
    ComPtr<ID3D12Fence> fence;
    UINT64 fenceValues[BACK_BUFFER_COUNT];
    HANDLE fenceEvent;
    UINT64 currentValue;
    
    bool Initialize(ID3D12Device* device) {
        for (UINT i = 0; i < BACK_BUFFER_COUNT; i++) {
            fenceValues[i] = 0;
        }
        currentValue = 1;
        
        if (FAILED(device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&fence)))) {
            return false;
        }
        
        fenceEvent = CreateEvent(nullptr, FALSE, FALSE, nullptr);
        if (fenceEvent == nullptr) {
            return false;
        }
        
        return true;
    }
    
    void WaitForGPU(ID3D12CommandQueue* queue, UINT frameIndex) {
        // Señalar el fence
        UINT64 fenceValue = fenceValues[frameIndex];
        if (SUCCEEDED(queue->Signal(fence.Get(), fenceValue))) {
            // Esperar hasta que el GPU complete hasta este valor
            if (fence->GetCompletedValue() < fenceValue) {
                if (SUCCEEDED(fence->SetEventOnCompletion(fenceValue, fenceEvent))) {
                    WaitForSingleObject(fenceEvent, INFINITE);
                }
            }
            fenceValues[frameIndex] = fenceValue + 1;
        }
    }
    
    void WaitForFrame(ID3D12CommandQueue* queue, UINT frameIndex) {
        WaitForGPU(queue, frameIndex);
    }
    
    void Cleanup() {
        if (fenceEvent) {
            CloseHandle(fenceEvent);
            fenceEvent = nullptr;
        }
        fence.Reset();
    }
};

// ═══════════════════════════════════════════════════════════════════════════
// SINGLETON GLOBAL (opcional, puede ser instancia en main)
// ═══════════════════════════════════════════════════════════════════════════

static FenceManager* g_fenceManager = nullptr;

FenceManager* GetFenceManager() {
    return g_fenceManager;
}

void SetFenceManager(FenceManager* manager) {
    g_fenceManager = manager;
}

