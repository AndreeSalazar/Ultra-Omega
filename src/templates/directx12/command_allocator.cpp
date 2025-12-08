/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 COMMAND ALLOCATOR - Allocator de comandos
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: command_allocator.cpp
 * Descripción: Gestión de allocators para command lists
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include <iostream>

// Allocators por frame
ComPtr<ID3D12CommandAllocator> g_commandAllocators[BACK_BUFFER_COUNT];

// ═══════════════════════════════════════════════════════════════════════════
// CREAR COMMAND ALLOCATORS
// ═══════════════════════════════════════════════════════════════════════════

void CreateCommandAllocators() {
    for (UINT i = 0; i < BACK_BUFFER_COUNT; i++) {
        DX_CHECK(GetDevice()->CreateCommandAllocator(
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            IID_PPV_ARGS(&g_commandAllocators[i])));
    }
    
    std::cout << "Command Allocators creados (" << BACK_BUFFER_COUNT << ")" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER ALLOCATOR DEL FRAME ACTUAL
// ═══════════════════════════════════════════════════════════════════════════

ID3D12CommandAllocator* GetCurrentCommandAllocator(UINT frameIndex) {
    return g_commandAllocators[frameIndex].Get();
}

// ═══════════════════════════════════════════════════════════════════════════
// RESET ALLOCATOR
// ═══════════════════════════════════════════════════════════════════════════

void ResetCommandAllocator(UINT frameIndex) {
    DX_CHECK(g_commandAllocators[frameIndex]->Reset());
}

