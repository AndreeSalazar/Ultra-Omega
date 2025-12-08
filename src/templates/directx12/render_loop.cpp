/* ═══════════════════════════════════════════════════════════════════════════
 * DIRECTX12 RENDER LOOP - Loop principal de renderizado
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: render_loop.cpp
 * Descripción: Loop principal de renderizado con sincronización
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "directx12_types.h"
#include "device.cpp"
#include "swapchain.cpp"
#include "command_list.cpp"
#include "buffers.cpp"
#include <iostream>

// Fences para sincronización
ComPtr<ID3D12Fence> g_fence;
UINT64 g_fenceValue[BACK_BUFFER_COUNT];
HANDLE g_fenceEvent;

// ═══════════════════════════════════════════════════════════════════════════
// CREAR FENCES
// ═══════════════════════════════════════════════════════════════════════════

void CreateFences() {
    for (UINT i = 0; i < BACK_BUFFER_COUNT; i++) {
        g_fenceValue[i] = 0;
    }
    
    DX_CHECK(GetDevice()->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&g_fence)));
    g_fenceValue[0] = 1;
    
    // Crear evento para esperar
    g_fenceEvent = CreateEvent(nullptr, FALSE, FALSE, nullptr);
    if (g_fenceEvent == nullptr) {
        DX_CHECK(HRESULT_FROM_WIN32(GetLastError()));
    }
    
    std::cout << "Fences creados" << std::endl;
}

// ═══════════════════════════════════════════════════════════════════════════
// ESPERAR AL FRAME ANTERIOR
// ═══════════════════════════════════════════════════════════════════════════

void WaitForPreviousFrame(UINT frameIndex) {
    // Si el frame anterior aún no ha terminado, esperar
    if (g_fence->GetCompletedValue() < g_fenceValue[frameIndex]) {
        DX_CHECK(g_fence->SetEventOnCompletion(g_fenceValue[frameIndex], g_fenceEvent));
        WaitForSingleObject(g_fenceEvent, INFINITE);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RENDERIZAR UN FRAME
// ═══════════════════════════════════════════════════════════════════════════

void RenderFrame() {
    UINT frameIndex = GetCurrentFrameIndex();
    
    // Esperar al frame anterior
    WaitForPreviousFrame(frameIndex);
    
    // Reset command list
    ResetCommandList(frameIndex);
    
    // Actualizar constant buffer (ejemplo)
    FrameConstants constants = {};
    // ... inicializar constantes ...
    UpdateConstantBuffer(frameIndex, constants);
    
    // Configurar viewport y scissor
    D3D12_VIEWPORT viewport = {};
    viewport.Width = (float)WIDTH;
    viewport.Height = (float)HEIGHT;
    viewport.MaxDepth = 1.0f;
    
    D3D12_RECT scissorRect = {};
    scissorRect.right = (LONG)WIDTH;
    scissorRect.bottom = (LONG)HEIGHT;
    
    ID3D12GraphicsCommandList* commandList = GetCommandList();
    
    // Set pipeline state
    commandList->SetPipelineState(GetPipelineState());
    commandList->SetGraphicsRootSignature(GetRootSignature());
    
    // Set descriptor heaps
    // ID3D12DescriptorHeap* heaps[] = { ... };
    // commandList->SetDescriptorHeaps(_countof(heaps), heaps);
    
    // Set root descriptor (constant buffer)
    commandList->SetGraphicsRootConstantBufferView(0, GetConstantBufferGpuAddress(frameIndex));
    
    // Set viewport y scissor
    commandList->RSSetViewports(1, &viewport);
    commandList->RSSetScissorRects(1, &scissorRect);
    
    // Transición del render target a render target state
    CD3DX12_RESOURCE_BARRIER barrier = CD3DX12_RESOURCE_BARRIER::Transition(
        GetCurrentRenderTarget(),
        D3D12_RESOURCE_STATE_PRESENT,
        D3D12_RESOURCE_STATE_RENDER_TARGET);
    commandList->ResourceBarrier(1, &barrier);
    
    // Obtener handle del RTV
    CD3DX12_CPU_DESCRIPTOR_HANDLE rtvHandle = GetCurrentRTVHandle();
    
    // Limpiar render target
    const float clearColor[] = { 0.0f, 0.2f, 0.4f, 1.0f };
    commandList->ClearRenderTargetView(rtvHandle, clearColor, 0, nullptr);
    
    // Set render target
    commandList->OMSetRenderTargets(1, &rtvHandle, FALSE, nullptr);
    
    // Set vertex e index buffers
    commandList->IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    commandList->IASetVertexBuffers(0, 1, GetVertexBufferView());
    commandList->IASetIndexBuffer(GetIndexBufferView());
    
    // Dibujar
    commandList->DrawIndexedInstanced(6, 1, 0, 0, 0); // Ejemplo: 6 índices
    
    // Transición de vuelta a present state
    barrier = CD3DX12_RESOURCE_BARRIER::Transition(
        GetCurrentRenderTarget(),
        D3D12_RESOURCE_STATE_RENDER_TARGET,
        D3D12_RESOURCE_STATE_PRESENT);
    commandList->ResourceBarrier(1, &barrier);
    
    // Cerrar y ejecutar command list
    ExecuteCommandList();
    
    // Present
    PresentSwapChain();
    
    // Señalar fence
    g_fenceValue[frameIndex]++;
    DX_CHECK(GetCommandQueue()->Signal(g_fence.Get(), g_fenceValue[frameIndex]));
}

