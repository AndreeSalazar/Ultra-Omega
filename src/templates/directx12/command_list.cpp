/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 COMMAND LIST - Lista de comandos de renderizado
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: command_list.cpp
 * Descripción: Creación y manejo de command lists
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include "command_allocator.cpp"
#include "root_signature.cpp"
#include "pipeline_state.cpp"
#include <iostream>

// Command list principal
ComPtr<ID3D12GraphicsCommandList> g_commandList;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR COMMAND LIST
// ═══════════════════════════════════════════════════════════════════════════

void CreateCommandList() {
    DX_CHECK(GetDevice()->CreateCommandList(
        0,
        D3D12_COMMAND_LIST_TYPE_DIRECT,
        g_commandAllocators[0].Get(),
        GetPipelineState(),
        IID_PPV_ARGS(&g_commandList)));
    
    // Empezar en estado cerrado (closed)
    g_commandList->Close();
    
    std::cout << "Command List creada" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// OBTENER COMMAND LIST
// ═══════════════════════════════════════════════════════════════════════════

ID3D12GraphicsCommandList* GetCommandList() {
    return g_commandList.Get();
}

// ═══════════════════════════════════════════════════════════════════════════
// RESET COMMAND LIST
// ═══════════════════════════════════════════════════════════════════════════

void ResetCommandList(UINT frameIndex) {
    DX_CHECK(ResetCommandAllocator(frameIndex));
    DX_CHECK(g_commandList->Reset(
        GetCurrentCommandAllocator(frameIndex),
        GetPipelineState()));
}

// ═══════════════════════════════════════════════════════════════════════════
// EJECUTAR Y CERRAR COMMAND LIST
// ═══════════════════════════════════════════════════════════════════════════

void ExecuteCommandList() {
    DX_CHECK(g_commandList->Close());
    
    ID3D12CommandList* ppCommandLists[] = { g_commandList.Get() };
    GetCommandQueue()->ExecuteCommandLists(_countof(ppCommandLists), ppCommandLists);
}

